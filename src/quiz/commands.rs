use std::time::Duration;

use crate::utils::models::*;
use crate::{Context, Error};
use poise::{
    serenity_prelude::{self as serenity, futures::StreamExt},
    ReplyHandle,
};
use tokio::time::sleep;
use tracing::info;

use super::unpack::unpack_quiz;
const DEFAULT_WAIT_TIME: u64 = 10;
const INFORM: u64 = 3;
const TIMEOUT: u64 = 3600; // 1 hour

/// Start a quiz competition
#[poise::command(slash_command)]
pub async fn quiz(ctx: Context<'_>) -> Result<(), Error> {
    let msg = ctx
        .send(|s| {
            s.reply(true)
                .components(|c| {
                    c.create_action_row(|a| {
                        a.create_button(|b| {
                            b.custom_id("solo")
                                .style(serenity::ButtonStyle::Primary)
                                .label("Solo")
                        })
                        .create_button(|b| {
                            b.custom_id("compete")
                                .style(serenity::ButtonStyle::Primary)
                                .label("Competitive")
                                .disabled(true)
                        })
                    })
                })
                .embed(|e| e.title("Quiz").description("Choose your quiz mode"))
        })
        .await?;
    let resp = msg.clone().into_message().await?;
    let cib = resp
        .await_component_interactions(&ctx.serenity_context().shard)
        .timeout(std::time::Duration::from_secs(TIMEOUT));
    let mut cic = cib.build();
    while let Some(mci) = &cic.next().await {
        match mci.data.custom_id.as_str() {
            "solo" => {
                mci.defer(&ctx.http()).await?;
                return solo(&ctx, &msg).await;
            }
            "compete" => {
                mci.defer(&ctx.http()).await?;
            }
            _ => {}
        }
    }
    Ok(())
}

async fn solo(ctx: &Context<'_>, msg: &ReplyHandle<'_>) -> Result<(), Error> {
    let questions = match unpack_quiz(){
        Ok(questions) => questions,
        Err(e) => {
            info!("Error: {}", e);
            msg.edit(*ctx,|s|
                s.content("Error: Failed to unpack quiz file")).await?;
            return Err(e);
        }
    };
    msg.edit(*ctx, |e| {
        e.embed(|e| {
            e.title("Quiz").description(format!(
                r#"
                There are a total of {} questions in the quiz.
                Every question has 4 or 5 choices.
                Please select the most suitable one
                For each correct question, you will get 1 point
                If you don't know the answer, please choose a random option to continue to the next question
                Press the button below to start the quiz
                "#, questions.len())
            )
        })
        .components(|c| {
            c.create_action_row(|a| {
                a.create_button(|b| {
                    b.custom_id("start")
                        .style(serenity::ButtonStyle::Primary)
                        .label("Start")
                })
            })
        })
    })
    .await?;
    let mut game = Game::new(
        questions,
        Mode::SinglePlayer(Player::new(ctx.author().clone())),
    );
    let mut resp = msg
        .clone()
        .into_message()
        .await?
        .await_component_interactions(&ctx.serenity_context().shard)
        .author_id(ctx.author().id)
        .timeout(std::time::Duration::from_secs(TIMEOUT))
        .build();
    
    while let Some(mci) = &resp.next().await {
        if mci.data.custom_id.as_str() == "start" {
            mci.defer(&ctx.http()).await?;
            break;
        }
    }
    loop {
        prompt(
            ctx,
            msg,
            game.get_content(),
            format!("Your score is {}", game.get_player_score(None)),
            game.index as i32,
        )
        .await?;
        if let Some(mci) = &resp.next().await {
            match mci.data.custom_id.as_str() {
                "0" | "1" | "2" | "3" | "4" => {
                    mci.defer(ctx.http()).await?;
                    let option = &game.get_content().choices
                        [mci.data.custom_id.parse::<usize>().unwrap_or(0)];
                    if option.value {
                        inform(ctx, msg, true, option.content.clone()).await?;
                        game.update_score(None);
                    } else {
                        let correct_option =
                            game.get_content().get_correct_choice().content.clone();
                        inform(ctx, msg, false, correct_option).await?;
                    }
                }
                "giveup" => {
                    mci.defer(ctx.http()).await?;
                    return result(ctx, msg, &game).await;
                }
                _ => {}
            }
            sleep(Duration::from_secs(INFORM)).await;
            if !game.next() {
                break;
            }
        }
    }

    result(ctx, msg, &game).await
}

async fn prompt(
    ctx: &Context<'_>,
    msg: &ReplyHandle<'_>,
    question: &Question,
    info: String,
    index: i32,
) -> Result<(), Error> {
    let choices = question.get_options();
    msg.edit(*ctx, |e| {
        e.embed(|e| {
            e.title(format!("Question {}", index + 1))
                .description(format!("**{}**", question.question))
                .fields(choices.clone())
                .footer(|f| f.text(info))
        })
        .components(|c| {
            c.create_action_row(|a| {
                for (i, choice) in choices.iter().enumerate() {
                    a.create_button(|b| {
                        b.custom_id(i.to_string())
                            .style(serenity::ButtonStyle::Primary)
                            .label(format!("{}", choice.0.chars().next().unwrap()))
                    });
                }
                a
            })
            .create_action_row(|a| {
                a.create_button(|b| {
                    b.custom_id("giveup")
                        .style(serenity::ButtonStyle::Danger)
                        .label("End the game")
                })
            })
        })
    })
    .await?;
    Ok(())
}
async fn inform(
    ctx: &Context<'_>,
    msg: &ReplyHandle<'_>,
    correct: bool,
    correct_option: String,
) -> Result<(), Error> {
    if correct {
        msg.edit(*ctx, |e| {
            e.embed(|e| {
                e.title("Correct! You gain 1 point!")
                    .description(format!("Correct answer: {correct_option}"))
            })
            .components(|c| c)
        })
        .await?;
    } else {
        msg.edit(*ctx, |e| {
            e.embed(|e| {
                e.title("Incorrect!")
                    .description(format!("Correct answer: {correct_option}"))
            })
            .components(|c| c)
        })
        .await?;
    }
    Ok(())
}
async fn result(ctx: &Context<'_>, msg: &ReplyHandle<'_>, game: &Game) -> Result<(), Error> {
    let list = game.get_final_result();
    msg.edit(*ctx, |e| {
        e.components(|c| c).embed(|e| {
            e.title("Result")
                .description(format!(
                    "Thanks for playing\nMaximum score: {}",
                    game.content.len()
                ))
                .fields(list)
        })
    })
    .await?;
    Ok(())
}

use poise::{
    serenity_prelude::{self as serenity, GatewayIntents},
    Event, FrameworkError,
};
use std::{fs::File, sync::Arc};
use tracing::{error, info, instrument, trace};
use tracing_subscriber::{filter, prelude::*};
mod quiz;
mod utils;
pub struct Data {} 
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Unable to load the .env file. Check if it has been created.");

    if let Err(e) = create_subscriber() {
        // Change to a panic!() if you really need logging to work
        println!("Unable to create subscriber: {}", e);
    }

    if let Err(e) = run().await {
        panic!("Error trying to run the bot: {}", e);
    }
}

#[instrument]
async fn run() -> Result<(), Error> {
    // A list of commands to register. Remember to add the function for the command in this vec, otherwise it won't appear in the command list.
    let commands = vec![quiz::commands::quiz()];

    let token = std::env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN is not set. Set it as an environment variable.");
    info!("Setting up the bot...");
    info!("Generating options");
    let options = poise::FrameworkOptions {
        commands,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                if let Event::Ready { data_about_bot } = event {
                    let bot_name = data_about_bot.user.name.to_owned();
                    info!("{username} is online", username = bot_name);
                    println!("{} is online!", bot_name);
                }

                Ok(())
            })
        },
        pre_command: |ctx| {
            Box::pin(async move {
                trace!(
                    "Executing command: {cmd_name}",
                    cmd_name = ctx.command().qualified_name
                );
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                trace!(
                    "Finished executing command: {cmd_name}",
                    cmd_name = ctx.command().qualified_name
                );
            })
        },
        //owners: std::collections::HashSet::from([UserId::from_str(&owner_id)?]),
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };
    info!("Options generated successfully!");

    info!("Generating framework...");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .initialize_owners(true)
        .options(options)
        .intents(
            GatewayIntents::non_privileged()
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::GUILD_MEMBERS,
        )
        .build()
        .await?;
    info!("Framework generated successfully!");

    let shard_manager = framework.shard_manager().clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register the ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    info!("Bot starting...");
    println!("Starting the bot...");
    framework.start().await?;

    Ok(())
}

fn create_subscriber() -> Result<(), Error> {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    let file = File::create("debug.log")?;
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));

    tracing_subscriber::registry()
        .with(
            stdout_log
                .with_filter(filter::LevelFilter::INFO)
                .and_then(debug_log),
        )
        .init();

    Ok(())
}

async fn on_error(error: FrameworkError<'_, Data, Error>) {
    match error {
        FrameworkError::Setup { error, .. } => {
            panic!("Failed to start the bot: {:?}", error);
        }
        FrameworkError::Command { error, ctx } => {
            error!(
                "Error executing command {} in guild: {}: {:?}",
                ctx.command().qualified_name,
                ctx.guild().unwrap().name,
                error
            );
        }
        FrameworkError::CommandCheckFailed { error, ctx } => {
            error!(
                "Error executing the pre-command check for {}{:?}",
                ctx.command().qualified_name,
                error
            );
        }
        FrameworkError::ArgumentParse { error, input, ctx } => {
            error!(
                "Error parsing arguments for {} with input(s) {:?}: {:?}",
                ctx.command().qualified_name,
                input,
                error
            );
        }
        FrameworkError::EventHandler {
            error,
            ctx: _,
            event: _,
            framework: _,
        } => {
            error!("Error executing event handler: {:?}", error);
        }
        FrameworkError::CommandPanic { payload, ctx } => {
            error!(
                "A command has panicked: {:?}",
                payload.unwrap_or_else(|| "Failed to get panic message.".to_string()),
            );
            ctx.say(
                "The command has failed, please contact the bot operators if the issue persists.",
            )
            .await
            .unwrap();
        }
        FrameworkError::CommandStructureMismatch {
            description,
            ctx: _,
        } => {
            error!(
                "The command's structure had a mismatch: {}.\n Most likely the command was updated but not reregistered on Discord. Try reregistering the command and try again",
                description
            );
        }
        FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
        } => {
            ctx.say(format!(
                "This command is still on cooldown. Try again in {} seconds",
                remaining_cooldown.as_secs()
            ))
            .await
            .unwrap();
        }
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
        } => {
            error!(
                "The bot lacks the following permissions to complete the task: {:?}",
                missing_permissions
            );
            ctx.say("I do not have the required permissions to do this, sorry :(")
                .await
                .unwrap();
        }
        FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
        } => {
            info!(
                "The user requires the following permissions to complete the task: {:?}",
                missing_permissions
            );
            ctx.say("You do not have the required permissions to do this, sorry :(")
                .await
                .unwrap();
        }
        FrameworkError::NotAnOwner { ctx } => {
            ctx.say("This command is only available to the bot owners.")
                .await
                .unwrap();
        }
        FrameworkError::GuildOnly { ctx } => {
            ctx.say("This command is only available in a Discord server.")
                .await
                .unwrap();
        }
        FrameworkError::DmOnly { ctx } => {
            ctx.say("This command is only available in a DM.")
                .await
                .unwrap();
        }
        FrameworkError::NsfwOnly { ctx } => {
            ctx.say("This command is only available in a NSFW channel.")
                .await
                .unwrap();
        }
        FrameworkError::DynamicPrefix { error, ctx: _, msg } => {
            error!(
                "A dynamic prefix error occured for message \"{:?}\": {:?}",
                msg, error
            );
        }
        FrameworkError::UnknownCommand {
            ctx: _,
            msg: _,
            prefix,
            msg_content,
            framework: _,
            invocation_data: _,
            trigger: _,
        } => {
            info!("A user tried to trigger an unknown command with the bot's prefix of {} with the message: {}", prefix, msg_content);
        }
        FrameworkError::UnknownInteraction {
            ctx: _,
            framework: _,
            interaction,
        } => {
            error!(
                "An interaction occured that was unspecified: {:?}",
                interaction
            );
        }
        _ => {
            error!("An unknown error occurred!");
        }
    }
}

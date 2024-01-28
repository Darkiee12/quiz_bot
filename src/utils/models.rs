use poise::serenity_prelude as serenity;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Question {
    pub question: String,
    kind: String,
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub content: String,
    pub value: bool,
}

pub struct Player{
    pub name: serenity::User,
    pub score: i32,
    pub answered: bool,
}
pub struct Game{
    pub content: Vec<Question>,
    pub index: usize,
    pub mode: Mode,
}

pub enum Mode {
    SinglePlayer(Player),
    MultiplePlayers(Vec<Player>),
}

impl Game {
    pub fn new(content: Vec<Question>, mode: Mode) -> Self {
        Self {
            content,
            index: 0,
            mode,
        }
    }

    pub fn add_player(&mut self, player: &serenity::User ) {
        match &mut self.mode {
            Mode::SinglePlayer(p) => {
                *p = Player::new(player.clone());
            }
            Mode::MultiplePlayers(players) => {
                // Add the player to the existing vector of players
                players.push(Player::new(player.clone()));
            }
        }
    }

    pub fn get_player(&self, player: &serenity::User) -> Option<&Player> {
        match &self.mode {
            Mode::SinglePlayer(existing_player) => {
                if &existing_player.name == player {
                    Some(existing_player)
                } else {
                    None
                }
            }
            Mode::MultiplePlayers(players) => {
                players.iter().find(|p| &p.name == player)
            }
        }
    }

    pub fn player_exists(&self, player: &serenity::User) -> bool {
        match &self.mode {
            Mode::SinglePlayer(existing_player) => &existing_player.name == player,
            Mode::MultiplePlayers(players) => players.iter().any(|p| &p.name == player),
        }
    }

    pub fn get_content(&self) -> &Question {
        &self.content[self.index]
    }

    pub fn get_final_result(&self) -> Vec<(String, &str, bool)> {
        match &self.mode {
            Mode::SinglePlayer(player) => vec![(format!("{}: {}", player.name, player.score), "", false)],
            Mode::MultiplePlayers(players) => players
                .iter()
                .map(|p| (format!("{}: {}", p.name, p.score), "", false))
                .collect(),
        }
    }
    
    pub fn get_player_score(&self, player: Option<&serenity::User>) -> i32 {
        match &self.mode {
            Mode::SinglePlayer(p) => {
                p.score
            }
            Mode::MultiplePlayers(players) => {
                players.iter().find(|p| &p.name == player.unwrap()).map_or(0, |p| p.score)
            }
        }
    }
    
    pub fn update_score(&mut self, player: Option<&serenity::User>) {
        match &mut self.mode {
            Mode::SinglePlayer(p) => {
                p.update_score();
            }
            Mode::MultiplePlayers(players) => {
                if let Some(p) = players.iter_mut().find(|p| &p.name == player.unwrap()) {
                    p.update_score();
                }
            }
        }
    }

    pub fn next(&mut self) -> bool {
        self.index += 1;
        self.index < self.content.len()
    }
}

impl Question {
    pub fn get_options(&self) -> Vec<(String, &str, bool)> {
        self.choices
            .iter()
            .enumerate()
            .map(|(index, c)| {
                let letter = (b'A' + index as u8) as char;
                (format!("{} {}", letter, c.content.clone()), "", false)
            })
            .collect()
    }
    pub fn get_correct_choice(&self) -> &Choice {
        // Assuming there is at least one correct choice
        self.choices.iter().find(|&choice| choice.value).unwrap()
    }
    
}

impl Player {
    pub fn new(name: serenity::User) -> Self {
        Self {
            name,
            score: 0,
            answered: false,
        }
    }

    pub fn update_score(&mut self) {
        self.score += 1;
        self.answered = true;
    }

    pub fn reset_answer(&mut self) {
        self.answered = false;
    }
}

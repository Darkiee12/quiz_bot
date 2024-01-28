use crate::utils::models::Question;
use crate::Error;
use rand::seq::SliceRandom;
use serde_json;
use std::io::Read;

pub fn unpack_quiz() -> Result<Vec<Question>, Error> {
    let quiz_path = "./quiz.json";
    let mut quiz_file = std::fs::File::open(quiz_path)?;
    let mut contents = String::new();
    quiz_file.read_to_string(&mut contents)?;

    let mut quiz: Vec<Question> = serde_json::from_str(&contents)?;
    let mut rng = rand::thread_rng();
    quiz.shuffle(&mut rng);
    Ok(quiz)
}

use anyhow::Result;

mod logic;
use logic::difficulty;
use logic::engine::game;

fn main() -> Result<()> {
    let difficulty_level = difficulty::choose_difficulty()?;
    game(difficulty_level);

    Ok(())
}

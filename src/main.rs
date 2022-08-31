use tactic::{ui::UI, game_outcome::GameOutcome, player_type::PlayerType};

fn main() -> crossterm::Result<()>
{
    let mut ui = UI::new(PlayerType::Human, PlayerType::AI)?;

    let game_outcome = ui.game_loop()?;
    let game_board = ui.take_game_board();

    println!("{}", game_board);
    match game_outcome {
        GameOutcome::Incomplete => {
           println!("Game exited"); 
        },
        GameOutcome::Draw => {
            println!("Result: Draw");
        },
        GameOutcome::PlayerO(_) => {
            println!("Result: O wins!");
        },
        GameOutcome::PlayerX(_) => {
            println!("Result: X wins!");
        }
    }
    Ok(())
}

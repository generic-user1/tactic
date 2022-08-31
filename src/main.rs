use tactic::{ui::UI, game_outcome::GameOutcome};

fn main() -> crossterm::Result<()>
{
    let ui = UI::default();

    let (game_outcome, game_board) = ui.game_loop()?;
    
    drop(ui);

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

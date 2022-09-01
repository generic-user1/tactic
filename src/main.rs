use tactic::{
    ui::UI, 
    game_outcome::GameOutcome, 
    player_type::PlayerType, 
    active_player::ActivePlayer,
    ai::AiPlayer
};

fn main() -> crossterm::Result<()>
{
    let player_x = PlayerType::Human;
    let player_o = PlayerType::AI(AiPlayer::new(0.5));

    let mut ui = UI::new(player_x, player_o)?;

    loop {
        let game_outcome = ui.game_loop()?;
        if game_outcome == GameOutcome::Incomplete || !ui.play_again_menu()? {
            break;
        } else {
            match game_outcome {
                GameOutcome::PlayerX(_) => {*ui.active_player_mut() = ActivePlayer::PlayerO},
                GameOutcome::PlayerO(_) => {*ui.active_player_mut() = ActivePlayer::PlayerX},
                _ => {
                    // do nothing if neither player won
                    // the active player will flip-flop naturally
                }
            }
        }
    };

    let player_x_score = ui.player_x_score();
    let player_o_score = ui.player_o_score();
    let number_of_draws = ui.number_of_draws();
    let number_of_games = ui.number_of_games();
    let final_game_board = ui.take_game_board();

    println!("Final board: {}", final_game_board);
    println!("X score:     {}\t({:.2}%)", player_x_score, 
        if number_of_games != 0 {
            ((player_x_score as f64)/(number_of_games as f64))*100.0
        } else {
            0.0
        });
    println!("O score:     {}\t({:.2}%)", player_o_score, 
        if number_of_games != 0 {
            ((player_o_score as f64)/(number_of_games as f64))*100.0
        } else {
            0.0
        });
    println!("Draws:       {}\t({:.2}%)", number_of_draws,
        if number_of_games != 0 {
            ((number_of_draws as f64)/(number_of_games as f64))*100.0
        } else {
            0.0    
        });
    println!("Total Games: {}", number_of_games);
    
    Ok(())
}

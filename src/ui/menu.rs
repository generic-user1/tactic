//! UI implementations for menus

use std::io::{stdout, Write};
use crossterm::{
    style::Print,
    cursor::{self, MoveToColumn, MoveToRow, MoveToNextLine},
    terminal::{Clear, ClearType},
    event::{self, Event, KeyEvent, KeyCode, KeyModifiers},
    QueueableCommand,
    ExecutableCommand
};

use crate::game_outcome::GameOutcome;

impl super::UI{
    
    /// The post-game menu 
    /// 
    /// Allows user to view score, the results of the previous game, 
    /// and choose whether to play another game.
    /// 
    /// Returns `true` if user chooses to play another game, `false` otherwise
    pub fn play_again_menu(&mut self) -> crossterm::Result<bool>
    {
        self.draw_play_again_menu()?;

        //loop until a valid event is read
        let play_again = loop {
            match event::read()?{
                Event::Key(key_event) => {
                    match key_event {
                        KeyEvent{code:KeyCode::Char('y'), ..} => {
                            break true;
                        },
                        KeyEvent{code:KeyCode::Enter, ..} => {
                            break true;
                        }
                        KeyEvent{code:KeyCode::Char('n'), ..} => {
                            break false;
                        },
                        KeyEvent{code:KeyCode::Char('q'), ..} => {
                            break false;
                        },
                        KeyEvent{code:KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, ..} => {
                            break false;
                        },
                        _ => {
                            //ignore other key events
                        }
                    }
                },
                //redraw screen upon resize
                Event::Resize(_, _) => {
                    self.draw_play_again_menu()?;
                },
                _ => {
                    //ignore other type of event
                }
            }
        };

        stdout().execute(cursor::Show)?;
        Ok(play_again)
    }

    /// Draws the play again menu
    fn draw_play_again_menu(&self) -> crossterm::Result<()>
    {
        stdout()
            .queue(Clear(ClearType::All))?
            .queue(cursor::Hide)?
            .queue(MoveToColumn(0))?
            .queue(MoveToRow(0))?
            .flush()?;

        self.draw_game()?;

        let game_outcome_text = match self.game_board.game_outcome(){
            GameOutcome::PlayerX(_) => "Player X wins!",
            GameOutcome::PlayerO(_) => "Player O wins!",
            GameOutcome::Draw => "Draw!",
            GameOutcome::Incomplete => "Game finished early!"
        };
        let player_x_score = self.player_x_score();
        let player_o_score = self.player_o_score();
        let number_of_draws = self.number_of_draws();
        let number_of_games = self.number_of_games();
        stdout()
            .queue(MoveToRow(5))?
            .queue(Print(game_outcome_text))?
            
            .queue(MoveToNextLine(1))?
            .queue(MoveToColumn(0))?
            .queue(Print(format!("X score:     {}\t({:.2}%)", player_x_score, 
                    ((player_x_score as f64)/(number_of_games as f64))*100.0)))?
            
            .queue(MoveToNextLine(1))?
            .queue(MoveToColumn(0))?
            .queue(Print(format!("O score:     {}\t({:.2}%)", player_o_score, 
                ((player_o_score as f64)/(number_of_games as f64))*100.0)))?

            .queue(MoveToNextLine(1))?
            .queue(MoveToColumn(0))?
            .queue(Print(format!("Draws:       {}\t({:.2}%)", number_of_draws,
                ((number_of_draws as f64)/(number_of_games as f64))*100.0)))?

            .queue(MoveToNextLine(1))?
            .queue(MoveToColumn(0))?
            .queue(Print(format!("Total games: {}", number_of_games)))?
            
            .queue(MoveToNextLine(2))?
            .queue(MoveToColumn(0))?
            .queue(Print("Play again? Press y for yes or n for no"))?
            .flush()?;
        Ok(())
    }
}
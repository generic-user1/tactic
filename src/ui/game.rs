//! UI implementations for playing games of tic-tac-toe

use std::io::{stdout, Write};

use crossterm::{
    terminal::{self, Clear, ClearType},
    style::{Print, StyledContent, ContentStyle, Stylize},
    cursor::{self, MoveToColumn, MoveToRow, MoveToNextLine},
    QueueableCommand,
    ExecutableCommand
};

use crate::{
    game_outcome::{GameOutcome, WinPosition},
    gameboard::{GameBoard, BoardSpaceLocation},
    player_type::PlayerType,
    ai::AiError, 
    game_settings::GameMode
};

impl super::UI{
    /// The main game loop
    ///
    /// Allows player X to claim one space, then allows player O to claim one space.
    /// Continues alternating between players until either the game is finished or a user
    /// quits the game.
    pub fn game_loop(&mut self) -> crossterm::Result<GameOutcome>
    {
        //update terminal size
        (self.terminal_x_size, self.terminal_y_size) = terminal::size()?;
        
        self.reset_cursor_pos();

        self.game_board = GameBoard::new();
        let mut game_outcome = self.game_board.game_outcome();
        
        stdout().execute(Clear(ClearType::All))?;

        // keep playing game until game outcome is finished 
        // or exit flag is set (because user chose to quit)
        while !(game_outcome.game_finished() || self.exit_flag){
            stdout()
                //hide the cursor while drawing game board
                .queue(cursor::Hide)?
                .queue(MoveToColumn(0))?
                .queue(MoveToRow(0))?
                .flush()?;

            // only print game board if terminal is large enough
            if self.terminal_x_size >= Self::TERMSIZE_MIN_X && self.terminal_y_size >= Self::TERMSIZE_MIN_Y {
                self.draw_game(None)?;
                stdout()
                    .queue(MoveToRow(6))?
                    .queue(Print(format!("{}'s turn", self.active_player.get_char())))?
                    .queue(MoveToRow(7))?.queue(MoveToColumn(0))?
                    .queue(Print(format!(
                        "Use arrow keys to select space. Press 'Enter' or '{}' to place. Press q to quit.",
                        self.active_player.get_char()
                    )))?
                    // position cursor in the appropriate space
                    .queue(MoveToColumn(((self.cursor_x_pos as u16) * 4) + 1))?
                    .queue(MoveToRow((self.cursor_y_pos as u16) * 2))?
                    // show the cursor again
                    .queue(cursor::Show)?

                    .flush()?;
            } else {
                // print error message instead of game board if terminal is too small
                stdout()
                    .execute(Print("Terminal too small! Please enlarge terminal"))?;
            }

            match self.active_player_type() {
                PlayerType::Human => self.handle_next_event()?,
                PlayerType::AI(ai_player) => {
                    match ai_player.do_turn(&self.game_board, &self.active_player){
                        Ok(new_board) =>{
                            self.game_board = new_board;
                            self.switch_active_player();
                        },
                        Err(ai_error) => {
                            if ai_error == AiError::NoMovesFound{
                                panic!("No moves found despite game not being finished");
                            }
                        }
                    }
                }
            }

            game_outcome = self.game_board.game_outcome();
        }

        match game_outcome {
            GameOutcome::PlayerX(_) => {
                match self.game_mode {
                    GameMode::Classic => {
                        self.player_x_score +=1;
                    },
                    GameMode::Reverse => {
                        self.player_o_score +=1;
                    }
                }
                
            },
            GameOutcome::PlayerO(_) => {
                match self.game_mode {
                    GameMode::Classic => {
                        self.player_o_score +=1;
                    },
                    GameMode::Reverse => {
                        self.player_x_score +=1;
                    }
                }
                
            },
            GameOutcome::Draw => {
                self.number_of_draws +=1;
            }
            GameOutcome::Incomplete => {
                //do nothing
            }
        }

        Ok(game_outcome)
    }

    /// Writes the game board's state to stdout
    /// 
    /// If a [WinPosition] is passed, highlights the winning spaces
    /// 
    /// Causes no change in cursor position, as its position is reset after drawing.
    pub(crate) fn draw_game(&self, win_position: Option<WinPosition>) -> crossterm::Result<()>
    {   
        const HORIZ_LINE: &str = "-----------"; 

        let (cursor_col, cursor_row) = cursor::position()?;

        let top_row = format!(" {} | {} | {}",
            Self::get_styled_space(BoardSpaceLocation::TopLeft, &self.game_board, win_position),
            Self::get_styled_space(BoardSpaceLocation::TopMiddle, &self.game_board, win_position),
            Self::get_styled_space(BoardSpaceLocation::TopRight, &self.game_board, win_position)
        );
        let middle_row = format!(" {} | {} | {}",
            Self::get_styled_space(BoardSpaceLocation::MiddleLeft, &self.game_board, win_position),
            Self::get_styled_space(BoardSpaceLocation::MiddleMiddle, &self.game_board, win_position),
            Self::get_styled_space(BoardSpaceLocation::MiddleRight, &self.game_board, win_position)
        );
        let bottom_row = format!(" {} | {} | {}",
        Self::get_styled_space(BoardSpaceLocation::BottomLeft, &self.game_board, win_position),
        Self::get_styled_space(BoardSpaceLocation::BottomMiddle, &self.game_board, win_position),
        Self::get_styled_space(BoardSpaceLocation::BottomRight, &self.game_board, win_position)
        );
        
        stdout()
            .queue(Print(top_row))?
            .queue(MoveToNextLine(1))?
            .queue(MoveToColumn(cursor_col))?
            
            .queue(Print(HORIZ_LINE))?
            .queue(MoveToNextLine(1))?
            .queue(MoveToColumn(cursor_col))?
            
            .queue(Print(middle_row))?
            .queue(MoveToNextLine(1))?
            .queue(MoveToColumn(cursor_col))?
            
            .queue(Print(HORIZ_LINE))?
            .queue(MoveToNextLine(1))?
            .queue(MoveToColumn(cursor_col))?

            .queue(Print(bottom_row))?
            .queue(MoveToNextLine(1))?
            .queue(MoveToColumn(cursor_col))?
            
            .queue(MoveToRow(cursor_row))?
            .queue(MoveToColumn(cursor_col))?;
            Ok(())
    }

    /// Returns the char at the given [BoardSpaceLocation], highlighted
    /// if the location is included in the given [WinPosition]
    /// 
    /// If `win_position` is [None], all letters will be styled normally
    fn get_styled_space(
        location: BoardSpaceLocation, 
        game_board: &GameBoard,
        win_position: Option<WinPosition>
    ) -> StyledContent<char>
    {
        let space_char = game_board.space(location).get_char();

        if let Some(win_position) = win_position {
            let win_locations = win_position.as_board_spaces();
            
            if win_locations.contains(&location) {
                space_char.negative()
            } else {
                StyledContent::new(ContentStyle::new(), space_char)
            }
        } else {
            StyledContent::new(ContentStyle::new(), space_char)
        }
    }
}
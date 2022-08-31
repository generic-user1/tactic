//! User interface utilities, including the main game loop

use std::io::{stdout, Write};

use crate::{
    gameboard::{GameBoard, BoardSpaceLocation, BoardSpace},
    player_type::PlayerType,
    game_outcome::GameOutcome
};
use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    style::Print,
    event::{self, Event, KeyEvent, KeyCode, KeyModifiers},
    cursor::{self, MoveToNextLine, MoveToColumn, MoveToRow},
    QueueableCommand, ExecutableCommand
};

const TERMSIZE_MIN_X: u16 = 11;
const TERMSIZE_MIN_Y: u16 = 8;

/// Struct used to manage the game UI
/// 
/// Manages setup and cleanup tasks.
pub struct UI{
    player_x: PlayerType,
    player_o: PlayerType
}
impl UI{

    /// Performs setup tasks needed by the UI
    /// 
    /// Called by the constructor of this struct
    fn setup_terminal() -> crossterm::Result<()>
    {
        terminal::enable_raw_mode()?;
        stdout()
            .queue(EnterAlternateScreen)?
            .flush()?;
        Ok(())
    }

    /// Performs cleanup tasks needed by the UI
    /// 
    /// Called by the [Drop] implementation of this struct
    fn cleanup_terminal() -> crossterm::Result<()>
    {
        stdout()
            .queue(LeaveAlternateScreen)?
            .flush()?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Writes the game board's state to stdout
    /// 
    /// Causes no change in cursor position, as its position is reset after drawing.
    fn draw_game(board: &GameBoard) -> crossterm::Result<()>
    {   
        const HORIZ_LINE: &str = "-----------"; 

        let (cursor_col, cursor_row) = cursor::position()?;

        let top_row = format!(" {} | {} | {}",
            board.space(BoardSpaceLocation::TopLeft),
            board.space(BoardSpaceLocation::TopMiddle),
            board.space(BoardSpaceLocation::TopRight)
        );
        let middle_row = format!(" {} | {} | {}",
            board.space(BoardSpaceLocation::MiddleLeft),
            board.space(BoardSpaceLocation::MiddleMiddle),
            board.space(BoardSpaceLocation::MiddleRight)
        );
        let bottom_row = format!(" {} | {} | {}",
            board.space(BoardSpaceLocation::BottomLeft),
            board.space(BoardSpaceLocation::BottomMiddle),
            board.space(BoardSpaceLocation::BottomRight)
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
            //.flush()
    }

    /// Sets up the terminal for running the game
    /// 
    /// Cleanup of the terminal is performed by the [Drop] implementation of this struct
    pub fn new(player_x: PlayerType, player_o: PlayerType) -> crossterm::Result<Self>
    {
        Self::setup_terminal()?;
        Ok(Self{player_x, player_o})
    }

    /// The main game loop
    ///
    /// Allows player X to claim one space, then allows player O to claim one space.
    /// Continues alternating between players until either the game is finished or a user
    /// quits the game.
    pub fn game_loop(&self) -> crossterm::Result<(GameOutcome, GameBoard)>
    {
        let (mut term_x, mut term_y) = terminal::size()?;

        if self.player_x == PlayerType::AI || self.player_o == PlayerType::AI{
            todo!("AI players not yet implemented");
        }

        let mut cursor_x: u8 = 0;
        let mut cursor_y: u8 = 0;

        // if false, it's player x turn
        // if true, it's player o turn
        let mut player_o_turn = false;

        let mut game_board = GameBoard::new();
        let mut game_outcome = GameOutcome::analyze_game(&game_board);
        // keep playing game until game outcome is finished 
        //(or the loop is broken out of because a user chose to quit)
        while !game_outcome.game_finished(){
            stdout()
                .queue(Clear(ClearType::All))?
                .queue(MoveToColumn(0))?
                .queue(MoveToRow(0))?
                .queue(cursor::Hide)?;
            if term_x >= TERMSIZE_MIN_X && term_y >= TERMSIZE_MIN_Y {
                Self::draw_game(&game_board)?;
                stdout()
                    .queue(MoveToRow(6))?
                    .queue(Print(
                        if player_o_turn {"O's turn"} else {"X's turn"}
                    ))?
                    .queue(MoveToRow(7))?.queue(MoveToColumn(0))?
                    .queue(Print(
                        "Use arrow keys to select space. Press Enter to place. Press q to quit."
                    ))?
                    // position cursor in the appropriate space
                    .queue(MoveToColumn(((cursor_x as u16) * 4) + 1))?
                    .queue(MoveToRow((cursor_y as u16) * 2))?

                    .queue(cursor::Show)?

                    .flush()?;
            } else {
                stdout().execute(Print("Terminal too small! Please enlarge terminal"))?;
            }

            
            match event::read()? {
                Event::Key(key_event) => {
                    match key_event {
                        KeyEvent{code:KeyCode::Right, ..} => {
                            if cursor_x < 2{
                                cursor_x += 1;
                            }
                        },
                        KeyEvent{code:KeyCode::Left, ..} => {
                            if cursor_x > 0{
                                cursor_x -= 1;
                            }
                        },
                        KeyEvent{code:KeyCode::Down, ..} => {
                            if cursor_y < 2 {
                                cursor_y += 1;
                            }
                        },
                        KeyEvent{code:KeyCode::Up, ..} => {
                            if cursor_y > 0{
                                cursor_y -= 1;
                            }
                        },
                        KeyEvent{code:KeyCode::Enter, ..} => {

                            let desired_location = 
                                BoardSpaceLocation::from_coordinates((cursor_x, cursor_y));
                            let desired_space = 
                                game_board.space_mut(desired_location);
                            
                            // only update space and switch players if selected space is empty
                            if desired_space == &BoardSpace::Empty {
                                //write active player letter to this space
                                *desired_space = if player_o_turn {
                                    BoardSpace::O
                                } else {
                                    BoardSpace::X
                                };
                                //switch player
                                player_o_turn = !player_o_turn;
                                //reset cursor position
                                cursor_x = 0;
                                cursor_y = 0;
                            }
                        }
                        KeyEvent{code:KeyCode::Char('q'), ..} => {
                            break;
                        },
                        KeyEvent{code:KeyCode::Char('c'), modifiers:KeyModifiers::CONTROL, ..} => {
                            break;
                        }
                        _ => {
                            //ignore other KeyEvents
                        }
                    }
                },
                Event::Resize(new_x, new_y) =>
                {
                    term_x = new_x;
                    term_y = new_y;
                }
                _ => {
                    //ignore other Events
                }
            }

            game_outcome = GameOutcome::analyze_game(&game_board);
        }

        Ok((game_outcome, game_board))
    }

    /// Returns a reference to the [PlayerType] of the X player
    pub fn player_x(&self) -> &PlayerType
    {
        &self.player_x
    }
    
    /// Returns a reference to the [PlayerType] of the O player
    pub fn player_o(&self) -> &PlayerType
    {
        &self.player_o
    }

    /// Returns a mutable reference to the [PlayerType] of the X player
    pub fn player_x_mut(&mut self) -> &mut PlayerType
    {
        &mut self.player_x
    }
    
    /// Returns a mutable reference to the [PlayerType] of the O player
    pub fn player_o_mut(&mut self) -> &mut PlayerType
    {
        &mut self.player_o
    }
}

impl Drop for UI {
    /// Cleans up the terminal as this UI is dropped out of scope.
    /// [Read More](https://doc.rust-lang.org/1.62.1/core/ops/trait.Drop.html#tymethod.drop)
    fn drop(&mut self) {
        if UI::cleanup_terminal().is_err(){
            panic!("Failed to cleanup terminal when dropping UI");
        }
    }
}

impl Default for UI {
    /// Sets up and returns an instance of UI with the default player types.
    /// [Read More](https://doc.rust-lang.org/1.62.1/core/default/trait.Default.html#tymethod.default)
    fn default() -> Self {
        match Self::new(PlayerType::default(), PlayerType::default()){
            Ok(instance) => instance,
            Err(_) => panic!("failed to create default UI instance")
        }
    }
}


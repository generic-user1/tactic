//! User interface utilities, including the main game loop

use std::io::{stdout, Write};

use crate::{
    gameboard::{GameBoard, BoardSpaceLocation},
    player_type::PlayerType,
    game_outcome::GameOutcome,
    active_player::ActivePlayer,
    ai
};
use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    style::Print,
    cursor::{self, MoveToNextLine, MoveToColumn, MoveToRow},
    QueueableCommand, ExecutableCommand
};

const TERMSIZE_MIN_X: u16 = 11;
const TERMSIZE_MIN_Y: u16 = 8;

//declare event_handling module which contains
//event handling impl's for the UI struct
mod event_handling;

/// Struct used to manage the game UI
/// 
/// Manages setup and cleanup tasks, as well as storing game state
/// (which player's turn is active, cursor position, etc.)
/// 
///# Notes
/// 
/// While an instance of this struct is in scope, the terminal will be in 'raw mode' (and
/// in an alternate screen). This means that many things that operate on [std::io::stdout]
/// will not work as expected (such as [println!]). 
/// 
/// To return the terminal to normal, the `UI` instance must be destroyed. 
/// This can be done by calling [drop] on it it (e.g. `drop(ui_instance)`), 
/// by using the [UI::take_game_board] method, or by allowing it to fall out of scope.
pub struct UI{
    player_x: PlayerType,
    player_o: PlayerType,
    active_player: ActivePlayer,
    cursor_x_pos: u8,
    cursor_y_pos: u8,
    game_board: GameBoard,
    terminal_x_size: u16,
    terminal_y_size: u16,
    exit_flag: bool
}

impl UI{
    /// Sets up the terminal for running the game
    /// 
    /// Cleanup of the terminal is performed by the [Drop] implementation of this struct
    pub fn new(player_x: PlayerType, player_o: PlayerType) -> crossterm::Result<Self>
    {
        Self::setup_terminal()?;
        let (terminal_x_size, terminal_y_size) = terminal::size()?;
        let new_instance = Self{
            player_x,
            player_o,
            active_player: ActivePlayer::PlayerX,
            cursor_x_pos: 0,
            cursor_y_pos: 0,
            game_board: GameBoard::new(),
            terminal_x_size,
            terminal_y_size,
            exit_flag: false
        };
        Ok(new_instance)
    }

    /// Returns a reference to the [GameBoard] of this `UI`
    /// 
    /// Unlike [UI::take_game_board], this does not consume the `UI` instance.
    /// If you are done with the `UI` instance when calling this function, consider
    /// [UI::take_game_board] instead.
    pub fn borrow_game_board(&self) -> &GameBoard
    {
        &self.game_board
    }

    /// Consumes this `UI` and returns the [GameBoard]
    /// 
    /// Unlike [UI::borrow_game_board], this consumes the `UI` instance. 
    /// If you want to keep the `UI` instance, consider [UI::borrow_game_board] instead. 
    pub fn take_game_board(mut self) -> GameBoard
    {
        let game_board = std::mem::take(&mut self.game_board);
        drop(self);
        game_board
    }

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

        self.active_player = ActivePlayer::PlayerX;

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
            if self.terminal_x_size >= TERMSIZE_MIN_X && self.terminal_y_size >= TERMSIZE_MIN_Y {
                self.draw_game()?;
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
                PlayerType::AI => {
                    if ai::do_turn(&mut self.game_board, &self.active_player){
                        self.switch_active_player();
                    }
                }
            }

            game_outcome = self.game_board.game_outcome();
        }

        Ok(game_outcome)
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
    fn draw_game(&self) -> crossterm::Result<()>
    {   
        const HORIZ_LINE: &str = "-----------"; 

        let (cursor_col, cursor_row) = cursor::position()?;

        let top_row = format!(" {} | {} | {}",
            self.game_board.space(BoardSpaceLocation::TopLeft),
            self.game_board.space(BoardSpaceLocation::TopMiddle),
            self.game_board.space(BoardSpaceLocation::TopRight)
        );
        let middle_row = format!(" {} | {} | {}",
            self.game_board.space(BoardSpaceLocation::MiddleLeft),
            self.game_board.space(BoardSpaceLocation::MiddleMiddle),
            self.game_board.space(BoardSpaceLocation::MiddleRight)
        );
        let bottom_row = format!(" {} | {} | {}",
            self.game_board.space(BoardSpaceLocation::BottomLeft),
            self.game_board.space(BoardSpaceLocation::BottomMiddle),
            self.game_board.space(BoardSpaceLocation::BottomRight)
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

    /// Resets cursor position to (1,1)
    fn reset_cursor_pos(&mut self)
    {
        self.cursor_x_pos = 1;
        self.cursor_y_pos = 1;
    }

    /// Returns the PlayerType of the currently active player
    fn active_player_type(&self) -> &PlayerType
    {
        match self.active_player{
            ActivePlayer::PlayerO => &self.player_o,
            ActivePlayer::PlayerX => &self.player_x
        }
    }
}

impl Drop for UI {
    /// Cleans up the terminal as this UI is dropped out of scope.
    /// [Read More](https://doc.rust-lang.org/1.62.1/core/ops/trait.Drop.html#tymethod.drop)
    fn drop(&mut self) 
    {
        if UI::cleanup_terminal().is_err(){
            panic!("Failed to cleanup terminal when dropping UI");
        }
    }
}

impl Default for UI {
    /// Sets up and returns an instance of UI with the default player types.
    /// [Read More](https://doc.rust-lang.org/1.62.1/core/default/trait.Default.html#tymethod.default)
    fn default() -> Self 
    {
        match Self::new(PlayerType::default(), PlayerType::default()){
            Ok(instance) => instance,
            Err(_) => panic!("failed to create default UI instance")
        }
    }
}
//! User interface utilities, including the main game loop

use std::io::{stdout, Write};

use crate::{
    gameboard::GameBoard,
    player_type::PlayerType,
    active_player::ActivePlayer
};
use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand
};

//declare event_handling module which contains
//event handling impl's for the UI struct
mod event_handling;

//declare game module which contains
//game related impl's for the UI struct
mod game;

//declare play_again_menu module which contains
//post-game menu impl's for the UI struct
mod play_again_menu;

//declare setup_menu module which contains
//pre-game menu impl's for the UI struct
mod setup_menu;

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
    exit_flag: bool,
    player_x_score: u32,
    player_o_score: u32,
    number_of_draws: u32
}

impl UI{

    const TERMSIZE_MIN_X: u16 = 11;
    const TERMSIZE_MIN_Y: u16 = 8;

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
            exit_flag: false,
            player_x_score: 0,
            player_o_score: 0,
            number_of_draws: 0
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

    /// Returns the score (number of games won) of the X player
    pub fn player_x_score(&self) -> u32
    {
        self.player_x_score
    }

    /// Returns the score (number of games won) of the O player
    pub fn player_o_score(&self) -> u32
    {
        self.player_o_score
    }

    /// Returns the number of games that resulted in a draw
    pub fn number_of_draws(&self) -> u32
    {
        self.number_of_draws
    }

    /// Returns the total number of games played
    pub fn number_of_games(&self) -> u32
    {
        self.player_x_score + self.player_o_score + self.number_of_draws
    }

    /// Returns a reference to the currently active player
    pub fn active_player(&self) -> &ActivePlayer
    {
        &self.active_player
    }

    /// Returns a mutable reference to the currently active player
    pub fn active_player_mut(&mut self) -> &mut ActivePlayer
    {
        &mut self.active_player
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
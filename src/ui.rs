//! User interface utilities, including the main game loop

use std::io::{stdout, Write};

use crate::{
    gameboard::{GameBoard, BoardSpaceLocation},
    player_type::PlayerType,
    game_outcome::GameOutcome
};
use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    style::Print,
    cursor::{self, MoveToNextLine, MoveToColumn, MoveToRow},
    QueueableCommand
};

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
            .queue(MoveToColumn(cursor_col))?
            
            .flush()
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
    pub fn game_loop(&self) -> crossterm::Result<GameOutcome>
    {
        if self.player_x == PlayerType::AI || self.player_o == PlayerType::AI{
            todo!("AI players not yet implemented");
        }

        stdout()
            .queue(Clear(ClearType::All))?
            .queue(MoveToColumn(0))?
            .queue(MoveToRow(0))?
            .flush()?;

        // if false, it's player x turn
        // if true, it's player o turn
        let mut player_o_turn = false;

        let mut game_board = GameBoard::new();
        let mut game_outcome = GameOutcome::analyze_game(&game_board);
        // keep playing game until game outcome is finished 
        //(or the loop is broken out of because a user chose to quit)
        while !game_outcome.game_finished(){
            Self::draw_game(&game_board)?;

            // input stuff here

            game_outcome = GameOutcome::analyze_game(&game_board);
            player_o_turn = !player_o_turn;

            //TEMPORARY
            game_outcome = GameOutcome::Draw;
            std::thread::sleep(std::time::Duration::from_secs(5));
            //END TEMPORARY
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


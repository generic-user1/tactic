//! Utilities to determine the winner of a game (if any)

use crate::gameboard::{GameBoard, BoardSpace, BoardSpaceLocation};

mod win_position_constants;

/// The row, column, or diagonal that a game was won with
/// 
///# Notes
/// 
/// The [get_winner] function will check each win position in the order
/// that `WinPosition` variants are defined.
#[derive(Debug)]
pub enum WinPosition {
    TopRow,
    MiddleRow,
    BottomRow,
    LeftColumn,
    MiddleColumn,
    RightColumn,
    TopLeftToBottomRight,
    BottomLeftToTopRight
}

impl WinPosition{
    /// Returns an array of [BoardSpacePosition] with each space contained in this `WinPosition`
    pub const fn as_board_spaces(&self) -> &'static [BoardSpaceLocation; 3]
    {
        match self {
            Self::TopRow => &win_position_constants::TOP_ROW,
            Self::MiddleRow => &win_position_constants::MIDDLE_ROW,
            Self::BottomRow => &win_position_constants::BOTTOM_ROW,
            Self::LeftColumn => &win_position_constants::LEFT_COLUMN,
            Self::MiddleColumn => &win_position_constants::MIDDLE_COLUMN,
            Self::RightColumn => &win_position_constants::RIGHT_COLUMN,
            Self::TopLeftToBottomRight => &win_position_constants::TOP_LEFT_TO_BOTTOM_RIGHT,
            Self::BottomLeftToTopRight => &win_position_constants::BOTTOM_LEFT_TO_TOP_RIGHT
        }
    }

    /// Returns an iterator over all variants of `WinPosition`
    pub fn all() -> impl Iterator<Item = Self>
    {
        const VARIANTS: [WinPosition; 8] = [
            WinPosition::TopRow,
            WinPosition::MiddleRow,
            WinPosition::BottomRow,
            WinPosition::LeftColumn,
            WinPosition::MiddleColumn,
            WinPosition::RightColumn,
            WinPosition::TopLeftToBottomRight,
            WinPosition::BottomLeftToTopRight
        ];

        VARIANTS.into_iter()
    }
}

/// Analyzes a given [GameBoard] for a winner
/// 
/// The results of analysis are returned as a tuple `(winning_player, win_position)`.
/// 
/// The winning player is represented as a [BoardSpace] variant; 
/// [BoardSpace::X] if player X won, [BoardSpace::O] if player O won, 
/// or [BoardSpace::Empty] if neither player won.
/// 
/// The win position is represented as an optional [WinPosition]; 
/// `Some(WinPosition)` if a winner is found or `None` if no winner is found.
pub fn get_winner(board: &GameBoard) -> (BoardSpace, Option<WinPosition>)
{
    for win_position in WinPosition::all(){

        //get iter over the BoardSpace in each position
        let mut board_space_values = 
            win_position.as_board_spaces().iter().map(|board_space|{
                board.space(*board_space)
            });
        
        // consume first value from iter and store as possible winner
        let possible_winner = board_space_values.next().unwrap();
        // is_winner is set to true if possible_winner is not Empty
        // AND if all other values from iter match possible winner
        let is_winner = possible_winner != &BoardSpace::Empty && 
            board_space_values.all(|board_space|{
                board_space == possible_winner
            });
        if is_winner {
            return (possible_winner.clone(), Some(win_position));
        }

    }

    // return this if all win positions were checked and no winner was found
    (BoardSpace::Empty, None)
}
//! Utilities to determine the winner of a game (if any)

use crate::gameboard::{GameBoard, BoardSpace, BoardSpaceLocation};

mod win_position_constants;

/// The outcome of a game, if any
/// 
/// The `PlayerX` and `PlayerO` variants represent that the game
/// was won by the indicated player, and include the [WinPosition] the game was won with.
/// 
/// The `Draw` variant represents that the game is finished (no more moves can be played)
/// but that there was no winner.
/// 
/// The `Incomplete` variant represents that neither player has won, but that there
/// are still moves that can be played.
#[derive(Debug, PartialEq, Eq)]
pub enum GameOutcome{
    PlayerX(WinPosition),
    PlayerO(WinPosition),
    Draw,
    Incomplete
}
impl GameOutcome {
    
    /// Analyzes a given [GameBoard] for a winner
    /// 
    ///# Notes
    /// 
    /// If a game has multiple valid win positions,
    /// only one win position (and therefore one winner) is selected.
    /// The prioritiy of win positions is defined by the order that [WinPosition] variants
    /// are defined in; the earlier variants are higher priority than the later variants.
    /// 
    /// More specifically, the priority is defined by the order that variants are returned by
    /// the [WinPosition::all] function, but this order and the variant definition order 
    /// should be identical.
    pub fn analyze_game(board: &GameBoard) -> GameOutcome
    {
        for win_position in WinPosition::all(){

            //get iter over the BoardSpace in each position
            let mut board_space_values = 
                win_position.as_board_spaces().iter().map(|board_space|{
                    board.space(*board_space)
                });
            
            // consume first value from iter and store as possible winner
            let possible_winner = board_space_values.next().unwrap();
            // set is_winner to true if the rest of values from iter match possible_winner
            let is_winner = board_space_values.all(|board_space|{
                board_space == possible_winner});

            if is_winner{
                match possible_winner {
                    BoardSpace::Empty => {/* do nothing */},
                    BoardSpace::X => {return GameOutcome::PlayerX(win_position);},
                    BoardSpace::O => {return GameOutcome::PlayerO(win_position);}
                }
            }
        }

        // At this point, we have determined that neither player has won, 
        // as all win positions have been checked and no winner was found.
        // The return value will now be Incomplete if empty spaces were found,
        // or Draw if no empty spaces were found (indicating no more possible moves)
        for (_, space) in board.all_spaces() {
            if space == &BoardSpace::Empty {
                return GameOutcome::Incomplete;
            }
        }
        GameOutcome::Draw
    }

    /// Returns `true` if the game is finished
    /// 
    /// The game is finished if there are no more moves to be played or a player has won.
    pub fn game_finished(&self) -> bool
    {
        !matches!(self, Self::Incomplete)  
    }

    /// Returns `true` if the game has been won
    pub fn game_won(&self) -> bool
    {
        !matches!(self, Self::Draw | Self::Incomplete)
    }
}

/// The row, column, or diagonal that a game was won with
/// 
///# Notes
/// 
/// The [GameOutcome::analyze_game] function will check each win position in the order
/// that `WinPosition` variants are defined.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    /// Returns an array of [BoardSpaceLocation] with each space contained in this `WinPosition`
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
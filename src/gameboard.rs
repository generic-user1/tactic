//! Representation of a tic-tac-toe game board

use std::fmt::{Display, Write};
use crate::game_outcome::GameOutcome;

/// The state of a single space on a game board
/// 
/// A BoardSpace represents the three states a space on the tic-tac-toe
/// game board can be in: occupied by an X, occupied by an O, or not occupied at all
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum BoardSpace {
    #[default]
    Empty,
    X,
    O
}
impl BoardSpace {
    /// Returns the character used to represent this variant of `BoardSpace`
    /// 
    ///# Notes 
    /// 
    /// The [Display] implementation for `BoardSpace` is equivalent
    /// to the return value of this function.
    pub fn get_char(&self) -> char
    {
        match self {
            Self::Empty => ' ',
            Self::X => 'X',
            Self::O => 'O'
        }
    }
}
impl Display for BoardSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.get_char())?;

        Ok(())
    }
}

/// Enum representing all the possible space locations on a game board
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoardSpaceLocation {
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleLeft,
    MiddleMiddle,
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight
}

impl BoardSpaceLocation {

    /// Returns the coordinates `(x,y)` of this `BoardSpaceLocation` variant
    ///
    ///# Notes
    /// 
    /// `(0,0)` corresponds to `TopLeft` and `(2,2)` corresponds
    /// to `BottomRight`.
    pub const fn as_coordinates(&self) -> (u8, u8)
    {
        match self {
            Self::TopLeft => (0,0),
            Self::TopMiddle => (1,0),
            Self::TopRight => (2,0),
            Self::MiddleLeft => (0,1),
            Self::MiddleMiddle => (1,1),
            Self::MiddleRight => (2,1),
            Self::BottomLeft => (0,2),
            Self::BottomMiddle => (1,2),
            Self::BottomRight => (2,2)
        }
    }

    /// Returns the `BoardSpaceLocation` variant that corresponds to the given coordinates
    /// 
    ///# Notes
    /// 
    /// `(0,0)` corresponds to `TopLeft` and `(2,2)` corresponds
    /// to `BottomRight`.
    /// 
    ///# Panics 
    /// 
    /// This function panics if either `x` or `y` is greater than `2`, as `2` is the maximum
    /// coordinate in either dimension
    pub fn from_coordinates((x, y): (u8, u8)) -> Self
    {
        for board_space_location in Self::all() {
            if board_space_location.as_coordinates() == (x,y) {
                return board_space_location;
            }
        }

        panic!("Coordinates ({},{}) don't correspond to any BoardSpaceLocation", x, y);
    }

    /// Returns an iterator over all variants of `BoardSpaceLocation`
    pub fn all() -> impl Iterator<Item = Self>
    {
        const VARIANTS: [BoardSpaceLocation; 9] = [
            BoardSpaceLocation::TopLeft,
            BoardSpaceLocation::TopMiddle,
            BoardSpaceLocation::TopRight,
            BoardSpaceLocation::MiddleLeft,
            BoardSpaceLocation::MiddleMiddle,
            BoardSpaceLocation::MiddleRight,
            BoardSpaceLocation::BottomLeft,
            BoardSpaceLocation::BottomMiddle,
            BoardSpaceLocation::BottomRight
        ];

        VARIANTS.into_iter()
    }

}

/// Representation of a tic-tac-toe game board
/// 
/// That is, represents a square divided into 9 equally sized square spaces.
/// The state of each space is represented as a [BoardSpace].
///
#[derive(Default, Clone)]
pub struct GameBoard {
    board_state: [[BoardSpace; 3]; 3]
}

impl GameBoard {

    /// Returns a new `GameBoard` instance with all spaces initialized to [BoardSpace::Empty]
    /// 
    /// Equivalent to [GameBoard::default]
    pub fn new() -> Self
    {
        GameBoard::default()
    }

    /// Returns a reference to one of the board spaces
    pub fn space(&self, space_location: BoardSpaceLocation) -> &BoardSpace
    {
        self.space_by_coordinates(space_location.as_coordinates())
    }

    /// Returns a mutable reference to one of the board spaces
    pub fn space_mut(&mut self, space_location: BoardSpaceLocation) -> &mut BoardSpace
    {
        self.space_by_coordinates_mut(space_location.as_coordinates())
    }

    /// Returns an iterator over all board spaces
    /// 
    /// Each value returned by the iterator is a tuple `(board_space_location, board_space)`.
    /// 
    /// This is a convinience function equivalent to calling [space](GameBoard::space) for each
    /// possible [BoardSpaceLocation] variant. Note that an iterator over all variants of 
    /// [BoardSpaceLocation] can be obtained with [BoardSpaceLocation::all](BoardSpaceLocation::all).
    /// 
    /// If you want mutable references to each board space, you will need to call 
    /// [space_mut](GameBoard::space_mut) repeatadly; an `all_spaces_mut` cannot exist 
    /// because it would need to return multiple mutable references to the same `GameBoard` 
    /// (which is disallowed by Rust's borrowing rules).
    pub fn all_spaces(&self) -> impl Iterator<Item = (BoardSpaceLocation, &BoardSpace)>
    {
        BoardSpaceLocation::all().map(|space_location|{
            (space_location, self.space(space_location))
        })
    }

    /// Returns a reference to one of the board spaces. Specifies which space using
    /// its coordinates.
    /// 
    ///# Notes
    /// 
    /// `(0,0)` corresponds to `TopLeft` and `(2,2)` corresponds
    /// to `BottomRight`.
    /// 
    ///# Panics 
    /// 
    /// This function panics if either `x` or `y` is greater than `2`, as `2` is the maximum
    /// coordinate in either dimension
    pub fn space_by_coordinates(&self, (x,y): (u8,u8)) -> &BoardSpace
    {
        let board_state_column = 
            // use match instead of expect so we can call panic! directly and use its formatting
            match self.board_state.get(x as usize) {
                Some(col) => col,
                None => {
                    panic!("Invalid coordinates ({},{}); maximum is (2,2)", x, y);
                }
            };

        match board_state_column.get(y as usize) {
            Some(space) => space,
            None => {
                panic!("Invalid coordinates ({},{}); maximum is (2,2)", x, y);
            }
        }
    }

    /// Returns a mutable reference to one of the board spaces. Specifies which space using
    /// its coordinates.
    /// 
    ///# Notes
    /// 
    /// `(0,0)` corresponds to `TopLeft` and `(2,2)` corresponds
    /// to `BottomRight`.
    /// 
    ///# Panics 
    /// 
    /// This function panics if either `x` or `y` is greater than `2`, as `2` is the maximum
    /// coordinate in either dimension
    pub fn space_by_coordinates_mut(&mut self, (x,y): (u8,u8)) -> &mut BoardSpace
    {
        let board_state_column = 
            // use match instead of expect so we can call panic! directly and use its formatting
            match self.board_state.get_mut(x as usize) {
                Some(col) => col,
                None => {
                    panic!("Invalid coordinates ({},{}); maximum is (2,2)", x, y);
                }
            };

        match board_state_column.get_mut(y as usize) {
            Some(space) => space,
            None => {
                panic!("Invalid coordinates ({},{}); maximum is (2,2)", x, y);
            }
        }
    }

    /// Returns the string representation of this `GameBoard`
    ///
    /// The return value of this method is meant to visually represent the board's state.
    /// It can be printed directly as a quick-and-dirty way of 'rendering' the board.
    /// 
    ///# Notes
    /// 
    /// The [Display] implementation for `GameBoard` is equivalent 
    /// to this function's return value.
    pub fn as_string(&self) -> String
    {
        format!("{}", self)
    }

    /// Returns the [GameOutcome] of this board
    /// 
    /// Convinence method for `GameOutcome::analyze_game(&board)`
    pub fn game_outcome(&self) -> GameOutcome
    {
        GameOutcome::analyze_game(self)
    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const HORIZ_LINE: &str = "-----------\n"; 
        f.write_fmt(format_args!("\n {} | {} | {}\n",
            self.space(BoardSpaceLocation::TopLeft),
            self.space(BoardSpaceLocation::TopMiddle),
            self.space(BoardSpaceLocation::TopRight)
        ))?;

        f.write_str(HORIZ_LINE)?;

        f.write_fmt(format_args!(" {} | {} | {}\n",
            self.space(BoardSpaceLocation::MiddleLeft),
            self.space(BoardSpaceLocation::MiddleMiddle),
            self.space(BoardSpaceLocation::MiddleRight)
        ))?;

        f.write_str(HORIZ_LINE)?;

        f.write_fmt(format_args!(" {} | {} | {}",
            self.space(BoardSpaceLocation::BottomLeft),
            self.space(BoardSpaceLocation::BottomMiddle),
            self.space(BoardSpaceLocation::BottomRight)
        ))?;

        Ok(())
    }
}
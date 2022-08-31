
pub mod gameboard;
pub mod game_outcome;
pub mod ui;
pub mod ai;

/// The PlayerType enum
pub mod player_type {
    /// Represents the type of a player (either human or AI)
    #[derive(Default, Debug, PartialEq, Eq)]
    pub enum PlayerType {
        #[default]
        Human,
        AI
    }
}

/// The ActivePlayer enum
pub mod active_player{
    use crate::gameboard::BoardSpace;


    /// Represents which player (X or O) is currently active
    #[derive(PartialEq, Eq, Clone)]
    pub enum ActivePlayer {
        PlayerX,
        PlayerO
    }
    impl ActivePlayer {
        /// Switches this PlayerTurn to the opposite player
        pub fn switch(&mut self)
        {
            *self = self.opposite();
        }

        /// Returns the opposite `PlayerTurn`
        pub fn opposite(&self) -> Self
        {
            match self {
                ActivePlayer::PlayerO => ActivePlayer::PlayerX,
                ActivePlayer::PlayerX => ActivePlayer::PlayerO
            }
        }

        /// Returns the character representing this ActivePlayer
        pub fn get_char(&self) -> char
        {
            match self {
                ActivePlayer::PlayerO => 'O',
                ActivePlayer::PlayerX => 'X'
            }
        }

        /// Returns the [BoardSpace] variant associated
        /// with this `ActivePlayer` variant
        pub fn get_board_space(&self) -> BoardSpace
        {
            match self {
                ActivePlayer::PlayerX => BoardSpace::X,
                ActivePlayer::PlayerO => BoardSpace::O
            }
        }
    }
}
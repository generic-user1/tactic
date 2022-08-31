
pub mod gameboard;
pub mod game_outcome;
pub mod ui;

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

/// Represents which player (X or O) is currently active
#[derive(PartialEq, Eq)]
    pub enum ActivePlayer {
        PlayerX,
        PlayerO
    }
    impl ActivePlayer {
        /// Switches this PlayerTurn to the opposite player
        pub fn switch(&mut self)
        {
            *self = match self {
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
    }
}

pub mod gameboard;
pub mod game_outcome;
pub mod ui;
pub mod ai;

/// The PlayerType enum
pub mod player_type {

    use crate::ai::AiPlayer;

    /// Represents the type of a player (either human or AI)
    #[derive(Default, Debug, PartialEq)]
    pub enum PlayerType {
        #[default]
        Human,
        AI(AiPlayer)
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


/// Enums to represent different game settings
mod game_settings{
    /// Determines the game mode to be played
    #[derive(Default)]
    pub enum GameMode{
        #[default]
        Classic,
        Reverse
    }
    /// Determines how many games will be played before auto-exiting
    #[derive(Default, PartialEq, Eq)]
    pub enum GameAutoquitMode {
        /// No limit
        #[default]
        Unlimited,
        /// Limit the total number of games
        GameNumberLimit,
        /// Limit the number of games that are not draws
        NonDrawNumberLimit,
        /// Limit the score of either player
        ScoreNumberLimit
    }
}
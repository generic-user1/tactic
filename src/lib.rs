
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
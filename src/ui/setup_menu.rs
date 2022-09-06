//! UI implementations for the pre-game menu
//!
//! todo: add pre-game menu with various options including:
//! 
//!- player types for x and o
//! 
//!  - possibly as single/multiplayer
//! 
//!  - possibly as human/ai for both players seperately
//! 
//!- game number limit (auto-exit when limit reached)
//! 
//!  - unlimited (must disallow if/when using 2 ai players)
//! 
//!  - to n (custom value) games
//! 
//!  - to n (custom value) non-draw games
//! 
//!  - to n (custom value) wins for a single player
//! 
//!  - to score percent? unsure of this one
//! 
//!- ai player difficulty 
//! 
//!- reverse mode
//! 
//!  - getting three of your own pieces in a row counts as a loss
//! 
//!  - try to force the opposite player to get three in a row
//! 
//!  - AI difficulty is reversed in this mode (if player inputs a 1, a 0 
//!    should be passed to the AiPlayer instance) 
//!
//! Menu should be a list of interactable options, where some keys (probably left and right arrows)
//! allow the user to change the setting, and up/down allow the user to select different settings to change
//! 
//! If we have lots of options, there will need to be scrolling support. Selecting an option that is too high
//! or too low to be viewable should scroll the entire menu. This could be handled with some kind of dynamic offset.
//! 
//! Option types would include:
//!
//! - "radio buttons" (select one option from a pre-defined set)
//!
//!   - toggles could be implemented as a 2-state "radio button" 
//! 
//!   - options could be implemented as enums with some method that gives a name to each option
//! 
//!   - may want 'descriptions' for some options, where there is some pre-defined space to print 
//!     a description of the currently selected option.
//! 
//! - numeric inputs (for both floats and ints)
//!   
//!   - bounds need to be configurable
//! 
//!   - float input could take an int from a known range in and scale it to some float output (0-100 scales to 0.0-1.0)
//!  
//! 
//! Options need to be capable of being 'hidden' or 'deactivated'
//! 
//! - either no display or a differently styled display to indicate the option is deactivated
//! 
//! - a deactivated option may or may not still be selectable, but should be unchangeable
//! 
//! 
//! Selected options need to be highlighted somehow, probably by filling the background of their text.

use crate::{active_player::ActivePlayer, player_type::PlayerType, ai::AiPlayer};

/// Representation of the state of the menu
/// 
///# Notes
/// 
/// This should only ever be used by the UI struct, as its constructor and destructor
/// take care of setup and cleanup tasks.
#[cfg(not_yet_implemented)]
struct SetupMenu {
    /// the AiPlayer instance returned here isn't used,
    /// it will be created if appropriate using the 'player_x_difficulty' option
    player_x_type: PlayerTypeMenuOption,

    /// the AiPlayer instance returned here isn't used,
    /// it will be created if appropriate using the 'player_o_difficulty' option
    player_o_type: PlayerTypeMenuOption,

    /// only used if player x is AI
    player_x_ai: DifficultyMenuOption,

    /// only used if player o is AI
    player_o_ai: DifficultyMenuOption,

    autoquit_mode: MenuOption<GameAutoquitMode>,

    autoquit_value: MenuOption<u32>,

    game_mode: DescribedMenuOption<GameMode>
    
}

struct DifficultyMenuOption {
    selected_difficulty: f64,
    player: ActivePlayer
}

impl DifficultyMenuOption {
    const DIFFICULTY_STEP: f64 = 0.1;

    /// Creates and returns a new PlayerTypeMenuOption for the specified player
    pub fn new(player: ActivePlayer) -> Self
    {
        Self{player, selected_difficulty: 0.5}
    }
}

impl MenuOption<AiPlayer> for DifficultyMenuOption {
    
    fn value(self) -> AiPlayer {
        AiPlayer::new(self.selected_difficulty)
    }

    fn current_value_name(&self) -> String {
        format!("{}", (self.selected_difficulty * 10.0) as u8)
    }

    fn option_name(&self) -> String {
        format!("Player {} Difficulty", self.player.get_char())
    }

    fn next_value(&mut self) -> Result<(),()> {
        let new_value = self.selected_difficulty + Self::DIFFICULTY_STEP;
        if new_value > 1.0 {
            Err(())
        } else {
            self.selected_difficulty = new_value;
            Ok(())
        }
    }

    fn prev_value(&mut self) -> Result<(),()> {
        let new_value = self.selected_difficulty - Self::DIFFICULTY_STEP;
        if new_value < 0.0 {
            Err(())
        } else {
            self.selected_difficulty = new_value;
            Ok(())
        }
    }

}

struct PlayerTypeMenuOption{
    selected_player_type: PlayerType,
    player: ActivePlayer
}

impl PlayerTypeMenuOption{
    /// Creates and returns a new PlayerTypeMenuOption for the specified player
    pub fn new(player: ActivePlayer, default_type: PlayerType) -> Self
    {
        Self{player, selected_player_type: default_type}
    }
}

impl MenuOption<PlayerType> for PlayerTypeMenuOption{
    fn value(self) -> PlayerType {
        self.selected_player_type
    }

    fn current_value_name(&self) -> String {
        match self.selected_player_type {
            PlayerType::Human => "Human".to_owned(),
            PlayerType::AI(_) => "AI".to_owned()
        }
    }

    fn option_name(&self) -> String {
        format!("Player {} Type", self.player.get_char())
    }

    fn next_value(&mut self) -> Result<(),()> {
        match self.selected_player_type {
            PlayerType::Human => self.selected_player_type = PlayerType::AI(AiPlayer::default()),
            PlayerType::AI(_) => self.selected_player_type = PlayerType::Human
        }
        Ok(())
    }

    fn prev_value(&mut self) -> Result<(),()> {
        self.next_value()
    }
}

/// Determines the game mode to be played
#[derive(Default)]
enum GameMode{
    #[default]
    Classic,
    Reverse
}

/// Determines how many games will be played before auto-exiting
#[derive(Default)]
enum GameAutoquitMode {
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

/// Menu option; allows user to configure some value
trait MenuOption<T> {

    /// Returns the display name of this option 
    /// 
    /// "Player X type", "AI difficulty", etc
    fn option_name(&self) -> String;

    /// Returns the display name of the currently selected 'value' for this option
    /// 
    /// Enums should select the next variant. Numerics should select the next value.
    fn current_value_name(&self) -> String;

    /// Changes the option to the next value 
    /// 
    /// This should switch to the 'next' value if possible
    /// 
    /// Returns Ok if this happens correctly, or Err if it doesn't (usually because we are already at maximum)
    fn next_value(&mut self) -> Result<(),()>;

    /// Changes the option to the previous value 
    /// 
    /// This should switch to the 'previous' value if possible
    /// 
    /// Returns Ok if this happens correctly, or Err if it doesn't (usually because we are already at maximum)
    fn prev_value(&mut self) -> Result<(),()>;

    /// Returns the currently selected value, consuming the instance of MenuOption
    fn value(self) -> T;

}

/// [MenuOption] with an added description of the currently selected value
trait DescribedMenuOption<T>: MenuOption<T> {

    /// Returns a description of the currently selected value
    fn description(&self) -> String;

}
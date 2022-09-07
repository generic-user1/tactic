//! UI implementations for the pre-game menu
//!
//! todo: pre-game menu rendering
//! 
//! Menu options need to be capable of being 'hidden' or 'deactivated'
//! 
//! - either no display or a differently styled display to indicate the option is deactivated
//! 
//! - a deactivated option may or may not still be selectable, but should be unchangeable
//! 
//! 
//! Selected options need to be highlighted somehow, probably by filling the background of their text.

// Declare menu_options module which contains definitions of
// menu option structs
mod menu_options;

use menu_options::{
    PlayerTypeMenuOption,
    DifficultyMenuOption,
    AutoquitModeMenuOption,
    AutoquitValueMenuOption,
    GameModeMenuOption
};

use crate::{active_player::ActivePlayer, player_type::PlayerType, ai::AiPlayer};

use super::UI;

/// Representation of the state of the menu
/// 
///# Notes
/// 
/// This should only ever be used by the UI struct, as its constructor and destructor
/// take care of setup and cleanup tasks.
pub(super) struct SetupMenu {
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

    autoquit_mode: AutoquitModeMenuOption,

    autoquit_value: AutoquitValueMenuOption,

    game_mode: GameModeMenuOption
    
}

impl SetupMenu{
    /// Creates and returns a new SetupMenu
    pub fn new() -> Self
    {
        Self {
            player_x_type: PlayerTypeMenuOption::new(
                ActivePlayer::PlayerX, 
                PlayerType::Human
            ),
            player_o_type: PlayerTypeMenuOption::new(
                ActivePlayer::PlayerO,
                PlayerType::AI(AiPlayer::default())
            ),
            player_x_ai: DifficultyMenuOption::new(ActivePlayer::PlayerX),
            player_o_ai: DifficultyMenuOption::new(ActivePlayer::PlayerO),
            autoquit_mode: AutoquitModeMenuOption::new(),
            autoquit_value: AutoquitValueMenuOption::new(),
            game_mode: GameModeMenuOption::new()
        }
    }

    /// Display menu until user submits choices
    /// 
    /// TODO: implement, currently a stub
    pub fn setup_menu_loop(&mut self) -> crossterm::Result<()>
    {
        //stub 
        Ok(())
    }

    /// Alter the given [UI] instance to match the settings of this `SetupMenu` 
    /// 
    /// Consumes this `SetupMenu` instance
    pub fn apply_settings(self, ui_instance: &mut UI)
    {
        ui_instance.player_x = match self.player_x_type.value() {
            PlayerType::Human => PlayerType::Human,
            PlayerType::AI(_) => {
                let ai_player = self.player_x_ai.value();
                PlayerType::AI(ai_player)
            }
        };

        ui_instance.player_o = match self.player_o_type.value() {
            PlayerType::Human => PlayerType::Human,
            PlayerType::AI(_) => {
                let ai_player = self.player_o_ai.value();
                PlayerType::AI(ai_player)
            }
        };

        ui_instance.game_autoquit_mode = self.autoquit_mode.value();
        ui_instance.game_autoquit_value = self.autoquit_value.value();
        ui_instance.game_mode = self.game_mode.value();
    }
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
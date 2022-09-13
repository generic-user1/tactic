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

// Declare menu_loop module which contains 
// menu interaction and rendering impls for SetupMenu
mod menu_loop;

use menu_options::{
    PlayerTypeMenuOption,
    DifficultyMenuOption,
    AutoquitModeMenuOption,
    AutoquitValueMenuOption,
    GameModeMenuOption
};

use crate::{active_player::ActivePlayer, player_type::PlayerType, ai::AiPlayer, game_settings::{GameMode, GameAutoquitMode}};

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

    game_mode: GameModeMenuOption,
    
    selected_option: SelectedOption

}

impl SetupMenu{

    const TERMSIZE_MIN_X: u16 = 54;
    const TERMSIZE_MIN_Y: u16 = 9;

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
            game_mode: GameModeMenuOption::new(),
            selected_option: SelectedOption::PlayerXType
        }
    }

    /// Selects the next option
    pub fn next_option(&mut self)
    {
        match self.selected_option {
            SelectedOption::PlayerXType => {
                if self.player_x_type.value() == &PlayerType::Human{
                    self.selected_option = SelectedOption::PlayerOType
                } else {
                    self.selected_option = SelectedOption::PlayerXAi
                }
            },
            SelectedOption::PlayerXAi => {
                self.selected_option = SelectedOption::PlayerOType
            }
            SelectedOption::PlayerOType => {
                if self.player_o_type.value() == &PlayerType::Human{
                    self.selected_option = SelectedOption::AutoquitMode
                } else {
                    self.selected_option = SelectedOption::PlayerOAi
                }
            },
            SelectedOption::PlayerOAi => {
                self.selected_option = SelectedOption::AutoquitMode
            },
            SelectedOption::AutoquitMode => {
                if self.autoquit_mode.value() == &GameAutoquitMode::Unlimited {
                    self.selected_option = SelectedOption::GameMode
                } else {
                    self.selected_option = SelectedOption::AutoquitValue
                }
            },
            SelectedOption::AutoquitValue => {
                self.selected_option = SelectedOption::GameMode
            },
            SelectedOption::GameMode => {
                self.selected_option = SelectedOption::PlayerXType
            }
            
        }
    }

    /// Selects the previous option
    pub fn prev_option(&mut self)
    {
        match self.selected_option {
            SelectedOption::PlayerXType => {
                self.selected_option = SelectedOption::GameMode
            },
            SelectedOption::PlayerXAi => {
                self.selected_option = SelectedOption::PlayerXType
            },
            SelectedOption::PlayerOType => {
                if self.player_x_type.value() == &PlayerType::Human{
                    self.selected_option = SelectedOption::PlayerXType
                } else {
                    self.selected_option = SelectedOption::PlayerXAi
                }
            },
            SelectedOption::PlayerOAi => {
                self.selected_option = SelectedOption::PlayerOType
            },
            SelectedOption::AutoquitMode => {
                if self.player_o_type.value() == &PlayerType::Human{
                    self.selected_option = SelectedOption::PlayerOType
                } else {
                    self.selected_option = SelectedOption::PlayerOAi
                }
            },
            SelectedOption::AutoquitValue => {
                self.selected_option = SelectedOption::AutoquitMode
            },
            SelectedOption::GameMode => {
                if self.autoquit_mode.value() == &GameAutoquitMode::Unlimited{
                    self.selected_option = SelectedOption::AutoquitMode
                } else {
                    self.selected_option = SelectedOption::AutoquitValue
                }
            }
        }
    }

    /// Alter the given [UI] instance to match the settings of this `SetupMenu` 
    /// 
    /// Consumes this `SetupMenu` instance
    pub fn apply_settings(self, ui_instance: &mut UI)
    {
        let game_mode = self.game_mode.value();
        ui_instance.player_x = match self.player_x_type.value() {
            PlayerType::Human => PlayerType::Human,
            PlayerType::AI(_) => {
                let ai_player = match game_mode {
                    GameMode::Classic => self.player_x_ai.value(),
                    GameMode::Reverse => self.player_x_ai.value().reverse_difficulty()
                };
                PlayerType::AI(ai_player)
            }
        };

        ui_instance.player_o = match self.player_o_type.value() {
            PlayerType::Human => PlayerType::Human,
            PlayerType::AI(_) => {
                let ai_player = match game_mode {
                    GameMode::Classic => self.player_o_ai.value(),
                    GameMode::Reverse => self.player_o_ai.value().reverse_difficulty()
                };
                PlayerType::AI(ai_player)
            }
        };

        ui_instance.game_autoquit_mode = self.autoquit_mode.consume();
        ui_instance.game_autoquit_value = self.autoquit_value.value();
        ui_instance.game_mode = game_mode;
    }
}

#[derive(PartialEq, Clone, Copy)]
enum SelectedOption{
    PlayerXType,
    PlayerOType,
    PlayerXAi,
    PlayerOAi,
    AutoquitMode,
    AutoquitValue,
    GameMode
}

impl SelectedOption{
    /// Returns an iterator over all SelectedOption variants
    pub fn all() -> impl Iterator<Item = SelectedOption>
    {
        const ALL_OPTIONS: [SelectedOption; 7] = [
            SelectedOption::PlayerXType,
            SelectedOption::PlayerXAi,
            SelectedOption::PlayerOType,
            SelectedOption::PlayerOAi,
            SelectedOption::AutoquitMode,
            SelectedOption::AutoquitValue,
            SelectedOption::GameMode
            ];

        ALL_OPTIONS.into_iter()
    }
}

/// Menu option; allows user to configure some value
trait MenuOption {

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

    /// Returns true when the maximum value has been reached (i.e. calling next_value will fail)
    fn at_maximum(&self) -> bool;

    /// Returns true when the minimum value has been reached (i.e. calling prev_value will fail)
    fn at_minimum(&self) -> bool;

    /// Optional description of the currently selected value
    fn description(&self) -> Option<String>;
}
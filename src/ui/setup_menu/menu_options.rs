//! MenuOption implementors for the setup menu

use crate::{
    active_player::ActivePlayer, 
    player_type::PlayerType, 
    ai::AiPlayer,
    game_settings::{GameMode, GameAutoquitMode}
};
use super::{MenuOption, DescribedMenuOption};

pub(super) struct GameModeMenuOption {
    selected_game_mode: GameMode
}

impl GameModeMenuOption {
    
    /// Creates and returns a new GameModeMenuOption for the specified player
    pub fn new() -> Self
    {
        Self{selected_game_mode: GameMode::Classic}
    }
}

impl DescribedMenuOption<GameMode> for GameModeMenuOption {
    fn description(&self) -> String {
        match self.selected_game_mode {
            GameMode::Classic => "Play to place three of your pieces in a row. ".to_owned() 
            + "Prevent your opponent from placing three of their pieces in a row.",
            GameMode::Reverse => "Play to avoid placing three of your pieces in a row. ".to_owned() 
            + "Try to force your opponent to place three of their pieces in a row."
        }
    }
}

impl MenuOption<GameMode> for GameModeMenuOption {
    fn value(self) -> GameMode {
        self.selected_game_mode
    }

    fn option_name(&self) -> String {
        "Game Mode".to_owned()
    }

    fn current_value_name(&self) -> String {
        match self.selected_game_mode {
            GameMode::Classic => "Classic".to_owned(),
            GameMode::Reverse => "Reverse".to_owned()
        }
    }

    fn next_value(&mut self) -> Result<(),()> {
        match self.selected_game_mode {
            GameMode::Classic => self.selected_game_mode = GameMode::Reverse,
            GameMode::Reverse => self.selected_game_mode = GameMode::Classic
        }
        Ok(())
    }

    fn prev_value(&mut self) -> Result<(),()> {
        self.next_value()
    }
}

pub(super) struct AutoquitValueMenuOption {
    selected_value: u32
}

impl AutoquitValueMenuOption {
    const AUTOQUIT_VALUE_STEP: u32 = 1;

    /// Creates and returns a new AutoquitValueMenuOption for the specified player
    pub fn new() -> Self
    {
        Self{selected_value: 1}
    }
}

impl MenuOption<u32> for AutoquitValueMenuOption {
    fn value(self) -> u32 {
        self.selected_value
    }

    fn option_name(&self) -> String {
        "Game Limit Value".to_owned()
    }

    fn current_value_name(&self) -> String {
        format!("{}", self.selected_value)
    }

    fn next_value(&mut self) -> Result<(),()> {
        if let Some(new_value) = self.selected_value.checked_add(Self::AUTOQUIT_VALUE_STEP) {
            self.selected_value = new_value;
            Ok(())
        } else {
            Err(())
        }
    }

    fn prev_value(&mut self) -> Result<(),()> {
        if let Some(new_value) = self.selected_value.checked_sub(Self::AUTOQUIT_VALUE_STEP) {
            if new_value > 0 {
                self.selected_value = new_value;
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

pub(super) struct AutoquitModeMenuOption{
    selected_mode: GameAutoquitMode
}

impl AutoquitModeMenuOption{
    /// Creates and returns a new AutoquitModeMenuOption for the specified player
    pub fn new() -> Self
    {
        Self{selected_mode:GameAutoquitMode::Unlimited}
    }
}

impl MenuOption<GameAutoquitMode> for AutoquitModeMenuOption {
    fn value(self) -> GameAutoquitMode {
        self.selected_mode
    }

    fn option_name(&self) -> String {
        "Game Limit Type".to_owned()
    }

    fn current_value_name(&self) -> String {
        match self.selected_mode {
            GameAutoquitMode::Unlimited => "Unlimited".to_owned(),
            GameAutoquitMode::GameNumberLimit => "Max number of total games".to_owned(),
            GameAutoquitMode::NonDrawNumberLimit => "Max number of won games".to_owned(),
            GameAutoquitMode::ScoreNumberLimit => "Max score of either player".to_owned()
        }
    }

    fn next_value(&mut self) -> Result<(),()> {
        match self.selected_mode {
            GameAutoquitMode::Unlimited => {
                self.selected_mode = GameAutoquitMode::GameNumberLimit;
            },
            GameAutoquitMode::GameNumberLimit => {
                self.selected_mode = GameAutoquitMode::NonDrawNumberLimit;
            },
            GameAutoquitMode::NonDrawNumberLimit => {
                self.selected_mode = GameAutoquitMode::ScoreNumberLimit;
            },
            GameAutoquitMode::ScoreNumberLimit => {
                return Err(());
            }
        }
        Ok(())
    }

    fn prev_value(&mut self) -> Result<(),()> {
        match self.selected_mode {
            GameAutoquitMode::Unlimited => {
                return Err(());
            },
            GameAutoquitMode::GameNumberLimit => {
                self.selected_mode = GameAutoquitMode::Unlimited
            },
            GameAutoquitMode::NonDrawNumberLimit => {
                self.selected_mode = GameAutoquitMode::GameNumberLimit;
            },
            GameAutoquitMode::ScoreNumberLimit => {
                self.selected_mode = GameAutoquitMode::NonDrawNumberLimit
            }
        }
        Ok(())
    }
}

pub(super) struct DifficultyMenuOption {
    selected_difficulty: f64,
    player: ActivePlayer
}

impl DifficultyMenuOption {
    const DIFFICULTY_STEP: f64 = 0.05;

    /// Creates and returns a new DifficultyMenuOption for the specified player
    pub fn new(player: ActivePlayer) -> Self
    {
        Self{player, selected_difficulty: 0.85}
    }
}

impl MenuOption<AiPlayer> for DifficultyMenuOption {
    
    fn value(self) -> AiPlayer {
        AiPlayer::new(self.selected_difficulty)
    }

    fn current_value_name(&self) -> String {
        format!("{}", (self.selected_difficulty * 100.0) as u8)
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

pub(super) struct PlayerTypeMenuOption{
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
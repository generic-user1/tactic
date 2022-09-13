//! MenuOption implementors for the setup menu

use crate::{
    active_player::ActivePlayer, 
    player_type::PlayerType, 
    ai::AiPlayer,
    game_settings::{GameMode, GameAutoquitMode}
};
use super::MenuOption;

pub(super) struct GameModeMenuOption {
    selected_game_mode: GameMode
}

impl GameModeMenuOption {
    
    /// Creates and returns a new GameModeMenuOption for the specified player
    pub fn new() -> Self
    {
        Self{selected_game_mode: GameMode::Classic}
    }

    pub fn value(self) -> GameMode
    {
        self.selected_game_mode
    }
}

impl MenuOption for GameModeMenuOption {

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

    fn at_maximum(&self) -> bool {
        false
    }

    fn at_minimum(&self) -> bool {
        false
    }

    fn description(&self) -> Option<String> {
        Some(match self.selected_game_mode {
            GameMode::Classic => "Play to place three of your pieces in a row. ".to_owned(),
            GameMode::Reverse => "Play to avoid placing three of your pieces in a row. ".to_owned()
        })
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

    pub fn value(self) -> u32
    {
        self.selected_value
    }
}

impl MenuOption for AutoquitValueMenuOption {

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

    fn at_maximum(&self) -> bool {
        self.selected_value == u32::MAX
    }

    fn at_minimum(&self) -> bool {
        self.selected_value == 1
    }

    fn description(&self) -> Option<String> {
        None
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

    pub fn value(&self) -> &GameAutoquitMode
    {
        &self.selected_mode
    }

    pub fn consume(self) -> GameAutoquitMode
    {
        self.selected_mode
    }
}

impl MenuOption for AutoquitModeMenuOption {

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

    fn at_maximum(&self) -> bool {
        self.selected_mode == GameAutoquitMode::ScoreNumberLimit
    }

    fn at_minimum(&self) -> bool {
        self.selected_mode == GameAutoquitMode::Unlimited
    }

    fn description(&self) -> Option<String> {
        None
    }
}

pub(super) struct DifficultyMenuOption {
    selected_difficulty: i8,
    player: ActivePlayer
}

impl DifficultyMenuOption {
    const DIFFICULTY_STEP: i8 = 5;

    /// Creates and returns a new DifficultyMenuOption for the specified player
    pub fn new(player: ActivePlayer) -> Self
    {
        Self{player, selected_difficulty: 85}
    }

    pub fn value(self) -> AiPlayer
    {
        AiPlayer::new(self.selected_difficulty as f64 / 100.0)
    }
}

impl MenuOption for DifficultyMenuOption {

    fn current_value_name(&self) -> String {
        format!("{}", self.selected_difficulty)
    }

    fn option_name(&self) -> String {
        format!("Player {} Difficulty", self.player.get_char())
    }

    fn next_value(&mut self) -> Result<(),()> {
        let new_value = self.selected_difficulty + Self::DIFFICULTY_STEP;
        if new_value > 100 {
            Err(())
        } else {
            self.selected_difficulty = new_value;
            Ok(())
        }
    }

    fn prev_value(&mut self) -> Result<(),()> {
        let new_value = self.selected_difficulty - Self::DIFFICULTY_STEP;
        if new_value < 0 {
            Err(())
        } else {
            self.selected_difficulty = new_value;
            Ok(())
        }
    }

    fn at_maximum(&self) -> bool {
        self.selected_difficulty == 100
    }

    fn at_minimum(&self) -> bool {
        self.selected_difficulty == 0
    }

    fn description(&self) -> Option<String> {
        None
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
    
    pub fn value(&self) -> &PlayerType
    {
        &self.selected_player_type
    }
}

impl MenuOption for PlayerTypeMenuOption{

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

    fn at_maximum(&self) -> bool {
        false
    }

    fn at_minimum(&self) -> bool {
        false
    }

    fn description(&self) -> Option<String> {
        None
    }
}
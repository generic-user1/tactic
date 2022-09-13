//! SetupMenu implementations for rendering and interaction

use std::io::{stdout, Write};

use crossterm::{
    terminal::{Clear, ClearType, self},
    cursor::{self, MoveToColumn, MoveToRow, MoveToNextLine},
    style::{Stylize, PrintStyledContent, Print},
    event::{self, Event, KeyEvent, KeyCode, KeyModifiers},
    QueueableCommand,
    ExecutableCommand
};

use crate::{
    player_type::PlayerType, 
    game_settings::GameAutoquitMode
};

use super::{
    SelectedOption,
    MenuOption
};

impl super::SetupMenu {

    /// Display menu until user submits choices
    /// 
    /// returns false if user exits, returns true if user accepts choices
    pub fn setup_menu_loop(&mut self) -> crossterm::Result<bool>
    {
        // hide cursor and clear screen
        stdout()
            .queue(cursor::Hide)?    
            .queue(Clear(ClearType::All))?
            .flush()?;

        let (mut term_x, mut term_y) = terminal::size()?;

        let return_val = loop {
            if term_x >= Self::TERMSIZE_MIN_X && term_y >= Self::TERMSIZE_MIN_Y {
                self.render_setup_menu()?;
            } else {
                stdout().execute(Print("Terminal too small! Please enlarge terminal"))?;
            }

            match event::read()? {
                Event::Resize(new_x,new_y) => {
                    //clear screen if resize is detected
                    stdout().execute(Clear(ClearType::All))?;
                    term_x = new_x;
                    term_y = new_y;
                },
                Event::Key(key_event) => match key_event {
                    KeyEvent{code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, ..} => {
                        break false;
                    },
                    KeyEvent{code: KeyCode::Char('q'), ..} => {
                        break false;
                    },
                    KeyEvent{code: KeyCode::Enter, ..} => {
                        break true;
                    },
                    KeyEvent{code: KeyCode::Up, ..} => {
                        self.prev_option();
                    },
                    KeyEvent{code: KeyCode::Down, ..} => {
                        self.next_option();
                    },
                    KeyEvent{code: KeyCode::Right, ..} => {
                        if self.get_option_mut(self.selected_option).next_value().is_err(){
                            //ignore error
                        };
                    }
                    KeyEvent{code: KeyCode::Left, ..} => {
                        if self.get_option_mut(self.selected_option).prev_value().is_err(){
                            //ignore error
                        };
                    },
                    _=>()
                },
                _ => ()
            }
        };
        //show cursor before exiting
        stdout().execute(cursor::Show)?;
        Ok(return_val)
    }

    /// Returns true if the given option should be rendered as enabled
    /// 
    /// Uses current state to determine whether option should be enabled
    fn option_enabled(&self, option: SelectedOption) -> bool
    {
        match option {
            SelectedOption::PlayerXAi => {
                self.player_x_type.value() != &PlayerType::Human
            },
            SelectedOption::PlayerOAi => {
                self.player_o_type.value() != &PlayerType::Human
            },
            SelectedOption::AutoquitValue => {
                self.autoquit_mode.value() != &GameAutoquitMode::Unlimited
            },
            _ => true
        }
    }

    /// Returns a reference to the given SelectedOption
    fn get_option(&self, option: SelectedOption) -> &dyn MenuOption
    {
        match option{
            SelectedOption::PlayerXType => &self.player_x_type,
            SelectedOption::PlayerOType => &self.player_o_type,
            SelectedOption::PlayerXAi => &self.player_x_ai,
            SelectedOption::PlayerOAi => &self.player_o_ai,
            SelectedOption::AutoquitMode => &self.autoquit_mode,
            SelectedOption::AutoquitValue => &self.autoquit_value,
            SelectedOption::GameMode => &self.game_mode
        }
    }

    /// Returns a mutable reference to the given SelectedOption
    fn get_option_mut(&mut self, option: SelectedOption) -> &mut dyn MenuOption
    {
        match option{
            SelectedOption::PlayerXType => &mut self.player_x_type,
            SelectedOption::PlayerOType => &mut self.player_o_type,
            SelectedOption::PlayerXAi => &mut self.player_x_ai,
            SelectedOption::PlayerOAi => &mut self.player_o_ai,
            SelectedOption::AutoquitMode => &mut self.autoquit_mode,
            SelectedOption::AutoquitValue => &mut self.autoquit_value,
            SelectedOption::GameMode => &mut self.game_mode
        }
    }

    /// Renders the passed option, accounting for enabled-ness, which option
    /// is currently selected by the user, and option bounds
    fn render_option(&self, option: SelectedOption) -> crossterm::Result<()>
    {   
        let is_enabled = self.option_enabled(option);

        let is_currently_selected = option == self.selected_option;

        let tabstring = if option == SelectedOption::PlayerOType
            || option == SelectedOption::PlayerXType || option == SelectedOption::GameMode 
            {
                "\t\t"
            } else {
                "\t"
            };

        let option = self.get_option(option);

        let option_text = if is_currently_selected{
            let left_arrow = (if option.at_minimum() {' '} else {'<'})
                .negative();

            let right_arrow = (if option.at_maximum() {' '} else {'>'})
                .negative();

            format!("{}:{}{} {} {}",
                option.option_name(), tabstring, left_arrow, option.current_value_name(), right_arrow
            )
        } else {
            format!("{}:{} {} ",
                option.option_name(), tabstring, option.current_value_name()
            )
        };

        let styled_option_text = if is_enabled {
            option_text.white()
        } else {
            option_text.dark_grey().crossed_out()
        };

        stdout()
            .queue(Clear(ClearType::CurrentLine))?
            .queue(PrintStyledContent(styled_option_text))?
            .flush()?;
    
        Ok(())
    }

    /// Renders the setup menu to the terminal
    fn render_setup_menu(&self) -> crossterm::Result<()>
    {
        // position cursor
        stdout()
            .queue(MoveToColumn(0))?
            .queue(MoveToRow(0))?
            .flush()?;

        for option in SelectedOption::all() {
            
            self.render_option(option)?;
            stdout()
                .queue(MoveToColumn(0))?
                .queue(MoveToNextLine(1))?
                .flush()?;

            //render description if needed
            if let Some(option_desc) = self.get_option(option).description(){
                stdout()
                    .queue(Clear(ClearType::CurrentLine))?
                    .queue(Print(option_desc))?
                    .queue(MoveToColumn(0))?
                    .queue(MoveToNextLine(1))?
                    .flush()?;
            }
        }

        Ok(())
    }
}
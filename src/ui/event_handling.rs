//! UI implementations for event handling

use crossterm::{
    event::{self, Event, KeyEvent, KeyCode, KeyModifiers},
    terminal::{self, Clear, ClearType},
    ExecutableCommand
};
use crate::{
    gameboard::{BoardSpaceLocation, BoardSpace},
    active_player::ActivePlayer
};
use std::io::stdout;

impl super::UI {

    /// Move cursor to the right (positive x) if possible
    /// 
    /// Returns `true` if successful, `false` if not
    pub(super) fn move_cursor_right(&mut self) -> bool
    {
        if self.cursor_x_pos < 2{
            self.cursor_x_pos += 1;
            true
        } else {
            false
        }
    }

    /// Move cursor to the left (negative x) if possible
    /// 
    /// Returns `true` if successful, `false` if not
    pub(super) fn move_cursor_left(&mut self) -> bool
    {
        if self.cursor_x_pos > 0{
            self.cursor_x_pos -= 1;
            true
        } else {
            false
        }   
    }

    /// Move cursor downwards (positive y) if possible
    /// 
    /// Returns `true` if successful, `false` if not
    pub(super) fn move_cursor_down(&mut self) -> bool
    {
        if self.cursor_y_pos < 2{
            self.cursor_y_pos += 1;
            true
        } else {
            false
        }   
    }

    /// Move cursor upwards (negative y) if possible
    /// 
    /// Returns `true` if successful, `false` if not
    pub(super) fn move_cursor_up(&mut self) -> bool
    {
        if self.cursor_y_pos > 0{
            self.cursor_y_pos -= 1;
            true
        } else {
            false
        }
    }

    /// Claim the selected space for the active player if possible
    /// 
    /// Returns `true` if successful, `false` if not
    pub(super) fn claim_space(&mut self) -> bool
    {
        let desired_location = 
            BoardSpaceLocation::from_coordinates((self.cursor_x_pos, self.cursor_y_pos));
        let desired_space = 
            self.game_board.space_mut(desired_location);
        
        // only update space and switch players if selected space is empty
        if desired_space == &BoardSpace::Empty {
            //write active player letter to this space
            *desired_space = match self.active_player {
                ActivePlayer::PlayerX => BoardSpace::X,
                ActivePlayer::PlayerO => BoardSpace::O
            };
            true
        } else {
            false
        }
    }

    /// Switches the active player and resets cursor position
    pub(super) fn switch_active_player(&mut self) 
    {
        //switch player
        self.active_player.switch();

        //reset cursor position
        self.reset_cursor_pos();
    }

    /// Blocks until a [crossterm::event::Event] is available, then handles it
    pub(super) fn handle_next_event(&mut self) -> crossterm::Result<()>
    {
        match event::read()? {
            Event::Key(key_event) => {
                match key_event {
                    KeyEvent{code:KeyCode::Right, ..} => {
                        self.move_cursor_right();
                    },
                    KeyEvent{code:KeyCode::Left, ..} => {
                        self.move_cursor_left();
                    },
                    KeyEvent{code:KeyCode::Down, ..} => {
                        self.move_cursor_down();
                    },
                    KeyEvent{code:KeyCode::Up, ..} => {
                        self.move_cursor_up();
                    },
                    KeyEvent{code:KeyCode::Enter, ..} => {
                        //attempt to claim space and switch turns if successful
                        if self.claim_space(){
                            self.switch_active_player();
                        }
                    },
                    KeyEvent{code:KeyCode::Char('x'), ..} => {
                        //attempt to claim space if active player is X
                        if self.active_player == ActivePlayer::PlayerX && self.claim_space(){
                            self.switch_active_player();
                        }
                    },
                    KeyEvent{code:KeyCode::Char('o'), ..} => {
                        //attempt to claim space if active player is O
                        if self.active_player == ActivePlayer::PlayerO && self.claim_space(){
                            self.switch_active_player();
                        }
                    },
                    KeyEvent{code:KeyCode::Char('q'), ..} => {
                        self.exit_flag = true;
                    },
                    KeyEvent{code:KeyCode::Char('c'), modifiers:KeyModifiers::CONTROL, ..} => {
                        self.exit_flag = true;
                    }
                    _ => {
                        //ignore other KeyEvents
                    }
                }
            },
            //ignore size returned by resize event as it is currently (as of crossterm 0.25) wrong on Windows
            Event::Resize(_,_) => {
                let (new_x, new_y) = terminal::size()?;
                self.terminal_x_size = new_x;
                self.terminal_y_size = new_y;
                stdout().execute(Clear(ClearType::All))?;
            }
            _ => {
                //ignore other Events
            }
        }

        Ok(())
    }
}
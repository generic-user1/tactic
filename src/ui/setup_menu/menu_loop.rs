//! SetupMenu implementations for rendering and interaction

use std::io::{stdout, Write};

use crossterm::{
    terminal::{Clear, ClearType},
    cursor::{self, MoveToColumn, MoveToRow},
    QueueableCommand,
    ExecutableCommand
};

impl super::SetupMenu {

    /// Display menu until user submits choices
    /// 
    /// TODO: implement, currently a stub
    pub fn setup_menu_loop(&mut self) -> crossterm::Result<()>
    {
        // hide cursor and clear screen
        stdout()
            .queue(cursor::Hide)?    
            .execute(Clear(ClearType::All))?;

        self.render_setup_menu()?;

        //stub
        Ok(())
    }

    /// Renders the setup menu to the terminal
    /// 
    /// TODO: implement, currently a stub
    fn render_setup_menu(&self) -> crossterm::Result<()>
    {
        // position cursor
        stdout()
            .queue(MoveToColumn(0))?
            .queue(MoveToRow(0))?
            .flush()?;

        // drawing stuff here

        Ok(())
    }
}
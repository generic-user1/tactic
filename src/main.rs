use tactic::ui::UI;

fn main() -> crossterm::Result<()>
{
    let ui = UI::default();

    ui.game_loop()?;

    Ok(())
}

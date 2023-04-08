use std::io;
use tui::{backend::CrosstermBackend, Terminal};

fn init_term() -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(())
}

use core::fmt::Display;
use std::io::{self, Write};

use crossterm::cursor;
use crossterm::queue;
use crossterm::style;
use crossterm::terminal;
use crossterm::Command;

#[derive(Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Default)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), io::Error> {
        terminal::enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_caret_to(Position { row: 0, col: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), io::Error> {
        Self::queue_command(terminal::Clear(terminal::ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), io::Error> {
        Self::queue_command(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }
    pub fn move_caret_to(position: Position) -> Result<(), io::Error> {
        Self::queue_command(cursor::MoveTo(
            position.col.try_into().unwrap(),
            position.row.try_into().unwrap(),
        ))?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), io::Error> {
        Self::queue_command(cursor::Hide)?;
        Ok(())
    }

    pub fn size() -> Result<Size, io::Error> {
        let (width, height) = terminal::size()?;
        Ok(Size {
            width: width.into(),
            height: height.into(),
        })
    }

    pub fn terminate() -> Result<(), io::Error> {
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), io::Error> {
        Self::queue_command(cursor::Show)?;
        Ok(())
    }

    pub fn print<T: Display>(arg: T) -> Result<(), io::Error> {
        Self::queue_command(style::Print(arg))?;
        Ok(())
    }

    pub fn execute() -> Result<(), io::Error> {
        io::stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), io::Error> {
        queue!(io::stdout(), command)?;
        Ok(())
    }
}

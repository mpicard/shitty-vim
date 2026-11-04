use std::io;

use super::terminal::{Position, Size, Terminal};
use crate::editor::buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {
    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let message = format!("{NAME} editor -- v{VERSION}");
        let len = message.len();
        if width <= len {
            return "~".to_string();
        }
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;
        let mut message = format!("~{}{}", " ".repeat(padding), message);
        message.truncate(width);
        message
    }

    fn draw_empty_row() -> Result<(), io::Error> {
        Terminal::print("~")?;
        Ok(())
    }

    pub fn load(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn render(&mut self) -> Result<(), io::Error> {
        if !self.needs_redraw {
            return Ok(());
        }
        let Size { height, width } = Terminal::size()?;
        if height == 0 || width == 0 {
            return Ok(());
        }
        let v_center = height / 3;

        for row in 1..height {
            if let Some(line) = self.buffer.lines.get(row) {
                let truncated = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(row, truncated)?;
            } else if row == v_center && self.buffer.is_empty() {
                Self::render_line(row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(row, "~");
            }
        }
        self.needs_redraw = false;
        Ok(())
    }

    fn render_line(at: usize, line_text: &str) -> Result<(), io::Error> {
        Terminal::move_caret_to(Position { row: at, col: 0 })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::env;
use std::io;
use std::panic;
use terminal::{Position, Size, Terminal};
use view::View;

mod buffer;
mod terminal;
mod view;

#[derive(Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, io::Error> {
        let hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        Ok(Self {
            should_quit: false,
            view,
        })
    }

    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: &Event) -> Result<(), io::Error> {
        match event {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                modifiers,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), &KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageUp
                    | KeyCode::PageDown
                    | KeyCode::End
                    | KeyCode::Home,
                    _,
                ) => {
                    self.move_point(*code)?;
                }
                _ => {}
            },
            Event::Resize(width_u16, height_u16) => {
                self.view.resize(Size {
                    height: (*height_u16).into(),
                    width: (*width_u16).into(),
                });
            }
            _ => {}
        }
        Ok(())
    }

    fn move_point(&mut self, code: KeyCode) -> Result<(), io::Error> {
        let Location { x, y } = self.location;
        let Size { width, height } = Terminal::size()?;
        let (max_x, max_y) = (width.saturating_sub(1), height.saturating_sub(1));

        self.location = match code {
            KeyCode::Up if y > 0 => Location { x, y: y - 1 },
            KeyCode::Down if y < max_y => Location { x, y: y + 1 },
            KeyCode::Left if x > 0 => Location { x: x - 1, y },
            KeyCode::Right if x < max_x => Location { x: x + 1, y },
            KeyCode::PageUp => Location { x, y: 0 },
            KeyCode::PageDown => Location { x, y: max_y },
            KeyCode::Home => Location { x: 0, y },
            KeyCode::End => Location { x: max_x, y },
            _ => return Ok(()),
        };
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), io::Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            // Good bye
        } else {
            self.view.render()?;
            Terminal::move_caret_to(terminal::Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    fn repl(&mut self) -> Result<(), io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = event::read()?;
            self.evaluate_event(&event)?;
            Terminal::execute()?;
        }
        Ok(())
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(filename) = args.get(1) {
            self.view.load(filename);
        }
    }
}

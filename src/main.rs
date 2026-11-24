use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use postgres::{Client, Config, NoTls, Statement};
use ratatui::{prelude::*, widgets};
use std::{fmt::Debug, io, time::Duration};

#[cfg(test)]
mod tests;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))?;

    let client = Client::connect("host=localhost user=postgres dbname=games-grid", NoTls);

    Ok(())
}

#[derive(Debug, Default)]
pub struct App {
    current_screen: Screen,

    db_config: Config,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit_or_back(),
            KeyCode::Char('c') => self.current_screen = Screen::Configure,
            _ => {}
        }
    }

    fn configure_db(&mut self) {}

    fn quit_or_back(&mut self) {
        match self.current_screen {
            Screen::Main => self.exit = true,
            Screen::Configure => self.current_screen = Screen::Main,
        }
    }

    fn screen_main(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Games Grid ".bold());
        let instructions = Line::from(vec![
            " Configure DB ".into(),
            "<C>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = widgets::Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(symbols::border::THICK);

        let counter_text = Text::from(Line::from(vec!["Value: ".into()]));

        widgets::Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn screen_configure(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Configure DataBase ".bold());
        let instructions = Line::from(vec![" Back ".into(), "<Q> ".blue().bold()]);
        let block = widgets::Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(symbols::border::THICK);

        let counter_text = Text::from(Line::from(vec!["Value: ".into()]));

        widgets::Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[derive(Debug, Default)]
enum Screen {
    #[default]
    Main,
    Configure,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.current_screen {
            Screen::Main => self.screen_main(area, buf),
            Screen::Configure => self.screen_configure(area, buf),
        }
    }
}

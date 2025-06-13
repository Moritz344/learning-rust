use color_eyre::Result;
use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    // runs the applications main loop until quit
    // pub bedeutet public zugängig für dateien außerhalb
    pub fn run(&mut self,terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            // || -> vergleichbar mit lambda in anderen sprachen
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&self,frame: &mut Frame) {
        // todo!() -> platzhalter
        frame.render_widget(self,frame.area());
    }

    fn exit_app(&mut self) {
        self.exit = true;
    }

    fn decrement_counter(&mut self) {
        if self.counter >= 1 {
            self.counter -= 1;

        }
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn handle_key_events(&mut self,key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit_app(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }

    }
    // updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()>{
        // todo!() -> platzhalter
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event)
            }
        _ => {}
        };
        Ok(())
    }
}

impl Widget for &App {
    fn render(self,area: Rect,buf: &mut Buffer) {
        let title = Line::from(" Counter App".bold());
        let instructions = Line::from(vec![
            " Decrement".into(),
            "<Left>".blue().bold(),
            " Increment".into(),
            " <Right>".blue().bold(),
            " Quit".into(),
            "<Q>".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area,buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore(); // reinigt darstellung
    app_result
}



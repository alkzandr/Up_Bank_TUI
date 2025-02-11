use std::io;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::BufReader;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Tabs, Widget},
    DefaultTerminal, Frame,
};

//------------ Code for parsing JSON data into UI ----------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize)]  // Add Serialize trait
struct Balance {
    #[serde(rename = "currencyCode")]
    currency_code: String,
    value: String,
    #[serde(rename = "valueInBaseUnits")]
    value_in_base_units: i32,
}

#[derive(Debug, Deserialize, Serialize)]  // Add Serialize trait
struct AccountAttributes {
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "accountType")]
    account_type: String,
    #[serde(rename = "ownershipType")]
    ownership_type: String,
    balance: Balance,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]  // Add Serialize trait
struct Account {
    #[serde(rename = "id")]
    account_id: String,
    #[serde(rename = "attributes")]
    attributes: AccountAttributes,
}

#[derive(Debug, Deserialize, Serialize)]  // Add Serialize trait
struct ApiResponse {
    data: Vec<Account>,
}
// ------------- End JSON parsing code ----------------------------------------------------------------------------------------------------------



fn main() -> io::Result<()> {
//------------Code for parsing JSON data into UI ----------------------------------------------------------------------------------------------------------------------------
    // let file = File::open("data.json")?;





// -------------- End JSON parsing code ---------------------------------------------------------------------------------------------------------

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    tab_index: usize,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
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
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            KeyCode::Tab => self.next_tab(),
            KeyCode::BackTab => self.prev_tab(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 5;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % 4;
    }

    fn prev_tab(&mut self) {
        if self.tab_index == 0 {
            self.tab_index = 3;
        } else {
            self.tab_index -= 1;
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Up Bank CLI ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Next Tab ".into(),
            "<Tab>".blue().bold(),
            " Prev Tab ".into(),
            "<Shift+Tab>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let tabs = Tabs::new(vec!["Spending Accounts", "Savings Accounts", "Investments", "Trends? Search?"].iter().cloned())
            .select(self.tab_index)
            .block(Block::bordered());

        let content = match self.tab_index {
            0 => Text::from(vec![Line::from(vec![
                "Value: ".into(),
                self.counter.to_string().yellow(),
            ])]),
            1 => Text::from("NAME: {} \n ACCOUNT:{} \n BALANCE:{} \n "),
            2 => Text::from("Settings Tab: Adjust settings here."),
            3 => Text::from("Spending trends graphed or a transactions search algorithm?"),
            _ => unreachable!(),
        };

        let content_paragraph = Paragraph::new(content).centered().block(block);

        tabs.render(area, buf);
        content_paragraph.render(area, buf);
    }
}

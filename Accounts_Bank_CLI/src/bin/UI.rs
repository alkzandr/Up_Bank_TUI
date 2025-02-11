use std::io;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::BufReader;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Stylize, Color},
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
    // Load the JSON file and parse it into ApiResponse
    let file = File::open("accounts_balance.json")?;
    let api_response: ApiResponse = serde_json::from_reader(file).expect("Error parsing JSON");

    // Initialize the terminal
    let mut terminal = ratatui::init();

    // Create the App and pass the api_response to it
    let app_result = App::default().with_api_response(api_response).run(&mut terminal);

    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    tab_index: usize,
    api_response: Option<ApiResponse>,
}

impl App {
    pub fn with_api_response(mut self, api_response: ApiResponse) -> Self {
        self.api_response = Some(api_response);
        self
    }

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

        // Access the first account's data from api_response
        let display_name = self.api_response.as_ref().and_then(|api_response| api_response.data.get(0)).map_or("N/A", |account| &account.attributes.display_name);
        let account_id = self.api_response.as_ref().and_then(|api_response| api_response.data.get(0)).map_or("N/A", |account| &account.account_id);
        let balance = self.api_response.as_ref().and_then(|api_response| api_response.data.get(0)).map_or("N/A", |account| &account.attributes.balance.value);

        let content = match self.tab_index {
            0 => Text::from(vec![Line::from(vec![
                "Display Name: ".into(),
                display_name.yellow(),
                "\nAccount ID: ".into(),
                account_id.yellow(),
                "\nBalance: ".into(),
                balance.yellow(),
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

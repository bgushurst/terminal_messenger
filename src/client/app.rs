//use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    ComposingMessage,
    HelpMenu,
    Exiting,
}

pub struct App {
    pub message_input: String, // the currently being edited message value.
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub messages: Vec<String>,
    pub scroll_offset: usize,
}

impl App {
    pub fn new() -> App {
        App {
            message_input: String::new(),
            current_screen: CurrentScreen::Main,
            messages: Vec::<String>::new(),
            scroll_offset: 0,
        }
    }
    pub fn handle_websocket_message(&mut self, message: String) {
        self.messages.push(message);
        self.scroll_offset = 0;
    }
    // Methods for scrolling up and down
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }
}

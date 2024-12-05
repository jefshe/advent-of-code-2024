use ratatui::widgets::ListState;

use crate::days::*;

pub struct AOCList {
    pub items: Vec<AOCDay>,
    pub state: ListState,
}

impl AOCList {
    pub fn default() -> Self {
        Self {
            items: vec![
                AOCDay::new("Day 5", Box::new(Day5::new())),
                AOCDay::new("Day 4", Box::new(Day4::new())),
                AOCDay::todo("Day 6"),
                AOCDay::todo("Day 7"),
                AOCDay::todo("Day 8"),
                AOCDay::todo("Day 9"),
                AOCDay::todo("Day 10"),
                AOCDay::todo("Day 11"),
                AOCDay::todo("Day 12"),
                AOCDay::todo("Day 13"),
                AOCDay::todo("Day 14"),
                AOCDay::todo("Day 15"),
                AOCDay::todo("Day 16"),
                AOCDay::todo("Day 17"),
                AOCDay::todo("Day 18"),
                AOCDay::todo("Day 19"),
                AOCDay::todo("Day 20"),
            ],
            state: ListState::default(),
        }
    }
}

pub struct AOCDay {
    pub title: String,
    pub day: Option<Box<dyn Day>>,
}

impl AOCDay {
    fn new(title: &str, day: Box<dyn Day>) -> Self {
        Self {
            title: title.to_string(),
            day: Some(day),
        }
    }
    fn todo(title: &str) -> Self {
        Self {
            title: title.to_string(),
            day: None,
        }
    }
}

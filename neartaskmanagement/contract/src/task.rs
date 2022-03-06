use crate::*;
use near_sdk::log;


#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub struct Task {
    id: String,
    text: String,
    day: String,
    reminder: bool,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: String::from(""),
            text: String::from(""),
            day: String::from(""),
            reminder: false,
        }
    }
}

impl Task {
    pub fn new(id: String, text: String, day: String, reminder: bool) -> Self {
        Task {
            id: id,
            text: text,
            day: day,
            reminder: reminder
        }
    }
}
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{serde::{Serialize, Deserialize}};

use crate::{TaskId};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Task {
    pub task_id: usize,
    text: String,
    day: String,
    reminder: bool,
}

impl Task {
    pub fn new(
        task_id: usize,
        text: String,
        day: String,
        reminder: bool) -> Self {

        Self {
            task_id,
            text,
            day,
            reminder
        }
    }

    pub fn get_task_id(&self) -> TaskId {
        self.task_id
    }

    pub fn get_task_content(&self) -> String {
        self.text.clone()
    }
}
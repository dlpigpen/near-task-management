/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{env, near_bindgen, setup_alloc};
extern crate uuid;
use uuid::Uuid;

pub mod account;

setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize)]
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

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tasks {
    tasks: LookupMap<String, Vec<Task>>,
}

impl Default for Tasks {
    fn default() -> Self {
        Self {
            tasks: LookupMap::new(b"a".to_vec()),
        }
    }
}

#[near_bindgen]
impl Tasks {
    pub fn add_task(&mut self, text: String, day: String, reminder: bool) {
        let account_id = env::signer_account_id();
        let contains_user = self.tasks.contains_key(&account_id);

        let udid = Uuid::new_v4().to_hyphenated_string();
        let task = Task::new( 
            udid,
            text,
            day,
             reminder
        );

        if contains_user {
            let mut templ_list = match self.tasks.get(&account_id) {
                Some(x) => x,
                None => vec![]
            };

            templ_list.push(task);
            self.tasks.insert(&account_id, &templ_list);
            
        } else {
            let fresh_vec = vec![task];
            self.tasks.insert(&account_id, &fresh_vec);
        }
    }

    pub fn get_tasks(self, user: String) -> Vec<Task> {
        let tasks = match self.tasks.get(&user) {
            Some(x) => x,
            None => vec![],
        };
        return tasks;
    }
}
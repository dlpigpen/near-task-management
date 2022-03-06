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
use near_sdk::collections::{LookupMap};
use near_sdk::{env, near_bindgen, setup_alloc};

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

        let random_id = self.generate_id();
        let task = Task::new( 
            random_id,
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

    #[result_serializer(borsh)]
    pub fn get_tasks(&self) -> Vec<Task> {
        let account_id = env::signer_account_id();
        let tasks = match self.tasks.get(&account_id) {
            Some(x) => x,
            None => vec![],
        };
        return tasks;
    }

    fn generate_id(&self) -> String {
        let account_id = env::signer_account_id();
        let total_task = self.get_total_task();
        let next_id = (total_task + 1).to_string();
        let new_id = [account_id, next_id].join("-");
        return new_id;
    }

    pub fn delete_a_task(&mut self, task_id: String) -> bool {
        let account_id = env::signer_account_id();
        let mut user_tasks = self.get_tasks();
        if user_tasks.len() > 0 {
            let index = user_tasks.iter().position(|x| *x.id == task_id).unwrap();
            user_tasks.remove(index);
            self.tasks.insert(&account_id, &user_tasks);
            return true
        }
        return false
    }

    pub fn get_total_task(&self) -> usize {
        let tasks  = self.get_tasks();
        return tasks.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "mitsori1.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn get_current_total_task() {
        let context = get_context(String::from("mitsori1.testnet"), 0);
        testing_env!(context);
        let mut contract = Tasks::default();
        contract.add_task(
            String::from("day1"),
            String::from("date1"),
            true
        );
        let total_task = contract.get_total_task();
        println!("Total task after increment: {}", total_task);
        assert_eq!(1, total_task, "Expected empty");
    }

    #[test]
    fn get_task_id() {
        let context = get_context(String::from("mitsori1.testnet"), 0);
        testing_env!(context);
        let contract = Tasks::default();
        let unique_id = contract.generate_id();
        println!("Unique id: {}", unique_id);
        assert_eq!(unique_id, "mitsori1.testnet-1", "Does not match id");
    }

    #[test]
    fn get_delete_task() {
        let context = get_context(String::from("mitsori1.testnet"), 0);
        testing_env!(context);
        let mut contract = Tasks::default();
         contract.add_task(
            String::from("day1"),
            String::from("date1"),
            true
        );
        let total_after_added = contract.get_total_task();
        assert_eq!(1, total_after_added, "Add function broken");

        let tasks = contract.get_tasks();
        let first_task = tasks.first().unwrap();
        assert_eq!(first_task.id, "mitsori1.testnet-1", "id does not match");

        let task_id = String::from(&first_task.id);
        contract.delete_a_task(task_id);
        let total_after_deleted = contract.get_total_task();
        assert_eq!(0, total_after_deleted, "After deleted wrong number");

    }

}
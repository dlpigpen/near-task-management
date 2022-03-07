use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId};
use task::Task;

setup_alloc!();

type TaskId = usize;
mod task;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tasks {
    pub next_task_id: usize,
    pub users_tasks: UnorderedMap<AccountId, Vec<usize>>,
    pub tasks: UnorderedMap<TaskId, Task>,
}

impl Default for Tasks {
    fn default() -> Self {
        Self {
            users_tasks: UnorderedMap::new(b"up".to_vec()),
            tasks: UnorderedMap::new(b"p".to_vec()),
            next_task_id: 0,
        }
    }
}

#[near_bindgen]
impl Tasks {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The state has been already initialized");
        Self {
           users_tasks: UnorderedMap::new(b"up".to_vec()),
            tasks: UnorderedMap::new(b"p".to_vec()),
            next_task_id: 0,
        }
    }

    pub fn create_task(&mut self, text: String, day: String, reminder: bool) -> usize {
        let task_id = self.get_next_task_id();

        let task = Task::new(
            task_id,
            text,
            day,
            reminder
        );

        self.tasks.insert(&task_id, &task);
        self.next_task_id = self.increase_post_id();

        let account_id = env::predecessor_account_id();
        let mut users_tasks = self.users_tasks.get(&account_id).unwrap_or(vec![]);
        users_tasks.push(task_id);
        self.users_tasks.insert(&account_id, &users_tasks);

        let task_content = task.get_task_content();
        env::log(format!("Task '{}' was created: ", task_content).as_bytes());

        task_id
    }

    pub fn get_user_tasks(&self, account_id: AccountId) -> Vec<Task> {
        let task_ids = self.users_tasks.get(&account_id).unwrap_or(vec![]);

        let mut u_tasks = Vec::new();
        for task_id in task_ids {
            let task = self.tasks.get(&task_id).unwrap();
            u_tasks.push(task);
        }
        u_tasks
    }

   pub fn get_task_by_id(&self, task_id: usize) -> Option<Task> {
        self.tasks.get(&task_id)
    }

    pub fn get_tasks(&self) -> Vec<Task> {
        let mut tasks = Vec::new();
        for task_id in self.tasks.keys() {
            tasks.push(self.tasks.get(&task_id).unwrap());
        }
        tasks
    }

    pub fn get_total_task(&self) -> u64 {
        self.tasks.len()
    }

    pub fn get_user_total_task(&self, account_id: String) -> usize {
        let utasks = self.users_tasks.get(&account_id).unwrap_or(vec![]);
        utasks.len()
    }

    pub fn delete_task_by_id(&mut self, task_id: usize) {
        let account_id = env::predecessor_account_id();
        self.tasks.remove(&task_id);

        let mut  utasks = self.users_tasks
            .get(&account_id)
            .unwrap_or(vec![]);
        utasks.retain(|&x| x != task_id);
        self.users_tasks.insert(&account_id, &utasks);

    }

    pub fn get_next_task_id(&self) -> usize {
        self.next_task_id
    }

    pub fn increase_post_id(&self) -> usize {
        self.next_task_id + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

   fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "tasktracker.mitsori1.testnet".to_string(),
            signer_account_id: "mitsori1.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "mitsori1.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1000000000000000000000000,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn create_task() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Tasks::default();
        contract.create_task(
        String::from("Task 1"),
        String::from("Feb 7"),
        false,
        );

        // log id
        env::log(format!("Task added: {}",
             contract.get_task_by_id(0)
                    .unwrap()
                    .get_task_content())
                    .as_bytes()
        );

        assert_eq!("Task 1".to_string(), contract.get_task_by_id(0).unwrap().get_task_content());

        let user_tasks = contract.get_user_tasks("mitsori1.testnet".to_string());
        assert_eq!(1, user_tasks.len());
        assert_eq!(0, user_tasks[0].get_task_id());
    }

    #[test]
    fn delete_task() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Tasks::default();
        contract.create_task(
            String::from("Task 1"),
            String::from("Feb 7"),
            false,
        );
        contract.delete_task_by_id(0);
        assert_eq!(0, contract.get_total_task(), "Delete does not work");

        assert_eq!(0, contract.get_user_total_task(String::from("mitsori1.testnet")), "User total post does not work");
    }
}
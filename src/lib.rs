use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct HelloWorld {}

#[near_bindgen]
impl HelloWorld {
    pub fn call(&self, name: String) -> String {
        let print_message = format!("Hello {}!", name);
        env::log(print_message.as_bytes());
        return print_message;
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new().is_view(is_view).build()
    }

    #[test]
    fn call() {
        let context = get_context(true);
        testing_env!(context);
        let contract = HelloWorld::default();
        assert_eq!("Hello Bob!".to_string(), contract.call("Bob".to_string()));
    }
}

/*
 * This is a Rust smart contract with these functions:
 *
 * 1. add_friend_request: accepts a friend request from current user asking  nextfriend.near to join his/her friendlist
 * 2. confirm_friend_request: nextfriend.near accepts friend request. nextfrined and current_user both appear on each others friendslist
 * 3. get_friend: find friend by account id
 * 4. get_friends_list: list of all current user's friends 
 * 5. get_pending_requests: list of all current user's list of users who have requested friendship
 * 6. TODO: hide friendslist from public
 *  
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc,  AccountId};
use near_sdk::collections::UnorderedMap;

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FriendsContract {
    list_owner: AccountId,
    friends: UnorderedMap<AccountId, String>,
}

impl Default for FriendsContract {
    fn default() -> Self {
        Self {
            list_owner: env::predecessor_account_id(),
            friends: UnorderedMap::new(b"s".to_vec()),
        }
    }
  }

#[near_bindgen]
impl FriendsContract {
    #[init]
    pub fn new() -> Self {
        // Initializing `friends` with unique key prefix.
        Self {
            list_owner: env::predecessor_account_id(),
            friends: UnorderedMap::new(b"s".to_vec()),
        }
    }

    pub fn add_friend_request(&mut self, friend_id: String) {
        self.list_owner = env::signer_account_id();
        
        let status = String::from("pending");
        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("adding Friend with ID: '{}' as '{}' for account '{}'", friend_id, status, self.list_owner).as_bytes());
        
        self.friends.insert(&friend_id, &status);
    }

    // `match` is similar to `switch` in other languages; here we use it to default to "No friend" if
    // self.friend.get(&friend_id) is not yet defined.
    // Learn more: https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
    pub fn get_friend(&self, friend_id: AccountId) -> String {
        match self.friends.get(&friend_id) {
            Some(_value) => friend_id.clone(),
            None => "No friend".to_string(),
        }
    }

    //get all friends fro UnorderedMap: https://www.near-sdk.io/contract-structure/collections
    pub fn get_friends(&self, from_index: u64, limit: u64) -> Vec<(AccountId, String)> {
        let keys = self.friends.keys_as_vector();
        let values = self.friends.values_as_vector();
        (from_index..std::cmp::min(from_index + limit, self.friends.len()))
            .map(|index| (keys.get(index).unwrap(), values.get(index).unwrap()))
            .collect()
    }


}

/*
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
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
    fn add_friend_request_then_get_friend() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = FriendsContract::default();

        contract.add_friend_request("howdy.near".to_string());

        assert_eq!(
            "howdy.near".to_string(),
            contract.get_friend("howdy.near".to_string())
        );
    }

}

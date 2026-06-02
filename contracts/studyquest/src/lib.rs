#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String,
};

#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Quest {
    pub title: String,
    pub target: u32,
    pub progress: u32,
    pub completed: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Quest(Address),
}

#[contractimpl]
impl Contract {
    pub fn create_quest(
        env: Env,
        user: Address,
        title: String,
        target: u32,
    ) -> Quest {
        user.require_auth();

        if target == 0 {
            panic!("target must be greater than zero");
        }

        let key = DataKey::Quest(user.clone());

        if env.storage().persistent().has(&key) {
            panic!("quest already exists");
        }

        let quest = Quest {
            title,
            target,
            progress: 0,
            completed: false,
        };

        env.storage().persistent().set(&key, &quest);

        quest
    }

    pub fn add_progress(
        env: Env,
        user: Address,
        amount: u32,
    ) -> Quest {
        user.require_auth();

        let key = DataKey::Quest(user.clone());

        let mut quest: Quest = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("quest not found"));

        if !quest.completed {
            let new_progress = quest.progress.saturating_add(amount);

            if new_progress >= quest.target {
                quest.progress = quest.target;
                quest.completed = true;
            } else {
                quest.progress = new_progress;
            }

            env.storage().persistent().set(&key, &quest);
        }

        quest
    }

    pub fn complete_quest(
        env: Env,
        user: Address,
    ) -> Quest {
        user.require_auth();

        let key = DataKey::Quest(user.clone());

        let mut quest: Quest = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("quest not found"));

        quest.progress = quest.target;
        quest.completed = true;

        env.storage().persistent().set(&key, &quest);

        quest
    }

    pub fn get_quest(
        env: Env,
        user: Address,
    ) -> Quest {
        let key = DataKey::Quest(user);

        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("quest not found"))
    }

    pub fn has_quest(
        env: Env,
        user: Address,
    ) -> bool {
        let key = DataKey::Quest(user);

        env.storage().persistent().has(&key)
    }
}

mod test;
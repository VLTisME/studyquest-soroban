#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

#[test]
fn test_create_and_update_quest() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let title = String::from_str(&env, "Finish Stellar workshop");

    let created = client.create_quest(&user, &title, &100);

    assert_eq!(created.title, title);
    assert_eq!(created.target, 100);
    assert_eq!(created.progress, 0);
    assert_eq!(created.completed, false);

    assert_eq!(client.has_quest(&user), true);

    let updated = client.add_progress(&user, &40);

    assert_eq!(updated.target, 100);
    assert_eq!(updated.progress, 40);
    assert_eq!(updated.completed, false);

    let completed = client.complete_quest(&user);

    assert_eq!(completed.progress, 100);
    assert_eq!(completed.completed, true);

    let saved = client.get_quest(&user);

    assert_eq!(saved.progress, 100);
    assert_eq!(saved.completed, true);
}
use cosmwasm_std::{coin, coins, Addr};
use cw_multi_test::App;

use crate::error::ContractError;
use crate::msg::ValueResp;
use crate::state::{State, STATE};

use super::contract::CountingContract;
use counting_contract_0_1::multitest::contract::CountingContract as CountingContract_0_1;
// used counting_contract_0_1 to avoid name conflict with the current contract, which is also called CountingContract. In addition, this is a path to counting contract v1.0.0, which we set in dev-dependencies in Cargo.toml

const ATOM: &str = "atom";

#[test]
fn query_value() {
    let owner = Addr::unchecked("owner");

    let mut app = App::default();

    let code_id = CountingContract::store_code(&mut app);
    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
        10,
        coin(10, ATOM),
    )
    .unwrap();

    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 10 });
}

#[test]
fn donate() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::default();

    let code_id = CountingContract::store_code(&mut app);
    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
        0,
        coin(10, ATOM),
    )
    .unwrap();

    contract.donate(&mut app, &sender, &[]).unwrap();

    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 0 });
}

#[test]
fn donate_with_funds() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, ATOM))
            .unwrap();
    });

    let code_id = CountingContract::store_code(&mut app);
    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
        None,
        coin(10, ATOM),
    )
    .unwrap();

    contract
        .donate(&mut app, &sender, &coins(10, ATOM))
        .unwrap();

    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 1 });
}

#[test]
fn expecting_no_funds() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::default();

    let code_id = CountingContract::store_code(&mut app);
    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
        None,
        coin(0, ATOM),
    )
    .unwrap();

    contract.donate(&mut app, &sender, &[]).unwrap();

    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 1 });
}

#[test]
fn reset() {
    let owner = Addr::unchecked("owner");

    let mut app = App::default();

    let code_id = CountingContract::store_code(&mut app);
    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
        None,
        coin(10, ATOM),
    )
    .unwrap();

    contract.reset(&mut app, &owner, 10).unwrap();

    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 10 });
}

#[test]
fn withdraw() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap();
    });

    let code_id = CountingContract::store_code(&mut app);
    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
        None,
        coin(10, ATOM),
    )
    .unwrap();

    contract
        .donate(&mut app, &sender, &coins(10, ATOM))
        .unwrap();

    contract.withdraw(&mut app, &owner).unwrap();

    assert_eq!(
        app.wrap().query_all_balances(owner).unwrap(),
        coins(10, "atom")
    );
    assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        vec![]
    );
}

#[test]
fn withdraw_to() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");
    let receiver = Addr::unchecked("receiver");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap();
    });

    let code_id = CountingContract::store_code(&mut app);
    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
        None,
        coin(10, ATOM),
    )
    .unwrap();

    contract
        .donate(&mut app, &sender, &coins(10, ATOM))
        .unwrap();

    contract
        .withdraw_to(&mut app, &owner, &receiver, coins(5, ATOM))
        .unwrap();

    assert_eq!(app.wrap().query_all_balances(owner).unwrap(), vec![]);
    assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
    assert_eq!(
        app.wrap().query_all_balances(receiver).unwrap(),
        coins(5, "atom")
    );
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        coins(5, "atom")
    );
}

#[test]
fn unauthorized_withdraw() {
    let owner = Addr::unchecked("owner");
    let member = Addr::unchecked("member");

    let mut app = App::default();

    let code_id = CountingContract::store_code(&mut app);
    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
        None,
        coin(10, ATOM),
    )
    .unwrap();

    let err = contract.withdraw(&mut app, &member).unwrap_err();
    assert_eq!(
        err,
        ContractError::Unauthorized {
            owner: owner.into()
        },
    );
}

#[test]
fn unauthorized_withdraw_to() {
    let owner = Addr::unchecked("owner");
    let member = Addr::unchecked("member");

    let mut app = App::default();

    let code_id = CountingContract::store_code(&mut app);
    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
        None,
        coin(10, ATOM),
    )
    .unwrap();

    let err = contract
        .withdraw_to(&mut app, &member, &owner, vec![])
        .unwrap_err();
    assert_eq!(
        err,
        ContractError::Unauthorized {
            owner: owner.into()
        },
    );
}

#[test]
fn unauthorized_reset() {
    let owner = Addr::unchecked("owner");
    let member = Addr::unchecked("member");

    let mut app = App::default();

    let code_id = CountingContract::store_code(&mut app);
    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        None,
        None,
        coin(10, ATOM),
    )
    .unwrap();

    let err = contract.reset(&mut app, &member, 10).unwrap_err();
    assert_eq!(
        err,
        ContractError::Unauthorized {
            owner: owner.into()
        },
    );
}


#[test]
fn migration() {
    let admin = Addr::unchecked("admin");
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap();
    }); 
    // using bank router, initialized balance with 10 atom to sender

    let old_code_id = CountingContract_0_1::store_code(&mut app);
    let new_code_id = CountingContract::store_code(&mut app);

    let contract = CountingContract_0_1::instantiate(
        &mut app,
        old_code_id,
        &owner,
        "Counting contract",
        &admin,
        None,
        coin(10, ATOM),
    )
    .unwrap();
    // instantiate old contract

    contract
        .donate(&mut app, &sender, &coins(10, ATOM))
        .unwrap();
    // donate 10 atom to contract

    let contract =
        CountingContract::migrate(&mut app, contract.into(), new_code_id,
        &admin).unwrap();
        // contract.into() converts the old contract to a new contract, which would use the new code_id
    // migrate old contract to new contract
    // the sender would be the admin, who is allowed to migrate the old contract and is the owner of the new contract

    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 1 });
    // query value of new contract
    // the value should be 1, because the old contract donated 10 atom to the new contract and the counter was incremented by 1

    let state = STATE.query(&app.wrap(), contract.addr().clone()).unwrap();
    // query state of new contract
    assert_eq!(
        state,
        State {
            counter: 1,
            minimal_donation: coin(10, ATOM),
            owner,
        }
    );
    // assert that the state of the new contract is correct
}

#[test]
fn migration_same_version() {
    let admin = Addr::unchecked("admin");
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap();
    }); // using router, api, and storage to initialize balance with 10 atom to sender
    // || is a closure, which is a function that can be passed as an argument

    let code_id = CountingContract::store_code(&mut app);
    // CountingContract is a struct that has a function called store_code
    // store_code is a function that takes a mutable reference to app and returns a code_id
    // &mut app is a mutable reference to app, which is passed to store_code
    // app is a mutable reference to App, App is a struct that refers to diffremt components of the blockchain like bank, router, api, and storage
    // assigning this to code_id

    let contract = CountingContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Counting contract",
        &admin,
        None,
        coin(10, ATOM),
    )
    .unwrap();
    // instantiate contract

    contract
        .donate(&mut app, &sender, &coins(10, ATOM))
        .unwrap();
        // donate 10 atom to contract

    let resp = contract.query_value(&app).unwrap();
    // assigning resp to the value of the contract

    assert_eq!(resp, ValueResp { value: 1 });
    // value should be 1, because the contract donated 10 atom to itself and the counter was incremented by 1

    let state = STATE.query(&app.wrap(), contract.addr().clone()).unwrap();
    // assigning state to the state of the contract by querying the state

    assert_eq!(
        state,
        State {
            counter: 1,
            minimal_donation: coin(10, ATOM),
            owner,
        }
    ); // assert that the state of the contract is correct, the state should be 1, the minimal donation should be 10 atom, and the owner should be owner
}
// test to ensure that the old contract can be migrated to the new one
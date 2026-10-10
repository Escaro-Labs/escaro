extern crate std;

use super::*;
use soroban_sdk::{IntoVal,
    testutils::{Address as _, Events as _, MockAuth, MockAuthInvoke},
    token::{StellarAssetClient, TokenClient},
    Address, Env, Event as _, String,
};

struct Setup<'a> {
    env: Env,
    escrow: EscrowClient<'a>,
    usdc: TokenClient<'a>,
    maintainer: Address,
    contributor: Address,
    repo: String,
    other_repo: String,
}

/// Blanket `mock_all_auths` is used only by the pure logic tests. The auth
/// tests below scope auth to exactly one address via `mock_auths`, so the
/// suite proves *who* is authorised, not just that some auth exists.
fn setup() -> Setup<'static> {
    let env = Env::default();
    env.mock_all_auths();

    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer);
    let maintainer = Address::generate(&env);
    StellarAssetClient::new(&env, &sac.address()).mint(&maintainer, &10_000);

    let id = env.register(Escrow, (sac.address(),));
    Setup {
        env: env.clone(),
        escrow: EscrowClient::new(&env, &id),
        usdc: TokenClient::new(&env, &sac.address()),
        contributor: Address::generate(&env),
        repo: String::from_str(&env, "stellar/stellar-cli"),
        other_repo: String::from_str(&env, "stellar/rs-soroban-env"),
        maintainer,
    }
}

#[test]
fn fund_locks_tokens_in_the_contract() {
    let s = setup();
    let id = s.escrow.fund(&s.maintainer, &s.repo, &842, &900);

    assert_eq!(id, 1);
    assert_eq!(s.usdc.balance(&s.maintainer), 9_100);
    assert_eq!(s.usdc.balance(&s.escrow.address), 900);
    let bounty = s.escrow.get(&id);
    assert_eq!(bounty.status, Status::Open);
    assert_eq!(bounty.assignee, None);
    assert_eq!(s.escrow.bounty_for(&s.repo, &842), Some(id));
}

#[test]
fn release_pays_the_assignee_and_emits_a_receipt() {
    let s = setup();
    let id = s.escrow.fund(&s.maintainer, &s.repo, &842, &900);
    s.escrow.assign(&id, &Some(s.contributor.clone()));
    let pr = String::from_str(&s.env, "https://github.com/stellar/stellar-cli/pull/1234");
    s.escrow.release(&id, &pr);

    let receipt = Receipt {
        id,
        contributor: s.contributor.clone(),
        repo: s.repo.clone(),
        issue: 842,
        pull_request: pr,
        amount: 900,
    };
    let events = s.env.events().all();
    assert!(events
        .events()
        .contains(&receipt.to_xdr(&s.env, &s.escrow.address)));

    assert_eq!(s.usdc.balance(&s.contributor), 900);
    assert_eq!(s.usdc.balance(&s.escrow.address), 0);
    assert_eq!(s.escrow.get(&id).status, Status::Paid);
    assert_eq!(s.escrow.bounty_for(&s.repo, &842), None);
}

#[test]
fn refund_returns_tokens_to_the_maintainer() {
    let s = setup();
    let id = s.escrow.fund(&s.maintainer, &s.repo, &842, &900);
    s.escrow.refund(&id);

    assert_eq!(s.usdc.balance(&s.maintainer), 10_000);
    assert_eq!(s.escrow.get(&id).status, Status::Refunded);
    assert_eq!(s.escrow.fund(&s.maintainer, &s.repo, &842, &500), 2);
}

#[test]
fn release_requires_an_assignee() {
    let s = setup();
    let id = s.escrow.fund(&s.maintainer, &s.repo, &842, &900);
    let pr = String::from_str(&s.env, "https://github.com/stellar/stellar-cli/pull/1");
    assert_eq!(s.escrow.try_release(&id, &pr), Err(Ok(Error::NotAssigned)));
}

#[test]
fn closed_bounties_cannot_be_paid_twice() {
    let s = setup();
    let id = s.escrow.fund(&s.maintainer, &s.repo, &842, &900);
    s.escrow.assign(&id, &Some(s.contributor.clone()));
    let pr = String::from_str(&s.env, "https://github.com/stellar/stellar-cli/pull/1");
    s.escrow.release(&id, &pr);

    assert_eq!(s.escrow.try_release(&id, &pr), Err(Ok(Error::NotOpen)));
    assert_eq!(s.escrow.try_refund(&id), Err(Ok(Error::NotOpen)));
    assert_eq!(s.escrow.try_assign(&id, &None), Err(Ok(Error::NotOpen)));
}

#[test]
fn an_issue_cannot_hold_two_open_bounties() {
    let s = setup();
    s.escrow.fund(&s.maintainer, &s.repo, &842, &900);
    assert_eq!(
        s.escrow.try_fund(&s.maintainer, &s.repo, &842, &100),
        Err(Ok(Error::AlreadyFunded))
    );
}

#[test]
fn amount_must_be_positive() {
    let s = setup();
    assert_eq!(
        s.escrow.try_fund(&s.maintainer, &s.repo, &842, &0),
        Err(Ok(Error::InvalidAmount))
    );
    assert_eq!(
        s.escrow.try_fund(&s.maintainer, &s.repo, &842, &-5),
        Err(Ok(Error::InvalidAmount))
    );
}

#[test]
fn unknown_bounty_is_not_found() {
    let s = setup();
    assert_eq!(s.escrow.try_get(&99), Err(Ok(Error::NotFound)));
}

/// The issue's headline scenario: three bounties across two repos. Ids are
/// handed out sequentially, each issue maps to its own bounty, and closing
/// one leaves the others untouched.
#[test]
fn three_bounties_are_isolated_across_two_repos() {
    let s = setup();

    let first = s.escrow.fund(&s.maintainer, &s.repo, &1, &100);
    let second = s.escrow.fund(&s.maintainer, &s.other_repo, &7, &200);
    let third = s.escrow.fund(&s.maintainer, &s.repo, &2, &300);

    // The counter is shared across repos: ids 1, 2, 3 in funding order.
    assert_eq!(first, 1);
    assert_eq!(second, 2);
    assert_eq!(third, 3);

    // Each issue resolves to its own bounty, and unknown ones resolve to none.
    assert_eq!(s.escrow.bounty_for(&s.repo, &1), Some(first));
    assert_eq!(s.escrow.bounty_for(&s.other_repo, &7), Some(second));
    assert_eq!(s.escrow.bounty_for(&s.repo, &2), Some(third));
    assert_eq!(s.escrow.bounty_for(&s.repo, &999), None);
    assert_eq!(s.escrow.bounty_for(&String::from_str(&s.env, "x/y"), &1), None);

    // Every bounty is visible with its own amount, independently of the others.
    assert_eq!(s.escrow.get(&first).amount, 100);
    assert_eq!(s.escrow.get(&second).amount, 200);
    assert_eq!(s.escrow.get(&third).amount, 300);

    // The contract escrows each bounty separately.
    assert_eq!(s.usdc.balance(&s.escrow.address), 600);
    assert_eq!(s.usdc.balance(&s.maintainer), 10_000 - 600);
}

/// Acceptance criterion: releasing one bounty does not change another's
/// escrowed balance or state.
#[test]
fn releasing_one_bounty_leaves_the_others_open_and_escrowed() {
    let s = setup();

    let first = s.escrow.fund(&s.maintainer, &s.repo, &1, &100);
    let second = s.escrow.fund(&s.maintainer, &s.other_repo, &7, &200);
    let third = s.escrow.fund(&s.maintainer, &s.repo, &2, &300);

    s.escrow.assign(&second, &Some(s.contributor.clone()));
    let pr = String::from_str(&s.env, "https://github.com/stellar/rs-soroban-env/pull/5");
    s.escrow.release(&second, &pr);

    assert_eq!(s.usdc.balance(&s.contributor), 200);

    // The other two keep their tokens escrowed and stay open.
    assert_eq!(s.escrow.get(&first).status, Status::Open);
    assert_eq!(s.escrow.get(&third).status, Status::Open);
    assert_eq!(s.escrow.bounty_for(&s.repo, &1), Some(first));
    assert_eq!(s.escrow.bounty_for(&s.repo, &2), Some(third));
    assert_eq!(s.usdc.balance(&s.escrow.address), 400);

    // The released issue is free to be funded again.
    assert_eq!(s.escrow.bounty_for(&s.other_repo, &7), None);
    assert_eq!(s.escrow.fund(&s.maintainer, &s.other_repo, &7, &50), 4);
}

/// Acceptance criterion: refunding one bounty does not change another's
/// escrowed balance or state.
#[test]
fn refunding_one_bounty_leaves_the_others_open_and_escrowed() {
    let s = setup();

    let first = s.escrow.fund(&s.maintainer, &s.repo, &1, &100);
    let second = s.escrow.fund(&s.maintainer, &s.other_repo, &7, &200);
    let third = s.escrow.fund(&s.maintainer, &s.repo, &2, &300);

    s.escrow.refund(&first);

    // Only the refunded amount returns to the maintainer.
    assert_eq!(s.usdc.balance(&s.maintainer), 10_000 - 500);
    assert_eq!(s.escrow.get(&first).status, Status::Refunded);

    // The other two keep their tokens escrowed and stay open.
    assert_eq!(s.escrow.get(&second).status, Status::Open);
    assert_eq!(s.escrow.get(&third).status, Status::Open);
    assert_eq!(s.usdc.balance(&s.escrow.address), 500);
    assert_eq!(s.escrow.bounty_for(&s.other_repo, &7), Some(second));
    assert_eq!(s.escrow.bounty_for(&s.repo, &2), Some(third));
}

/// Auth scoped to the maintainer only: a full fund cycle is authorised by the
/// maintainer, never the assignee, and the funded amount actually moves.
#[test]
fn only_the_maintainer_can_move_funds() {
    let env = Env::default();
    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer);
    let maintainer = Address::generate(&env);
    // Setup (mint, register) uses the blanket mock; every call under test
    // below switches to auth scoped to exactly one address.
    env.mock_all_auths();
    StellarAssetClient::new(&env, &sac.address()).mint(&maintainer, &10_000);
    let id = env.register(Escrow, (sac.address(),));
    let escrow = EscrowClient::new(&env, &id);
    let contributor = Address::generate(&env);
    let repo = String::from_str(&env, "stellar/stellar-cli");

    // Auth for exactly the maintainer, nothing else.
    let token = sac.address();
    env.mock_auths(&[MockAuth {
        address: &maintainer,
        invoke: &MockAuthInvoke {
            contract: &id,
            fn_name: "fund",
            args: (maintainer.clone(), repo.clone(), 842u32, 900i128).into_val(&env),
            sub_invokes: &[MockAuthInvoke {
                contract: &token,
                fn_name: "transfer",
                args: (maintainer.clone(), id.clone(), 900i128).into_val(&env),
                sub_invokes: &[],
            }],
        },
    }]);
    let bounty = escrow.fund(&maintainer, &repo, &842, &900);

    // The token transfer inside fund is authorised by the maintainer.
    let fund_auths = env.auths();
    assert_eq!(fund_auths.len(), 1);
    assert_eq!(fund_auths[0].0, maintainer);

    env.mock_auths(&[MockAuth {
        address: &maintainer,
        invoke: &MockAuthInvoke {
            contract: &id,
            fn_name: "assign",
            args: (bounty, Option::Some(contributor.clone())).into_val(&env),
            sub_invokes: &[],
        },
    }]);
    escrow.assign(&bounty, &Some(contributor.clone()));

    let pr = String::from_str(&env, "https://github.com/stellar/stellar-cli/pull/1");
    env.mock_auths(&[MockAuth {
        address: &maintainer,
        invoke: &MockAuthInvoke { contract: &id, fn_name: "release", args: (bounty, pr.clone()).into_val(&env), sub_invokes: &[] },
    }]);
    escrow.release(&bounty, &pr);

    // Assign and release are authorised by the maintainer, never the assignee.
    let move_auths = env.auths();
    assert_eq!(move_auths.len(), 1);
    assert_eq!(move_auths[0].0, maintainer);
    assert_ne!(move_auths[0].0, contributor);

    // And the released amount actually landed with the contributor.
    let usdc = TokenClient::new(&env, &sac.address());
    assert_eq!(usdc.balance(&contributor), 900);
    assert_eq!(usdc.balance(&escrow.address), 0);
}

/// Negative auth test with a specific expected failure: no authorisation at
/// all means `require_auth` aborts with `HostAuthError`.
#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")]
fn release_without_maintainer_auth_fails() {
    let s = setup();
    let id = s.escrow.fund(&s.maintainer, &s.repo, &842, &900);
    s.escrow.assign(&id, &Some(s.contributor.clone()));

    // Wipe the blanket mock: the release call finds no authorisation.
    s.env.set_auths(&[]);
    s.escrow.release(
        &id,
        &String::from_str(&s.env, "https://github.com/stellar/stellar-cli/pull/1"),
    );
}

/// Auth that names the wrong address (the contributor, not the maintainer)
/// must not satisfy the contract's maintainer requirement.
#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")]
fn release_authorized_for_the_contributor_instead_of_the_maintainer_fails() {
    let s = setup();
    let id = s.escrow.fund(&s.maintainer, &s.repo, &842, &900);
    s.escrow.assign(&id, &Some(s.contributor.clone()));

    s.env.set_auths(&[]);
    s.env.mock_auths(&[MockAuth {
        address: &s.contributor,
        invoke: &MockAuthInvoke {
            contract: &s.escrow.address,
            fn_name: "release",
            args: (id, String::from_str(&s.env, "https://github.com/stellar/stellar-cli/pull/1")).into_val(&s.env),
            sub_invokes: &[],
        },
    }]);
    s.escrow.release(
        &id,
        &String::from_str(&s.env, "https://github.com/stellar/stellar-cli/pull/1"),
    );
}

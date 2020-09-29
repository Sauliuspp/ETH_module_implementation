mod beacon_chain;

fn main() {
    let validator1 = beacon_chain::Validator {
        effective_balance: 50,
        slashed: true,
        activation_eligibility_epoch: 5,
        activation_epoch: 5,
        exit_epoch: 50,
        withdrawable_epoch: 50,
        next_custody_secret_to_reveal: 5,
        max_reveal_lateness: 50,
    };

    let validator2 = beacon_chain::Validator {
        effective_balance: 10,
        slashed: true,
        activation_eligibility_epoch: 10,
        activation_epoch: 1,
        exit_epoch: 10,
        withdrawable_epoch: 10,
        next_custody_secret_to_reveal: 10,
        max_reveal_lateness: 10,
    };

    let beacon_state = beacon_chain::BeaconState {
        validators: vec![validator1, validator2],
    };

    beacon_chain::get_committee_count_per_slot(&beacon_state, 7);
    //let result = beacon_chain::get_committee_count_per_slot(&beacon_state, 7);
    //println!("{} ", result);

    println!("Hello, world!");
}

#[cfg(test)]
#[test]
fn test1() {
    let validator1 = beacon_chain::Validator {
        effective_balance: 50,
        slashed: true,
        activation_eligibility_epoch: 5,
        activation_epoch: 5,
        exit_epoch: 50,
        withdrawable_epoch: 50,
        next_custody_secret_to_reveal: 5,
        max_reveal_lateness: 50,
    };

    let validator2 = beacon_chain::Validator {
        effective_balance: 10,
        slashed: true,
        activation_eligibility_epoch: 10,
        activation_epoch: 1,
        exit_epoch: 10,
        withdrawable_epoch: 10,
        next_custody_secret_to_reveal: 10,
        max_reveal_lateness: 10,
    };

    let beacon_state = beacon_chain::BeaconState {
        validators: vec![validator1, validator2],
    };
    assert_eq!(
        1,
        beacon_chain::get_committee_count_per_slot(&beacon_state, 7)
    );
}

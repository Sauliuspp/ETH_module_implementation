mod BeaconChain;

fn main() {
    let mut validator1 = BeaconChain::Validator {
        effective_balance: 50,
        slashed: true,
        activation_eligibility_epoch: 5,
        activation_epoch: 5,
        exit_epoch: 50,
        withdrawable_epoch: 50,
        next_custody_secret_to_reveal: 5,
        max_reveal_lateness: 50,
    };

    let mut validator2 = BeaconChain::Validator {
        effective_balance: 10,
        slashed: true,
        activation_eligibility_epoch: 10,
        activation_epoch: 1,
        exit_epoch: 10,
        withdrawable_epoch: 10,
        next_custody_secret_to_reveal: 10,
        max_reveal_lateness: 10,
    };

    let mut beaconState = BeaconChain::BeaconState{
        validators: vec![validator1, validator2]
    };

    let result = BeaconChain::get_committee_count_per_slot(&beaconState, 7);
    println!("{} ", result);
    
    println!("Hello, world!");
    
}

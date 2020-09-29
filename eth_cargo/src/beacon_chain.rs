use std::cmp;

const SHARD_BLOCK_OFFSETS: [u64; 12] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];
const TARGET_SHARD_BLOCK_SIZE: u64 = 262144;
const GASPRICE_ADJUSTMENT_COEFFICIENT: u64 = 8;
const MAX_GASPRICE: u64 = 16384;
const MIN_GASPRICE: u64 = 8;
const SLOTS_PER_EPOCH: u64 = 32;
const INITIAL_ACTIVE_SHARDS: u64 = 64;
const TARGET_COMMITTEE_SIZE: u64 = 128;

pub struct BeaconState {
    pub validators: Vec<Validator>,
}

pub struct Validator {
    pub effective_balance: u64,
    pub slashed: bool,
    pub activation_eligibility_epoch: u64,
    pub activation_epoch: u64,
    pub exit_epoch: u64,
    pub withdrawable_epoch: u64,
    pub next_custody_secret_to_reveal: u64,
    pub max_reveal_lateness: u64,
}

pub fn get_committee_count_per_slot(state: &BeaconState, epoch: u64) -> u64 {
    // Return the number of committees in each slot for the given ``epoch``.

    std::cmp::max(
        1,
        std::cmp::min(
            get_active_shard_count(state),
            get_active_validator_indices(state, epoch).len() as u64
                / SLOTS_PER_EPOCH
                / TARGET_COMMITTEE_SIZE,
        ),
    )
}

pub fn get_active_validator_indices(state: &BeaconState, epoch: u64) -> Vec<u64> {
    let mut validators = Vec::<u64>::new();
    for (i, v) in state.validators.iter().enumerate() {
        if is_active_validator(v, epoch) {
            validators.push(i as u64);
        }
    }
    validators
}

pub fn is_active_validator(validator: &Validator, epoch: u64) -> bool {
    // Check if ``validator`` is active.

    validator.activation_epoch <= epoch && epoch < validator.exit_epoch
}

pub fn get_active_shard_count(_state: &BeaconState) -> u64 {
    // Return the number of active shards.
    // Note that this puts an upper bound on the number of committees per slot.
    //
    INITIAL_ACTIVE_SHARDS
}

pub fn compute_offset_slots(start_slot: u64, end_slot: u64) -> Vec<u64> {
    let mut vector: Vec<u64> = Vec::new();

    for x in &SHARD_BLOCK_OFFSETS {
        if start_slot + x < end_slot {
            vector.push(start_slot + x);
        }
    }
    vector
}

pub fn compute_epoch_at_slot(slot: u64) -> u64 {
    slot / SLOTS_PER_EPOCH
}

pub fn compute_updated_gasprice(prev_gasprice: u64, shard_block_length: u64) -> u64 {
    if shard_block_length > TARGET_SHARD_BLOCK_SIZE {
        let mut delta = prev_gasprice * (shard_block_length - TARGET_SHARD_BLOCK_SIZE);

        delta /= TARGET_SHARD_BLOCK_SIZE;
        delta /= GASPRICE_ADJUSTMENT_COEFFICIENT;

        cmp::min(prev_gasprice + delta, MAX_GASPRICE)
    } else {
        let mut delta = prev_gasprice * (TARGET_SHARD_BLOCK_SIZE - shard_block_length);

        delta /= TARGET_SHARD_BLOCK_SIZE;
        delta /= GASPRICE_ADJUSTMENT_COEFFICIENT;

        cmp::max(prev_gasprice, MIN_GASPRICE + delta) - delta
    }
}

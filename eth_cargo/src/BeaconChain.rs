use std::cmp;

const SHARD_BLOCK_OFFSETS: [u64; 12] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];
const TARGET_SHARD_BLOCK_SIZE: u64 = 262144;
const GASPRICE_ADJUSTMENT_COEFFICIENT: u64 = 8;
const MAX_GASPRICE: u64 = 16384;
const MIN_GASPRICE: u64 = 8;
const SLOTS_PER_EPOCH : u64 = 32;



// def hash_tree_root(object: SSZSerializable) -> Root



pub fn compute_offset_slots(start_slot: u64, end_slot: u64) -> Vec<u64> {
    let mut vector: Vec<u64> = Vec::new();

    for x in &SHARD_BLOCK_OFFSETS {
        if start_slot + x < end_slot {
            vector.push(start_slot + x);
        }
    }
    return vector;
}



// nesukurtas BeaconState, dar neveiks

//pub fn get_latest_slot_for_shard(state: BeaconState, shard: u64) -> u64 {
//    return state.shard_states[shard].slot;
//}



// nesukurta funkcija, nesukurtas BeaconState
//pub fn get_shard_proposer_index(beacon_state: BeaconState, slot: Slot, shard: Shard) -> ValidatorIndex{
//    """
//    Return the proposer's index of shard block at ``slot``.
//    """
//    epoch = compute_epoch_at_slot(slot)
//    committee = get_shard_committee(beacon_state, epoch, shard)
//    seed = hash(get_seed(beacon_state, epoch, DOMAIN_SHARD_COMMITTEE) + uint_to_bytes(slot))
//    r = bytes_to_uint64(seed[:8])
//    return committee[r % len(committee)]
//}



pub fn compute_epoch_at_slot(slot: u64) -> u64 {
    return slot / SLOTS_PER_EPOCH;
}



//def get_shard_committee(beacon_state: BeaconState, epoch: Epoch, shard: Shard) -> Sequence[ValidatorIndex]:
//    """
//    Return the shard committee of the given ``epoch`` of the given ``shard``.
//    """
//    source_epoch = compute_committee_source_epoch(epoch, SHARD_COMMITTEE_PERIOD)
//    active_validator_indices = get_active_validator_indices(beacon_state, source_epoch)
//    seed = get_seed(beacon_state, source_epoch, DOMAIN_SHARD_COMMITTEE)
//    return compute_committee(
//        indices=active_validator_indices,
//        seed=seed,
//        index=shard,
//        count=get_active_shard_count(beacon_state),
//)



//def hash(data: bytes) -> Bytes32



//def get_seed(state: BeaconState, epoch: Epoch, domain_type: DomainType) -> Bytes32:
//    """
//    Return the seed at ``epoch``.
//    """
//    mix = get_randao_mix(state, Epoch(epoch + EPOCHS_PER_HISTORICAL_VECTOR - MIN_SEED_LOOKAHEAD - 1))  # Avoid underflow
//    return hash(domain_type + uint_to_bytes(epoch) + mix)



// def uint_to_bytes(n: uint) -> bytes



//def bytes_to_uint64(data: bytes) -> uint64:
//    """
//    Return the integer deserialization of ``data`` interpreted as ``ENDIANNESS``-endian.
//    """
//    return uint64(int.from_bytes(data, ENDIANNESS))



pub fn compute_updated_gasprice(prev_gasprice: u64, shard_block_length: u64) -> u64 {
    if shard_block_length > TARGET_SHARD_BLOCK_SIZE {

        let mut delta = prev_gasprice * (shard_block_length - TARGET_SHARD_BLOCK_SIZE);

        delta = delta / TARGET_SHARD_BLOCK_SIZE;
        delta = delta / GASPRICE_ADJUSTMENT_COEFFICIENT;

        return cmp::min(prev_gasprice + delta, MAX_GASPRICE);

    } else {

        let mut delta = prev_gasprice * (TARGET_SHARD_BLOCK_SIZE - shard_block_length);

        delta = delta / TARGET_SHARD_BLOCK_SIZE;
        delta = delta / GASPRICE_ADJUSTMENT_COEFFICIENT;
        
        return cmp::max(prev_gasprice, MIN_GASPRICE + delta) - delta;
    }
}
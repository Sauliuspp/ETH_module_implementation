use std::cmp;

const SHARD_BLOCK_OFFSETS: [u64; 12] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];
const TARGET_SHARD_BLOCK_SIZE: u64 = 262144;
const GASPRICE_ADJUSTMENT_COEFFICIENT: u64 = 8;
const MAX_GASPRICE: u64 = 16384;
const MIN_GASPRICE: u64 = 8;

pub fn compute_offset_slots(start_slot: u64, end_slot: u64) -> Vec<u64> {
    let mut vector: Vec<u64> = Vec::new();

    for x in &SHARD_BLOCK_OFFSETS {
        if start_slot + x < end_slot {
            vector.push(start_slot + x);
        }
    }
    return vector;
}

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
    }}

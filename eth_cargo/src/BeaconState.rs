use bytes::{Bytes, BytesMut, Buf, BufMut};

// nepadarytas structas, cia is ethereum 2.0 phase 0 nukopijuota class beaconstate
// REIKIA SUTVARKYTI IKI GALO

const SLOTS_PER_HISTORICAL_ROOT : u64 = 8192;
const HISTORICAL_ROOTS_LIMIT : u64 = 16777216;
const EPOCHS_PER_ETH1_VOTING_PERIOD : u64 = 32; 
const SLOTS_PER_EPOCH : u64 = 32;
const VALIDATOR_REGISTRY_LIMIT : u64 = 1099511627776;
const EPOCHS_PER_HISTORICAL_VECTOR : u64 = 65536;
const EPOCHS_PER_SLASHINGS_VECTOR : u64 = 8192;
const MAX_ATTESTATIONS : u64 = 128;
const MAX_SHARDS : u64 = 1024;
const MAX_EARLY_DERIVED_SECRET_REVEALS : u64 = 1;
const EARLY_DERIVED_SECRET_PENALTY_MAX_FUTURE_EPOCHS : u64 = 32768;
const JUSTIFICATION_BITS_LENGTH : u64 = 4;
const MAX_CUSTODY_CHUNK_CHALLENGE_RECORDS : u64 = 1048576;

struct BeaconState {
    genesis_time: u64,
    genesis_validators_root: Bytes,
    slot: u64,
    fork: Fork, // nepadarytas struct Fork
    // History
    latest_block_header: BeaconBlockHeader, // nepadarytas struct BeaconBlockHeader
    block_roots: vec![u64, SLOTS_PER_HISTORICAL_ROOT],
    state_roots: vec![u64, SLOTS_PER_HISTORICAL_ROOT],
    historical_roots: vec![u64, HISTORICAL_ROOTS_LIMIT],
    // Eth1
    eth1_data: Eth1Data,    // nepadarytas struct Eth1Data
    eth1_data_votes: vec![Eth1Data, EPOCHS_PER_ETH1_VOTING_PERIOD * SLOTS_PER_EPOCH],
    eth1_deposit_index: u64,
    // Registry
    validators: vec![Validator, VALIDATOR_REGISTRY_LIMIT], // nepadarytas struct Validator
    balances: vec![u64, VALIDATOR_REGISTRY_LIMIT],
    // Randomness
    randao_mixes: vec![u64, EPOCHS_PER_HISTORICAL_VECTOR],
    // Slashings
    slashings: vec![u64, EPOCHS_PER_SLASHINGS_VECTOR],  // Per-epoch sums of slashed effective balances
    // Attestations
    previous_epoch_attestations: vec![PendingAttestation, MAX_ATTESTATIONS * SLOTS_PER_EPOCH],  // nepadarytas struct PendingAttestation
    current_epoch_attestations: vec![PendingAttestation, MAX_ATTESTATIONS * SLOTS_PER_EPOCH], // nepadarytas struct PendingAttestation
    // Finality
    justification_bits: vec![JUSTIFICATION_BITS_LENGTH],  // Bit set for every recent justified epoch
    previous_justified_checkpoint: Checkpoint,  // Previous epoch snapshot
    current_justified_checkpoint: Checkpoint,   //nepadarytas struct Checkpoint
    finalized_checkpoint: Checkpoint,           //nepadarytas struct Checkpoint
    // Phase 1
    current_epoch_start_shard: Shard,           
    shard_states: vec![ShardState, MAX_SHARDS], 
    online_countdown: vec![OnlineEpochs, VALIDATOR_REGISTRY_LIMIT],  // not a raw byte array, considered its large size.
    current_light_committee: CompactCommittee,
    next_light_committee: CompactCommittee,

    // Custody game
    // Future derived secrets already exposed; contains the indices of the exposed validator
    // at RANDAO reveal period % EARLY_DERIVED_SECRET_PENALTY_MAX_FUTURE_EPOCHS
    exposed_derived_secrets: vec![vec![ValidatorIndex, MAX_EARLY_DERIVED_SECRET_REVEALS * SLOTS_PER_EPOCH], EARLY_DERIVED_SECRET_PENALTY_MAX_FUTURE_EPOCHS],
    custody_chunk_challenge_records: vec![CustodyChunkChallengeRecord, MAX_CUSTODY_CHUNK_CHALLENGE_RECORDS],
    custody_chunk_challenge_index: u64
}
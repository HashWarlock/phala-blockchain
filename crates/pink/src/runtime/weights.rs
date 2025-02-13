use core::marker::PhantomData;

use frame_support::weights::Weight;
use pallet_contracts::weights::WeightInfo;
use sp_core::Get;

/// Weights for pallet_contracts using the Substrate node and recommended hardware.
pub struct PinkWeights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PinkWeights<T> {
    // Storage: Contracts DeletionQueue (r:1 w:0)
    fn on_process_deletion_queue_batch() -> Weight {
        Weight::from_ref_time(3_089_000 as u64).saturating_add(T::DbWeight::get().reads(1 as u64))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    /// The range of component `k` is `[0, 1024]`.
    fn on_initialize_per_trie_key(k: u32) -> Weight {
        Weight::from_ref_time(13_917_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(901_000 as u64).saturating_mul(k as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(k as u64)))
    }
    // Storage: Contracts DeletionQueue (r:1 w:0)
    /// The range of component `q` is `[0, 128]`.
    fn on_initialize_per_queue_item(q: u32) -> Weight {
        Weight::from_ref_time(14_172_000 as u64)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(1_301_000 as u64).saturating_mul(q as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Contracts PristineCode (r:1 w:0)
    // Storage: Contracts CodeStorage (r:0 w:1)
    /// The range of component `c` is `[0, 64226]`.
    fn reinstrument(c: u32) -> Weight {
        Weight::from_ref_time(21_644_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(45_000 as u64).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System Account (r:1 w:1)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `c` is `[0, 131072]`.
    fn call_with_code_per_byte(c: u32) -> Weight {
        Weight::from_ref_time(234_349_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(46_000 as u64).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
    // Storage: Contracts CodeStorage (r:1 w:1)
    // Storage: Contracts Nonce (r:1 w:1)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System Account (r:1 w:1)
    // Storage: System EventTopics (r:3 w:3)
    // Storage: Contracts PristineCode (r:0 w:1)
    // Storage: Contracts OwnerInfoOf (r:0 w:1)
    /// The range of component `c` is `[0, 64226]`.
    /// The range of component `s` is `[0, 1048576]`.
    fn instantiate_with_code(c: u32, s: u32) -> Weight {
        Weight::from_ref_time(294_077_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(110_000 as u64).saturating_mul(c as u64))
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(8 as u64))
            .saturating_add(T::DbWeight::get().writes(9 as u64))
    }
    // Storage: Contracts CodeStorage (r:1 w:1)
    // Storage: Contracts Nonce (r:1 w:1)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System Account (r:1 w:1)
    // Storage: Contracts OwnerInfoOf (r:1 w:1)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `s` is `[0, 1048576]`.
    fn instantiate(s: u32) -> Weight {
        Weight::from_ref_time(199_028_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(8 as u64))
            .saturating_add(T::DbWeight::get().writes(7 as u64))
    }
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System Account (r:1 w:1)
    // Storage: System EventTopics (r:2 w:2)
    fn call() -> Weight {
        Weight::from_ref_time(176_247_000 as u64)
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
    // Storage: Contracts CodeStorage (r:1 w:1)
    // Storage: System EventTopics (r:1 w:1)
    // Storage: Contracts PristineCode (r:0 w:1)
    // Storage: Contracts OwnerInfoOf (r:0 w:1)
    /// The range of component `c` is `[0, 64226]`.
    fn upload_code(c: u32) -> Weight {
        Weight::from_ref_time(54_917_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(46_000 as u64).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
    // Storage: Contracts OwnerInfoOf (r:1 w:1)
    // Storage: System EventTopics (r:1 w:1)
    // Storage: Contracts CodeStorage (r:0 w:1)
    // Storage: Contracts PristineCode (r:0 w:1)
    fn remove_code() -> Weight {
        Weight::from_ref_time(37_611_000 as u64)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts OwnerInfoOf (r:2 w:2)
    // Storage: System EventTopics (r:3 w:3)
    fn set_code() -> Weight {
        Weight::from_ref_time(40_121_000 as u64)
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(6 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_caller(r: u32) -> Weight {
        Weight::from_ref_time(241_129_000 as u64)
            // Standard Error: 54_000
            .saturating_add(Weight::from_ref_time(35_413_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_is_contract(r: u32) -> Weight {
        Weight::from_ref_time(189_418_000 as u64)
            // Standard Error: 419_000
            .saturating_add(Weight::from_ref_time(207_107_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().reads((80 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_code_hash(r: u32) -> Weight {
        Weight::from_ref_time(203_928_000 as u64)
            // Standard Error: 439_000
            .saturating_add(Weight::from_ref_time(268_983_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().reads((80 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_own_code_hash(r: u32) -> Weight {
        Weight::from_ref_time(243_800_000 as u64)
            // Standard Error: 40_000
            .saturating_add(Weight::from_ref_time(38_797_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_caller_is_origin(r: u32) -> Weight {
        Weight::from_ref_time(239_667_000 as u64)
            // Standard Error: 27_000
            .saturating_add(Weight::from_ref_time(15_826_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_address(r: u32) -> Weight {
        Weight::from_ref_time(241_116_000 as u64)
            // Standard Error: 41_000
            .saturating_add(Weight::from_ref_time(35_402_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_gas_left(r: u32) -> Weight {
        Weight::from_ref_time(240_515_000 as u64)
            // Standard Error: 50_000
            .saturating_add(Weight::from_ref_time(35_144_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_balance(r: u32) -> Weight {
        Weight::from_ref_time(244_087_000 as u64)
            // Standard Error: 87_000
            .saturating_add(Weight::from_ref_time(110_236_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(7 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_value_transferred(r: u32) -> Weight {
        Weight::from_ref_time(241_774_000 as u64)
            // Standard Error: 50_000
            .saturating_add(Weight::from_ref_time(35_216_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_minimum_balance(r: u32) -> Weight {
        Weight::from_ref_time(241_146_000 as u64)
            // Standard Error: 54_000
            .saturating_add(Weight::from_ref_time(35_101_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_block_number(r: u32) -> Weight {
        Weight::from_ref_time(244_096_000 as u64)
            // Standard Error: 55_000
            .saturating_add(Weight::from_ref_time(34_612_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_now(r: u32) -> Weight {
        Weight::from_ref_time(242_978_000 as u64)
            // Standard Error: 53_000
            .saturating_add(Weight::from_ref_time(34_780_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    // Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
    /// The range of component `r` is `[0, 20]`.
    fn seal_weight_to_fee(r: u32) -> Weight {
        Weight::from_ref_time(246_175_000 as u64)
            // Standard Error: 86_000
            .saturating_add(Weight::from_ref_time(99_827_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(7 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_gas(r: u32) -> Weight {
        Weight::from_ref_time(168_655_000 as u64)
            // Standard Error: 16_000
            .saturating_add(Weight::from_ref_time(15_635_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_input(r: u32) -> Weight {
        Weight::from_ref_time(239_729_000 as u64)
            // Standard Error: 52_000
            .saturating_add(Weight::from_ref_time(33_477_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `n` is `[0, 1024]`.
    fn seal_input_per_kb(n: u32) -> Weight {
        Weight::from_ref_time(296_718_000 as u64)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(9_616_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 1]`.
    fn seal_return(r: u32) -> Weight {
        Weight::from_ref_time(237_666_000 as u64)
            // Standard Error: 497_000
            .saturating_add(Weight::from_ref_time(2_090_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `n` is `[0, 1024]`.
    fn seal_return_per_kb(n: u32) -> Weight {
        Weight::from_ref_time(239_842_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(184_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    // Storage: Contracts DeletionQueue (r:1 w:1)
    // Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// The range of component `r` is `[0, 1]`.
    fn seal_terminate(r: u32) -> Weight {
        Weight::from_ref_time(240_563_000 as u64)
            // Standard Error: 519_000
            .saturating_add(Weight::from_ref_time(52_855_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().reads((5 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
            .saturating_add(T::DbWeight::get().writes((6 as u64).saturating_mul(r as u64)))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    // Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
    /// The range of component `r` is `[0, 20]`.
    fn seal_random(r: u32) -> Weight {
        Weight::from_ref_time(248_136_000 as u64)
            // Standard Error: 94_000
            .saturating_add(Weight::from_ref_time(137_406_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(7 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_deposit_event(r: u32) -> Weight {
        Weight::from_ref_time(253_433_000 as u64)
            // Standard Error: 105_000
            .saturating_add(Weight::from_ref_time(242_337_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `t` is `[0, 4]`.
    /// The range of component `n` is `[0, 16]`.
    fn seal_deposit_event_per_topic_and_kb(t: u32, n: u32) -> Weight {
        Weight::from_ref_time(478_106_000 as u64)
            // Standard Error: 557_000
            .saturating_add(Weight::from_ref_time(176_325_000 as u64).saturating_mul(t as u64))
            // Standard Error: 153_000
            .saturating_add(Weight::from_ref_time(67_413_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().reads((80 as u64).saturating_mul(t as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
            .saturating_add(T::DbWeight::get().writes((80 as u64).saturating_mul(t as u64)))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_debug_message(r: u32) -> Weight {
        Weight::from_ref_time(172_751_000 as u64)
            // Standard Error: 37_000
            .saturating_add(Weight::from_ref_time(26_536_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    /// The range of component `r` is `[0, 10]`.
    fn seal_set_storage(r: u32) -> Weight {
        Weight::from_ref_time(196_276_000 as u64)
            // Standard Error: 428_000
            .saturating_add(Weight::from_ref_time(416_783_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().reads((80 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
            .saturating_add(T::DbWeight::get().writes((80 as u64).saturating_mul(r as u64)))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    /// The range of component `n` is `[0, 8]`.
    fn seal_set_storage_per_new_kb(n: u32) -> Weight {
        Weight::from_ref_time(532_439_000 as u64)
            // Standard Error: 1_323_000
            .saturating_add(Weight::from_ref_time(93_843_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(52 as u64))
            .saturating_add(T::DbWeight::get().reads((7 as u64).saturating_mul(n as u64)))
            .saturating_add(T::DbWeight::get().writes(50 as u64))
            .saturating_add(T::DbWeight::get().writes((7 as u64).saturating_mul(n as u64)))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    /// The range of component `n` is `[0, 8]`.
    fn seal_set_storage_per_old_kb(n: u32) -> Weight {
        Weight::from_ref_time(511_358_000 as u64)
            // Standard Error: 1_144_000
            .saturating_add(Weight::from_ref_time(68_754_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(52 as u64))
            .saturating_add(T::DbWeight::get().reads((7 as u64).saturating_mul(n as u64)))
            .saturating_add(T::DbWeight::get().writes(50 as u64))
            .saturating_add(T::DbWeight::get().writes((7 as u64).saturating_mul(n as u64)))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    /// The range of component `r` is `[0, 10]`.
    fn seal_clear_storage(r: u32) -> Weight {
        Weight::from_ref_time(204_133_000 as u64)
            // Standard Error: 498_000
            .saturating_add(Weight::from_ref_time(406_798_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(7 as u64))
            .saturating_add(T::DbWeight::get().reads((80 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
            .saturating_add(T::DbWeight::get().writes((80 as u64).saturating_mul(r as u64)))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    /// The range of component `n` is `[0, 8]`.
    fn seal_clear_storage_per_kb(n: u32) -> Weight {
        Weight::from_ref_time(489_339_000 as u64)
            // Standard Error: 1_269_000
            .saturating_add(Weight::from_ref_time(70_700_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(51 as u64))
            .saturating_add(T::DbWeight::get().reads((7 as u64).saturating_mul(n as u64)))
            .saturating_add(T::DbWeight::get().writes(49 as u64))
            .saturating_add(T::DbWeight::get().writes((7 as u64).saturating_mul(n as u64)))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    /// The range of component `r` is `[0, 10]`.
    fn seal_get_storage(r: u32) -> Weight {
        Weight::from_ref_time(211_344_000 as u64)
            // Standard Error: 399_000
            .saturating_add(Weight::from_ref_time(330_244_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().reads((80 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    /// The range of component `n` is `[0, 8]`.
    fn seal_get_storage_per_kb(n: u32) -> Weight {
        Weight::from_ref_time(449_353_000 as u64)
            // Standard Error: 1_027_000
            .saturating_add(Weight::from_ref_time(153_022_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(51 as u64))
            .saturating_add(T::DbWeight::get().reads((7 as u64).saturating_mul(n as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    /// The range of component `r` is `[0, 10]`.
    fn seal_contains_storage(r: u32) -> Weight {
        Weight::from_ref_time(216_197_000 as u64)
            // Standard Error: 341_000
            .saturating_add(Weight::from_ref_time(305_401_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().reads((80 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    /// The range of component `n` is `[0, 8]`.
    fn seal_contains_storage_per_kb(n: u32) -> Weight {
        Weight::from_ref_time(423_033_000 as u64)
            // Standard Error: 878_000
            .saturating_add(Weight::from_ref_time(61_940_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(51 as u64))
            .saturating_add(T::DbWeight::get().reads((7 as u64).saturating_mul(n as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    /// The range of component `r` is `[0, 10]`.
    fn seal_take_storage(r: u32) -> Weight {
        Weight::from_ref_time(204_244_000 as u64)
            // Standard Error: 448_000
            .saturating_add(Weight::from_ref_time(429_399_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(7 as u64))
            .saturating_add(T::DbWeight::get().reads((80 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
            .saturating_add(T::DbWeight::get().writes((80 as u64).saturating_mul(r as u64)))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    /// The range of component `n` is `[0, 8]`.
    fn seal_take_storage_per_kb(n: u32) -> Weight {
        Weight::from_ref_time(516_945_000 as u64)
            // Standard Error: 1_412_000
            .saturating_add(Weight::from_ref_time(162_098_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(52 as u64))
            .saturating_add(T::DbWeight::get().reads((7 as u64).saturating_mul(n as u64)))
            .saturating_add(T::DbWeight::get().writes(49 as u64))
            .saturating_add(T::DbWeight::get().writes((7 as u64).saturating_mul(n as u64)))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_transfer(r: u32) -> Weight {
        Weight::from_ref_time(170_412_000 as u64)
            // Standard Error: 761_000
            .saturating_add(Weight::from_ref_time(1_367_307_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(7 as u64))
            .saturating_add(T::DbWeight::get().reads((80 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
            .saturating_add(T::DbWeight::get().writes((80 as u64).saturating_mul(r as u64)))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_call(r: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
            // Standard Error: 9_269_000
            .saturating_add(Weight::from_ref_time(17_505_281_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(7 as u64))
            .saturating_add(T::DbWeight::get().reads((160 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
            .saturating_add(T::DbWeight::get().writes((160 as u64).saturating_mul(r as u64)))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_delegate_call(r: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
            // Standard Error: 8_780_000
            .saturating_add(Weight::from_ref_time(17_368_867_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads((158 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes((79 as u64).saturating_mul(r as u64)))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:81 w:81)
    // Storage: Contracts CodeStorage (r:2 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:82 w:82)
    /// The range of component `t` is `[0, 1]`.
    /// The range of component `c` is `[0, 1024]`.
    fn seal_call_per_transfer_clone_kb(t: u32, c: u32) -> Weight {
        Weight::from_ref_time(11_076_579_000 as u64)
            // Standard Error: 6_568_000
            .saturating_add(Weight::from_ref_time(1_158_818_000 as u64).saturating_mul(t as u64))
            // Standard Error: 9_000
            .saturating_add(Weight::from_ref_time(9_731_000 as u64).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(167 as u64))
            .saturating_add(T::DbWeight::get().reads((81 as u64).saturating_mul(t as u64)))
            .saturating_add(T::DbWeight::get().writes(163 as u64))
            .saturating_add(T::DbWeight::get().writes((81 as u64).saturating_mul(t as u64)))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    // Storage: Contracts Nonce (r:1 w:1)
    // Storage: Contracts OwnerInfoOf (r:80 w:80)
    /// The range of component `r` is `[0, 20]`.
    fn seal_instantiate(r: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
            // Standard Error: 24_125_000
            .saturating_add(Weight::from_ref_time(22_830_521_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(8 as u64))
            .saturating_add(T::DbWeight::get().reads((400 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
            .saturating_add(T::DbWeight::get().writes((400 as u64).saturating_mul(r as u64)))
    }
    // Storage: System Account (r:81 w:81)
    // Storage: Contracts ContractInfoOf (r:81 w:81)
    // Storage: Contracts CodeStorage (r:2 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Contracts Nonce (r:1 w:1)
    // Storage: Contracts OwnerInfoOf (r:1 w:1)
    // Storage: System EventTopics (r:82 w:82)
    /// The range of component `t` is `[0, 1]`.
    /// The range of component `s` is `[0, 960]`.
    fn seal_instantiate_per_transfer_salt_kb(t: u32, s: u32) -> Weight {
        Weight::from_ref_time(13_739_440_000 as u64)
            // Standard Error: 79_000
            .saturating_add(Weight::from_ref_time(126_148_000 as u64).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(249 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(t as u64)))
            .saturating_add(T::DbWeight::get().writes(247 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(t as u64)))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_hash_sha2_256(r: u32) -> Weight {
        Weight::from_ref_time(241_753_000 as u64)
            // Standard Error: 60_000
            .saturating_add(Weight::from_ref_time(55_067_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `n` is `[0, 1024]`.
    fn seal_hash_sha2_256_per_kb(n: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
            // Standard Error: 95_000
            .saturating_add(Weight::from_ref_time(320_367_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_hash_keccak_256(r: u32) -> Weight {
        Weight::from_ref_time(239_849_000 as u64)
            // Standard Error: 80_000
            .saturating_add(Weight::from_ref_time(67_626_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `n` is `[0, 1024]`.
    fn seal_hash_keccak_256_per_kb(n: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
            // Standard Error: 106_000
            .saturating_add(Weight::from_ref_time(247_771_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_hash_blake2_256(r: u32) -> Weight {
        Weight::from_ref_time(242_162_000 as u64)
            // Standard Error: 58_000
            .saturating_add(Weight::from_ref_time(45_169_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `n` is `[0, 1024]`.
    fn seal_hash_blake2_256_per_kb(n: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
            // Standard Error: 95_000
            .saturating_add(Weight::from_ref_time(97_479_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_hash_blake2_128(r: u32) -> Weight {
        Weight::from_ref_time(240_072_000 as u64)
            // Standard Error: 53_000
            .saturating_add(Weight::from_ref_time(44_847_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `n` is `[0, 1024]`.
    fn seal_hash_blake2_128_per_kb(n: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
            // Standard Error: 95_000
            .saturating_add(Weight::from_ref_time(97_432_000 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_ecdsa_recover(r: u32) -> Weight {
        Weight::from_ref_time(374_614_000 as u64)
            // Standard Error: 634_000
            .saturating_add(Weight::from_ref_time(2_968_637_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    /// The range of component `r` is `[0, 20]`.
    fn seal_ecdsa_to_eth_address(r: u32) -> Weight {
        Weight::from_ref_time(249_022_000 as u64)
            // Standard Error: 408_000
            .saturating_add(Weight::from_ref_time(2_062_013_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: System Account (r:1 w:0)
    // Storage: Contracts ContractInfoOf (r:1 w:1)
    // Storage: Contracts CodeStorage (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: System EventTopics (r:2 w:2)
    // Storage: Contracts OwnerInfoOf (r:16 w:16)
    /// The range of component `r` is `[0, 20]`.
    fn seal_set_code_hash(r: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
            // Standard Error: 1_536_000
            .saturating_add(Weight::from_ref_time(1_099_219_000 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads((158 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes((158 as u64).saturating_mul(r as u64)))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64const(r: u32) -> Weight {
        Weight::from_ref_time(70_276_000 as u64)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(933_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64load(r: u32) -> Weight {
        Weight::from_ref_time(70_309_000 as u64)
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(2_977_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64store(r: u32) -> Weight {
        Weight::from_ref_time(71_165_000 as u64)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(2_686_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_select(r: u32) -> Weight {
        Weight::from_ref_time(69_872_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(2_374_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_if(r: u32) -> Weight {
        Weight::from_ref_time(69_891_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(2_629_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_br(r: u32) -> Weight {
        Weight::from_ref_time(69_747_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_639_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_br_if(r: u32) -> Weight {
        Weight::from_ref_time(69_262_000 as u64)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(2_142_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_br_table(r: u32) -> Weight {
        Weight::from_ref_time(68_808_000 as u64)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(2_342_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `e` is `[1, 256]`.
    fn instr_br_table_per_entry(e: u32) -> Weight {
        Weight::from_ref_time(73_245_000 as u64)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(3_000 as u64).saturating_mul(e as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_call(r: u32) -> Weight {
        Weight::from_ref_time(71_308_000 as u64)
            // Standard Error: 10_000
            .saturating_add(Weight::from_ref_time(7_333_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_call_indirect(r: u32) -> Weight {
        Weight::from_ref_time(83_967_000 as u64)
            // Standard Error: 12_000
            .saturating_add(Weight::from_ref_time(9_205_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `p` is `[0, 128]`.
    fn instr_call_indirect_per_param(p: u32) -> Weight {
        Weight::from_ref_time(93_600_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(546_000 as u64).saturating_mul(p as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_local_get(r: u32) -> Weight {
        Weight::from_ref_time(70_449_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(1_052_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_local_set(r: u32) -> Weight {
        Weight::from_ref_time(70_326_000 as u64)
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(998_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_local_tee(r: u32) -> Weight {
        Weight::from_ref_time(70_525_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_467_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_global_get(r: u32) -> Weight {
        Weight::from_ref_time(73_703_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(1_495_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_global_set(r: u32) -> Weight {
        Weight::from_ref_time(73_578_000 as u64)
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(1_546_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_memory_current(r: u32) -> Weight {
        Weight::from_ref_time(70_379_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(934_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 1]`.
    fn instr_memory_grow(r: u32) -> Weight {
        Weight::from_ref_time(71_069_000 as u64)
            // Standard Error: 114_000
            .saturating_add(Weight::from_ref_time(182_540_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64clz(r: u32) -> Weight {
        Weight::from_ref_time(70_188_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_358_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64ctz(r: u32) -> Weight {
        Weight::from_ref_time(69_970_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_366_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64popcnt(r: u32) -> Weight {
        Weight::from_ref_time(70_352_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_356_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64eqz(r: u32) -> Weight {
        Weight::from_ref_time(70_229_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(1_354_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64extendsi32(r: u32) -> Weight {
        Weight::from_ref_time(70_202_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_355_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64extendui32(r: u32) -> Weight {
        Weight::from_ref_time(70_065_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_358_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i32wrapi64(r: u32) -> Weight {
        Weight::from_ref_time(70_252_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_356_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64eq(r: u32) -> Weight {
        Weight::from_ref_time(70_049_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_823_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64ne(r: u32) -> Weight {
        Weight::from_ref_time(70_519_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(1_815_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64lts(r: u32) -> Weight {
        Weight::from_ref_time(69_953_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(1_834_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64ltu(r: u32) -> Weight {
        Weight::from_ref_time(70_299_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_818_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64gts(r: u32) -> Weight {
        Weight::from_ref_time(70_141_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(1_825_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64gtu(r: u32) -> Weight {
        Weight::from_ref_time(70_209_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(1_827_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64les(r: u32) -> Weight {
        Weight::from_ref_time(69_980_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(1_831_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64leu(r: u32) -> Weight {
        Weight::from_ref_time(70_022_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_829_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64ges(r: u32) -> Weight {
        Weight::from_ref_time(70_030_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_826_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64geu(r: u32) -> Weight {
        Weight::from_ref_time(70_170_000 as u64)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(1_833_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64add(r: u32) -> Weight {
        Weight::from_ref_time(69_895_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_826_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64sub(r: u32) -> Weight {
        Weight::from_ref_time(69_932_000 as u64)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(1_830_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64mul(r: u32) -> Weight {
        Weight::from_ref_time(70_091_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(1_825_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64divs(r: u32) -> Weight {
        Weight::from_ref_time(70_025_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(2_556_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64divu(r: u32) -> Weight {
        Weight::from_ref_time(71_910_000 as u64)
            // Standard Error: 19_000
            .saturating_add(Weight::from_ref_time(2_290_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64rems(r: u32) -> Weight {
        Weight::from_ref_time(70_268_000 as u64)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(2_550_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64remu(r: u32) -> Weight {
        Weight::from_ref_time(70_126_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(2_340_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64and(r: u32) -> Weight {
        Weight::from_ref_time(70_381_000 as u64)
            // Standard Error: 9_000
            .saturating_add(Weight::from_ref_time(1_844_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64or(r: u32) -> Weight {
        Weight::from_ref_time(70_095_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_844_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64xor(r: u32) -> Weight {
        Weight::from_ref_time(70_471_000 as u64)
            // Standard Error: 8_000
            .saturating_add(Weight::from_ref_time(1_836_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64shl(r: u32) -> Weight {
        Weight::from_ref_time(70_302_000 as u64)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(1_841_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64shrs(r: u32) -> Weight {
        Weight::from_ref_time(70_097_000 as u64)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(1_850_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64shru(r: u32) -> Weight {
        Weight::from_ref_time(70_166_000 as u64)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(1_845_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64rotl(r: u32) -> Weight {
        Weight::from_ref_time(69_630_000 as u64)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(1_879_000 as u64).saturating_mul(r as u64))
    }
    /// The range of component `r` is `[0, 50]`.
    fn instr_i64rotr(r: u32) -> Weight {
        Weight::from_ref_time(70_101_000 as u64)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(1_861_000 as u64).saturating_mul(r as u64))
    }
}

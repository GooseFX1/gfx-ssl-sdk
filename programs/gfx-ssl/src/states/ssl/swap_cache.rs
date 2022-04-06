// valid sequences of ix are [RebalanceSwap] and [RebalanceSwap, Preswap, Swap]
// This means we should check at the beginning of Preswap and Swap
#[derive(Clone, Copy, Debug)]
pub enum Stage {
    NotStarted,
    RebalanceSwapped,
    PreSwapped,
}

impl Default for Stage {
    fn default() -> Self {
        Self::NotStarted
    }
}

#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "no-entrypoint", derive(Debug))]
pub struct SwapCache {
    pub stage: Stage,

    // Result after rebalance swap
    pub amount_in: u64,
    pub min_out: u64,
    pub oracle_price: [u32; 4],

    // Result after preswap
    pub liability_in_after: u64,
    pub liability_out_after: u64,
    pub weight_in_after: u64,
    pub weight_out_after: u64,
    pub liability_out_excessive: u64,
}

const _: [u8; 80] = [0; std::mem::size_of::<SwapCache>()];

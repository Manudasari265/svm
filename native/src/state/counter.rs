use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Copy, Clone, BorshSerialize, BorshDeserialize)]
pub struct Counter {
    pub count: u64
}
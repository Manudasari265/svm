
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum CounterInstruction {
    Logging,
    Increment,
    Update,
}
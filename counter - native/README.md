# Program structure 
- [] `entrypoint.rs` - defines the entrypoint that routes incoming instructions
- [] `state.rs` - defines program-specific account state(account data)
- [] `instructions.rs` - defines the instructions that the program can execute
- [] `processor.rs` - defines the instruction handlers(functions) that implement the core business logic 
- [] `error.rs` - defines the custom errors 
#![no_std]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

#[cfg(feature = "std")]
extern crate std;

pinocchio_pubkey::declare_id!("7KuDrDJsLa2iKcUovWs7DFNYRdYJ12MyKyaJwnqmhSxy");
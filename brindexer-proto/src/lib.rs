#![allow(clippy::derive_partial_eq_without_eq)]
pub mod blockscout {
    pub mod brindexer {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/blockscout.brindexer.v1.rs"));
        }
    }
}

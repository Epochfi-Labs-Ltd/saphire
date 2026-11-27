//! One file per instruction card, grouped by segment:
//! `market/` = A (IX-01..10, 13..16, 25), `positions/` = B (IX-11, 12, 17..20,
//! 24), `settlement/` = C (IX-21..23: daily rates and settlement). See docs/skeleton.md for the mapping.

pub mod market;
pub mod positions;
pub mod settlement;

pub use market::*;
pub use positions::*;
pub use settlement::*;

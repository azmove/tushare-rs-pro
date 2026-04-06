//! Predefined data models for Tushare API responses.
//!
//! Each module contains strongly-typed structs that map 1:1 to Tushare API fields.
//! All structs derive `FromTushareData` for automatic conversion from API responses.
//!
//! # Design
//!
//! - **Data, not abstraction** — structs are plain data with no methods
//! - **One domain per module** — stock, fund, index, bond, etc.
//! - **Optional by default** — most fields are `Option<T>` because the API may omit them
//! - **Field names match API** — Rust field names mirror Tushare field names;
//!   `#[tushare(field = "...")]` is used only when Rust syntax demands it

pub mod stock;
pub mod fund;
pub mod index;
pub mod bond;
pub mod etf;
pub mod futures;
pub mod hk;
pub mod us;
pub mod options;
pub mod macro_data;
pub mod industry;
pub mod forex;

pub use stock::*;
pub use fund::*;
pub use index::*;
pub use bond::*;
pub use etf::*;
pub use futures::*;
pub use hk::*;
pub use us::*;
pub use options::*;
pub use macro_data::*;
pub use industry::*;
pub use forex::*;

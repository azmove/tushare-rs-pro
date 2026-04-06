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

pub mod bond;
pub mod etf;
pub mod forex;
pub mod fund;
pub mod futures;
pub mod hk;
pub mod index;
pub mod industry;
pub mod macro_data;
pub mod options;
pub mod stock;
pub mod us;

pub use bond::*;
pub use etf::*;
pub use forex::*;
pub use fund::*;
pub use futures::*;
pub use hk::*;
pub use index::*;
pub use industry::*;
pub use macro_data::*;
pub use options::*;
pub use stock::*;
pub use us::*;

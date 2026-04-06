//! # tushare-rs-pro — Tushare Pro API Rust SDK
//!
//! 面向量化交易和金融数据分析的 Rust 客户端库，提供对 [Tushare Pro](https://tushare.pro/)
//! 全部 API 的异步、类型安全访问。
//!
//! ## 核心特性
//!
//! - **77 个预定义模型** — 覆盖股票、基金、指数、债券、ETF、期货、期权、港股、美股、
//!   外汇、宏观经济、行业分类等 12 大领域，开箱即用
//! - **86 个 API 枚举** — 编译期类型安全，杜绝 API 名称拼写错误
//! - **Derive 宏** — `#[derive(DeriveFromTushareData)]` 自动将 API 响应映射到自定义 struct
//! - **异步架构** — 基于 `tokio` + `reqwest`，高性能并发请求
//! - **生产就绪** — 内置限流、指数退避重试、超时控制、全面错误处理
//!
//! ## 快速开始
//!
//! ### 使用预定义模型（推荐）
//!
//! ```no_run
//! use tushare_api::{TushareClient, Api, TushareEntityList, TushareRequest, request, params, fields};
//! use tushare_api::models::DailyModel;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = TushareClient::from_env()?;
//!
//!     let req = request!(Api::Daily, {
//!         "ts_code" => "000001.SZ",
//!         "start_date" => "20260101",
//!         "end_date" => "20260401"
//!     }, [
//!         "ts_code", "trade_date", "open", "high", "low", "close", "vol"
//!     ]);
//!
//!     let data: TushareEntityList<DailyModel> = client.call_api_as(req).await?;
//!
//!     for d in data.iter().take(3) {
//!         println!("{} 收盘: {:.2}", d.trade_date, d.close.unwrap_or(0.0));
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ### 自定义 Struct
//!
//! ```no_run
//! use tushare_api::{TushareClient, Api, TushareEntityList, TushareRequest, request, params, fields, DeriveFromTushareData};
//!
//! #[derive(Debug, Clone, DeriveFromTushareData)]
//! pub struct MyStock {
//!     pub ts_code: String,
//!     pub name: String,
//!     #[tushare(field = "list_date")]
//!     pub listing_date: Option<String>,
//! }
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = TushareClient::from_env()?;
//! let stocks: TushareEntityList<MyStock> = client.call_api_as(
//!     request!(Api::StockBasic, { "list_status" => "L" }, ["ts_code", "name", "list_date"])
//! ).await?;
//! println!("共 {} 只股票", stocks.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## 模块概览
//!
//! | 模块 | 说明 |
//! |------|------|
//! | [`client`] | 核心客户端 [`TushareClient`]，支持 `from_env()` / `new()` / `with_timeout()` |
//! | [`client_ex`] | 增强客户端 [`TushareClientEx`]，带限流和重试 |
//! | [`models`] | 77 个预定义数据模型，按领域分 12 个子模块 |
//! | [`api`] | [`Api`] 枚举（86 个变体），所有 Tushare API 的类型安全标识 |
//! | [`types`] | 请求/响应类型、`request!` 宏、[`TushareEntityList`] |
//! | [`traits`] | [`FromTushareData`] / [`FromTushareValue`] 转换 trait |
//! | [`error`] | [`TushareError`] 错误类型 |
//! | [`logging`] | 日志配置 |

// Allow the derive macro's generated `tushare_api::` paths to resolve within this crate
extern crate self as tushare_api;

pub mod api;
pub mod basic_types;
pub mod client;
pub mod client_ex;
pub mod custom_date_format;
pub mod error;
pub mod logging;
pub mod models;
pub mod third_party_types;
pub mod traits;
pub mod types;
pub mod utils;

// Re-export main types for convenience
pub use api::Api;
pub use client::{HttpClientConfig, TushareClient};
pub use client_ex::{RetryConfig, TushareClientEx};
pub use error::{TushareError, TushareResult};
pub use logging::{LogConfig, LogLevel, Logger};
pub use traits::{FromOptionalTushareValue, FromTushareData, FromTushareValue};
pub use types::{TushareData, TushareEntityList, TushareRequest, TushareResponse};
pub use utils::response_to_vec;

// Macros are automatically exported at the crate root via #[macro_export]

// Re-export procedural macros from tushare-rs-pro-derive
pub use tushare_rs_pro_derive::FromTushareData as DeriveFromTushareData;

// Re-export serde_json for user convenience
pub use serde_json;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TushareData;
    use serde_json::json;

    #[test]
    fn test_api_serialization() {
        let api = Api::StockBasic;
        assert_eq!(api.name(), "stock_basic");
    }

    #[test]
    fn test_tushare_request_creation() {
        let request = TushareRequest::new(
            Api::StockBasic,
            vec![("list_status".to_string(), "L".to_string())],
            vec!["ts_code".to_string(), "symbol".to_string()],
        );

        assert_eq!(request.api_name, Api::StockBasic);
        assert_eq!(request.params.len(), 1);
        assert_eq!(request.fields.len(), 2);
    }

    #[test]
    fn test_tushare_response_creation() {
        let response = TushareResponse {
            request_id: "test123".to_string(),
            code: 0,
            msg: None,
            data: Some(TushareData {
                fields: vec!["ts_code".to_string(), "name".to_string()],
                items: vec![
                    vec![json!("000001.SZ"), json!("平安银行")],
                    vec![json!("000002.SZ"), json!("万科A")],
                ],
                has_more: false,
                count: 2,
            }),
        };

        assert_eq!(response.code, 0);
        assert_eq!(
            response
                .data
                .as_ref()
                .map(|data| data.items.len())
                .unwrap_or(0),
            2
        );
        assert_eq!(
            response
                .data
                .as_ref()
                .map(|data| data.items.len())
                .unwrap_or(0),
            2
        );
    }
}

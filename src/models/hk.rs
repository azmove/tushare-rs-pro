//! Hong Kong stock data models.

use crate::DeriveFromTushareData;

/// 港股基本信息 (hk_basic)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct HkBasicModel {
    pub ts_code: String,
    pub name: String,
    pub area: Option<String>,
    pub industry: Option<String>,
    pub full_name: Option<String>,
    pub en_name: Option<String>,
    pub list_date: Option<String>,
}

/// 港股日线行情 (hk_daily)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct HkDailyModel {
    pub ts_code: String,
    pub trade_date: String,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub vol: Option<f64>,
    pub amount: Option<f64>,
}

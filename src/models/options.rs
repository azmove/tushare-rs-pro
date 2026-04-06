//! Options data models.

use crate::DeriveFromTushareData;

/// 期权合约基本信息 (opt_basic)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct OptBasicModel {
    pub ts_code: String,
    pub name: Option<String>,
    pub per_unit: Option<f64>,
    pub opt_code: Option<String>,
    pub opt_type: Option<String>,
    pub call_put: Option<String>,
    pub exercise_type: Option<String>,
}

/// 期权日行情 (opt_daily)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct OptDailyModel {
    pub ts_code: String,
    pub trade_date: String,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub pre_settle: Option<f64>,
    pub settle: Option<f64>,
}

//! Index data models.

use crate::DeriveFromTushareData;

/// 指数基本信息 (index_basic)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct IndexBasicModel {
    pub ts_code: String,
    pub name: String,
    pub market: Option<String>,
    pub publisher: Option<String>,
    pub base_date: Option<String>,
    pub base_point: Option<f64>,
}

/// 指数日线行情 (index_daily)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct IndexDailyModel {
    pub ts_code: String,
    pub trade_date: String,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub pre_close: Option<f64>,
    pub change: Option<f64>,
    pub pct_chg: Option<f64>,
    pub vol: Option<f64>,
    pub amount: Option<f64>,
}

/// 指数成分权重 (index_weight)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct IndexWeightModel {
    pub index_code: String,
    pub con_code: String,
    pub trade_date: Option<String>,
    pub weight: Option<f64>,
}

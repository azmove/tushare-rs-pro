//! US stock data models.

use crate::DeriveFromTushareData;

/// 美股基本信息 (us_basic)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct UsBasicModel {
    pub ts_code: String,
    pub name: String,
    pub en_name: Option<String>,
    pub cn_spell: Option<String>,
    pub exchange: Option<String>,
    pub market: Option<String>,
}

/// 美股日线行情 (us_daily)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct UsDailyModel {
    pub ts_code: String,
    pub trade_date: String,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub vol: Option<f64>,
    pub amount: Option<f64>,
}

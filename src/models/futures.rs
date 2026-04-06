//! Futures data models.

use crate::DeriveFromTushareData;

/// 期货合约基本信息 (fut_basic)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FuturesBasicModel {
    pub ts_code: String,
    pub name: String,
    pub exchange: Option<String>,
    pub product: Option<String>,
    pub list_date: Option<String>,
    pub delist_date: Option<String>,
    pub delivery_year: Option<String>,
    pub delivery_month: Option<String>,
    pub last_ed_date: Option<String>,
    pub trading_time: Option<String>,
}

/// 期货日线行情 (fut_daily)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FuturesDailyModel {
    pub ts_code: String,
    pub trade_date: String,
    pub pre_close: Option<f64>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub change: Option<f64>,
    pub pct_chg: Option<f64>,
    pub vol: Option<f64>,
    pub amount: Option<f64>,
    pub oi: Option<f64>,
    pub oi_chg: Option<f64>,
    pub delv_settle: Option<f64>,
}

/// 期货持仓数据 (fut_holding)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FuturesHoldModel {
    pub trade_date: Option<String>,
    pub symbol: Option<String>,
    pub broker: Option<String>,
    pub vol: Option<f64>,
    pub vol_chg: Option<f64>,
    pub long_hld: Option<f64>,
    pub long_chg: Option<f64>,
    pub short_hld: Option<f64>,
    pub short_chg: Option<f64>,
}

//! ETF data models.

use crate::DeriveFromTushareData;

/// ETF基本信息 (fund_basic with fund_type filter)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct EtfBasicModel {
    pub ts_code: String,
    pub name: String,
    pub fund_type: Option<String>,
    pub list_date: Option<String>,
    pub delist_date: Option<String>,
}

/// ETF日线行情 (fund_daily)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct EtfDailyModel {
    pub ts_code: String,
    pub trade_date: String,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub vol: Option<f64>,
    pub amount: Option<f64>,
}

/// ETF复权因子 (fund_adj)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct EtfAdjFactorModel {
    pub ts_code: String,
    pub trade_date: String,
    pub adj_factor: Option<f64>,
}

/// ETF份额 (fund_share)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct EtfShareModel {
    pub ts_code: String,
    pub trade_date: String,
    pub share_amount: Option<f64>,
}

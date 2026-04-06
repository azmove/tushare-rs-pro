//! Bond and convertible bond data models.

use crate::DeriveFromTushareData;

/// 可转债基本信息 (cb_basic)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct CbBasicModel {
    pub ts_code: String,
    pub bond_full_name: Option<String>,
    pub bond_short_name: Option<String>,
    pub stk_code: Option<String>,
    pub list_date: Option<String>,
}

/// 可转债日行情 (cb_daily)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct CbDailyModel {
    pub ts_code: String,
    pub trade_date: String,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub vol: Option<f64>,
    pub amount: Option<f64>,
}

/// 可转债发行 (cb_issue)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct CbIssueModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub issue_size: Option<f64>,
    pub issue_price: Option<f64>,
}

/// 国债收益率曲线 (yc_cb)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct BondYieldModel {
    pub date: String,
    pub yield_1y: Option<f64>,
    pub yield_2y: Option<f64>,
    pub yield_5y: Option<f64>,
    pub yield_10y: Option<f64>,
}

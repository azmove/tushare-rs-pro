//! Foreign exchange data models.

use crate::DeriveFromTushareData;

/// 外汇基本信息 (fx_obasic)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FxBasicModel {
    pub currency_code: String,
    pub currency_name: Option<String>,
    pub currency_enname: Option<String>,
}

/// 外汇日线行情 (fx_daily)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FxDailyModel {
    pub trade_date: String,
    pub fx_rate: Option<f64>,
    pub currency_code: Option<String>,
}

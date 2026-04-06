//! Fund data models.

use crate::DeriveFromTushareData;

/// 基金基本信息 (fund_basic)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FundBasicModel {
    pub ts_code: String,
    pub name: String,
    pub fund_type: Option<String>,
    pub fund_manager: Option<String>,
    pub management: Option<String>,
    pub trustee: Option<String>,
    pub establish_date: Option<String>,
    pub list_date: Option<String>,
    pub issue_date: Option<String>,
    pub delist_date: Option<String>,
    pub issue_amount: Option<f64>,
    pub m_fee: Option<f64>,
    pub c_fee: Option<f64>,
    pub duration_year: Option<f64>,
    pub pooling: Option<String>,
    pub pv: Option<f64>,
    pub p_value: Option<f64>,
    pub benchmark: Option<String>,
    pub status: Option<String>,
    pub invest_type: Option<String>,
    #[tushare(field = "type")]
    pub fund_category: Option<String>,
    pub trustee_fee: Option<f64>,
}

/// 基金净值数据 (fund_nav)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FundNavModel {
    pub ts_code: String,
    pub end_date: String,
    pub unit_nav: Option<f64>,
    pub accum_nav: Option<f64>,
    pub accum_div: Option<f64>,
    pub net_asset: Option<f64>,
    pub total_netasset: Option<f64>,
    pub adj_nav: Option<f64>,
}

/// 基金分红数据 (fund_div)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FundDividendModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub ex_date: Option<String>,
    pub pay_date: Option<String>,
    pub dividend: Option<f64>,
    pub record_date: Option<String>,
    pub dividend_pro: Option<String>,
}

/// 基金持仓数据 (fund_portfolio)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FundPortfolioModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub end_date: Option<String>,
    pub symbol: Option<String>,
    pub mkv: Option<f64>,
    pub amount: Option<f64>,
    pub stk_mkv_ratio: Option<f64>,
    pub stk_float_ratio: Option<f64>,
}

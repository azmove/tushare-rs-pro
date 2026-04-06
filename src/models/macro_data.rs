//! Macroeconomic data models.

use crate::DeriveFromTushareData;

/// Shibor利率 (shibor)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct ShiborModel {
    pub date: String,
    pub on: Option<f64>,
    #[tushare(field = "1w")]
    pub w_1: Option<f64>,
    #[tushare(field = "2w")]
    pub w_2: Option<f64>,
    #[tushare(field = "1m")]
    pub m_1: Option<f64>,
    #[tushare(field = "3m")]
    pub m_3: Option<f64>,
    #[tushare(field = "6m")]
    pub m_6: Option<f64>,
    #[tushare(field = "9m")]
    pub m_9: Option<f64>,
    #[tushare(field = "1y")]
    pub y_1: Option<f64>,
}

/// LPR利率 (cn_lpr)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct LprModel {
    pub date: String,
    pub lpr_1y: Option<f64>,
    pub lpr_5y: Option<f64>,
}

/// GDP数据 (cn_gdp)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct GdpModel {
    pub quarter: String,
    pub gdp: Option<f64>,
    pub gdp_yoy: Option<f64>,
}

/// CPI数据 (cn_cpi)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct CpiModel {
    pub month: String,
    pub cpi: Option<f64>,
    pub cpi_yoy: Option<f64>,
}

/// PPI数据 (cn_ppi)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct PpiModel {
    pub month: String,
    pub ppi: Option<f64>,
    pub ppi_yoy: Option<f64>,
}

/// M2货币供应量 (cn_m)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct M2Model {
    pub month: String,
    pub m2: Option<f64>,
    pub m2_yoy: Option<f64>,
}

/// PMI指数 (cn_pmi)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct PmiModel {
    pub month: String,
    pub pmi_m: Option<f64>,
    pub pmi_yoy: Option<f64>,
}

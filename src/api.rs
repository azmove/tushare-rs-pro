use serde::{Deserialize, Serialize};
use serde::de;
use std::fmt;

/// Tushare API enum types
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum Api {
    StockBasic,
    FundBasic,
    FundDaily,
    FundPortfolio,
    Daily,      
    DailyBasic,
    MoneyflowMktDc,
    Weekly,
    Monthly,
    IndexDaily,
    IndexWeekly,
    IndexMonthly,
    TradeCal,
    Margin,
    StockCompany,
    MarginDetail,
    StkHoldernumber,
    ThsIndex, 
    ThsMember,
    ThsDaily,
    ThsHot,
    FinaMainbz,
    FinaMainbzVip,
    FinaIndicator,
    Balancesheet,
    Income,
    Cashflow,
    IndexBasic,
    IndexDailyBasic,
    Moneyflow,
    MoneyflowIndustryThs,

    UsBasic,
    UsDaily,

    // Stock reference data
    DailyShare,
    StStock,
    HsConst,
    NameChange,
    StkManagers,
    NewShare,
    AdjFactor,
    StkLimit,
    SuspendD,
    Forecast,
    Express,
    Dividend,
    Top10Holders,
    Top10Floatholders,
    PledgeStat,
    Repurchase,
    ShareFloat,
    BlockTrade,
    StkHoldchange,
    HkHold,
    TopList,
    TopInst,
    MoneyflowHsgt,
    LimitListD,

    // Fund
    FundNav,
    FundDiv,
    FundAdj,
    FundShare,

    // Bond
    CbBasic,
    CbDaily,
    CbIssue,
    YcCb,

    // Index
    IndexWeight,

    // Futures
    FutBasic,
    FutDaily,
    FutHolding,

    // HK
    HkBasic,
    HkDaily,

    // Options
    OptBasic,
    OptDaily,

    // Macro economic
    Shibor,
    CnLpr,
    CnGdp,
    CnCpi,
    CnPpi,
    CnM,
    CnPmi,

    // Industry
    BoDaily,
    BoWeekly,

    // Forex
    FxObasic,
    FxDaily,

    Custom(String), // other apis specified by name
}

const API_STOCK_BASIC: &str = "stock_basic";
const API_FUND_BASIC: &str = "fund_basic";
const API_FUND_DAILY: &str = "fund_daily";
const API_FUND_PORTFOLIO: &str = "fund_portfolio";
const API_DAILY: &str = "daily";
const API_DAILY_BASIC: &str = "daily_basic";
const API_MONEYFLOW_MKT_DC: &str = "moneyflow_mkt_dc";
const API_WEEKLY: &str = "weekly";
const API_MONTHLY: &str = "monthly";
const API_INDEX_DAILY: &str = "index_daily";
const API_INDEX_WEEKLY: &str = "index_weekly";
const API_INDEX_MONTHLY: &str = "index_monthly";
const API_TRADE_CAL: &str = "trade_cal";
const API_MARGIN: &str = "margin";
const API_STOCK_COMPANY: &str = "stock_company";
const API_MARGIN_DETAIL: &str = "margin_detail";
const API_STK_HOLDERNUMBER: &str = "stk_holdernumber";
const API_THS_INDEX: &str = "ths_index";
const API_THS_MEMBER: &str = "ths_member";
const API_THS_DAILY: &str = "ths_daily";
const API_THS_HOT: &str = "ths_hot";
const API_FINA_MAINBZ: &str = "fina_mainbz";
const API_FINA_MAINBZ_VIP: &str = "fina_mainbz_vip";
const API_FINA_INDICATOR: &str = "fina_indicator";
const API_BALANCESHEET: &str = "balancesheet";
const API_INCOME: &str = "income";
const API_CASHFLOW: &str = "cashflow";
const API_INDEX_BASIC: &str = "index_basic";
const API_INDEX_DAILY_BASIC: &str = "index_daily_basic";
const API_MONEYFLOW: &str = "moneyflow";
const API_MONEYFLOW_INDUSTRY_THS: &str = "moneyflow_industry_ths";
const API_US_BASIC: &str = "us_basic";
const API_US_DAILY: &str = "us_daily";

const API_DAILY_SHARE: &str = "daily_share";
const API_ST_STOCK: &str = "st_stock";
const API_HS_CONST: &str = "hs_const";
const API_NAME_CHANGE: &str = "namechange";
const API_STK_MANAGERS: &str = "stk_managers";
const API_NEW_SHARE: &str = "new_share";
const API_ADJ_FACTOR: &str = "adj_factor";
const API_STK_LIMIT: &str = "stk_limit";
const API_SUSPEND_D: &str = "suspend_d";
const API_FORECAST: &str = "forecast";
const API_EXPRESS: &str = "express";
const API_DIVIDEND: &str = "dividend";
const API_TOP10_HOLDERS: &str = "top10_holders";
const API_TOP10_FLOATHOLDERS: &str = "top10_floatholders";
const API_PLEDGE_STAT: &str = "pledge_stat";
const API_REPURCHASE: &str = "repurchase";
const API_SHARE_FLOAT: &str = "share_float";
const API_BLOCK_TRADE: &str = "block_trade";
const API_STK_HOLDCHANGE: &str = "stk_holdchange";
const API_HK_HOLD: &str = "hk_hold";
const API_TOP_LIST: &str = "top_list";
const API_TOP_INST: &str = "top_inst";
const API_MONEYFLOW_HSGT: &str = "moneyflow_hsgt";
const API_LIMIT_LIST_D: &str = "limit_list_d";

const API_FUND_NAV: &str = "fund_nav";
const API_FUND_DIV: &str = "fund_div";
const API_FUND_ADJ: &str = "fund_adj";
const API_FUND_SHARE: &str = "fund_share";

const API_CB_BASIC: &str = "cb_basic";
const API_CB_DAILY: &str = "cb_daily";
const API_CB_ISSUE: &str = "cb_issue";
const API_YC_CB: &str = "yc_cb";

const API_INDEX_WEIGHT: &str = "index_weight";

const API_FUT_BASIC: &str = "fut_basic";
const API_FUT_DAILY: &str = "fut_daily";
const API_FUT_HOLDING: &str = "fut_holding";

const API_HK_BASIC: &str = "hk_basic";
const API_HK_DAILY: &str = "hk_daily";

const API_OPT_BASIC: &str = "opt_basic";
const API_OPT_DAILY: &str = "opt_daily";

const API_SHIBOR: &str = "shibor";
const API_CN_LPR: &str = "cn_lpr";
const API_CN_GDP: &str = "cn_gdp";
const API_CN_CPI: &str = "cn_cpi";
const API_CN_PPI: &str = "cn_ppi";
const API_CN_M: &str = "cn_m";
const API_CN_PMI: &str = "cn_pmi";

const API_BO_DAILY: &str = "bo_daily";
const API_BO_WEEKLY: &str = "bo_weekly";

const API_FX_OBASIC: &str = "fx_obasic";
const API_FX_DAILY: &str = "fx_daily";

impl Api {
    fn from_api_str(value: &str) -> Option<Self> {
        let v = value.trim();
        if v.is_empty() {
            return None;
        }

        match v {
            API_STOCK_BASIC => Some(Api::StockBasic),
            API_FUND_BASIC => Some(Api::FundBasic),
            API_FUND_DAILY => Some(Api::FundDaily),
            API_FUND_PORTFOLIO => Some(Api::FundPortfolio),
            API_DAILY => Some(Api::Daily),
            API_DAILY_BASIC => Some(Api::DailyBasic),
            API_MONEYFLOW_MKT_DC => Some(Api::MoneyflowMktDc),
            API_WEEKLY => Some(Api::Weekly),
            API_MONTHLY => Some(Api::Monthly),
            API_INDEX_DAILY => Some(Api::IndexDaily),
            API_INDEX_WEEKLY => Some(Api::IndexWeekly),
            API_INDEX_MONTHLY => Some(Api::IndexMonthly),
            API_TRADE_CAL => Some(Api::TradeCal),
            API_MARGIN => Some(Api::Margin),
            API_STOCK_COMPANY => Some(Api::StockCompany),
            API_MARGIN_DETAIL => Some(Api::MarginDetail),
            API_STK_HOLDERNUMBER => Some(Api::StkHoldernumber),
            API_THS_INDEX => Some(Api::ThsIndex),
            API_THS_MEMBER => Some(Api::ThsMember),
            API_THS_DAILY => Some(Api::ThsDaily),
            API_THS_HOT => Some(Api::ThsHot),
            API_FINA_MAINBZ => Some(Api::FinaMainbz),
            API_FINA_MAINBZ_VIP => Some(Api::FinaMainbzVip),
            API_FINA_INDICATOR => Some(Api::FinaIndicator),
            API_BALANCESHEET => Some(Api::Balancesheet),
            API_INCOME => Some(Api::Income),
            API_CASHFLOW => Some(Api::Cashflow),
            API_INDEX_BASIC => Some(Api::IndexBasic),
            API_INDEX_DAILY_BASIC => Some(Api::IndexDailyBasic),
            API_MONEYFLOW => Some(Api::Moneyflow),
            API_MONEYFLOW_INDUSTRY_THS => Some(Api::MoneyflowIndustryThs),
            API_US_BASIC => Some(Api::UsBasic),
            API_US_DAILY => Some(Api::UsDaily),
            API_DAILY_SHARE => Some(Api::DailyShare),
            API_ST_STOCK => Some(Api::StStock),
            API_HS_CONST => Some(Api::HsConst),
            API_NAME_CHANGE => Some(Api::NameChange),
            API_STK_MANAGERS => Some(Api::StkManagers),
            API_NEW_SHARE => Some(Api::NewShare),
            API_ADJ_FACTOR => Some(Api::AdjFactor),
            API_STK_LIMIT => Some(Api::StkLimit),
            API_SUSPEND_D => Some(Api::SuspendD),
            API_FORECAST => Some(Api::Forecast),
            API_EXPRESS => Some(Api::Express),
            API_DIVIDEND => Some(Api::Dividend),
            API_TOP10_HOLDERS => Some(Api::Top10Holders),
            API_TOP10_FLOATHOLDERS => Some(Api::Top10Floatholders),
            API_PLEDGE_STAT => Some(Api::PledgeStat),
            API_REPURCHASE => Some(Api::Repurchase),
            API_SHARE_FLOAT => Some(Api::ShareFloat),
            API_BLOCK_TRADE => Some(Api::BlockTrade),
            API_STK_HOLDCHANGE => Some(Api::StkHoldchange),
            API_HK_HOLD => Some(Api::HkHold),
            API_TOP_LIST => Some(Api::TopList),
            API_TOP_INST => Some(Api::TopInst),
            API_MONEYFLOW_HSGT => Some(Api::MoneyflowHsgt),
            API_LIMIT_LIST_D => Some(Api::LimitListD),
            API_FUND_NAV => Some(Api::FundNav),
            API_FUND_DIV => Some(Api::FundDiv),
            API_FUND_ADJ => Some(Api::FundAdj),
            API_FUND_SHARE => Some(Api::FundShare),
            API_CB_BASIC => Some(Api::CbBasic),
            API_CB_DAILY => Some(Api::CbDaily),
            API_CB_ISSUE => Some(Api::CbIssue),
            API_YC_CB => Some(Api::YcCb),
            API_INDEX_WEIGHT => Some(Api::IndexWeight),
            API_FUT_BASIC => Some(Api::FutBasic),
            API_FUT_DAILY => Some(Api::FutDaily),
            API_FUT_HOLDING => Some(Api::FutHolding),
            API_HK_BASIC => Some(Api::HkBasic),
            API_HK_DAILY => Some(Api::HkDaily),
            API_OPT_BASIC => Some(Api::OptBasic),
            API_OPT_DAILY => Some(Api::OptDaily),
            API_SHIBOR => Some(Api::Shibor),
            API_CN_LPR => Some(Api::CnLpr),
            API_CN_GDP => Some(Api::CnGdp),
            API_CN_CPI => Some(Api::CnCpi),
            API_CN_PPI => Some(Api::CnPpi),
            API_CN_M => Some(Api::CnM),
            API_CN_PMI => Some(Api::CnPmi),
            API_BO_DAILY => Some(Api::BoDaily),
            API_BO_WEEKLY => Some(Api::BoWeekly),
            API_FX_OBASIC => Some(Api::FxObasic),
            API_FX_DAILY => Some(Api::FxDaily),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Api::StockBasic => API_STOCK_BASIC.to_string(),
            Api::FundBasic => API_FUND_BASIC.to_string(),
            Api::FundDaily => API_FUND_DAILY.to_string(),
            Api::FundPortfolio => API_FUND_PORTFOLIO.to_string(),
            Api::Daily => API_DAILY.to_string(),
            Api::DailyBasic => API_DAILY_BASIC.to_string(),
            Api::MoneyflowMktDc => API_MONEYFLOW_MKT_DC.to_string(),
            Api::Weekly => API_WEEKLY.to_string(),
            Api::Monthly => API_MONTHLY.to_string(),
            Api::IndexDaily => API_INDEX_DAILY.to_string(),
            Api::IndexWeekly => API_INDEX_WEEKLY.to_string(),
            Api::IndexMonthly => API_INDEX_MONTHLY.to_string(),
            Api::TradeCal => API_TRADE_CAL.to_string(),
            Api::Margin => API_MARGIN.to_string(),
            Api::StockCompany => API_STOCK_COMPANY.to_string(),
            Api::MarginDetail => API_MARGIN_DETAIL.to_string(),
            Api::StkHoldernumber => API_STK_HOLDERNUMBER.to_string(),
            Api::ThsIndex => API_THS_INDEX.to_string(),
            Api::ThsMember => API_THS_MEMBER.to_string(),
            Api::ThsDaily => API_THS_DAILY.to_string(),
            Api::ThsHot => API_THS_HOT.to_string(),
            Api::FinaMainbz => API_FINA_MAINBZ.to_string(),
            Api::FinaMainbzVip => API_FINA_MAINBZ_VIP.to_string(),
            Api::FinaIndicator => API_FINA_INDICATOR.to_string(),
            Api::Balancesheet => API_BALANCESHEET.to_string(),
            Api::Income => API_INCOME.to_string(),
            Api::Cashflow => API_CASHFLOW.to_string(),
            Api::IndexBasic => API_INDEX_BASIC.to_string(),
            Api::IndexDailyBasic => API_INDEX_DAILY_BASIC.to_string(),
            Api::Moneyflow => API_MONEYFLOW.to_string(),
            Api::MoneyflowIndustryThs => API_MONEYFLOW_INDUSTRY_THS.to_string(),
            Api::UsBasic => API_US_BASIC.to_string(),
            Api::UsDaily => API_US_DAILY.to_string(),
            Api::DailyShare => API_DAILY_SHARE.to_string(),
            Api::StStock => API_ST_STOCK.to_string(),
            Api::HsConst => API_HS_CONST.to_string(),
            Api::NameChange => API_NAME_CHANGE.to_string(),
            Api::StkManagers => API_STK_MANAGERS.to_string(),
            Api::NewShare => API_NEW_SHARE.to_string(),
            Api::AdjFactor => API_ADJ_FACTOR.to_string(),
            Api::StkLimit => API_STK_LIMIT.to_string(),
            Api::SuspendD => API_SUSPEND_D.to_string(),
            Api::Forecast => API_FORECAST.to_string(),
            Api::Express => API_EXPRESS.to_string(),
            Api::Dividend => API_DIVIDEND.to_string(),
            Api::Top10Holders => API_TOP10_HOLDERS.to_string(),
            Api::Top10Floatholders => API_TOP10_FLOATHOLDERS.to_string(),
            Api::PledgeStat => API_PLEDGE_STAT.to_string(),
            Api::Repurchase => API_REPURCHASE.to_string(),
            Api::ShareFloat => API_SHARE_FLOAT.to_string(),
            Api::BlockTrade => API_BLOCK_TRADE.to_string(),
            Api::StkHoldchange => API_STK_HOLDCHANGE.to_string(),
            Api::HkHold => API_HK_HOLD.to_string(),
            Api::TopList => API_TOP_LIST.to_string(),
            Api::TopInst => API_TOP_INST.to_string(),
            Api::MoneyflowHsgt => API_MONEYFLOW_HSGT.to_string(),
            Api::LimitListD => API_LIMIT_LIST_D.to_string(),
            Api::FundNav => API_FUND_NAV.to_string(),
            Api::FundDiv => API_FUND_DIV.to_string(),
            Api::FundAdj => API_FUND_ADJ.to_string(),
            Api::FundShare => API_FUND_SHARE.to_string(),
            Api::CbBasic => API_CB_BASIC.to_string(),
            Api::CbDaily => API_CB_DAILY.to_string(),
            Api::CbIssue => API_CB_ISSUE.to_string(),
            Api::YcCb => API_YC_CB.to_string(),
            Api::IndexWeight => API_INDEX_WEIGHT.to_string(),
            Api::FutBasic => API_FUT_BASIC.to_string(),
            Api::FutDaily => API_FUT_DAILY.to_string(),
            Api::FutHolding => API_FUT_HOLDING.to_string(),
            Api::HkBasic => API_HK_BASIC.to_string(),
            Api::HkDaily => API_HK_DAILY.to_string(),
            Api::OptBasic => API_OPT_BASIC.to_string(),
            Api::OptDaily => API_OPT_DAILY.to_string(),
            Api::Shibor => API_SHIBOR.to_string(),
            Api::CnLpr => API_CN_LPR.to_string(),
            Api::CnGdp => API_CN_GDP.to_string(),
            Api::CnCpi => API_CN_CPI.to_string(),
            Api::CnPpi => API_CN_PPI.to_string(),
            Api::CnM => API_CN_M.to_string(),
            Api::CnPmi => API_CN_PMI.to_string(),
            Api::BoDaily => API_BO_DAILY.to_string(),
            Api::BoWeekly => API_BO_WEEKLY.to_string(),
            Api::FxObasic => API_FX_OBASIC.to_string(),
            Api::FxDaily => API_FX_DAILY.to_string(),
            Api::Custom(name) => name.clone(),
        }
    }
}

impl<'de> Deserialize<'de> for Api {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ApiVisitor;

        impl<'de> de::Visitor<'de> for ApiVisitor {
            type Value = Api;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string API name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Some(api) = Api::from_api_str(value) {
                    return Ok(api);
                }
                Ok(Api::Custom(value.to_string()))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_str(ApiVisitor)
    }
}

/// Serialize Api enum to string
pub fn serialize_api_name<S>(api: &Api, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&api.name())
}

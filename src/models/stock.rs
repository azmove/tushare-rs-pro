//! Stock market data models.

use crate::DeriveFromTushareData;

/// 股票基本信息 (stock_basic)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct StockBasicModel {
    pub ts_code: String,
    pub symbol: String,
    pub name: String,
    pub area: Option<String>,
    pub industry: Option<String>,
    pub fullname: Option<String>,
    pub enname: Option<String>,
    pub cnspell: Option<String>,
    pub market: Option<String>,
    pub exchange: Option<String>,
    pub curr_type: Option<String>,
    pub list_status: Option<String>,
    pub list_date: Option<String>,
    pub delist_date: Option<String>,
    pub is_hs: Option<String>,
    pub act_name: Option<String>,
    pub act_ent_type: Option<String>,
}

/// 交易日历 (trade_cal)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct TradeCalModel {
    pub exchange: String,
    pub cal_date: String,
    pub is_open: String,
    pub pretrade_date: Option<String>,
}

/// 每日股本 (daily_share)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct DailyShareModel {
    pub ts_code: String,
    pub trade_date: String,
    pub total_share: Option<f64>,
    pub float_share: Option<f64>,
    pub free_share: Option<f64>,
    pub total_mv: Option<f64>,
    pub circ_mv: Option<f64>,
}

/// ST股票列表 (st_stock)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct StStockModel {
    pub ts_code: String,
    pub name: String,
    pub st_date: Option<String>,
    pub end_date: Option<String>,
}

/// 沪深港通成分股 (hs_const)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct HsConstModel {
    pub ts_code: String,
    pub name: String,
    pub area: Option<String>,
    pub industry: Option<String>,
    pub market: Option<String>,
    pub list_date: Option<String>,
}

/// 股票曾用名 (namechange)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct NameChangeModel {
    pub ts_code: String,
    pub name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub ann_date: Option<String>,
    pub change_reason: Option<String>,
}

/// 上市公司基本信息 (stock_company)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct StockCompanyModel {
    pub ts_code: String,
    pub exchange: String,
    pub chairman: Option<String>,
    pub manager: Option<String>,
    pub secretary: Option<String>,
    pub reg_capital: Option<f64>,
    pub setup_date: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub introduction: Option<String>,
    pub website: Option<String>,
    pub employees: Option<i64>,
    pub main_business: Option<String>,
    pub business_scope: Option<String>,
}

/// 上市公司管理层 (stk_managers)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct StkManagerModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub resume: Option<String>,
    pub gender: Option<String>,
}

/// IPO新股上市 (new_share)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct NewShareModel {
    pub ts_code: String,
    pub sub_code: Option<String>,
    pub name: String,
    pub ipo_date: Option<String>,
    pub issue_date: Option<String>,
    pub amount: Option<f64>,
    pub market_amount: Option<f64>,
    pub price: Option<f64>,
    pub pe: Option<f64>,
    pub limit_amount: Option<f64>,
    pub ballot: Option<f64>,
    pub win_date: Option<String>,
    pub list_date: Option<String>,
    pub market: Option<String>,
}

// ==================== 行情数据 ====================

/// 日线行情 (daily)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct DailyModel {
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

/// 周线行情 (weekly)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct WeeklyModel {
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

/// 月线行情 (monthly)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct MonthlyModel {
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

/// 复权因子 (adj_factor)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct AdjFactorModel {
    pub ts_code: String,
    pub trade_date: String,
    pub adj_factor: Option<f64>,
}

/// 每日指标 (daily_basic)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct DailyBasicModel {
    pub ts_code: String,
    pub trade_date: String,
    pub close: Option<f64>,
    pub turnover_rate: Option<f64>,
    pub turnover_rate_f: Option<f64>,
    pub volume_ratio: Option<f64>,
    pub pe: Option<f64>,
    pub pe_ttm: Option<f64>,
    pub pb: Option<f64>,
    pub ps: Option<f64>,
    pub ps_ttm: Option<f64>,
    pub dv_ratio: Option<f64>,
    pub dv_ttm: Option<f64>,
    pub total_share: Option<f64>,
    pub float_share: Option<f64>,
    pub free_share: Option<f64>,
    pub total_mv: Option<f64>,
    pub circ_mv: Option<f64>,
}

/// 每日涨跌停价格 (stk_limit)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct StkLimitModel {
    pub ts_code: String,
    pub trade_date: String,
    pub up_limit: Option<f64>,
    pub down_limit: Option<f64>,
}

/// 每日停复牌信息 (suspend_d)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct SuspendModel {
    pub ts_code: String,
    pub suspend_date: Option<String>,
    pub resume_date: Option<String>,
    pub suspend_reason: Option<String>,
}

// ==================== 财务数据 ====================

/// 利润表 (income)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct IncomeModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub f_ann_date: Option<String>,
    pub end_date: String,
    pub report_type: Option<String>,
    pub comp_type: Option<String>,
    pub basic_eps: Option<f64>,
    pub diluted_eps: Option<f64>,
    pub total_revenue: Option<f64>,
    pub revenue: Option<f64>,
    pub oper_cost: Option<f64>,
    pub total_cogs: Option<f64>,
    pub sell_exp: Option<f64>,
    pub admin_exp: Option<f64>,
    pub fin_exp: Option<f64>,
    pub operate_profit: Option<f64>,
    pub total_profit: Option<f64>,
    pub income_tax: Option<f64>,
    pub n_income: Option<f64>,
    pub n_income_attr_p: Option<f64>,
    pub minority_gain: Option<f64>,
    pub rd_exp: Option<f64>,
}

/// 资产负债表 (balancesheet)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct BalancesheetModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub f_ann_date: Option<String>,
    pub end_date: String,
    pub report_type: Option<String>,
    pub comp_type: Option<String>,
    pub total_assets: Option<f64>,
    pub total_liab: Option<f64>,
    pub total_hldr_eqy_exc_min_int: Option<f64>,
    pub total_hldr_eqy_inc_min_int: Option<f64>,
    pub cap_rese: Option<f64>,
    pub undistr_porfit: Option<f64>,
    pub surplus_rese: Option<f64>,
    pub special_rese: Option<f64>,
    pub money_cap: Option<f64>,
    pub trad_asset: Option<f64>,
    pub notes_receiv: Option<f64>,
    pub accounts_receiv: Option<f64>,
    pub inventories: Option<f64>,
    pub fix_assets: Option<f64>,
    pub total_cur_assets: Option<f64>,
    pub total_nca: Option<f64>,
    pub total_cur_liab: Option<f64>,
    pub total_ncl: Option<f64>,
}

/// 现金流量表 (cashflow)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct CashflowModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub f_ann_date: Option<String>,
    pub end_date: String,
    pub report_type: Option<String>,
    pub comp_type: Option<String>,
    pub n_cashflow_act: Option<f64>,
    pub n_cashflow_inv_act: Option<f64>,
    pub n_cash_flows_fnc_act: Option<f64>,
    pub c_fr_sale_sg: Option<f64>,
    pub rcv_tax_incr: Option<f64>,
    pub n_depos_incr_fi: Option<f64>,
    pub c_paid_for_taxes: Option<f64>,
    pub c_paid_to_for_emp: Option<f64>,
    pub pay_dist_dpcp_int_exp: Option<f64>,
    pub cash_recp_sg_and_a: Option<f64>,
    pub cash_pay_acq_const_fia: Option<f64>,
}

/// 业绩预告 (forecast)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct ForecastModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub end_date: Option<String>,
    #[tushare(field = "type")]
    pub forecast_type: Option<String>,
    pub p_change_min: Option<f64>,
    pub p_change_max: Option<f64>,
    pub net_profit_min: Option<f64>,
    pub net_profit_max: Option<f64>,
    pub last_parent_net: Option<f64>,
    pub first_ann_date: Option<String>,
    pub summary: Option<String>,
    pub change_reason: Option<String>,
}

/// 业绩快报 (express)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct ExpressModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub end_date: Option<String>,
    pub revenue: Option<f64>,
    pub operate_profit: Option<f64>,
    pub total_profit: Option<f64>,
    pub n_income: Option<f64>,
    pub total_assets: Option<f64>,
    pub total_hldr_eqy_exc_min_int: Option<f64>,
    pub diluted_eps: Option<f64>,
    pub diluted_roe: Option<f64>,
    pub yoy_net_profit: Option<f64>,
    pub bps: Option<f64>,
    pub perf_summary: Option<String>,
}

/// 分红送股 (dividend)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct DividendModel {
    pub ts_code: String,
    pub end_date: Option<String>,
    pub ann_date: Option<String>,
    pub div_proc: Option<String>,
    pub stk_div: Option<f64>,
    pub stk_bo_rate: Option<f64>,
    pub stk_co_rate: Option<f64>,
    pub cash_div: Option<f64>,
    pub cash_div_tax: Option<f64>,
    pub record_date: Option<String>,
    pub ex_date: Option<String>,
    pub pay_date: Option<String>,
    pub div_listdate: Option<String>,
    pub imp_ann_date: Option<String>,
}

/// 财务指标 (fina_indicator)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FinaIndicatorModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub end_date: String,
    pub roe: Option<f64>,
    pub roe_waa: Option<f64>,
    pub roe_dt: Option<f64>,
    pub roa: Option<f64>,
    pub roa2: Option<f64>,
    pub netprofit_margin: Option<f64>,
    pub grossprofit_margin: Option<f64>,
    pub current_ratio: Option<f64>,
    pub quick_ratio: Option<f64>,
    pub debt_to_assets: Option<f64>,
    pub assets_turn: Option<f64>,
    pub op_yoy: Option<f64>,
    pub ebt_yoy: Option<f64>,
    pub netprofit_yoy: Option<f64>,
    pub dt_netprofit_yoy: Option<f64>,
    pub tr_yoy: Option<f64>,
    pub or_yoy: Option<f64>,
    pub equity_yoy: Option<f64>,
}

/// 主营业务构成 (fina_mainbz)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FinaMainbzModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub end_date: String,
    #[tushare(field = "type")]
    pub bz_type: Option<String>,
    pub bz_item: Option<String>,
    pub bz_sales: Option<f64>,
    pub bz_profit: Option<f64>,
    pub bz_cost: Option<f64>,
    pub curr_type: Option<String>,
}

// ==================== 参考数据 ====================

/// 前十大股东 (top10_holders)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct Top10HoldersModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub end_date: Option<String>,
    pub holder_name: Option<String>,
    pub hold_amount: Option<f64>,
    pub hold_ratio: Option<f64>,
    pub hold_float_ratio: Option<f64>,
    pub hold_change: Option<f64>,
    pub holder_type: Option<String>,
}

/// 前十大流通股东 (top10_floatholders)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct Top10FloatholdersModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub end_date: Option<String>,
    pub holder_name: Option<String>,
    pub hold_amount: Option<f64>,
    pub hold_ratio: Option<f64>,
    pub hold_float_ratio: Option<f64>,
    pub hold_change: Option<f64>,
    pub holder_type: Option<String>,
}

/// 股权质押统计 (pledge_stat)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct PledgeStatModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub end_date: Option<String>,
    pub pledge_amount: Option<f64>,
    pub pledge_ratio: Option<f64>,
    pub total_amount: Option<f64>,
    pub total_ratio: Option<f64>,
}

/// 股票回购 (repurchase)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct RepurchaseModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub end_date: Option<String>,
    #[tushare(field = "proc")]
    pub proc_status: Option<String>,
    pub exp_date: Option<String>,
    pub amount: Option<f64>,
    pub vol: Option<f64>,
    pub high_limit: Option<f64>,
    pub low_limit: Option<f64>,
}

/// 限售股解禁 (share_float)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct ShareFloatModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub float_date: Option<String>,
    pub float_share: Option<f64>,
    pub float_ratio: Option<f64>,
    pub holder_name: Option<String>,
    pub share_type: Option<String>,
}

/// 大宗交易 (block_trade)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct BlockTradeModel {
    pub ts_code: String,
    pub trade_date: Option<String>,
    pub price: Option<f64>,
    pub vol: Option<f64>,
    pub amount: Option<f64>,
    pub buyer: Option<String>,
    pub seller: Option<String>,
}

/// 股东人数 (stk_holdernumber)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct StkHoldernumberModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub end_date: Option<String>,
    pub holder_num: Option<i64>,
}

/// 股东增减持 (stk_holdchange)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct StkHoldchangeModel {
    pub ts_code: String,
    pub ann_date: Option<String>,
    pub holder_name: Option<String>,
    pub hold_amount: Option<f64>,
    pub change_amount: Option<f64>,
    pub change_ratio: Option<f64>,
}

// ==================== 特色数据 ====================

/// 沪深股通持股明细 (hk_hold)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct HkHoldModel {
    pub ts_code: String,
    pub trade_date: Option<String>,
    pub hold_amount: Option<f64>,
    pub hold_ratio: Option<f64>,
}

/// 龙虎榜每日统计 (top_list)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct TopListModel {
    pub ts_code: String,
    pub trade_date: Option<String>,
    pub name: Option<String>,
    pub close: Option<f64>,
    pub pct_change: Option<f64>,
    pub turnover_rate: Option<f64>,
    pub amount: Option<f64>,
    pub reason: Option<String>,
}

/// 龙虎榜机构明细 (top_inst)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct TopInstModel {
    pub ts_code: String,
    pub trade_date: Option<String>,
    pub exalter: Option<String>,
    pub buy: Option<f64>,
    pub buy_rate: Option<f64>,
    pub sell: Option<f64>,
    pub sell_rate: Option<f64>,
    pub net_buy: Option<f64>,
}

/// 融资融券交易汇总 (margin)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct MarginModel {
    pub trade_date: Option<String>,
    pub rzye: Option<f64>,
    pub rzmre: Option<f64>,
    pub rzche: Option<f64>,
    pub rqye: Option<f64>,
    pub rqmcl: Option<f64>,
    pub rqchl: Option<f64>,
    pub rzrqye: Option<f64>,
}

/// 融资融券交易明细 (margin_detail)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct MarginDetailModel {
    pub ts_code: String,
    pub trade_date: Option<String>,
    pub rzye: Option<f64>,
    pub rqye: Option<f64>,
    pub rzmre: Option<f64>,
    pub rzche: Option<f64>,
    pub rqmcl: Option<f64>,
    pub rqchl: Option<f64>,
}

/// 个股资金流向 (moneyflow)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct MoneyflowModel {
    pub ts_code: String,
    pub trade_date: Option<String>,
    pub buy_sm_vol: Option<f64>,
    pub buy_sm_amount: Option<f64>,
    pub sell_sm_vol: Option<f64>,
    pub sell_sm_amount: Option<f64>,
    pub buy_lg_vol: Option<f64>,
    pub buy_lg_amount: Option<f64>,
    pub sell_lg_vol: Option<f64>,
    pub sell_lg_amount: Option<f64>,
    pub net_mf_vol: Option<f64>,
    pub net_mf_amount: Option<f64>,
}

/// 沪深港通资金流向 (moneyflow_hsgt)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct HsgtMoneyflowModel {
    pub trade_date: Option<String>,
    pub ggt_ss: Option<f64>,
    pub ggt_sz: Option<f64>,
    pub hgt: Option<f64>,
    pub sgt: Option<f64>,
    pub north_money: Option<f64>,
    pub south_money: Option<f64>,
}

/// 涨跌停和炸板数据 (limit_list_d)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct LimitListDModel {
    pub trade_date: Option<String>,
    pub ts_code: Option<String>,
    pub name: Option<String>,
    pub close: Option<f64>,
    pub pct_chg: Option<f64>,
    pub up_stat: Option<String>,
    pub limit: Option<String>,
    pub fd_amount: Option<f64>,
    pub first_time: Option<String>,
    pub last_time: Option<String>,
    pub open_times: Option<i32>,
}

/// 同花顺概念板块 (ths_index)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct ThsIndexModel {
    pub ts_code: String,
    pub name: Option<String>,
    pub count: Option<i32>,
    pub exchange: Option<String>,
    pub list_date: Option<String>,
    #[tushare(field = "type")]
    pub ths_type: Option<String>,
}

/// 同花顺概念成分 (ths_member)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct ThsMemberModel {
    pub ts_code: String,
    pub name: Option<String>,
    pub con_code: Option<String>,
    pub con_name: Option<String>,
    pub weight: Option<f64>,
    pub in_date: Option<String>,
    pub out_date: Option<String>,
}

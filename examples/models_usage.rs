//! 演示新预定义模型的用法
//!
//! 运行: TUSHARE_TOKEN=your_token cargo run --example models_usage

use tushare_api::{
    Api, TushareClient, TushareEntityList, TushareRequest,
    fields, params, request,
    models::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 1. 股票基本信息 — 使用预定义 StockBasicModel
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("═══ 1. 股票基本信息 (StockBasicModel) ═══");

    let req = request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "symbol", "name", "area", "industry", "market", "list_date"
    ]);

    let stocks: TushareEntityList<StockBasicModel> = client.call_api_as(req).await?;
    println!("在线股票总数: {}", stocks.len());
    for s in stocks.iter().take(5) {
        println!(
            "  {} {} | {} | {}",
            s.ts_code,
            s.name,
            s.area.as_deref().unwrap_or("-"),
            s.industry.as_deref().unwrap_or("-")
        );
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 2. 日线行情 — 使用预定义 DailyModel
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("\n═══ 2. 比亚迪日线行情 (DailyModel) ═══");

    let req = request!(Api::Daily, {
        "ts_code" => "002594.SZ",
        "start_date" => "20260101",
        "end_date" => "20260401"
    }, [
        "ts_code", "trade_date", "open", "high", "low", "close", "vol", "amount"
    ]);

    let daily: TushareEntityList<DailyModel> = client.call_api_as(req).await?;
    println!("获取 {} 条日线数据", daily.len());
    for d in daily.iter().take(3) {
        println!(
            "  {} | 开 {:.2} 高 {:.2} 低 {:.2} 收 {:.2} | 量 {:.0}",
            d.trade_date,
            d.open.unwrap_or(0.0),
            d.high.unwrap_or(0.0),
            d.low.unwrap_or(0.0),
            d.close.unwrap_or(0.0),
            d.vol.unwrap_or(0.0)
        );
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 3. 财务指标 — 使用预定义 FinaIndicatorModel
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("\n═══ 3. 美的集团财务指标 (FinaIndicatorModel) ═══");

    let req = request!(Api::FinaIndicator, {
        "ts_code" => "000333.SZ"
    }, [
        "ts_code", "end_date", "roe", "roa", "grossprofit_margin",
        "netprofit_margin", "current_ratio", "debt_to_assets"
    ]);

    let fina: TushareEntityList<FinaIndicatorModel> = client.call_api_as(req).await?;
    println!("获取 {} 期财务指标", fina.len());
    for f in fina.iter().take(4) {
        println!(
            "  {} | ROE {:.2}% | ROA {:.2}% | 毛利率 {:.2}% | 资产负债率 {:.2}%",
            f.end_date,
            f.roe.unwrap_or(0.0),
            f.roa.unwrap_or(0.0),
            f.grossprofit_margin.unwrap_or(0.0),
            f.debt_to_assets.unwrap_or(0.0)
        );
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 4. 指数成分权重 — 使用预定义 IndexWeightModel
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("\n═══ 4. 沪深300成分权重 (IndexWeightModel) ═══");

    let req = request!(Api::IndexWeight, {
        "index_code" => "399300.SZ",
        "start_date" => "20260101",
        "end_date" => "20260401"
    }, [
        "index_code", "con_code", "trade_date", "weight"
    ]);

    let weights: TushareEntityList<IndexWeightModel> = client.call_api_as(req).await?;
    println!("获取 {} 条权重数据", weights.len());
    for w in weights.iter().take(5) {
        println!(
            "  {} | 成分 {} | 权重 {:.4}%",
            w.trade_date.as_deref().unwrap_or("-"),
            w.con_code,
            w.weight.unwrap_or(0.0)
        );
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 5. 可转债行情 — 使用预定义 CbDailyModel
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("\n═══ 5. 可转债日行情 (CbDailyModel) ═══");

    let req = request!(Api::CbDaily, {
        "ts_code" => "113009.SH",
        "start_date" => "20260101",
        "end_date" => "20260401"
    }, [
        "ts_code", "trade_date", "open", "high", "low", "close", "vol", "amount"
    ]);

    let cb: TushareEntityList<CbDailyModel> = client.call_api_as(req).await?;
    println!("获取 {} 条可转债行情", cb.len());
    for c in cb.iter().take(3) {
        println!(
            "  {} | 收 {:.2} | 量 {:.0}",
            c.trade_date,
            c.close.unwrap_or(0.0),
            c.vol.unwrap_or(0.0)
        );
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 6. 期货持仓 — 使用预定义 FuturesHoldModel
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("\n═══ 6. 期货持仓排名 (FuturesHoldModel) ═══");

    let req = request!(Api::FutHolding, {
        "trade_date" => "20260320",
        "symbol" => "IF"
    }, [
        "trade_date", "symbol", "broker", "vol", "vol_chg", "long_hld", "short_hld"
    ]);

    let holds: TushareEntityList<FuturesHoldModel> = client.call_api_as(req).await?;
    println!("获取 {} 条持仓数据", holds.len());
    for h in holds.iter().take(5) {
        println!(
            "  {} | {} | 量 {:.0} | 多 {:.0} | 空 {:.0}",
            h.symbol.as_deref().unwrap_or("-"),
            h.broker.as_deref().unwrap_or("-"),
            h.vol.unwrap_or(0.0),
            h.long_hld.unwrap_or(0.0),
            h.short_hld.unwrap_or(0.0)
        );
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 7. 宏观经济 — Shibor 利率
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("\n═══ 7. Shibor利率 (ShiborModel) ═══");

    let req = request!(Api::Shibor, {
        "start_date" => "20260301",
        "end_date" => "20260401"
    }, [
        "date", "on", "1w", "1m", "3m", "6m", "1y"
    ]);

    let rates: TushareEntityList<ShiborModel> = client.call_api_as(req).await?;
    println!("获取 {} 天利率数据", rates.len());
    for r in rates.iter().take(3) {
        println!(
            "  {} | 隔夜 {:.4} | 1周 {:.4} | 1月 {:.4} | 1年 {:.4}",
            r.date,
            r.on.unwrap_or(0.0),
            r.w_1.unwrap_or(0.0),
            r.m_1.unwrap_or(0.0),
            r.y_1.unwrap_or(0.0)
        );
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 8. 龙虎榜 — TopListModel
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("\n═══ 8. 龙虎榜 (TopListModel) ═══");

    let req = request!(Api::TopList, {
        "trade_date" => "20260320"
    }, [
        "ts_code", "trade_date", "name", "close", "pct_change", "amount", "reason"
    ]);

    let tops: TushareEntityList<TopListModel> = client.call_api_as(req).await?;
    println!("获取 {} 条龙虎榜数据", tops.len());
    for t in tops.iter().take(5) {
        println!(
            "  {} {} | 收 {:.2} | 涨幅 {:.2}% | {}",
            t.ts_code,
            t.name.as_deref().unwrap_or("-"),
            t.close.unwrap_or(0.0),
            t.pct_change.unwrap_or(0.0),
            t.reason.as_deref().unwrap_or("-")
        );
    }

    println!("\n✅ 全部演示完成！共使用了 8 种预定义模型。");
    Ok(())
}

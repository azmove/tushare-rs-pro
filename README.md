# 🦀 tushare-rs-pro — Tushare Pro API Rust SDK

[![CI](https://github.com/azmove/tushare-rs-pro/actions/workflows/ci.yml/badge.svg)](https://github.com/azmove/tushare-rs-pro/actions)
[![Crates.io](https://img.shields.io/crates/v/tushare-api.svg)](https://crates.io/crates/tushare-api)
[![Documentation](https://docs.rs/tushare-api/badge.svg)](https://docs.rs/tushare-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **Rust SDK for [Tushare Pro](https://tushare.pro/) — 中国金融数据 API**
>
> 异步、类型安全、77 个预定义数据模型覆盖 12 大领域，derive macro 自动映射，开箱即用。

**tushare-rs-pro** 是一个面向量化交易和金融数据分析的 Rust 客户端库，提供：

- 🔒 编译期类型安全的 86 个 API 枚举，杜绝拼写错误
- 📊 77 个预定义 struct 覆盖股票、基金、指数、债券、ETF、期货、期权、港股、美股、外汇、宏观经济、行业分类
- ⚡ 基于 `tokio` + `reqwest` 的全异步架构
- 🔄 生产级重试、限流、超时控制
- 🏗️ `#[derive(DeriveFromTushareData)]` 宏自动将 API 响应映射到自定义 struct

## 目录

- [✨ 特性亮点](#-特性亮点)
- [📋 前置条件](#-前置条件)
- [📦 安装](#-安装)
- [🚀 快速开始](#-快速开始)
  - [1. 最简示例：获取股票列表](#1-最简示例获取股票列表)
  - [2. 获取日线行情](#2-获取日线行情)
  - [3. 使用 TushareClientEx（限流 + 自动重试）](#3-使用-tushareclientex限流--自动重试)
  - [4. 自定义 Struct](#4-自定义-struct不使用预定义模型)
- [📖 API 使用指南](#-api-使用指南)
  - [1. 如何导入 Tushare API](#1-如何导入-tushare-api)
  - [2. 如何创建 Tushare 客户端](#2-如何创建-tushare-客户端)
  - [2.1 使用 TushareClientEx(限流、失败重试)](#21-使用-tushareclientex)
  - [3. 如何发送请求](#3-如何发送请求)
  - [4. 将返回的数据转换为自定义结构体](#4-将返回的数据转换为自定义结构体)
  - [5. 如何设置日志](#5-如何设置日志)
- [🗂️ 预定义模型](#️-预定义模型)
- [🧪 运行示例](#-运行示例)
- [🙏 致谢](#-致谢)
- [📄 许可证](#-许可证)
- [📞 支持](#-支持)

## ✨ 特性亮点

| 特性 | 说明 |
|------|------|
| 🚀 异步 | 基于 `tokio` + `reqwest`，高性能并发请求 |
| 🔒 类型安全 | 86 个 `Api` 枚举变体，编译期保证 API 名称正确 |
| 📦 预定义模型 | 77 个 struct 覆盖 12 大领域，`#[derive]` 自动映射 |
| 🔄 生产就绪 | 限流、指数退避重试、超时、全面错误处理 |
| 📊 第三方类型 | 可选支持 `rust_decimal`、`chrono`、`uuid`、`bigdecimal` |
| 🏗️ 开发者友好 | 宏和构建器模式，自定义 struct 几行代码搞定 |

## 📋 前置条件

- **Rust** ≥ 1.85（2024 edition）
- **Tushare API Token**：在 [Tushare Pro](https://tushare.pro/) 注册获取

## 📦 安装

### 方式一：作为依赖引入（推荐）

```toml
[dependencies]
tushare-rs-pro = { git = "https://github.com/azmove/tushare-rs-pro.git" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### 方式二：从源码构建

```bash
git clone https://github.com/azmove/tushare-rs-pro.git
cd tushare-rs-pro
cargo build    # 自动编译主库 + tushare-derive 宏
cargo test     # 运行 14 个测试
```

> **项目结构说明：** `tushare-derive/` 是 proc macro crate，提供 `#[derive(DeriveFromTushareData)]`。
> Rust 要求 proc macro 必须在独立 crate 中，这与 `serde`/`serde_derive` 是相同模式。
> 用户无需关心它——Cargo workspace 会自动处理编译。

## 🚀 快速开始

### 1. 最简示例：获取股票列表

```rust
use tushare_api::{TushareClient, Api, TushareEntityList, request};
use tushare_api::models::StockBasicModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置 Token（也可设环境变量 TUSHARE_TOKEN）
    let client = TushareClient::new("your_token_here");

    // 构建请求
    let req = request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "name", "industry", "list_date"
    ]);

    // 调用并解析为强类型
    let stocks: TushareEntityList<StockBasicModel> = client.call_api_as(req).await?;

    for s in stocks.iter().take(5) {
        println!("{} {} 行业:{}", 
            s.ts_code, s.name, 
            s.industry.as_deref().unwrap_or("-"));
    }
    Ok(())
}
```

### 2. 获取日线行情

```rust
use tushare_api::{TushareClient, Api, TushareEntityList, request};
use tushare_api::models::DailyModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?; // 从 TUSHARE_TOKEN 环境变量读取

    let req = request!(Api::Daily, {
        "ts_code" => "000001.SZ",
        "start_date" => "20260101",
        "end_date" => "20260401"
    }, [
        "ts_code", "trade_date", "open", "high", "low", "close", "vol"
    ]);

    let data: TushareEntityList<DailyModel> = client.call_api_as(req).await?;

    for d in data.iter().take(3) {
        println!("{} 开:{:.2} 高:{:.2} 低:{:.2} 收:{:.2}",
            d.trade_date,
            d.open.unwrap_or(0.0),
            d.high.unwrap_or(0.0),
            d.low.unwrap_or(0.0),
            d.close.unwrap_or(0.0));
    }
    Ok(())
}
```

### 3. 使用 TushareClientEx（限流 + 自动重试）

```rust
use std::time::Duration;
use tushare_api::{TushareClient, TushareClientEx, RetryConfig, Api, TushareEntityList, request};
use tushare_api::models::IncomeModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inner = TushareClient::from_env()?;

    // 包装为 ClientEx：限流 + 指数退避重试
    let client = TushareClientEx::new(inner)
        .with_api_min_interval(Api::Income, Duration::from_secs(1)) // 同一 API 至少间隔 1 秒
        .with_retry_config(RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(5),
        });

    let req = request!(Api::Income, {
        "ts_code" => "000001.SZ",
        "period" => "20251231"
    }, [
        "ts_code", "ann_date", "end_date", "revenue", "n_income"
    ]);

    let data: TushareEntityList<IncomeModel> = client.call_api_as(&req).await?;
    
    for d in data.iter() {
        println!("{} 营收:{:?} 净利润:{:?}", 
            d.end_date, d.revenue, d.n_income);
    }
    Ok(())
}
```

### 4. 自定义 Struct（不使用预定义模型）

```rust
use tushare_api::{DeriveFromTushareData, TushareClient, Api, TushareEntityList, request};

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct MyStock {
    pub ts_code: String,
    pub name: String,
    #[tushare(field = "list_date")]   // API 字段名映射
    pub listing_date: Option<String>,
    #[tushare(skip)]                   // 跳过，使用 Default
    pub local_note: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    let req = request!(Api::StockBasic, {
        "list_status" => "L"
    }, ["ts_code", "name", "list_date"]);

    let stocks: TushareEntityList<MyStock> = client.call_api_as(req).await?;
    println!("共 {} 只股票", stocks.len());
    Ok(())
}
```

## 📖 API 使用指南

### 1. 如何导入 Tushare API

```rust
// 基础导入
use tushare_api::{TushareClient, TushareRequest, TushareResponse, Api, TushareResult};

// 便捷宏
use tushare_api::{params, fields, request};

// 日志配置
use tushare_api::{LogLevel, LogConfig, Logger};

// HTTP 客户端配置和连接池设置
use tushare_api::{TushareClientBuilder, HttpClientConfig};

// 自定义超时
use std::time::Duration;
```

### 2. 如何创建 Tushare 客户端

库提供多种创建客户端的方式：

#### 方法 1：使用环境变量（推荐）

```rust
// 首先设置环境变量
std::env::set_var("TUSHARE_TOKEN", "your_token_here");

// 使用默认超时设置
let client = TushareClient::from_env()?;

// 使用自定义超时设置
let client = TushareClient::from_env_with_timeout(
    Duration::from_secs(10),  // 连接超时 10 秒
    Duration::from_secs(60)   // 请求超时 60 秒
)?;
```

#### 方法 2：直接使用 Token

```rust
// 使用默认超时设置
let client = TushareClient::new("your_token_here");

// 使用自定义超时设置
let client = TushareClient::with_timeout(
    "your_token_here",
    Duration::from_secs(5),   // 连接超时 5 秒
    Duration::from_secs(60)   // 请求超时 60 秒
);
```

#### 方法 3：使用构建器模式

```rust
// 基础构建器，包含超时和日志
let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_connect_timeout(Duration::from_secs(10))
    .with_timeout(Duration::from_secs(60))
    .with_log_level(LogLevel::Debug)
    .log_requests(true)
    .log_responses(false)
    .log_sensitive_data(false)
    .log_performance(true)
    .build()?;

// 高级构建器，包含连接池设置
let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_connect_timeout(Duration::from_secs(5))
    .with_timeout(Duration::from_secs(60))
    .with_pool_max_idle_per_host(20)  // 每个主机最多 20 个空闲连接
    .with_pool_idle_timeout(Duration::from_secs(60))  // 连接保持 60 秒
    .with_log_level(LogLevel::Info)
    .build()?;

// 使用 HttpClientConfig 进行高级 HTTP 设置
let http_config = HttpClientConfig::new()
    .with_connect_timeout(Duration::from_secs(3))
    .with_timeout(Duration::from_secs(30))
    .with_pool_max_idle_per_host(15)
    .with_pool_idle_timeout(Duration::from_secs(45));

let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_http_config(http_config)
    .with_log_level(LogLevel::Debug)
    .build()?;
```

### 2.1 使用 TushareClientEx

`TushareClientEx` 是对 `TushareClient` 的包装，用于提供额外能力（如按 API 的最小间隔限流、失败重试等）。

```rust
use std::time::Duration;
use tushare_api::{Api, TushareClient, TushareClientEx, TushareEntityList, request};
use tushare_api::client_ex::RetryConfig;
use tushare_api::DeriveFromTushareData;

#[derive(Debug, Clone, DeriveFromTushareData)]
struct Stock {
    ts_code: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inner = TushareClient::from_env()?;

    let client = TushareClientEx::new(inner)
        .with_api_min_interval(Api::Daily, Duration::from_secs(10))
        .with_retry_config(RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(5),
        });

    let req = request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "name"
    ]);

    let stocks: TushareEntityList<Stock> = client.call_api_as(&req).await?;
    println!("{}", stocks.len());

    Ok(())
}
```

### 3. 如何发送请求

#### 方法 1：使用便捷宏（推荐）

```rust
use tushare_api::{request, Api};

// 带参数和字段的单个 API 调用
let response = client.call_api(request!(Api::StockBasic, {
    "list_status" => "L",
    "exchange" => "SSE"
}, [
    "ts_code", "name", "industry", "area"
])).await?;

// 无参数的 API 调用
let response = client.call_api(request!(Api::TradeCal, {}, [
    "exchange", "cal_date", "is_open"
])).await?;

// 无字段的 API 调用（获取所有字段）
let response = client.call_api(request!(Api::StockBasic, {
    "list_status" => "L"
}, [])).await?;
```

#### 方法 2：使用单独的宏

```rust
use tushare_api::{TushareRequest, Api, params, fields};

let request = TushareRequest {
    api_name: Api::StockBasic,
    params: params!("list_status" => "L", "exchange" => "SSE"),
    fields: fields!("ts_code", "name", "industry"),
};

let response = client.call_api(request).await?;
```

#### 方法 3：手动构建

```rust
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("list_status".to_string(), "L".to_string());

let request = TushareRequest {
    api_name: Api::StockBasic,
    params,
    fields: vec!["ts_code".to_string(), "name".to_string()],
};

let response = client.call_api(request).await?;
```

#### 方法 4：直接使用字符串参数

```rust
// 直接传 JSON 字符串（适合快速调试/复制粘贴 API 请求）
let response = client
    .call_api(
        r#"
        {
            "api_name": "stock_basic",
            "params": { "list_status": "L", "exchange": "SSE" },
            "fields": ["ts_code", "name", "industry", "area"]
        }
        "#,
    )
    .await?;
```

#### 方法 5：任何实现了 `TryInto<TushareRequest>` 的类型都可以作为参数

`call_api` 的签名是泛型的：

```rust
pub async fn call_api<T>(&self, request: T) -> TushareResult<TushareResponse>
where
    T: TryInto<TushareRequest>,
    <T as TryInto<TushareRequest>>::Error: Into<TushareError>,
```

所以你可以直接传入：

```rust
// 1) 直接传 TushareRequest
let req = TushareRequest {
    api_name: Api::StockBasic,
    params: params!("list_status" => "L"),
    fields: fields!("ts_code", "name"),
};
let response = client.call_api(req).await?;

// 2) 直接传 &str 
let response = client.call_api(r#"{
    "api_name": "stock_basic",
    "params": { "list_status": "L" },
    "fields": ["ts_code", "name"]
}"#).await?;

// 3) 直接传 String
let json = r#"{
    "api_name": "stock_basic",
    "params": { "list_status": "L" },
    "fields": ["ts_code", "name"]
}"#.to_string();
let response = client.call_api(json).await?;
```

### 4. 将返回的数据转换为自定义结构体

该库提供了强大的过程宏，可以自动将 Tushare API 响应转换为强类型的 Rust 结构体，无需手动解析。

#### 使用过程宏

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList};
use tushare_api::DeriveFromTushareData;

// 使用自动转换定义您的结构体
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct Stock {
    pub ts_code: String,
    pub symbol: String,
    pub name: String,
    pub area: Option<String>,
    pub industry: Option<String>,
    pub market: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    // 使用 call_api_as 进行直接转换到 TushareEntityList<Stock>
    let stocks: TushareEntityList<Stock> = client.call_api_as(request!(Api::StockBasic, {
        "list_status" => "L",
        "exchange" => "SSE"
    }, [
        "ts_code", "symbol", "name", "area", "industry", "market"
    ])).await?;
    
    // 直接访问数据
    println!("找到 {} 只股票:", stocks.len());
    for stock in stocks.iter().take(5) {
        println!("  {}: {} ({})", stock.ts_code, stock.name, stock.market);
    }
    
    // 访问分页信息
    println!("当前页面: {} 条记录", stocks.len());
    println!("总记录数: {}", stocks.count());
    println!("是否还有更多页面: {}", stocks.has_more());
    
    Ok(())
}
```

#### 字段映射和可选字段

```rust
use tushare_api::DeriveFromTushareData;

// 带字段映射和可选字段的高级结构体
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct StockInfo {
    pub ts_code: String,
    
    // 将 API 字段 "symbol" 映射到结构体字段 "stock_symbol"
    #[tushare(field = "symbol")]
    pub stock_symbol: String,
    
    pub name: String,
    
    // 可选字段会自动处理
    pub area: Option<String>,
    pub industry: Option<String>,
    
    // 跳过 API 响应中不存在的字段
    #[tushare(skip)]
    pub calculated_value: f64,
}

// 实现 Default 以便使用
impl Default for StockInfo {
    fn default() -> Self {
        Self {
            ts_code: String::new(),
            stock_symbol: String::new(),
            name: String::new(),
            area: None,
            industry: None,
            calculated_value: 0.0,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    let stock_info: TushareEntityList<StockInfo> = client.call_api_as(request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "symbol", "name", "area", "industry"
    ])).await?;
    
    for info in stock_info.iter().take(3) {
        println!("股票: {} ({}) - 行业: {:?}", 
                 info.name, info.stock_symbol, info.industry);
    }
    
    Ok(())
}
```

#### 生成的结构体说明

当您使用新的泛型分页容器时，您会得到一个清晰、类型安全的接口：

```rust
// 您的原始结构体
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct Stock {
    pub ts_code: String,
    pub name: String,
    pub area: Option<String>,
}

// 使用泛型 TushareEntityList<T> 容器：
// TushareEntityList<Stock> {
//     pub items: Vec<Stock>,        // 您的数据项
//     pub has_more: bool,           // 分页：是否还有更多页面？
//     pub count: i64,               // 分页：总记录数
// }
```

**当您调用：**
```rust
let stocks: TushareEntityList<Stock> = client.call_api_as(request).await?;
// 或者
let stocks = client.call_api_as::<Stock>(request).await?;
```

**您会得到一个 `TushareEntityList<Stock>` 结构体，包含：**
- **`items`** - `Vec<Stock>` 包含实际转换后的数据
- **`has_more`** - `bool` 表示是否还有更多页面可获取
- **`count`** - `i64` 显示可用的总记录数

**以及这些自动生成的方法：**
- `stocks.len()` - 当前页面的项目数量
- `stocks.is_empty()` - 当前页面是否为空
- `stocks.items()` - 获取项目切片
- `stocks.has_more()` - 检查是否还有更多页面
- `stocks.count()` - 获取总记录数
- `stocks.iter()` - 遍历项目（通过 Deref）
- `for stock in &stocks { ... }` - 直接迭代支持

#### 分页支持

`TushareEntityList<T>` 容器提供内置分页支持，具有清晰直观的接口：

- `items: Vec<T>` - 实际的数据项
- `has_more: bool` - 是否还有更多页面可用
- `count: i64` - 总记录数

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList};
use tushare_api::DeriveFromTushareData;

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct Stock {
    pub ts_code: String,
    pub name: String,
    pub area: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    // 获取分页结果
    let stocks: TushareEntityList<Stock> = client.call_api_as(request!(Api::StockBasic, {
        "list_status" => "L",
        "limit" => "100",
        "offset" => "0"
    }, [
        "ts_code", "name", "area"
    ])).await?;
    
    // 访问分页信息
    println!("当前页面: {} 只股票", stocks.len());
    println!("总可用数量: {} 只股票", stocks.count());
    println!("是否还有更多页面: {}", stocks.has_more());
    
    // 遍历当前页面的项目
    for stock in &stocks {
        println!("{}: {} ({})", 
                 stock.ts_code, 
                 stock.name, 
                 stock.area.as_deref().unwrap_or("未知"));
    }
    
    // 直接访问项目
    let first_stock = &stocks.items()[0];
    println!("第一只股票: {}", first_stock.name);
    
    Ok(())
}
```

#### 支持的字段类型

过程宏支持以下 Rust 类型：

- `String` - 必需的字符串字段
- `Option<String>` - 可选的字符串字段
- `f64` - 必需的浮点数
- `Option<f64>` - 可选的浮点数
- `i64` - 必需的整数
- `Option<i64>` - 可选的整数
- `bool` - 必需的布尔值
- `Option<bool>` - 可选的布尔值

#### 自定义日期格式支持

库支持使用 `#[tushare(date_format = "...")]` 属性进行自定义日期格式解析。这在处理返回非标准日期格式的 API 时特别有用。

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList, DeriveFromTushareData};

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct CustomDateFormats {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    // 标准日期格式（自动检测：YYYYMMDD、YYYY-MM-DD 等）
    #[tushare(field = "trade_date")]
    pub trade_date: chrono::NaiveDate,
    
    // 欧洲日期格式：DD/MM/YYYY
    #[tushare(field = "european_date", date_format = "%d/%m/%Y")]
    pub european_date: chrono::NaiveDate,
    
    // 美国日期格式：MM-DD-YYYY
    #[tushare(field = "us_date", date_format = "%m-%d-%Y")]
    pub us_date: chrono::NaiveDate,
    
    // 德国日期格式：DD.MM.YYYY
    #[tushare(field = "german_date", date_format = "%d.%m.%Y")]
    pub german_date: Option<chrono::NaiveDate>,
    
    // 自定义日期时间格式：YYYY/MM/DD HH:MM
    #[tushare(field = "custom_datetime", date_format = "%Y/%m/%d %H:%M")]
    pub custom_datetime: chrono::NaiveDateTime,
    
    // 中文日期格式：YYYY年MM月DD日
    #[tushare(field = "chinese_date", date_format = "%Y年%m月%d日")]
    pub chinese_date: Option<chrono::NaiveDate>,
    
    // UTC 日期时间格式：YYYY-MM-DD HH:MM:SS +ZZZZ
    #[tushare(field = "utc_datetime", date_format = "%Y-%m-%d %H:%M:%S %z")]
    pub utc_datetime: chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    // API 调用示例（注意：实际 API 可能不返回这些确切格式）
    let data: TushareEntityList<CustomDateFormats> = client.call_api_as(request!(
        Api::StockBasic, {
            "list_status" => "L",
            "limit" => "10"
        }, [
            "ts_code", "trade_date", "european_date", "us_date", 
            "german_date", "custom_datetime", "chinese_date", "utc_datetime"
        ]
    )).await?;
    
    for record in data.iter() {
        println!("股票: {} - 交易日期: {}", record.stock_code, record.trade_date);
        println!("  欧洲格式: {}", record.european_date);
        println!("  美国格式: {}", record.us_date);
        println!("  德国格式: {:?}", record.german_date);
        println!("  自定义日期时间: {:?}", record.custom_datetime);
        println!("  中文格式: {:?}", record.chinese_date);
        println!("  UTC 时间: {}", record.utc_datetime);
        println!("---");
    }
    
    Ok(())
}
```

##### 常用日期格式模式

| 格式字符串 | 示例输入 | 说明 |
|-----------|---------|------|
| `"%Y-%m-%d"` | `"2024-03-15"` | ISO 日期格式 |
| `"%d/%m/%Y"` | `"15/03/2024"` | 欧洲格式 |
| `"%m-%d-%Y"` | `"03-15-2024"` | 美国格式 |
| `"%d.%m.%Y"` | `"15.03.2024"` | 德国格式 |
| `"%Y年%m月%d日"` | `"2024年03月15日"` | 中文格式 |
| `"%Y%m%d"` | `"20240315"` | 紧凑格式 |
| `"%Y-%m-%d %H:%M:%S"` | `"2024-03-15 14:30:00"` | 日期时间格式 |
| `"%Y/%m/%d %H:%M"` | `"2024/03/15 14:30"` | 自定义日期时间 |
| `"%Y-%m-%d %H:%M:%S %z"` | `"2024-03-15 14:30:00 +0800"` | 带时区格式 |

##### 自定义日期格式的优势

- **精确控制**：为每个字段指定确切的格式
- **无需包装类型**：直接使用 chrono 类型
- **类型安全**：编译时格式验证
- **灵活性**：支持可选字段
- **清晰语法**：声明式且直观
- **错误处理**：详细的错误信息便于调试

#### 第三方类型支持

库通过可选的特性标志为流行的第三方类型提供内置支持。这对于需要高精度算术或日期/时间处理的金融应用程序特别有用。

##### 启用第三方类型

在您的 `Cargo.toml` 中添加所需的特性：

```toml
[dependencies]
# 启用特定类型
tushare-api = { version = "1.2.7", features = ["rust_decimal", "chrono"] }

# 或启用所有第三方类型
tushare-api = { version = "1.2.7", features = ["all_types"] }
```

##### 高精度小数示例

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList, DeriveFromTushareData};

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FinancialData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    #[tushare(field = "trade_date")]
    pub date: String,
    
    // 用于金融计算的高精度小数
    #[tushare(field = "close")]
    pub close_price: rust_decimal::Decimal,
    
    #[tushare(field = "vol")]
    pub volume: Option<rust_decimal::Decimal>,
    
    #[tushare(field = "amount")]
    pub amount: Option<rust_decimal::Decimal>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    let data: TushareEntityList<FinancialData> = client.call_api_as(request!(
        Api::Daily, {
            "ts_code" => "000001.SZ",
            "trade_date" => "20240315"
        }, [
            "ts_code", "trade_date", "close", "vol", "amount"
        ]
    )).await?;
    
    for record in data.iter() {
        println!("股票: {} - 价格: {} 日期: {}", 
                 record.stock_code, 
                 record.close_price, 
                 record.date);
    }
    
    Ok(())
}
```

##### 日期/时间类型示例

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList, DeriveFromTushareData};

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct DateTimeData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    // 从 YYYYMMDD 格式自动解析
    #[tushare(field = "trade_date")]
    pub trade_date: chrono::NaiveDate,
    
    // 可选的日期时间字段
    #[tushare(field = "update_time")]
    pub update_time: Option<chrono::NaiveDateTime>,
    
    // 高精度价格
    #[tushare(field = "close")]
    pub close_price: rust_decimal::Decimal,
}
```

##### 支持的第三方类型

| 类型 | 特性标志 | 说明 | 示例值 |
|------|---------|------|--------|
| `rust_decimal::Decimal` | `rust_decimal` | 高精度小数 | `"123.456"`, `123.456` |
| `bigdecimal::BigDecimal` | `bigdecimal` | 任意精度 | `"999999999999999999999.123"` |
| `chrono::NaiveDate` | `chrono` | 无时区日期 | `"20240315"`, `"2024-03-15"` |
| `chrono::NaiveDateTime` | `chrono` | 无时区日期时间 | `"2024-03-15 14:30:00"` |
| `chrono::DateTime<Utc>` | `chrono` | UTC 日期时间 | RFC3339 格式 |
| `uuid::Uuid` | `uuid` | UUID 类型 | `"550e8400-e29b-41d4-a716-446655440000"` |

详细文档和示例请参阅 [第三方类型指南](docs/THIRD_PARTY_TYPES.md)。

#### 手动转换（替代方法）

如果您不想使用过程宏，仍然可以使用手动方法：

```rust
use tushare_api::{TushareClient, Api, request, utils::response_to_vec, traits::FromTushareData};
use tushare_api::error::TushareError;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Stock {
    pub ts_code: String,
    pub name: String,
    pub area: Option<String>,
}

// 手动实现 FromTushareData
impl FromTushareData for Stock {
    fn from_row(fields: &[String], values: &[Value]) -> Result<Self, TushareError> {
        let ts_code_idx = fields.iter().position(|f| f == "ts_code")
            .ok_or_else(|| TushareError::ParseError("缺少 ts_code 字段".to_string()))?;
        let name_idx = fields.iter().position(|f| f == "name")
            .ok_or_else(|| TushareError::ParseError("缺少 name 字段".to_string()))?;
        let area_idx = fields.iter().position(|f| f == "area");
            
        Ok(Stock {
            ts_code: values[ts_code_idx].as_str()
                .ok_or_else(|| TushareError::ParseError("无效的 ts_code".to_string()))?
                .to_string(),
            name: values[name_idx].as_str()
                .ok_or_else(|| TushareError::ParseError("无效的 name".to_string()))?
                .to_string(),
            area: area_idx.and_then(|idx| values[idx].as_str().map(|s| s.to_string())),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    // 获取原始响应
    let response = client.call_api(request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "name", "area"
    ])).await?;
    
    // 转换为 Vec<Stock>
    let stocks = response_to_vec::<Stock>(response)?;
    
    println!("找到 {} 只股票", stocks.len());
    for stock in stocks.iter().take(3) {
        println!("  {}: {} - 地区: {:?}", stock.ts_code, stock.name, stock.area);
    }
    
    Ok(())
}
```

### 5. 如何设置日志

#### 使用 `env_logger`

```rust
// 设置日志级别并初始化日志器
std::env::set_var("RUST_LOG", "tushare_api=debug");
env_logger::init();

// 创建带日志配置的客户端
let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_log_level(LogLevel::Debug)
    .log_requests(true)        // 记录请求详情
    .log_responses(false)      // 不记录响应内容（可能很大）
    .log_sensitive_data(false) // 不记录敏感数据如 token
    .log_performance(true)     // 记录性能指标
    .build()?;
```

#### 使用 `tracing`（可选特性）

首先，在您的 `Cargo.toml` 中启用 tracing 特性：

```toml
[dependencies]
tushare-api = { version = "1.2.7", features = ["tracing"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

然后在您的代码中：

```rust
use tracing_subscriber;

// 初始化 tracing 订阅器
std::env::set_var("RUST_LOG", "tushare_api=trace");
tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

// 客户端配置保持不变
let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_log_level(LogLevel::Trace)
    .build()?;
```

#### 使用 `tracing-log` 桥接

```rust
use tracing_subscriber;
use tracing_log::LogTracer;

// 初始化 log-to-tracing 桥接
LogTracer::init()?;

// 设置 tracing 订阅器
std::env::set_var("RUST_LOG", "tushare_api=debug");
tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();
```

#### 日志级别和输出

- **`LogLevel::Off`**：无日志
- **`LogLevel::Error`**：仅错误
- **`LogLevel::Warn`**：错误和警告
- **`LogLevel::Info`**：基本 API 调用信息（默认）
- **`LogLevel::Debug`**：详细的请求/响应信息
- **`LogLevel::Trace`**：所有信息，包括原始响应内容

示例日志输出：
```
INFO  [abc123] Starting Tushare API call: stock_basic, params count: 2, fields count: 3
DEBUG [abc123] API request details - API: stock_basic, params: {...}, fields: [...]
DEBUG [abc123] Sending HTTP request to Tushare API
DEBUG [abc123] Received HTTP response, status code: 200
INFO  [abc123] API call successful, duration: 245ms, data rows returned: 100
```

## 🗂️ 预定义模型

库内置 **77 个预定义数据模型**，覆盖 Tushare 12 大数据领域。无需手动定义 struct，直接使用即可获得强类型返回。

### 快速使用

```rust
use tushare_api::{TushareClient, Api, TushareEntityList, request};
use tushare_api::models::DailyModel;  // 导入预定义模型

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;

    let req = request!(Api::Daily, {
        "ts_code" => "000001.SZ",
        "start_date" => "20260101",
        "end_date" => "20260401"
    }, [
        "ts_code", "trade_date", "open", "high", "low", "close", "vol"
    ]);

    // 直接解析为强类型
    let data: TushareEntityList<DailyModel> = client.call_api_as(req).await?;

    for d in data.iter().take(3) {
        println!("{} 收盘: {:.2}", d.trade_date, d.close.unwrap_or(0.0));
    }
    Ok(())
}
```

### 模型一览

#### 📈 股票 (`models::stock`) — 42 个模型

| 模型 | API | 说明 |
|------|-----|------|
| `StockBasicModel` | `stock_basic` | 股票基本信息 |
| `TradeCalModel` | `trade_cal` | 交易日历 |
| `DailyShareModel` | `daily_share` | 每日股本 |
| `StStockModel` | `st_stock` | ST股票列表 |
| `HsConstModel` | `hs_const` | 沪深港通成分股 |
| `NameChangeModel` | `namechange` | 股票曾用名 |
| `StockCompanyModel` | `stock_company` | 上市公司基本信息 |
| `StkManagerModel` | `stk_managers` | 上市公司管理层 |
| `NewShareModel` | `new_share` | IPO新股上市 |
| `DailyModel` | `daily` | 日线行情 |
| `WeeklyModel` | `weekly` | 周线行情 |
| `MonthlyModel` | `monthly` | 月线行情 |
| `AdjFactorModel` | `adj_factor` | 复权因子 |
| `DailyBasicModel` | `daily_basic` | 每日指标（PE/PB/换手率等） |
| `StkLimitModel` | `stk_limit` | 涨跌停价格 |
| `SuspendModel` | `suspend_d` | 停复牌信息 |
| `IncomeModel` | `income` | 利润表 |
| `BalancesheetModel` | `balancesheet` | 资产负债表 |
| `CashflowModel` | `cashflow` | 现金流量表 |
| `ForecastModel` | `forecast` | 业绩预告 |
| `ExpressModel` | `express` | 业绩快报 |
| `DividendModel` | `dividend` | 分红送股 |
| `FinaIndicatorModel` | `fina_indicator` | 财务指标（ROE/ROA等） |
| `FinaMainbzModel` | `fina_mainbz` | 主营业务构成 |
| `Top10HoldersModel` | `top10_holders` | 前十大股东 |
| `Top10FloatholdersModel` | `top10_floatholders` | 前十大流通股东 |
| `PledgeStatModel` | `pledge_stat` | 股权质押统计 |
| `RepurchaseModel` | `repurchase` | 股票回购 |
| `ShareFloatModel` | `share_float` | 限售股解禁 |
| `BlockTradeModel` | `block_trade` | 大宗交易 |
| `StkHoldernumberModel` | `stk_holdernumber` | 股东人数 |
| `StkHoldchangeModel` | `stk_holdchange` | 股东增减持 |
| `HkHoldModel` | `hk_hold` | 沪深股通持股明细 |
| `TopListModel` | `top_list` | 龙虎榜每日统计 |
| `TopInstModel` | `top_inst` | 龙虎榜机构明细 |
| `MarginModel` | `margin` | 融资融券汇总 |
| `MarginDetailModel` | `margin_detail` | 融资融券明细 |
| `MoneyflowModel` | `moneyflow` | 个股资金流向 |
| `HsgtMoneyflowModel` | `moneyflow_hsgt` | 沪深港通资金流向 |
| `LimitListDModel` | `limit_list_d` | 涨跌停和炸板数据 |
| `ThsIndexModel` | `ths_index` | 同花顺概念板块 |
| `ThsMemberModel` | `ths_member` | 同花顺概念成分 |

#### 💰 基金 (`models::fund`) — 4 个模型

| 模型 | API | 说明 |
|------|-----|------|
| `FundBasicModel` | `fund_basic` | 基金基本信息 |
| `FundNavModel` | `fund_nav` | 基金净值 |
| `FundDividendModel` | `fund_div` | 基金分红 |
| `FundPortfolioModel` | `fund_portfolio` | 基金持仓 |

#### 📊 指数 (`models::index`) — 3 个模型

| 模型 | API | 说明 |
|------|-----|------|
| `IndexBasicModel` | `index_basic` | 指数基本信息 |
| `IndexDailyModel` | `index_daily` | 指数日线行情 |
| `IndexWeightModel` | `index_weight` | 指数成分权重 |

#### 🏦 债券 (`models::bond`) — 4 个模型

| 模型 | API | 说明 |
|------|-----|------|
| `CbBasicModel` | `cb_basic` | 可转债基本信息 |
| `CbDailyModel` | `cb_daily` | 可转债日行情 |
| `CbIssueModel` | `cb_issue` | 可转债发行 |
| `BondYieldModel` | `yc_cb` | 国债收益率曲线 |

#### 📈 ETF (`models::etf`) — 4 个模型

| 模型 | API | 说明 |
|------|-----|------|
| `EtfBasicModel` | `fund_basic` | ETF基本信息 |
| `EtfDailyModel` | `fund_daily` | ETF日线行情 |
| `EtfAdjFactorModel` | `fund_adj` | ETF复权因子 |
| `EtfShareModel` | `fund_share` | ETF份额 |

#### 🛢️ 期货 (`models::futures`) — 3 个模型

| 模型 | API | 说明 |
|------|-----|------|
| `FuturesBasicModel` | `fut_basic` | 期货合约信息 |
| `FuturesDailyModel` | `fut_daily` | 期货日线行情 |
| `FuturesHoldModel` | `fut_holding` | 期货持仓排名 |

#### 🇭🇰 港股 (`models::hk`) — 2 个模型

| 模型 | API | 说明 |
|------|-----|------|
| `HkBasicModel` | `hk_basic` | 港股基本信息 |
| `HkDailyModel` | `hk_daily` | 港股日线行情 |

#### 🇺🇸 美股 (`models::us`) — 2 个模型

| 模型 | API | 说明 |
|------|-----|------|
| `UsBasicModel` | `us_basic` | 美股基本信息 |
| `UsDailyModel` | `us_daily` | 美股日线行情 |

#### 📋 期权 (`models::options`) — 2 个模型

| 模型 | API | 说明 |
|------|-----|------|
| `OptBasicModel` | `opt_basic` | 期权合约信息 |
| `OptDailyModel` | `opt_daily` | 期权日行情 |

#### 🏛️ 宏观经济 (`models::macro_data`) — 7 个模型

| 模型 | API | 说明 |
|------|-----|------|
| `ShiborModel` | `shibor` | Shibor利率 |
| `LprModel` | `cn_lpr` | LPR利率 |
| `GdpModel` | `cn_gdp` | GDP数据 |
| `CpiModel` | `cn_cpi` | CPI消费指数 |
| `PpiModel` | `cn_ppi` | PPI生产指数 |
| `M2Model` | `cn_m` | M2货币供应量 |
| `PmiModel` | `cn_pmi` | PMI指数 |

#### 🎬 行业 (`models::industry`) — 2 个模型

| 模型 | API | 说明 |
|------|-----|------|
| `MovieDailyModel` | `bo_daily` | 每日电影票房 |
| `MovieWeeklyModel` | `bo_weekly` | 每周电影票房 |

#### 💱 外汇 (`models::forex`) — 2 个模型

| 模型 | API | 说明 |
|------|-----|------|
| `FxBasicModel` | `fx_obasic` | 外汇基本信息 |
| `FxDailyModel` | `fx_daily` | 外汇日线行情 |

### 自定义模型

预定义模型不满足需求时，仍可使用派生宏自定义：

```rust
use tushare_api::DeriveFromTushareData;

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct MyCustomStock {
    pub ts_code: String,
    pub name: String,
    #[tushare(field = "list_date")]   // 字段名映射
    pub listing_date: Option<String>,
    #[tushare(skip)]                   // 跳过（使用 Default）
    pub calculated: f64,
}
```

## 🧪 运行示例

项目内置 15 个完整示例，涵盖从基础用法到高级特性。

```bash
# 设置 Token
export TUSHARE_TOKEN="your_token_here"
```

### 入门示例

| 示例 | 命令 | 说明 |
|------|------|------|
| 基础用法 | `cargo run --example basic_usage` | 最简单的 API 调用 |
| 环境变量 | `cargo run --example env_usage` | 从环境变量读取 Token |
| 预定义模型 ✨ | `cargo run --example models_usage` | 使用 77 个内置模型 |

### 数据转换

| 示例 | 命令 | 说明 |
|------|------|------|
| 派生宏转换 | `cargo run --example macro_conversion_example` | `#[derive(DeriveFromTushareData)]` 用法 |
| 股票转换 | `cargo run --example stock_conversion_example` | 股票数据 struct 映射 |
| 简单转换 | `cargo run --example simple_stock_conversion` | 最简转换示例 |
| 自定义类型 | `cargo run --example custom_type_example` | 字段映射、skip、Option 等 |
| 调试转换 | `cargo run --example debug_custom_type` | 调试 derive 宏生成的代码 |

### 高级特性

| 示例 | 命令 | 说明 |
|------|------|------|
| 泛型 API | `cargo run --example generic_api_usage` | 泛型请求封装 |
| 分页 | `cargo run --example generic_pagination_example` | 分页查询模式 |
| 日期格式 | `cargo run --example date_format_attribute_example` | `#[tushare(date_format)]` 用法 |
| 自定义日期 | `cargo run --example custom_date_format_example` | chrono 日期类型支持 |
| 第三方类型 | `cargo run --example third_party_types_example` | rust_decimal / chrono / uuid 等 |

### 日志与追踪

| 示例 | 命令 | 说明 |
|------|------|------|
| 日志 | `cargo run --example logging_example` | env_logger 配置 |
| Tracing | `cargo run --example tracing_example --features tracing` | tracing 生态集成 |

## 🙏 致谢

本项目基于以下优秀的开源项目构建：

- **[rock117/tushare-api](https://github.com/rock117/tushare-api)** — 核心 SDK 架构、derive 宏系统、HTTP 客户端、重试机制等均源自此项目。感谢 [@rock117](https://github.com/rock117) 设计了优雅的 `DeriveFromTushareData` 宏和完善的 API 抽象层。
- **[kehanzzz/tushare-sdk](https://github.com/kehanzzz/tushare-sdk)** — 77 个预定义数据模型的字段定义参考了此项目。感谢 [@kehanzzz](https://github.com/kehanzzz) 整理了覆盖 12 个领域的完整数据结构。
- **[Tushare Pro](https://tushare.pro)** — 提供丰富的中国金融数据 API 服务。所有模型字段已对照官方文档逐一验证。

## 📄 许可证

本项目采用 MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件。

## 📞 支持

- 🐛 [Issues](https://github.com/azmove/tushare-rs-pro/issues)
- 💬 [Discussions](https://github.com/azmove/tushare-rs-pro/discussions)
- 📖 [Tushare Pro 官方文档](https://tushare.pro/document/2)

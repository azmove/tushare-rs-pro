//! Industry data models.

use crate::DeriveFromTushareData;

/// 每日电影票房 (bo_daily)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct MovieDailyModel {
    pub date: String,
    pub name: Option<String>,
    pub box_office: Option<f64>,
    pub sum_box_office: Option<f64>,
}

/// 每周电影票房 (bo_weekly)
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct MovieWeeklyModel {
    pub date: String,
    pub list: Option<String>,
    pub movie_name: Option<String>,
    pub box_office: Option<f64>,
    pub box_office_yoy: Option<f64>,
}

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, ToSchema, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum ColorSchema {
    Red,
    Green,
    Blue,
    Yellow,
    Pink,
    Purple,
    Orange,
    Brown,
    Gray,
}

#[derive(Debug, Clone, Copy, ToSchema, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum StatusCategorySchema {
    TaskDifinition,
    WorkWaiting,
    WorkInProgress,
    Blocked,
    TestWaiting,
    TestInProgress,
    Done,
    Canceled,
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "status_category")]
#[sqlx(rename_all = "snake_case")]
pub enum DBStatusCategory {
    TaskDifinition,
    WorkWaiting,
    WorkInProgress,
    Blocked,
    TestWaiting,
    TestInProgress,
    Done,
    Canceled,
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "color")]
#[sqlx(rename_all = "snake_case")]
pub enum DBColor {
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

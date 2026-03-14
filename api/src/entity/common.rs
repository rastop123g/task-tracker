use crate::{
    db::common::{DBColor, DBStatusCategory},
    protocol::common::{ColorSchema, StatusCategorySchema},
};

#[derive(Debug, Clone, Copy)]
pub enum StatusCategoryEntity {
    TaskDifinition,
    WorkWaiting,
    WorkInProgress,
    Blocked,
    TestWaiting,
    TestInProgress,
    Done,
    Canceled,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorEntity {
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

impl From<StatusCategorySchema> for StatusCategoryEntity {
    fn from(category: StatusCategorySchema) -> Self {
        match category {
            StatusCategorySchema::TaskDifinition => StatusCategoryEntity::TaskDifinition,
            StatusCategorySchema::WorkWaiting => StatusCategoryEntity::WorkWaiting,
            StatusCategorySchema::WorkInProgress => StatusCategoryEntity::WorkInProgress,
            StatusCategorySchema::Blocked => StatusCategoryEntity::Blocked,
            StatusCategorySchema::TestWaiting => StatusCategoryEntity::TestWaiting,
            StatusCategorySchema::TestInProgress => StatusCategoryEntity::TestInProgress,
            StatusCategorySchema::Done => StatusCategoryEntity::Done,
            StatusCategorySchema::Canceled => StatusCategoryEntity::Canceled,
        }
    }
}

impl From<StatusCategoryEntity> for StatusCategorySchema {
    fn from(category: StatusCategoryEntity) -> Self {
        match category {
            StatusCategoryEntity::TaskDifinition => StatusCategorySchema::TaskDifinition,
            StatusCategoryEntity::WorkWaiting => StatusCategorySchema::WorkWaiting,
            StatusCategoryEntity::WorkInProgress => StatusCategorySchema::WorkInProgress,
            StatusCategoryEntity::Blocked => StatusCategorySchema::Blocked,
            StatusCategoryEntity::TestWaiting => StatusCategorySchema::TestWaiting,
            StatusCategoryEntity::TestInProgress => StatusCategorySchema::TestInProgress,
            StatusCategoryEntity::Done => StatusCategorySchema::Done,
            StatusCategoryEntity::Canceled => StatusCategorySchema::Canceled,
        }
    }
}

impl From<ColorSchema> for ColorEntity {
    fn from(color: ColorSchema) -> Self {
        match color {
            ColorSchema::Red => ColorEntity::Red,
            ColorSchema::Green => ColorEntity::Green,
            ColorSchema::Blue => ColorEntity::Blue,
            ColorSchema::Yellow => ColorEntity::Yellow,
            ColorSchema::Pink => ColorEntity::Pink,
            ColorSchema::Purple => ColorEntity::Purple,
            ColorSchema::Orange => ColorEntity::Orange,
            ColorSchema::Brown => ColorEntity::Brown,
            ColorSchema::Gray => ColorEntity::Gray,
        }
    }
}

impl From<ColorEntity> for ColorSchema {
    fn from(color: ColorEntity) -> Self {
        match color {
            ColorEntity::Red => ColorSchema::Red,
            ColorEntity::Green => ColorSchema::Green,
            ColorEntity::Blue => ColorSchema::Blue,
            ColorEntity::Yellow => ColorSchema::Yellow,
            ColorEntity::Pink => ColorSchema::Pink,
            ColorEntity::Purple => ColorSchema::Purple,
            ColorEntity::Orange => ColorSchema::Orange,
            ColorEntity::Brown => ColorSchema::Brown,
            ColorEntity::Gray => ColorSchema::Gray,
        }
    }
}

impl From<ColorEntity> for DBColor {
    fn from(color: ColorEntity) -> Self {
        match color {
            ColorEntity::Red => DBColor::Red,
            ColorEntity::Green => DBColor::Green,
            ColorEntity::Blue => DBColor::Blue,
            ColorEntity::Yellow => DBColor::Yellow,
            ColorEntity::Pink => DBColor::Pink,
            ColorEntity::Purple => DBColor::Purple,
            ColorEntity::Orange => DBColor::Orange,
            ColorEntity::Brown => DBColor::Brown,
            ColorEntity::Gray => DBColor::Gray,
        }
    }
}

impl From<DBColor> for ColorEntity {
    fn from(color: DBColor) -> Self {
        match color {
            DBColor::Red => ColorEntity::Red,
            DBColor::Green => ColorEntity::Green,
            DBColor::Blue => ColorEntity::Blue,
            DBColor::Yellow => ColorEntity::Yellow,
            DBColor::Pink => ColorEntity::Pink,
            DBColor::Purple => ColorEntity::Purple,
            DBColor::Orange => ColorEntity::Orange,
            DBColor::Brown => ColorEntity::Brown,
            DBColor::Gray => ColorEntity::Gray,
        }
    }
}

impl From<StatusCategoryEntity> for DBStatusCategory {
    fn from(category: StatusCategoryEntity) -> Self {
        match category {
            StatusCategoryEntity::TaskDifinition => DBStatusCategory::TaskDifinition,
            StatusCategoryEntity::WorkWaiting => DBStatusCategory::WorkWaiting,
            StatusCategoryEntity::WorkInProgress => DBStatusCategory::WorkInProgress,
            StatusCategoryEntity::Blocked => DBStatusCategory::Blocked,
            StatusCategoryEntity::TestWaiting => DBStatusCategory::TestWaiting,
            StatusCategoryEntity::TestInProgress => DBStatusCategory::TestInProgress,
            StatusCategoryEntity::Done => DBStatusCategory::Done,
            StatusCategoryEntity::Canceled => DBStatusCategory::Canceled,
        }
    }
}

impl From<DBStatusCategory> for StatusCategoryEntity {
    fn from(category: DBStatusCategory) -> Self {
        match category {
            DBStatusCategory::TaskDifinition => StatusCategoryEntity::TaskDifinition,
            DBStatusCategory::WorkWaiting => StatusCategoryEntity::WorkWaiting,
            DBStatusCategory::WorkInProgress => StatusCategoryEntity::WorkInProgress,
            DBStatusCategory::Blocked => StatusCategoryEntity::Blocked,
            DBStatusCategory::TestWaiting => StatusCategoryEntity::TestWaiting,
            DBStatusCategory::TestInProgress => StatusCategoryEntity::TestInProgress,
            DBStatusCategory::Done => StatusCategoryEntity::Done,
            DBStatusCategory::Canceled => StatusCategoryEntity::Canceled,
        }
    }
}

use crate::error::ApiResult;

pub trait AppTrim {
    fn app_trim(&mut self);
}

impl AppTrim for String {
    fn app_trim(&mut self) {
        *self = self.trim().to_owned();
    }
}

impl<T: AppTrim> AppTrim for Option<T> {
    fn app_trim(&mut self) {
        if let Some(v) = self {
            v.app_trim();
        }
    }
}

impl<T: AppTrim> AppTrim for Vec<T> {
    fn app_trim(&mut self) {
        for v in self {
            v.app_trim();
        }
    }
}

pub trait FieldValidate {
    fn field_validate(&self) -> ApiResult<()>;
}

impl<T: FieldValidate> FieldValidate for Option<T> {
    fn field_validate(&self) -> ApiResult<()> {
        if let Some(v) = self {
            v.field_validate()?;
        }
        Ok(())
    }
}

impl<T: FieldValidate> FieldValidate for Vec<T> {
    fn field_validate(&self) -> ApiResult<()> {
        for v in self {
            v.field_validate()?;
        }
        Ok(())
    }
}

// Трейт для упрощенного мапинга try_into в Vec
pub trait TryIntoVec<T>: IntoIterator {
    fn try_into_vec(self) -> Result<Vec<T>, <T as TryFrom<Self::Item>>::Error>
    where
        T: TryFrom<Self::Item>;
}

impl<I, T> TryIntoVec<T> for I
where
    I: IntoIterator,
    T: TryFrom<I::Item>,
{
    fn try_into_vec(self) -> Result<Vec<T>, T::Error> {
        self.into_iter().map(T::try_from).collect()
    }
}

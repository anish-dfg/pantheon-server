use anyhow::Error;

pub struct DbError(Error);

impl<E> From<E> for DbError
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

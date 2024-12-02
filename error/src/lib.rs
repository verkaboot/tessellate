use bevy::ecs::query::{QueryEntityError, QuerySingleError};
use derive_more::derive::From;

pub type Result<'w, T> = core::result::Result<T, Error<'w>>;

#[allow(dead_code)]
#[derive(Debug, From)]
pub enum Error<'w> {
    #[from]
    Custom(String),

    #[from]
    QuerySingleError(QuerySingleError),

    #[from]
    QueryEntityError(QueryEntityError<'w>),
}

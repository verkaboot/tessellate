use bevy::{
    ecs::{
        query::{QueryEntityError, QuerySingleError},
        system::RunSystemError,
    },
    render::camera::ViewportConversionError,
};
use derive_more::derive::From;

pub type Result<T> = core::result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    #[from]
    QuerySingleError(QuerySingleError),

    #[from(QueryEntityError<'_>)]
    QueryEntityError,

    #[from]
    ViewConversionError(ViewportConversionError),

    #[from]
    RunSystemError(RunSystemError),
}

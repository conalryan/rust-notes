use crate::errors::AppError;
use actix_web::HttpResponse;

// Declare users submodule
// located in ./routes/users.rs
//
// If you want to restrict the visibility of an item to a specific scope then
// you can use one of pub(in path), pub(crate), pub(super), or pub(self), where path is a given module path.
//
// pub(self) is equivalent to nothing at all, i.e. pub(self) mod foo is the same as mod foo which is actually private.
// Why? The answer lies in macros that can generate code with visibility specifiers. If a macro outputs code like pub($arg) where $arg
// is an input argument, you might want to specify that the item should be private, so passing self as the argument achieves that goal.

pub(super) mod users;
pub(super) mod posts;
pub(super) mod comments;

fn convert<T, E>(res: Result<T, E>) -> Result<HttpResponse, AppError>
// We put trait bounds on the generic parameters to specify that we can only accept input arguments
// if the success variant is a type that can be serialized to JSON, i.e. T: serde::Serialize,
// and we can get an AppError from the error variant, i.e. AppError: From<E>.
where
    T: serde::Serialize,
    AppError: From<E>,
{
    res.map(|d| HttpResponse::Ok().json(d))
        .map_err(Into::into)
}

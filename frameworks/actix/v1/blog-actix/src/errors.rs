use actix_web::error::BlockingError;
use actix_web::web::HttpResponse;
use diesel::result::DatabaseErrorKind::UniqueViolation; 
use diesel::result::Error::{DatabaseError,NotFound}; 
use std::fmt;

// We automatically implemented the Debug trait with the derive attribute on our struct 
// which allows us to format instances of our type with the debug string formatter: {:?}.
#[derive(Debug)]
pub enum AppError {
    RecordAlreadyExists,
    RecordNotFound,
    DatabaseError(diesel::result::Error),
    // OperationCanceled is related to a actix_web error having to do with an async operation which we will explain later.
    OperationCanceled,
}

// Display Trait
// Implementing Display let’s us print our type with {}. 
// We must implement this trait because a different trait we want to implement 
// requires Debug and Display to be implemented.
//
// The implementation is pretty straightforward and most implementations of Display look like this. 
// The macro write! is like println! except the first argument is a “Writer” 
// and it returns a Result in the case of an error. 
// The println! macro can panic in certain error scenarios rather than returning a Result.
impl fmt::Display for AppError {
    // The &mut fmt::Formatter argument implements a trait that makes it a “Writer” 
    // so typically you just use write!(f, ...) and fill in the ... with whatever 
    // you want to represent your type when it is formatted using {}.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::RecordAlreadyExists => write!(f, "This record violates a unique constraint"),
            AppError::RecordNotFound => write!(f, "This record does not exist"),
            AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AppError::OperationCanceled => write!(f, "The running operation was canceled"),
        }
    }
}

// From Into Traits
// Rust usually does not implicit convert one type to another.
// You must explicitly implement one of these traits to be able to take advantage of some automatic type conversions.
// In the standard library and most code you will encounter only From is implemented 
// and then Into is automatically satisfied for free.

// From<diesel::result::Error> for AppError means that you will be given an instance 
// of diesel::result::Error and are expected to return an instance of AppError. 
//
// Encapsulation here means that we do not have to have any code destructing Diesel errors anywhere else in our code. 
// We can just convert any Diesel error into our AppError and then only deal with that type in our code. 
// This layer of abstraction allows us to reduce the surface area of necessary changes 
// if we decide to change how we handle database errors across the entire application.
impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        // We match on the Diesel error to handle the two specific cases we care about.
        match e {
            DatabaseError(UniqueViolation, _) => AppError::RecordAlreadyExists,
            NotFound => AppError::RecordNotFound,
            // _ represents more data that we don’t care about. 
            // We just care that whatever query we executed resulted in this specific type of error.
            _ => AppError::DatabaseError(e),
        }
    }
}

// BlockingError<T> is an actix web specific error that we will encounter when we get to the implementation of our handlers. 
// Our handlers will return futures but we must use blocking code to interact with the database. 
// Therefore our handlers will run blocking code which can either succeed 
// or can fail because the future was canceled or the underlying blocking code returned an error.
impl From<BlockingError<AppError>> for AppError {
    fn from(e: BlockingError<AppError>) -> Self {
        match e {
            BlockingError::Error(inner) => inner,
            BlockingError::Canceled => AppError::OperationCanceled,
        }
    }
}

// Errors as responses
// The main advantage of creating our own error type is that we define how to turn an 
// instance of AppError into an HTTP response and therefore automatically get nice 
// error responses by just returning an error from a handler. 
#[derive(Debug, Serialize)]
struct ErrorResponse {
    err: String,
}

// Actix web defines a trait ResponseError which allows you to specify how the type 
// inside a Err variant of a Result gets turned into a response.
//
// This trait is why we implemented Display for our error. 
// First ResponseError has the trait bound Debug + Display which means that 
// in order to implement ResponseError for your type, your type must also implement Debug and Display
impl actix_web::ResponseError for AppError {
    // The trait requires error_response to be implemented which we do by matching on our error 
    // and setting useful response codes to the cases we care about and 500 otherwise, 
    // and then using the Display formatting to create an error message to return as JSON.
    fn error_response(&self) -> HttpResponse {
        let err = format!("{}", self);
        let mut builder = match self {
            AppError::RecordAlreadyExists => HttpResponse::BadRequest(),
            AppError::RecordNotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),
        };
        builder.json(ErrorResponse { err })
    }

    // The trait also has a method render_response which has a default implementation, but the
    // default overrides the content type and data which is not what we want. 
    // So we instead just implement this method to return the same thing as our 
    // error_response method which is what we want. 
    // When we get to our handler implementations we will see where this is called.
    fn render_response(&self) -> HttpResponse {
        self.error_response()
    }
}

use crate::errors::AppError;
use crate::schema::{users};
use diesel::prelude::*;

// Models
// The next module we are going to implement will be our layer that contains the interactions with the database. 
// We will define the Rust representations of our data model as well as functions for 
// how to get those objects from the database


// In order to make our lives easier we are going to define our own Result type 
// which will be an alias for Result in the standard library with the error type fixed as our AppError type:
//
// This way we can just return Result<T> and not have to writen AppError everywhere 
// because throughout this module all errors will be of the AppError type so it is 
// just noisy to have to write it everywhere.
type Result<T> = std::result::Result<T, AppError>;

// User struct
// We need a Rust struct to represent a user in the database.
//
// Queryable is a trait that indicates this struct can be constructed from a database query. From the Diesel docs:
// Diesel represents the return type of a query as a tuple. 
// The purpose of this trait is to convert from a tuple of Rust values that have been deserialized into your struct
// So basically by deriving this trait we can automatically get a User struct from queries of the users table.
//
// Identifiable is a trait that indicates that this struct represents a single row in a database table. 
// It assumes a primary key named id but you can configure the derive attribute 
// if you want to change the name of the primary key. It is required for associations which we will use later.
#[derive(Queryable, Identifiable, Serialize, Debug, PartialEq)]
pub struct User {
    // i32 because that maps to the database integer type.
    pub id: i32,
    // String because the database column is a VARCHAR.
    pub username: String,
}

// This code is slightly more complex because we are using Sqlite instead of a backend that supports a RETURNING clause. 
// Sqlite does not support getting the id of a just inserted row as part of the insert statement.
// Instead we have to do another query to actually get the data back out to build a User struct. 
// Because of this we run both queries inside a transaction to ensure that the logic of fetching the most recently inserted user actually returns the user that we just inserted.
pub fn create_user(conn: &SqliteConnection, username: &str) -> Result<User> {
    conn.transaction(|| {
        diesel::insert_into(users::table)
            .values((users::username.eq(username),))
            .execute(conn)?;

        users::table
            .order(users::id.desc())
            .select((users::id, users::username))
            .first(conn)
            .map_err(Into::into)
    })
}

// Fetching User
// Two ways to find a user:
// - by id
// - by username
// Rather than write two different functions for these use cases we are going to write 
// one function which takes an enum that encapsulates which key to use for looking up the user.
//
// We have seen the 'static lifetime before but this is our first instance of a 
// generic lifetime for a type we are creating.
//
// Lifetimes
// lifetimes of references in Rust are checked by the compiler to ensure that 
// the data referenced outlives the reference to it. In other words, 
// the concept of lifetimes guarantees that we will not try to access memory 
// after it has been deallocated while still being able to have access to data that we do not own.
//
// Lifetimes live in the same space as generic types and can be thought of as a kind of type.
//
// Here our type UserKey<'a> specifies that it has one generic lifetime parameter named 'a. 
// We need to specify this generic parameter so that we can give a definite lifetime to the 
// string reference inside our Username variant.
// It is possible to use the special lifetime 'static and not make our enum generic 
// but that would force us to only be able to use static strings.
pub enum UserKey<'a> {
    Username(&'a str),
    ID(i32),
}

pub fn find_user<'a>(conn: &SqliteConnection, key: UserKey<'a>) -> Result<User> {
    match key {
        UserKey::Username(name) => users::table
            .filter(users::username.eq(name))
            .select((users::id, users::username))
            .first::<User>(conn)
            .map_err(AppError::from),
        UserKey::ID(id) => users::table
            .find(id)
            .select((users::id, users::username))
            .first::<User>(conn)
            .map_err(Into::into),
    }
}

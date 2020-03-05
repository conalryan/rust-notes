use crate::errors::AppError;
use crate::schema::comments;
use crate::schema::posts;
use crate::schema::users;
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

// The concept of an association in Diesel is always from child to parent, i.e. there is no “has many” like in other ORMs. 
// Declaring the association between two records requires the belongs_to attribute on the child and specifies the name of the struct that represents the parent.
//
// struct also needs to derive Associations. Deriving this trait uses the information in belongs_to to generate the relevant code to make joins possible.
#[derive(Queryable, Associations, Identifiable, Serialize, Debug)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Debug)]
#[belongs_to(User)]
#[belongs_to(Post)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub body: String,
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

pub fn create_post(conn: &SqliteConnection, user: &User, title: &str, body: &str) -> Result<Post> {
    conn.transaction(|| {
        diesel::insert_into(posts::table)
            .values((
                posts::user_id.eq(user.id),
                posts::title.eq(title),
                posts::body.eq(body),
            ))
            .execute(conn)?;

        posts::table
            .order(posts::id.desc())
            // select(posts::all_columns) which is a shorthand that Diesel provides so that we do not have to write out a tuple with each column explicitly listed.
            .select(posts::all_columns)
            .first(conn)
            .map_err(Into::into)
    })
}

pub fn publish_post(conn: &SqliteConnection, post_id: i32) -> Result<Post> {
    conn.transaction(|| {
        // Issuing an update to the database uses the aptly named update function from Diesel.
        // The argument to update can be:
        // - a table: If you pass just a table then the update applies to all rows of that table which is typically not what you want.
        // - a filtered table: which is what we use here
        // - a reference to a struct that implements the Identifiable trait
        // Diesel also has a trait called AsChangeset which you can derive which allows you to take a value like post 
        // and call diesel::update(...).set(&post) to set all of the fields (except the primary key) on the struct 
        // based on the current state of that struct.
        diesel::update(posts::table.filter(posts::id.eq(post_id)))
            .set(posts::published.eq(true))
            .execute(conn)?;

        posts::table
            .find(post_id)
            .select(posts::all_columns)
            .first(conn)
            .map_err(Into::into)
    })
}

pub fn create_comment(
    conn: &SqliteConnection,
    user_id: i32,
    post_id: i32,
    body: &str,
) -> Result<Comment> {
    conn.transaction(|| {
        diesel::insert_into(comments::table)
            .values((
                comments::user_id.eq(user_id),
                comments::post_id.eq(post_id),
                comments::body.eq(body),
            ))
            .execute(conn)?;

        comments::table
            .order(comments::id.desc())
            .select(comments::all_columns)
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

// The return type of this function is a list of tuples where the first element is a post and the
// second element is the author. 
//
// Diesel is built around queries that have this flat result structure. 
//
// You might be used to other ORMs where a post object would have an author field which contains 
// an embedded user object. 
//
// In most uses of Diesel you will find tuples being used to represent related models rather 
// than hierarchical structs.
pub fn all_posts(conn: &SqliteConnection) -> Result<Vec<((Post, User), Vec<(Comment, User)>)>> {
    let query = posts::table
        // order newest first based on id, normally you'd base on timestamp
        .order(posts::id.desc())
        .filter(posts::published.eq(true))
        .inner_join(users::table)
        .select((posts::all_columns, (users::id, users::username)));
    let posts_with_user = query.load::<(Post, User)>(conn)?;
    // We then use the unzip method on std::iter::Iterator which turns an iterator of pairs into a pair of iterators
    // In this case we turn Vec<(Post, User)> into (Vec<Post>, Vec<User>).
    let (posts, post_users): (Vec<_>, Vec<_>) = posts_with_user.into_iter().unzip();

    // We can then fetch all of the comments that belong to those posts by passing a reference to that 
    // vector to belonging_to which we get from deriving Associations on Comment.
    let comments = Comment::belonging_to(&posts)
        .inner_join(users::table)
        .select((comments::all_columns, (users::id, users::username)))
        .load::<(Comment, User)>(conn)?
        // To associate the comments into chunks indexed by the posts we use the grouped_by method provided by Diesel.
        // Note this does not generate a GROUP BY statement in SQL rather it is just operating on the 
        // data structures in memory of already loaded data. 
        // In the end this transforms a Vec<(Comment, User)> into Vec<Vec<(Comment, User)>>.
        .grouped_by(&posts);

    // Finally, we can use the zip method on iterator to take all of these vectors 
    // and combine them into the output format we were looking for. 
    // posts.into_iter().zip(post_- users) just turns (Vec<Post>, Vec<User>) back into Vec<(Post, User)>.
    // zip(comments) takes Vec<(Post, User)> and Vec<Vec<(Comment, User)>> and puts them together into a single vector of our desired return type.
    Ok(posts.into_iter().zip(post_users).zip(comments).collect())
}

// As the author is the same for all of these posts we only return a vector of posts rather 
// than the tuple of our previous function.
pub fn user_posts(
    conn: &SqliteConnection,
    user_id: i32,
) -> Result<Vec<(Post, Vec<(Comment, User)>)>> {
    let posts = posts::table
        .filter(posts::user_id.eq(user_id))
        .order(posts::id.desc())
        .select(posts::all_columns)
        .load::<Post>(conn)?;

    let comments = Comment::belonging_to(&posts)
        .inner_join(users::table)
        .select((comments::all_columns, (users::id, users::username)))
        .load::<(Comment, User)>(conn)?
        .grouped_by(&posts);

    Ok(posts.into_iter().zip(comments).collect())
}

pub fn post_comments(conn: &SqliteConnection, post_id: i32) -> Result<Vec<(Comment,User)>> {
    comments::table
        .filter(comments::post_id.eq(post_id))
        .inner_join(users::table)
        .select((comments::all_columns, (users::id, users::username)))
        .load::<(Comment, User)>(conn)
        .map_err(Into::into)
}

// We are going to fetch all comments made by a particular user, but just fetching the comments alone 
// would be lacking some important information, notably information about the post the comment is on. 
// So we want to fetch the post for each comment, but we don’t want to fetch all of the post data because 
// that would be too much. Instead we are going to make a new struct to represent a subset of the post data 
// that we want to fetch alongside each comment.
#[derive(Queryable, Serialize, Debug)]
pub struct PostWithComment {
    pub id: i32,
    pub title: String,
    pub published: bool,
}

pub fn user_comments(
    conn: &SqliteConnection,
    user_id: i32,
) -> Result<Vec<(Comment, PostWithComment)>> {
    comments::table
        .filter(comments::user_id.eq(user_id))
        .inner_join(posts::table)
        .select((
            comments::all_columns,
            (posts::id, posts::title, posts::published),
        ))
        .load::<(Comment, PostWithComment)>(conn)
        .map_err(Into::into)
}

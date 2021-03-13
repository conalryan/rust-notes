use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use futures::Future;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users/{id}/posts")
            .route(web::post().to_async(add_post))
            .route(web::get().to_async(user_posts)),
    )
    .service(web::resource("/posts").route(web::get().to_async(all_posts)))
    .service(web::resource("/posts/{id}/publish").route(web::post().to_async(publish_post)));
}

#[derive(Debug, Serialize, Deserialize)]
struct PostInput {
    title: String,
    body: String,
}

// We take that path as input as well as the post as JSON and the database pool.
//
// We wrote our create_post function to take a user struct as input rather than just a plain id, 
// therefore we need to convert the id we take as input into a User before we can use it. 
// We do that so the error that results from a missing user happens first before we even try to create a post.
fn add_post(
    user_id: web::Path<i32>,
    post: web::Json<PostInput>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        let key = models::UserKey::ID(user_id.into_inner());
        // We accept an user_id but our models create_post will need an user struct.
        // We use the and_then method on Result to continue on to creating a post only in the case where we actually found a user.
        models::find_user(conn, key).and_then(|user| {
            let post = post.into_inner();
            let title = post.title;
            let body = post.body;
            // create posts requires a user.
            models::create_post(conn, &user, title.as_str(), body.as_str())
        })
    })
    // convert function to map the result into our expected form.
    .then(convert)
}

fn publish_post(
    post_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        models::publish_post(conn, post_id.into_inner())
    })
    .then(convert)
}

fn user_posts(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        models::user_posts(conn, user_id.into_inner())
    })
    .then(convert)
}

fn all_posts(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        models::all_posts(conn)
    })
    .then(convert)
}

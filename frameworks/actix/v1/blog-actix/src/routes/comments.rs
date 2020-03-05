use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use futures::Future;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users/{id}/comments").route(web::get().to_async(user_comments)))
        .service(
            web::resource("/posts/{id}/comments")
                .route(web::post().to_async(add_comment))
                .route(web::get().to_async(post_comments)),
        );
}

#[derive(Debug, Serialize, Deserialize)]
struct CommentInput {
    user_id: i32,
    body: String,
}

fn add_comment(
    post_id: web::Path<i32>,
    comment: web::Json<CommentInput>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse,Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        let data = comment.into_inner();
        // Here we assume the user id correct, without checking first.
        //  If the database has foreign key constraints then passing a bad post id will result in an error at the database level. 
        //  If the database does not support those constraints or you do not specify them then this would be a source of bugs 
        //  if you did not otherwise validate the input. The design is up to you.
        let user_id = data.user_id;
        let body = data.body;
        models::create_comment(conn, user_id, post_id.into_inner(), body.as_str())
    })
    .then(convert)
}

fn post_comments(
    post_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        models::post_comments(conn, post_id.into_inner())
    })
    .then(convert)
}

fn user_comments(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        models::user_comments(conn, user_id.into_inner())
    })
    .then(convert)
} 

use crate::errors::AppError;
// Note that our convert function in the routes module (routes.rs) was not public but we are using it here. 
// Private items are visible to the module they are defined in as well as all descendants.
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse};
use futures::Future;

// The signature of this function is specified by Actix web. 
// The only parameter is a mutable reference to a service configuration object. 
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::post().to_async(create_user)))
        .service(web::resource("/users/find/{name}").route(web::get().to_async(find_user)))
        .service(web::resource("/users/{id}").route(web::get().to_async(get_user)));
}

#[derive(Debug, Serialize, Deserialize)]
struct UserInput {
    username: String,
}

// Future<Item, Error> is a future in the traditional sense, an object that represents a computation which can be queried for a result or an error.
// Actix web is designed to work with both synchronous and asynchronous handlers, but so far we have only used the synchronous ones.
// The syntax impl Future means that we are going to return some type that implements the Future trait, 
// but we are not telling you exactly what that type is. This gives us some flexibilty and is necessary for some types which are hard (or impossible) to write.
fn create_user(
    item: web::Json<UserInput>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    // Diesel is synchronous, it does not directly support futures for interacting with the database. 
    // Therefore we use web::block which executes a blocking function on a thread pool 
    // and returns a future that resolves to the result of the function execution.
    web::block(move || {
        let conn = &pool.get().unwrap();
        let username = item.into_inner().username;
        models::create_user(conn, username.as_str())
    })
    // Finally, we can use our convert function to turn the result of the call to  models::create_user into the response we desire. 
    // Note that here we see why we implemented From<BlockingError<AppError>> for our AppError type. 
    // The map_err function inside convert relies on that From implementation.
    .then(convert)
}

fn find_user(
    name: web::Path<String>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let name = name.into_inner();
        let key = models::UserKey::Username(name.as_str());
        models::find_user(conn, key)
    })
    .then(convert)
}

fn get_user(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let id = user_id.into_inner();
        let key = models::UserKey::ID(id);
        models::find_user(conn, key)
    })
    .then(convert)
}

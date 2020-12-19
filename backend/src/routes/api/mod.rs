use warp::Filter;

use crate::endpoints;
use crate::handlers;

pub fn init() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let routes = available().or(status()).or(new_user());

    let api = warp::path("api");
    api.and(routes)
}

fn available() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(endpoints::api::AVAILABLE)
        .and(warp::get())
        .and_then(handlers::available::handler)
}

fn status() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(endpoints::api::STATUS)
        .and(warp::post())
        .and_then(handlers::status::handler)
}

fn new_user() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(endpoints::api::NEWUSER)
        .and(warp::post())
        .and_then(handlers::user::new::handler)
}

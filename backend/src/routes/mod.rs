mod api;

use warp::Filter;

pub fn init() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api = api::init();
    return api;
}

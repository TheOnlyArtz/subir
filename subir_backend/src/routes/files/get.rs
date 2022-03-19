use tide::prelude::*;
use tide::Body;
use tide::Request;
use tide::Response; // Pulls in the json! macro.

use crate::{app::AppState, redis_service::Queries};

pub async fn get_all_files(req: Request<AppState>) -> tide::Result {
    let files = req.state().redis.lock().await.get_files(0, -1)?;

    Ok(Response::from(Body::from_json(&files).unwrap()))
}

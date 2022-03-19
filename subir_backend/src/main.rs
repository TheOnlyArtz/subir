use std::{io::Result, sync::Arc};

use app::{AppState, FileChannelMessage};
use cloud_storage::Client;
use flume::Receiver;
use routes::files::put::upload_file_route;
use tide::{
    http::headers::HeaderValue,
    security::{CorsMiddleware, Origin},
};
use tokio::{spawn, sync::Mutex};

pub mod app;
pub mod redis_service;
pub mod routes;

use redis_service::{Queries, RedisService};

use crate::{redis_service::RedisSavedFileInfo, routes::files::get::get_all_files};

#[tokio::main]
async fn main() -> tide::Result<()> {
    // Load the .env file into the process
    dotenv::dotenv().ok();

    // Spawn new instance of a redis client
    let redis_client = redis::Client::open(format!(
        "redis://:{}@127.0.0.1/",
        std::env::var("REDIS_PASSWORD").unwrap()
    ))?;

    // Connect to the redis via the redis client and construct a RedisService struct
    let redis_service = redis_service::RedisService::from(redis_client)?;
    let redis_service = Arc::new(Mutex::new(redis_service));

    // Fire up a tide logger to debug requests
    tide::log::start();

    // initial the channel which handles file upload requests
    let (file_tx, file_rx) = flume::unbounded();

    // connect to google storage API
    let gs_client = Client::default();

    // define the state of our tide application
    let state = AppState {
        files_sender: file_tx,
        redis: Arc::clone(&redis_service),
    };

    // spawn a new runtime which will listen to incoming files
    // and upload them
    spawn(async move {
        // discard the error
        let _ = file_handler(file_rx, gs_client, Arc::clone(&redis_service)).await;
    });

    let mut app = tide::with_state::<AppState>(state);

    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, PUT, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    app.with(cors);

    app.at("/").put(upload_file_route).get(get_all_files);

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}

async fn file_handler(
    rx: Receiver<FileChannelMessage>,
    gs_client: Client,
    redis_service: Arc<Mutex<RedisService>>,
) -> Result<()> {
    loop {
        let message = rx.recv_async().await;

        if let Ok(message) = message {
            let object = gs_client
                .object()
                .create(
                    "tothanim-crimes",
                    message.bytes,
                    &message.name,
                    &message.r#type,
                )
                .await;

            if let Ok(object) = object {
                let redis_save_info = RedisSavedFileInfo {
                    media_link: object.media_link,
                    name: message.name,
                    timestamp: object.time_created.timestamp_millis(),
                };

                let _ = redis_service.lock().await.push_file_info(redis_save_info);
            }
        }
    }
}

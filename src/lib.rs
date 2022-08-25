use std::convert::Infallible;


use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};
use warp::Filter;

use crate::{settings::Settings, app::{YariCommand}, db::YariHashMapKeyspace};

mod db;
mod settings;
mod app;

pub async fn entry() {
    let settings: Settings = Settings::new().expect("Failed to read config file");
    let app_settings = settings.clone();
    println!("{:?}", &settings);
    
    let (tx, rx) = mpsc::channel(32);
    let tx_internal_sender = tx.clone();

    // let mut app = YariApp::new(&settings, YariHashMapKeyspace::default());
    
    tokio::spawn(async move {
        start_webserver(tx).await;
    });
    crate::app::run_app(YariHashMapKeyspace::new(10, 0.25, 20), &app_settings, rx, tx_internal_sender).await;
}

async fn start_webserver(tx: mpsc::Sender<YariCommand>) {
    let tx_get = tx.clone();
    let tx_set = tx.clone();

    let get = warp::path!("get" / String)
        .and_then(move |key: String| {
            let tx_get = tx_get.clone();
            async move {
                let (resp_tx, resp_rx) = oneshot::channel();
                let key2 = key.clone();
                let value = YariCommand::Get { key: key2, resp: resp_tx };
                tx_get.send(value).await.unwrap();
                let res = resp_rx.await.unwrap();

                Ok::<String, Infallible>(format!("GET: key: {}, response: {:?}", key, res.unwrap()))
            }
        });
    let set = warp::path!("set" / String / String)
        .and_then(move |key: String, value: String| {
            let tx_set = tx_set.clone();
            async move {
                let (resp_tx, resp_rx) = oneshot::channel();
                let key2 = key.clone();
                let set_command = YariCommand::Set { key: key2, value: Bytes::from(value), resp: resp_tx };

                tx_set.send(set_command).await.unwrap();
                let res = resp_rx.await.unwrap();

                Ok::<String, Infallible>(format!("SET: key: {}, response: {:?}", key, res.unwrap()))
            }
        });
    warp::serve(get.or(set))
        // .run((settings.server.http.address.parse::<IpAddr>().unwrap(), settings.server.http.port))
        .run(([127, 0, 0, 1], 3030)).await;
}




#[derive(Debug)]
pub enum YariError {
    UndefinedError,
    NotImplementedError
}

pub(crate) type YariResult<T> = std::result::Result<T, YariError>;

#[cfg(test)]
mod tests {

    
}
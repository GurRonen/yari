use std::{convert::Infallible, time::Duration};

use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};
use warp::Filter;

use crate::app::YariCommand;

pub(crate) async fn start_webserver(tx: mpsc::Sender<YariCommand>) {
    let tx_get = tx.clone();
    let tx_set = tx.clone();
    let tx_expire = tx.clone();

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
                let set_command = YariCommand::Set { key: key.clone(), value: Bytes::from(value), resp: resp_tx };

                tx_set.send(set_command).await.unwrap();
                let res = resp_rx.await.unwrap();

                Ok::<String, Infallible>(format!("SET: key: {}, response: {:?}", key, res.unwrap()))
            }
        });
    let expire = warp::path!("expire" / String / u64)
        .and_then(move |key: String, expiration: u64| {
            let tx_expire = tx_expire.clone();
            async move {
                let (resp_tx, resp_rx) = oneshot::channel();
                let expire_command = YariCommand::Expire { key: key.clone(), duration: Duration::from_millis(expiration), resp: resp_tx };

                tx_expire.send(expire_command).await.unwrap();
                let res = resp_rx.await.unwrap();

                Ok::<String, Infallible>(format!("EXPIRE: key: {}, response: {:?}", key, res.unwrap()))
            }

    });
    warp::serve(get.or(set).or(expire))
        // .run((settings.server.http.address.parse::<IpAddr>().unwrap(), settings.server.http.port))
        .run(([127, 0, 0, 1], 3030)).await;
}

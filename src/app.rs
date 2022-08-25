use std::time::Duration;

use bytes::Bytes;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    oneshot,
};

use crate::{db::YariKeyspace, settings::Settings};

#[derive(Debug)]
pub(crate) enum YariCommand {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<Option<Bytes>>,
    },
    Expire {
        key: String,
        duration: Duration,
        resp: Responder<bool>,
    },
    InternalActiveExpire {},
}

type Responder<T> = oneshot::Sender<crate::YariResult<T>>;

pub(crate) async fn run_app<T: YariKeyspace + Send + 'static>(
    db: T,
    settings: &Settings,
    rx: Receiver<YariCommand>,
    tx: Sender<YariCommand>,
) {
    let expiration_duration = Duration::from_millis((1000 / settings.expiration.active.ticks_per_second).into());

    let expiration_task = tokio::spawn(async move { start_expiration_loop(expiration_duration, tx).await });
    let db_task = tokio::spawn(async move { start_db_access_loop(rx, db).await; });

    tokio::select! {
            _ = db_task => {} 
            _ = expiration_task => {}
    };
}

async fn start_db_access_loop<T: YariKeyspace + Send + 'static>(
    mut rx: Receiver<YariCommand>,
    mut db: T,
) {
    while let Some(command) = rx.recv().await {
        #[allow(unused_must_use)]
        match command {
            YariCommand::Get { key, resp } => {
                let get_response = db.yari_get(&key);
                resp.send(Ok(get_response.map(|f| f.clone())));
            }
            YariCommand::Set { key, value, resp } => {
                db.yari_set(key, value);
                resp.send(Ok(Some(Bytes::from("OK"))));
            }
            YariCommand::Expire {
                key,
                duration,
                resp,
            } => {
                let expiration_response = db.yari_expire(&key, duration);
                resp.send(Ok(expiration_response));
            }
            YariCommand::InternalActiveExpire {} => {
                db.yari_active_expiration();
            }
        }
    }
}

async fn start_expiration_loop(expiration_duration: Duration, tx: Sender<YariCommand>) {
    let mut interval = tokio::time::interval(expiration_duration);
    loop {
        interval.tick().await;
        tx.send(YariCommand::InternalActiveExpire {}).await.unwrap();
    }
}

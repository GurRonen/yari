
use log::info;
use tokio::sync::{mpsc};


use crate::{settings::Settings, db::YariHashMapKeyspace};

mod db;
mod settings;
mod app;
mod http_server;
mod redis_server;
mod bloom;

pub async fn entry() {
    let settings: Settings = Settings::new().expect("Failed to read config file");
    info!("{:?}", &settings);
    
    let (tx, rx) = mpsc::channel(settings.app.commands_depth);
    let tx_internal_sender = tx.clone();

    // let mut app = YariApp::new(&settings, YariHashMapKeyspace::default());
    
    tokio::spawn(async move {
        http_server::start_webserver(tx).await;
    });
    crate::app::run_app(YariHashMapKeyspace::new(10, 0.25, 20), &settings, rx, tx_internal_sender).await;
}


#[derive(Debug)]
pub enum YariErrorKind {
    RedisTcpError,
    UndefinedError,
    NotImplementedError
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type YariResult<T> = std::result::Result<T, YariErrorKind>;

#[cfg(test)]
mod tests {

    
}
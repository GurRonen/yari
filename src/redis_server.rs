// use std::{time::Duration, sync::Arc};

// use log::{info, error};
// use tokio::{sync::{mpsc, Semaphore, broadcast}, net::{TcpListener, TcpStream}, time};
// use warp::hyper::client::connect::Connection;

// use crate::{app::YariCommand, YariErrorKind};



// #[derive(Debug)]
// struct Listener {
//     db: mpsc::Sender<YariCommand>,
//     listener: TcpListener,
//     limit_connections: Arc<Semaphore>,
//     shutdown_notifier: broadcast::Sender<()>,
//     shutdown_complete_rx: mpsc::Receiver<()>,
//     shutdown_complete_tx: mpsc::Sender<()>,

// }

// #[derive(Debug)]
// struct ConnectionHandle {
//     db: mpsc::Sender<YariCommand>,
//     connetion: Connection
// }

// pub(crate) async fn run(listener: TcpListener, tx: mpsc::Sender<YariCommand>) {
//     let (shutdown_notify, _) = broadcast::channel(1);
//     let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);
    
//     let mut server = Listener {
//         listener,
//         db: tx,
//         limit_connections: 250,
//         shutdown_notifier: shutdown_notify,
//         shutdown_complete_rx,
//         shutdown_complete_tx,
//     };

//     tokio::select! {
//         res = server.run() => {
//             if let Err(err) = res {
//                 error!("failed to accept connections, {}", err);
//             }
//         }
//         _ = shutdown => {
//             info!("shutting down");
//         }
//     }

// }

// impl Listener {
//     async fn run(&mut self) -> crate::YariResult<()> {
//         info!("accepting inbound connections");

//         loop {
//             let permit = self
//             .limit_connections
//             .clone()
//             .acquire_owned()
//             .await
//             .unwrap();

//             let socket = self.accept().await?;

//             // let mut handler = Handler
//         }

//     }

//     async fn accept(&mut self) -> crate::YariResult<TcpStream> {
//         let mut backoff = 1;

//         loop {
//             match self.listener.accept().await {
//                 Ok((socket, _)) => return Ok(socket),
//                 Err(err) => {
//                     if backoff > 64 {
//                         return Err(YariErrorKind::RedisTcpError);
//                     }
//                 }
//             }

//             time::sleep(Duration::from_secs(backoff)).await;
//             backoff *= 2;
//         }
        
//     }
// }

// impl ConnectionHandle {
//     #[instrument(skip(self))]
//     async fn run(&mut self) -> crate::YariResult<()> {
//         while self.shut
//     }
// }
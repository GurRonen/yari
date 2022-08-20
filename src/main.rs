mod settings;

use std::net::IpAddr;

use settings::Settings;
use warp::Filter;

#[tokio::main]
async fn main() {
    let settings: Settings = Settings::new().expect("Failed to read config file");
    println!("{:?}", settings);

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run((
            settings
                .server.http.address.parse::<IpAddr>().expect("Invalid IP Address"),
            settings.server.http.port,
        ))
        .await;
}

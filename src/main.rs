use std::{collections::HashMap, net::SocketAddr};

use env_logger::Env;
use warp::{
    http,
    hyper::{self, header::CONTENT_TYPE},
    Filter,
};

use error::Error;

pub mod error;

async fn read_temperature_sensor() -> Result<String, Error> {
    let resp = reqwest::get("http://192.168.178.200")
        .await?
        .json::<HashMap<String, f64>>()
        .await?;
    println!("{:#?}", resp);

    Ok(format!("temperature {}\n", resp.get("temperature").unwrap()))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = Env::default().filter_or("RUST_LOG", "temperature_exporter=info");
    env_logger::init_from_env(env);

    let bind_address = dotenv::var("BIND_ADDRESS")
        .unwrap_or("0.0.0.0:9184".to_string())
        .parse::<SocketAddr>()
        .expect("parsing BIND_ADDRESS");

    let route = warp::get()
        .and(warp::path("metrics"))
        .and(warp::addr::remote())
        .and_then(move |addr: Option<SocketAddr>| {
            if let Some(addr) = addr {
                log::info!("incoming request from {}", addr);
            }

            async move {
                let metric = read_temperature_sensor().await.unwrap();
                
                let response = http::Response::builder()
                    .status(200)
                    .header(CONTENT_TYPE, "text/html")
                    .body(metric)
                    .map_err(|e| Error::from(e))?;

                Ok(response) as Result<hyper::Response<String>, warp::Rejection>
            }
        });

    log::info!("binding Prometheus exporter on http://{}", &bind_address);

    warp::serve(route).run(bind_address).await;

    Ok(())
}

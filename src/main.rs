use env_logger::Env;
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;

use error::Error;

pub mod error;

async fn read_temperature_sensor() -> Result<String, Error> {
    // let resp = reqwest::get("http://192.168.178.200")
    let resp = reqwest::get("http://home.crsh.cc:4680")
        .await?
        .json::<HashMap<String, f64>>()
        .await?;
    println!("{:#?}", resp);

    Ok(format!("temperature {}\n", resp.get("temperature").unwrap()))
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer);

    let contents = read_temperature_sensor().await.unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes());

}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = Env::default().filter_or("RUST_LOG", "temperature_exporter=info");
    env_logger::init_from_env(env);

    let listener = TcpListener::bind("0.0.0.0:9184").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await?;
        handle_connection(socket).await;
    }

    Ok(())
}

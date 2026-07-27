use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
};

use pit_server::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    let listener = TcpListener::bind(address).await?;

    println!("listening on http://{address}");
    axum::serve(listener, app()).await?;
    Ok(())
}

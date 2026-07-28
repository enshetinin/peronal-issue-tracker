use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
};

use pit_application::TicketRepository;
use pit_domain::{
    ids::{ProjectId, TicketId, UserId},
    ticket::{Priority, Ticket},
};
use pit_infrastructure::InMemoryTicketRepository;
use pit_server::{AppState, app};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut ticket_repository = InMemoryTicketRepository::default();

    let ticket = Ticket::new(
        TicketId::new(100),
        ProjectId::new(10),
        UserId::new(1),
        String::from("Implement auth"),
        String::from("Some descr bla bla"),
        Priority::High,
    );

    ticket_repository
        .save(ticket)
        .expect("Failed to save ticket");

    let state = AppState::new(ticket_repository);

    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    let listener = TcpListener::bind(address).await?;

    println!("listening on http://{address}");
    axum::serve(listener, app(state)).await?;
    Ok(())
}

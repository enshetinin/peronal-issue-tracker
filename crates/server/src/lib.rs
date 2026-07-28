use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use pit_application::TicketRepository;
// use pit_domain::ids::ProjectId;
use pit_domain::{
    ids::TicketId,
    ticket::{Priority, Ticket, TicketStatus},
};
use pit_infrastructure::InMemoryTicketRepository;
use serde::Serialize;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct AppState {
    ticket_repository: Arc<RwLock<InMemoryTicketRepository>>,
}

impl AppState {
    pub fn new(ticket_repository: InMemoryTicketRepository) -> Self {
        Self {
            ticket_repository: Arc::new(RwLock::new(ticket_repository)),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TicketResponse {
    id: u64,
    project_id: u64,
    created_by: u64,
    assigned_to: Option<u64>,
    title: String,
    description: String,
    status: &'static str,
    priority: &'static str,
}

impl From<Ticket> for TicketResponse {
    fn from(ticket: Ticket) -> Self {
        Self {
            id: ticket.id().get(),
            project_id: ticket.project_id().get(),
            created_by: ticket.created_by().get(),
            assigned_to: ticket.assigned_to().map(|user_id| user_id.get()),
            title: ticket.title().to_owned(),
            description: ticket.description().to_owned(),
            status: status_name(ticket.status()),
            priority: priority_name(ticket.priority()),
        }
    }
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/tickets/{ticket_id}", get(get_ticket))
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn get_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<u64>,
) -> Result<Json<TicketResponse>, StatusCode> {
    let repository = state.ticket_repository.read().await;

    let ticket = repository
        .find_by_id(TicketId::new(ticket_id))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(TicketResponse::from(ticket)))
}

fn status_name(status: TicketStatus) -> &'static str {
    match status {
        TicketStatus::Backlog => "backlog",
        TicketStatus::Todo => "todo",
        TicketStatus::InProgress => "in_progress",
        TicketStatus::Done => "done",
    }
}

fn priority_name(priority: Priority) -> &'static str {
    match priority {
        Priority::Low => "low",
        Priority::Medium => "medium",
        Priority::High => "high",
        Priority::Urgent => "urgent",
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode, header},
    };
    use pit_application::TicketRepository;
    use pit_domain::{
        ids::{ProjectId, TicketId, UserId},
        ticket::{Priority, Ticket},
    };
    use pit_infrastructure::InMemoryTicketRepository;
    use serde_json::Value;
    use tower::ServiceExt;

    use super::{AppState, app};

    fn test_state() -> AppState {
        let mut ticket_repository = InMemoryTicketRepository::new();

        let ticket = Ticket::new(
            TicketId::new(100),
            ProjectId::new(10),
            UserId::new(1),
            String::from("Implement authentication"),
            String::from("Create login and registration"),
            Priority::High,
        );

        ticket_repository
            .save(ticket)
            .expect("the in-memory repository cannot fail");

        AppState::new(ticket_repository)
    }

    #[tokio::test]
    async fn health_endpoint_returns_ok() {
        let response = app(AppState::default())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/health")
                    .body(Body::empty())
                    .expect("the test request should be valid"),
            )
            .await
            .expect("the router should handle the request");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn existing_ticket_is_returned_as_json() {
        let response = app(test_state())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/tickets/100")
                    .body(Body::empty())
                    .expect("the test request should be valid"),
            )
            .await
            .expect("the router should handle the request");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&header::HeaderValue::from_static("application/json")),
        );

        let body = to_bytes(response.into_body(), 16 * 1024)
            .await
            .expect("the response body should be readable");

        let json: Value =
            serde_json::from_slice(&body).expect("the response should contain valid JSON");

        assert_eq!(json["id"].as_u64(), Some(100));
        assert_eq!(json["project_id"].as_u64(), Some(10));
        assert_eq!(json["created_by"].as_u64(), Some(1));
        assert_eq!(json["assigned_to"], Value::Null);
        assert_eq!(json["title"].as_str(), Some("Implement authentication"));
        assert_eq!(json["status"].as_str(), Some("backlog"));
        assert_eq!(json["priority"].as_str(), Some("high"));
    }

    #[tokio::test]
    async fn missing_ticket_returns_not_found() {
        let response = app(test_state())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/tickets/999")
                    .body(Body::empty())
                    .expect("the test request should be valid"),
            )
            .await
            .expect("the router should handle the request");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn invalid_ticket_id_returns_bad_request() {
        let response = app(test_state())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/tickets/not-a-number")
                    .body(Body::empty())
                    .expect("the test request should be valid"),
            )
            .await
            .expect("the router should handle the request");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn unknown_route_returns_not_found() {
        let response = app(AppState::default())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/unknown")
                    .body(Body::empty())
                    .expect("the test request should be valid"),
            )
            .await
            .expect("the router should handle the request");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}

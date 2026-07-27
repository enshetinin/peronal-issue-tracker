use pit_domain::{
    ids::{ProjectId, TicketId},
    project::Project,
    ticket::Ticket,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryError {
    Unavailable,
}

pub trait ProjectRepository {
    fn find_by_id(&self, project_id: ProjectId) -> Result<Option<Project>, RepositoryError>;

    fn save(&mut self, project: Project) -> Result<(), RepositoryError>;
}

pub trait TicketRepository {
    fn find_by_id(&self, ticket_id: TicketId) -> Result<Option<Ticket>, RepositoryError>;

    fn save(&mut self, ticket: Ticket) -> Result<(), RepositoryError>;
}

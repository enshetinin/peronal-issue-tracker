use std::collections::HashMap;

use pit_application::{ProjectRepository, RepositoryError, TicketRepository};
use pit_domain::{
    ids::{ProjectId, TicketId},
    project::Project,
    ticket::Ticket,
};

#[derive(Debug, Default)]
pub struct InMemoryProjectRepository {
    projects: HashMap<ProjectId, Project>,
}

impl InMemoryProjectRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ProjectRepository for InMemoryProjectRepository {
    fn find_by_id(&self, project_id: ProjectId) -> Result<Option<Project>, RepositoryError> {
        Ok(self.projects.get(&project_id).cloned())
    }

    fn save(&mut self, project: Project) -> Result<(), RepositoryError> {
        self.projects.insert(project.id(), project);

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct InMemoryTicketRepository {
    tickets: HashMap<TicketId, Ticket>,
}

impl InMemoryTicketRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TicketRepository for InMemoryTicketRepository {
    fn find_by_id(&self, ticket_id: TicketId) -> Result<Option<Ticket>, RepositoryError> {
        Ok(self.tickets.get(&ticket_id).cloned())
    }

    fn save(&mut self, ticket: Ticket) -> Result<(), RepositoryError> {
        self.tickets.insert(ticket.id(), ticket);

        Ok(())
    }
}

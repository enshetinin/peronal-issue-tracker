use std::collections::HashMap;

use crate::ids::TicketId;
use crate::{
    ids::{ProjectId},
    project::Project,
    ticket::Ticket,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryError {
    Unavailable,
}

pub trait ProjectRepository {
    fn find_by_id(&self, id: ProjectId) -> Result<Option<Project>, RepositoryError>;
    fn save(&mut self, project: Project) -> Result<(), RepositoryError>;
}

pub trait TicketRepository {
    fn find_by_id(&self, id: TicketId) -> Result<Option<Ticket>, RepositoryError>;
    fn save(&mut self, ticket: Ticket) -> Result<(), RepositoryError>;
}

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
    fn find_by_id(&self, id: ProjectId) -> Result<Option<Project>, RepositoryError> {
        Ok(self.projects.get(&id).cloned())
    }

    fn save(&mut self, project: Project) -> Result<(), RepositoryError> {
        let project_id = project.id();
        let _prev_project = self.projects.insert(project_id, project);
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
        let ticket_id = ticket.id();
        let _prev_ticket = self.tickets.insert(ticket_id, ticket);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ids::{ProjectId, TicketId, UserId},
        project::Project,
        ticket::{Priority, Ticket},
    };

    use super::{
        InMemoryProjectRepository, InMemoryTicketRepository, ProjectRepository, TicketRepository,
    };

    #[test]
    fn saved_project_can_be_found_by_id() {
        let mut repository = InMemoryProjectRepository::new();

        let project = Project::new(
            ProjectId::new(10),
            UserId::new(1),
            String::from("Atlas"),
            String::new(),
        )
        .expect("the test project has a valid name");

        assert_eq!(repository.save(project), Ok(()));

        let stored_project = repository
            .find_by_id(ProjectId::new(10))
            .expect("the in-memory repository should be available")
            .expect("the saved project should exist");

        assert_eq!(stored_project.id(), ProjectId::new(10));
        assert_eq!(stored_project.name(), "Atlas");
    }

    #[test]
    fn missing_project_returns_none() {
        let repository = InMemoryProjectRepository::new();

        let result = repository
            .find_by_id(ProjectId::new(99))
            .expect("the in-memory repository should be available");

        assert!(result.is_none());
    }

    #[test]
    fn saving_ticket_again_replaces_previous_value() {
        let mut repository = InMemoryTicketRepository::new();

        let mut ticket = Ticket::new(
            TicketId::new(100),
            ProjectId::new(10),
            UserId::new(1),
            String::from("Primer título"),
            String::new(),
            Priority::Medium,
        );

        assert_eq!(repository.save(ticket.clone()), Ok(()));

        assert_eq!(
            ticket.change_title(String::from("Título actualizado")),
            Ok(())
        );
        assert_eq!(repository.save(ticket), Ok(()));

        let stored_ticket = repository
            .find_by_id(TicketId::new(100))
            .expect("the in-memory repository should be available")
            .expect("the saved ticket should exist");

        assert_eq!(stored_ticket.title(), "Título actualizado");
    }
}

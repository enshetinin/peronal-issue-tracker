use crate::{
    ids::{ProjectId, TicketId, UserId},
    project::Project,
    repository::{ProjectRepository, RepositoryError, TicketRepository},
    ticket::Ticket,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignTicketError {
    ProjectMismatch {
        expected_project_id: ProjectId,
        ticket_project_id: ProjectId,
    },
    AssigneeIsNotProjectMember {
        project_id: ProjectId,
        user_id: UserId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignTicketByIdError {
    ProjectNotFound { project_id: ProjectId },
    TicketNotFound { ticket_id: TicketId },
    AssignmentFailed(AssignTicketError),
    RepositoryFailed(RepositoryError),
}

pub fn assign_ticket(
    project: &Project,
    ticket: &mut Ticket,
    assignee_id: UserId,
) -> Result<(), AssignTicketError> {
    if ticket.project_id() != project.id() {
        return Err(AssignTicketError::ProjectMismatch {
            expected_project_id: project.id(),
            ticket_project_id: ticket.project_id(),
        });
    }

    if !project.contains_member(assignee_id) {
        return Err(AssignTicketError::AssigneeIsNotProjectMember {
            project_id: project.id(),
            user_id: assignee_id,
        });
    }

    ticket.assign_to(assignee_id);
    Ok(())
}

pub fn unassign_ticket(
    project: &Project,
    ticket: &mut Ticket,
) -> Result<Option<UserId>, AssignTicketError> {
    if ticket.project_id() != project.id() {
        return Err(AssignTicketError::ProjectMismatch {
            expected_project_id: project.id(),
            ticket_project_id: ticket.project_id(),
        });
    }

    Ok(ticket.take_assignee())
}

pub fn assign_ticket_by_id<P, T>(
    project_repository: &P,
    ticket_repository: &mut T,
    project_id: ProjectId,
    ticket_id: TicketId,
    assignee_id: UserId,
) -> Result<(), AssignTicketByIdError>
where
    P: ProjectRepository,
    T: TicketRepository,
{
    let project = project_repository
        .find_by_id(project_id)
        .map_err(AssignTicketByIdError::RepositoryFailed)?
        .ok_or(AssignTicketByIdError::ProjectNotFound { project_id })?;

    let mut ticket = ticket_repository
        .find_by_id(ticket_id)
        .map_err(AssignTicketByIdError::RepositoryFailed)?
        .ok_or(AssignTicketByIdError::TicketNotFound { ticket_id })?;

    assign_ticket(&project, &mut ticket, assignee_id)
        .map_err(AssignTicketByIdError::AssignmentFailed)?;

    ticket_repository
        .save(ticket)
        .map_err(AssignTicketByIdError::RepositoryFailed)?;

    Ok(())
}

fn unassign_ticket_by_id<P, T>(
    project_repository: &P,
    ticket_repository: &mut T,
    project_id: ProjectId,
    ticket_id: TicketId,
) -> Result<Option<UserId>, AssignTicketByIdError>
where
    P: ProjectRepository,
    T: TicketRepository,
{
    let project = project_repository
        .find_by_id(project_id)
        .map_err(AssignTicketByIdError::RepositoryFailed)?
        .ok_or(AssignTicketByIdError::ProjectNotFound { project_id })?;

    let mut ticket = ticket_repository
        .find_by_id(ticket_id)
        .map_err(AssignTicketByIdError::RepositoryFailed)?
        .ok_or(AssignTicketByIdError::TicketNotFound { ticket_id })?;

    let previous_assignee =
        unassign_ticket(&project, &mut ticket).map_err(AssignTicketByIdError::AssignmentFailed)?;

    ticket_repository
        .save(ticket)
        .map_err(AssignTicketByIdError::RepositoryFailed)?;

    Ok(previous_assignee)
}

#[cfg(test)]
mod tests {
    use super::{
        AssignTicketByIdError, AssignTicketError, assign_ticket, assign_ticket_by_id,
        unassign_ticket, unassign_ticket_by_id,
    };

    use crate::{
        ids::{ProjectId, TicketId, UserId},
        project::Project,
        repository::{
            InMemoryProjectRepository, InMemoryTicketRepository, ProjectRepository,
            TicketRepository,
        },
        ticket::{Priority, Ticket},
    };

    fn test_project() -> Project {
        let mut project = Project::new(
            ProjectId::new(10),
            UserId::new(1),
            String::from("Personal issue tracker"),
            String::from("Maintain of the projects and tickets"),
        )
        .expect("the test project has a valid name");

        project
            .add_member(UserId::new(2))
            .expect("user 2 is allready in the project");
        project
    }

    fn test_ticket() -> Ticket {
        Ticket::new(
            TicketId::new(100),
            ProjectId::new(10),
            UserId::new(1),
            String::from("Implement auth"),
            String::from("Create login and register"),
            Priority::High,
        )
    }

    #[test]
    fn project_mismatch_does_not_remove_persisted_assignee() {
        let mut project_repository = InMemoryProjectRepository::new();
        let mut ticket_repository = InMemoryTicketRepository::new();

        assert_eq!(project_repository.save(test_project()), Ok(()));
        assert_eq!(ticket_repository.save(test_ticket()), Ok(()));

        assert_eq!(
            assign_ticket_by_id(
                &project_repository,
                &mut ticket_repository,
                ProjectId::new(10),
                TicketId::new(100),
                UserId::new(2),
            ),
            Ok(())
        );

        let another_project = Project::new(
            ProjectId::new(20),
            UserId::new(3),
            String::from("Another project"),
            String::new(),
        )
        .expect("the second test project has a valid name");

        assert_eq!(project_repository.save(another_project), Ok(()));

        let result = unassign_ticket_by_id(
            &project_repository,
            &mut ticket_repository,
            ProjectId::new(20),
            TicketId::new(100),
        );

        assert_eq!(
            result,
            Err(AssignTicketByIdError::AssignmentFailed(
                AssignTicketError::ProjectMismatch {
                    expected_project_id: ProjectId::new(20),
                    ticket_project_id: ProjectId::new(10),
                }
            ))
        );

        let stored_ticket = ticket_repository
            .find_by_id(TicketId::new(100))
            .expect("the in-memory repository should be available")
            .expect("the test ticket should exist");

        assert_eq!(stored_ticket.assigned_to(), Some(UserId::new(2)));
    }

    #[test]
    fn unassigning_ticket_by_id_returns_previous_assignee_and_saves_ticket() {
        let mut project_repository = InMemoryProjectRepository::new();
        let mut ticket_repository = InMemoryTicketRepository::new();

        assert_eq!(project_repository.save(test_project()), Ok(()));
        assert_eq!(ticket_repository.save(test_ticket()), Ok(()));

        assert_eq!(
            assign_ticket_by_id(
                &project_repository,
                &mut ticket_repository,
                ProjectId::new(10),
                TicketId::new(100),
                UserId::new(2),
            ),
            Ok(())
        );

        let result = unassign_ticket_by_id(
            &project_repository,
            &mut ticket_repository,
            ProjectId::new(10),
            TicketId::new(100),
        );

        assert_eq!(result, Ok(Some(UserId::new(2))));

        let stored_ticket = ticket_repository
            .find_by_id(TicketId::new(100))
            .expect("the id-memory repository should be available")
            .expect("the test ticket should exist");

        assert_eq!(stored_ticket.assigned_to(), None);
    }

    #[test]
    fn unassigning_ticket_returns_previous_assignee() {
        let project = test_project();
        let mut ticket = test_ticket();

        assert_eq!(assign_ticket(&project, &mut ticket, UserId::new(2)), Ok(()));
        assert_eq!(ticket.assigned_to(), Some(UserId::new(2)));

        let result = unassign_ticket(&project, &mut ticket);

        assert_eq!(result, Ok(Some(UserId::new(2))));
        assert_eq!(ticket.assigned_to(), None);
    }

    #[test]
    fn unassigning_unassigned_ticket_return_none() {
        let project = test_project();
        let mut ticket = test_ticket();

        let result = unassign_ticket(&project, &mut ticket);

        assert_eq!(result, Ok(None));
        assert_eq!(ticket.assigned_to(), None);
    }

    #[test]
    fn project_member_can_be_assigned_to_the_ticket() {
        let project = test_project();
        let mut ticket = test_ticket();
        let result = assign_ticket(&project, &mut ticket, UserId::new(1));
        assert_eq!(result, Ok(()));
        assert_eq!(ticket.assigned_to(), Some(UserId::new(1)));
    }

    #[test]
    fn user_from_outside_project_cannot_be_assigned() {
        let project = test_project();
        let mut ticket = test_ticket();
        let result = assign_ticket(&project, &mut ticket, UserId::new(99));
        assert!(result.is_err());
        assert_eq!(ticket.assigned_to(), None);
    }

    #[test]
    fn ticket_from_another_project_cannot_be_assigned() {
        let project = test_project();

        let mut ticket = Ticket::new(
            TicketId::new(200),
            ProjectId::new(20),
            UserId::new(1),
            String::from("External ticket"),
            String::new(),
            Priority::High,
        );

        let result = assign_ticket(&project, &mut ticket, UserId::new(2));
        assert_eq!(
            result,
            Err(AssignTicketError::ProjectMismatch {
                expected_project_id: ProjectId::new(10),
                ticket_project_id: ProjectId::new(20)
            })
        );
        assert_eq!(ticket.assigned_to(), None);
    }

    #[test]
    fn assigning_another_user_replace_previous() {
        let mut project = test_project();
        project
            .add_member(UserId::new(3))
            .expect("user 3 is allready in the project");
        let mut ticket = test_ticket();

        assert_eq!(assign_ticket(&project, &mut ticket, UserId::new(2)), Ok(()));
        assert_eq!(assign_ticket(&project, &mut ticket, UserId::new(3)), Ok(()));
        assert_eq!(ticket.assigned_to(), Some(UserId::new(3)));
    }

    #[test]
    fn assigning_ticket_by_id_updates_stored_ticket() {
        let mut project_repository = InMemoryProjectRepository::new();
        let mut ticket_repository = InMemoryTicketRepository::new();

        assert_eq!(project_repository.save(test_project()), Ok(()));
        assert_eq!(ticket_repository.save(test_ticket()), Ok(()));

        let result = assign_ticket_by_id(
            &project_repository,
            &mut ticket_repository,
            ProjectId::new(10),
            TicketId::new(100),
            UserId::new(2),
        );

        assert_eq!(result, Ok(()));

        let stored_ticket = ticket_repository
            .find_by_id(TicketId::new(100))
            .expect("the in-memory repository should be available")
            .expect("the test ticket should exist");

        assert_eq!(stored_ticket.assigned_to(), Some(UserId::new(2)));
    }

    #[test]
    fn assigning_ticket_by_id_rejects_missing_project() {
        let project_repository = InMemoryProjectRepository::new();
        let mut ticket_repository = InMemoryTicketRepository::new();

        assert_eq!(ticket_repository.save(test_ticket()), Ok(()));

        let result = assign_ticket_by_id(
            &project_repository,
            &mut ticket_repository,
            ProjectId::new(99),
            TicketId::new(100),
            UserId::new(2),
        );

        assert_eq!(
            result,
            Err(AssignTicketByIdError::ProjectNotFound {
                project_id: ProjectId::new(99),
            })
        );
    }

    #[test]
    fn assigning_ticket_by_id_rejects_missing_ticket() {
        let mut project_repository = InMemoryProjectRepository::new();
        let mut ticket_repository = InMemoryTicketRepository::new();

        assert_eq!(project_repository.save(test_project()), Ok(()));

        let result = assign_ticket_by_id(
            &project_repository,
            &mut ticket_repository,
            ProjectId::new(10),
            TicketId::new(999),
            UserId::new(2),
        );

        assert_eq!(
            result,
            Err(AssignTicketByIdError::TicketNotFound {
                ticket_id: TicketId::new(999),
            })
        );
    }

    #[test]
    fn rejected_assignment_does_not_update_stored_ticket() {
        let mut project_repository = InMemoryProjectRepository::new();
        let mut ticket_repository = InMemoryTicketRepository::new();

        assert_eq!(project_repository.save(test_project()), Ok(()));
        assert_eq!(ticket_repository.save(test_ticket()), Ok(()));

        let result = assign_ticket_by_id(
            &project_repository,
            &mut ticket_repository,
            ProjectId::new(10),
            TicketId::new(100),
            UserId::new(99),
        );

        assert_eq!(
            result,
            Err(AssignTicketByIdError::AssignmentFailed(
                AssignTicketError::AssigneeIsNotProjectMember {
                    project_id: ProjectId::new(10),
                    user_id: UserId::new(99),
                }
            ))
        );

        let stored_ticket = ticket_repository
            .find_by_id(TicketId::new(100))
            .expect("the in-memory repository should be available")
            .expect("the test ticket should exist");

        assert_eq!(stored_ticket.assigned_to(), None);
    }
}

use pit_application::{
    AssignTicketByIdError, AssignTicketError, ProjectRepository, TicketRepository,
    assign_ticket_by_id, unassign_ticket_by_id,
};
use pit_domain::{
    ids::{ProjectId, TicketId, UserId},
    project::Project,
    ticket::{Priority, Ticket},
};
use pit_infrastructure::{InMemoryProjectRepository, InMemoryTicketRepository};

fn test_project() -> Project {
    let mut project = Project::new(
        ProjectId::new(10),
        UserId::new(1),
        String::from("Personal issue tracker"),
        String::from("Manage projects and tickets"),
    )
    .expect("the test project has a valid name");

    project
        .add_member(UserId::new(2))
        .expect("user 2 is not already in the project");

    project
}

fn test_ticket() -> Ticket {
    Ticket::new(
        TicketId::new(100),
        ProjectId::new(10),
        UserId::new(1),
        String::from("Implement authentication"),
        String::from("Create login and registration"),
        Priority::High,
    )
}

#[test]
fn unassigning_ticket_by_id_returns_previous_assignee() {
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
        .expect("the in-memory repository should be available")
        .expect("the test ticket should exist");

    assert_eq!(stored_ticket.assigned_to(), None);
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
    .expect("the second project has a valid name");

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

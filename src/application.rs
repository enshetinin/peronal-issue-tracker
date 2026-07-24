use crate::{
    ids::{ProjectId, UserId},
    project::Project,
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

#[cfg(test)]
mod tests {
    use super::{assign_ticket, unassign_ticket, AssignTicketError};
    use crate::{
        ids::{ProjectId, TicketId, UserId},
        project::Project,
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
    fn unassigning_ticket_returns_previous_assignee() {
        let project = test_project();
        let mut ticket = test_ticket();

        assert_eq!(
            assign_ticket(&project, &mut ticket, UserId::new(2)),
            Ok(())
        );
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
}

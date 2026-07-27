use pit_domain::{
    ids::{ProjectId, TicketId, UserId},
    project::Project,
    ticket::Ticket,
};

use crate::repository::{ProjectRepository, RepositoryError, TicketRepository};

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

pub fn unassign_ticket_by_id<P, T>(
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

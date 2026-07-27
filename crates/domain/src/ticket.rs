use crate::ids::{ProjectId, TicketId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketStatus {
    Backlog,
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketError {
    EmptyTitle,
    InvalidStatusTransition {
        from: TicketStatus,
        to: TicketStatus,
    },
}

#[derive(Debug, Clone)]
pub struct Ticket {
    id: TicketId,
    project_id: ProjectId,
    created_by: UserId,
    assigned_to: Option<UserId>,
    title: String,
    description: String,
    status: TicketStatus,
    priority: Priority,
}

impl Ticket {
    pub fn new(
        id: TicketId,
        project_id: ProjectId,
        created_by: UserId,
        title: String,
        description: String,
        priority: Priority,
    ) -> Self {
        Self {
            id,
            project_id,
            created_by,
            assigned_to: None,
            title,
            description,
            status: TicketStatus::Backlog,
            priority,
        }
    }

    pub fn id(&self) -> TicketId {
        self.id
    }

    pub fn project_id(&self) -> ProjectId {
        self.project_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> TicketStatus {
        self.status
    }

    pub fn priority(&self) -> Priority {
        self.priority
    }

    pub fn created_by(&self) -> UserId {
        self.created_by
    }

    pub fn move_to(&mut self, new_status: TicketStatus) -> Result<(), TicketError> {
        if !self.can_move_to(new_status) {
            return Err(TicketError::InvalidStatusTransition {
                from: self.status,
                to: new_status,
            });
        }

        self.status = new_status;
        Ok(())
    }

    pub fn change_priority(&mut self, new_priority: Priority) {
        self.priority = new_priority;
    }

    pub fn change_title(&mut self, new_title: String) -> Result<(), TicketError> {
        if new_title.trim().is_empty() {
            return Err(TicketError::EmptyTitle);
        }

        self.title = new_title;
        Ok(())
    }

    pub fn is_completed(&self) -> bool {
        self.status == TicketStatus::Done
    }

    fn can_move_to(&self, new_status: TicketStatus) -> bool {
        matches!(
            (self.status, new_status),
            (TicketStatus::Backlog, TicketStatus::Todo)
                | (TicketStatus::Todo, TicketStatus::Backlog)
                | (TicketStatus::Todo, TicketStatus::InProgress)
                | (TicketStatus::InProgress, TicketStatus::Todo)
                | (TicketStatus::InProgress, TicketStatus::Done)
                | (TicketStatus::Done, TicketStatus::InProgress)
        )
    }

    pub fn assigned_to(&self) -> Option<UserId> {
        self.assigned_to
    }

    pub fn assign_to(&mut self, user_id: UserId) {
        self.assigned_to = Some(user_id);
    }

    pub(crate) fn unassign(&mut self) {
        self.assigned_to = None;
    }

    pub fn is_assigned(&self) -> bool {
        self.assigned_to.is_some()
    }

    pub fn take_assignee(&mut self) -> Option<UserId> {
        self.assigned_to.take()
    }
}

#[cfg(test)]
mod tests {
    use super::{Priority, Ticket, TicketError, TicketStatus};
    use crate::ids::{ProjectId, TicketId, UserId};

    fn test_ticket() -> Ticket {
        Ticket::new(
            TicketId::new(1),
            ProjectId::new(13),
            UserId::new(1),
            String::from("Test title"),
            String::from("Test description"),
            Priority::Medium,
        )
    }

    #[test]
    fn ticket_created_by() {
        let ticket = test_ticket();
        assert_eq!(ticket.created_by(), UserId::new(1));
    }

    #[test]
    fn assign_user_id_to_ticket() {
        let mut ticket = test_ticket();
        ticket.assign_to(UserId::new(25));
        assert_eq!(ticket.assigned_to(), Some(UserId::new(25)));
        assert!(ticket.is_assigned());
    }

    #[test]
    fn assign_replace_prev_assign() {
        let mut ticket = test_ticket();
        ticket.assign_to(UserId::new(25));
        ticket.assign_to(UserId::new(13));
        assert_eq!(ticket.assigned_to(), Some(UserId::new(13)));
    }

    #[test]
    fn unassign_ticket_removes_user() {
        let mut ticket = test_ticket();
        ticket.assign_to(UserId::new(25));
        ticket.unassign();
        assert_eq!(ticket.assigned_to(), None);
        assert!(!ticket.is_assigned());
    }

    #[test]
    fn test_ticket_has_id_and_project_id() {
        let ticket = test_ticket();
        assert_eq!(ticket.project_id(), ProjectId::new(13));
        assert_eq!(ticket.id, TicketId::new(1));
    }

    #[test]
    fn change_title_updates_the_ticket() {
        let mut ticket = test_ticket();
        let result = ticket.change_title(String::from("Test title"));
        assert_eq!(result, Ok(()));
        assert_eq!(ticket.title(), "Test title");
    }

    #[test]
    fn empty_title_is_rejected() {
        let mut ticket = test_ticket();
        let result = ticket.change_title(String::from("   "));
        assert_eq!(result, Err(TicketError::EmptyTitle));
    }

    #[test]
    fn ticket_status_in_backlog() {
        let ticket = test_ticket();
        assert_eq!(ticket.status(), TicketStatus::Backlog);
        assert_eq!(ticket.priority(), Priority::Medium);
    }

    #[test]
    fn moving_ticket_update_status() {
        let mut ticket = test_ticket();
        let result = ticket.move_to(TicketStatus::Todo);
        assert_eq!(result, Ok(()));
        assert_eq!(ticket.status(), TicketStatus::Todo);
    }

    #[test]
    fn change_ticket_priority_up() {
        let mut ticket = test_ticket();
        ticket.change_priority(Priority::High);
        assert_eq!(ticket.priority(), Priority::High);
    }

    #[test]
    fn is_ticket_completed() {
        let mut ticket = test_ticket();

        assert_eq!(ticket.move_to(TicketStatus::Todo), Ok(()));
        assert_eq!(ticket.move_to(TicketStatus::InProgress), Ok(()));
        assert_eq!(ticket.move_to(TicketStatus::Done), Ok(()));

        assert!(ticket.is_completed());
    }

    #[test]
    fn invalid_transition_return_an_error() {
        let mut ticket = test_ticket();
        let result = ticket.move_to(TicketStatus::Done);
        assert_eq!(
            result,
            Err(TicketError::InvalidStatusTransition {
                from: TicketStatus::Backlog,
                to: TicketStatus::Done,
            })
        )
    }

    #[test]
    fn invalid_transition_doesnt_modify_ticket() {
        let mut ticket = test_ticket();
        let _result = ticket.move_to(TicketStatus::Done);
        assert_eq!(ticket.status(), TicketStatus::Backlog);
    }

    #[test]
    fn backlog_ticket_is_not_completed() {
        let ticket = test_ticket();
        assert!(!ticket.is_completed());
    }
}

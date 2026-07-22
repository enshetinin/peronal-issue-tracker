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

#[derive(Debug)]
pub struct Ticket {
    id: u64,
    title: String,
    description: String,
    status: TicketStatus,
    priority: Priority,
}

impl Ticket {
    pub fn new(id: u64, title: String, description: String, priority: Priority) -> Self {
        Self {
            id,
            title,
            description,
            status: TicketStatus::Backlog,
            priority,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
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

    pub fn move_to(&mut self, new_status: TicketStatus) {
        self.status = new_status;
    }

    pub fn change_priority(&mut self, new_priority: Priority) {
        self.priority = new_priority;
    }

    pub fn is_completed(&self) -> bool {
        self.status == TicketStatus::Done
    }
}

#[cfg(test)]
mod tests {
    use super::{Priority, Ticket, TicketStatus};

    fn test_ticket() -> Ticket {
        Ticket::new(
            1,
            String::from("Test title"),
            String::from("Test description"),
            Priority::Medium,
        )
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
        ticket.move_to(TicketStatus::InProgress);
        assert_eq!(ticket.status(), TicketStatus::InProgress);
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
        ticket.move_to(TicketStatus::Done);
        assert_eq!(ticket.is_completed(), true);
    }
}

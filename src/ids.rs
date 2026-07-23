#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TicketId(u64);

impl TicketId {
    pub fn new(value: u64) -> TicketId {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectId(u64);

impl ProjectId {
    pub fn new(value: u64) -> ProjectId {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(u64);

impl UserId {
    pub fn new(value: u64) -> UserId {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{ProjectId, TicketId, UserId};

    #[test]
    fn ticket_id_exposes_its_numeric_value() {
        let id = TicketId::new(13);
        assert_eq!(id.get(), 13);
    }

    #[test]
    fn project_id_exposes_its_numeric_value() {
        let id = ProjectId::new(13);
        assert_eq!(id.get(), 13);
    }

    #[test]
    fn user_id_exposes_its_numeric_value() {
        let id = UserId::new(1);
        assert_eq!(id.get(), 1);
    }

    #[test]
    fn ids_are_copy_types() {
        let original = TicketId::new(13);
        let copy = original;
        assert_eq!(copy.get(), 13);
    }
}

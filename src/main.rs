use peronal_issue_tracker::ticket::{Priority, Ticket, TicketStatus};

fn main() {
    let mut ticket = Ticket::new(
        1,
        String::from("Implement authentication"),
        String::from("Make all project private for only authenticated users"),
        Priority::High,
    );

    println!(
        "Ticket #{}: {}\nDescription: {}\nState: {:?}\nPriority: {:?}",
        ticket.id(),
        ticket.title(),
        ticket.description(),
        ticket.status(),
        ticket.priority(),
    );

    ticket.move_to(TicketStatus::InProgress);
    println!(
        "Change state off #{}: {} to {:?}",
        ticket.id(),
        ticket.title(),
        ticket.status()
    );
}

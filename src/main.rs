use peronal_issue_tracker::ticket::{Priority, Ticket, TicketStatus};
use peronal_issue_tracker::ids::{TicketId, ProjectId};
fn main() {
    let mut ticket = Ticket::new(
        TicketId::new(1),
        ProjectId::new(13),
        String::from("Implement authentication"),
        String::from("Make all project private for only authenticated users"),
        Priority::High,
    );

    println!(
        "Ticket #{}: {} of project: #{}\nDescription: {}\nState: {:?}\nPriority: {:?}",
        ticket.id().get(),
        ticket.title(),
        ticket.project_id().get(),
        ticket.description(),
        ticket.status(),
        ticket.priority(),
    );

    match ticket.move_to(TicketStatus::Todo) {
        Ok(()) => println!("Ticket #{}: done", ticket.id().get()),
        Err(error) => eprintln!("{:?}", error),
    }
}

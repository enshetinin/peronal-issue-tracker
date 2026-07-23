use peronal_issue_tracker::ticket::{Priority, Ticket, TicketStatus};
use peronal_issue_tracker::ids::{TicketId, ProjectId, UserId};
fn main() {
    let mut ticket = Ticket::new(
        TicketId::new(1),
        ProjectId::new(13),
        UserId::new(13),
        String::from("Implement authentication"),
        String::from("Make all project private for only authenticated users"),
        Priority::High,
    );

    print_ticket_assignment(&ticket);
    ticket.assign_to(UserId::new(13));
    print_ticket_assignment(&ticket);

    match ticket.move_to(TicketStatus::Todo) {
        Ok(()) => println!("Ticket #{}: done", ticket.id().get()),
        Err(error) => eprintln!("{:?}", error),
    }
}

fn print_ticket_assignment(ticket: &Ticket) {
    match ticket.assigned_to() {
        Some(user_id) => {
            println!("User #{} assigned to ticket #{}", user_id.get(), ticket.id().get());
        }
        None => {
            println!("No user assigned to ticket #{}", ticket.id().get());
        }
    }
}
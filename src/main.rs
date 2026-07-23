use peronal_issue_tracker::ticket::{Priority, Ticket, TicketStatus};
use peronal_issue_tracker::{
    ids::{ProjectId, TicketId, UserId},
    project::{Project, ProjectRole},
};
fn main() {
    let mut ticket = Ticket::new(
        TicketId::new(1),
        ProjectId::new(13),
        UserId::new(13),
        String::from("Implement authentication"),
        String::from("Make all project private for only authenticated users"),
        Priority::High,
    );

    let project_result = Project::new(
        ProjectId::new(1),
        UserId::new(10),
        String::from("Test issue tracker"),
        String::from("Some description bla bla..."),
    );

    let mut project = match project_result {
        Ok(project) => project,
        Err(error) => {
            eprintln!("No se pudo crear el proyecto: {error:?}");
            return;
        }
    };

    match project.add_member(UserId::new(20)) {
        Ok(_) => println!("Member added successfully"),
        Err(err) => eprintln!("Member added failed: {err:?}"),
    }

    println!(
        "The project `{}` has {} members",
        project.name(),
        project.members_count(),
    );

    print_member_role(&project, UserId::new(10));
    print_member_role(&project, UserId::new(20));
    print_member_role(&project, UserId::new(99));

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
            println!(
                "User #{} assigned to ticket #{}",
                user_id.get(),
                ticket.id().get()
            );
        }
        None => {
            println!("No user assigned to ticket #{}", ticket.id().get());
        }
    }
}

fn print_member_role(project: &Project, user_id: UserId) {
    match project.member_role(user_id) {
        Some(ProjectRole::Owner) => {
            println!("User : #{} is owner", user_id.get());
        }
        Some(ProjectRole::Member) => {
            println!("User #{} is member", user_id.get());
        }
        None => {
            println!("User #{} is not part of the project", user_id.get());
        }
    }
}

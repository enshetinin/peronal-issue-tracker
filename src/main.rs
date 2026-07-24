use peronal_issue_tracker::{
    application::assign_ticket,
    ids::{ProjectId, TicketId, UserId},
    project::Project,
    ticket::{Priority, Ticket},
};

fn main() {
    let mut project = match Project::new(
        ProjectId::new(1),
        UserId::new(10),
        String::from("Personal issue tracker"),
        String::from("Some description"),
    ) {
        Ok(project) => project,
        Err(error) => {
            eprintln!("Could not create project: {error:?}");
            return;
        }
    };

    if let Err(error) = project.add_member(UserId::new(20)) {
        eprintln!("Could not add member to project: {error:?}");
        return;
    }

    let mut ticket = Ticket::new(
        TicketId::new(100),
        project.id(),
        UserId::new(10),
        String::from("Implement auth"),
        String::from("Some description"),
        Priority::High,
    );

    match assign_ticket(&project, &mut ticket, UserId::new(20)) {
        Ok(()) => {
            println!(
                "Ticket #{} assigned to user #{}",
                ticket.id().get(),
                ticket.assigned_to().map(UserId::get).unwrap_or_default()
            );
        }
        Err(error) => {
            eprintln!("Could not assign to user #{error:?}");
        }
    }
}

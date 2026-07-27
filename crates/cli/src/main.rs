use pit_application::{ProjectRepository, TicketRepository, assign_ticket_by_id};
use pit_domain::{
    ids::{ProjectId, TicketId, UserId},
    project::Project,
    ticket::{Priority, Ticket},
};
use pit_infrastructure::{InMemoryProjectRepository, InMemoryTicketRepository};

fn main() {
    let mut project_repository = InMemoryProjectRepository::new();
    let mut ticket_repository = InMemoryTicketRepository::new();

    let mut project = match Project::new(
        ProjectId::new(1),
        UserId::new(10),
        String::from("Test title"),
        String::from("Test description"),
    ) {
        Ok(project) => project,
        Err(error) => {
            eprintln!("Couldnt create a project: {error:?}");
            return;
        }
    };

    if let Err(error) = project.add_member(UserId::new(20)) {
        eprintln!("Couldnt add a member {error:?}");
        return;
    }

    let ticket = Ticket::new(
        TicketId::new(100),
        project.id(),
        UserId::new(10),
        String::from("Implement auth"),
        String::from("Some dummy description"),
        Priority::High,
    );

    if let Err(error) = project_repository.save(project) {
        eprintln!("Couldnt save the project: {error:?}");
        return;
    }

    if let Err(error) = ticket_repository.save(ticket) {
        eprintln!("Couldnt save the ticket: {error:?}");
        return;
    }

    let result = assign_ticket_by_id(
        &project_repository,
        &mut ticket_repository,
        ProjectId::new(1),
        TicketId::new(100),
        UserId::new(20),
    );

    if let Err(error) = result {
        eprintln!("Couldnt assign the ticket: {error:?}");
        return;
    }

    match ticket_repository.find_by_id(TicketId::new(100)) {
        Ok(Some(ticket)) => {
            println!(
                "Ticket #{} assigned to {:?}",
                ticket.id().get(),
                ticket.assigned_to(),
            );
        }
        Ok(None) => {
            eprintln!("Saved ticket doesnt exist");
        }
        Err(error) => {
            eprintln!("Could not consult the ticket: {error:?}");
        }
    }
}

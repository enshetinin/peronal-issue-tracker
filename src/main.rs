fn build_welcome_message(project_name: &str) -> String {
    format!("Project {project_name} prepared to start")
}
fn main() {
    let project_name = "Personal Issue Tracker";
    let message = build_welcome_message(project_name);

    println!("{message}");
}

#[cfg(test)]
mod tests {
    use super::build_welcome_message;

    #[test]
    fn build_welcome_message_witch_project_name() {
        let message = build_welcome_message("Cool");
        assert_eq!(message, "Project Cool prepared to start");
    }
}

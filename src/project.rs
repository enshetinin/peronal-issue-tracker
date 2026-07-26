use crate::ids::{ProjectId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectRole {
    Owner,
    Member,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectMember {
    user_id: UserId,
    role: ProjectRole,
}

impl ProjectMember {
    pub fn user_id(&self) -> UserId {
        self.user_id
    }
    pub fn role(&self) -> ProjectRole {
        self.role
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectError {
    EmptyName,
    DuplicatedMember { user_id: UserId },
    MemberNotFound { user_id: UserId },
    CannotRemoveOwner { user_id: UserId },
}

#[derive(Debug, Clone)]
pub struct Project {
    id: ProjectId,
    name: String,
    description: String,
    members: Vec<ProjectMember>,
}

impl Project {
    pub fn new(
        id: ProjectId,
        owner_id: UserId,
        name: String,
        description: String,
    ) -> Result<Self, ProjectError> {
        if name.trim().is_empty() {
            return Err(ProjectError::EmptyName);
        }

        let owner = ProjectMember {
            user_id: owner_id,
            role: ProjectRole::Owner,
        };

        Ok(Self {
            id,
            name,
            description,
            members: vec![owner],
        })
    }

    pub fn id(&self) -> ProjectId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn members_count(&self) -> usize {
        self.members.len()
    }

    pub fn contains_member(&self, user_id: UserId) -> bool {
        self.members
            .iter()
            .any(|member| member.user_id() == user_id)
    }

    pub fn member_role(&self, user_id: UserId) -> Option<ProjectRole> {
        self.members
            .iter()
            .find(|member| member.user_id() == user_id)
            .map(|member| member.role)
    }

    pub fn add_member(&mut self, user_id: UserId) -> Result<(), ProjectError> {
        if self.contains_member(user_id) {
            return Err(ProjectError::DuplicatedMember { user_id });
        }

        self.members.push(ProjectMember {
            user_id,
            role: ProjectRole::Member,
        });

        Ok(())
    }

    pub fn remove_member(&mut self, user_id: UserId) -> Result<(), ProjectError> {
        let role = self
            .member_role(user_id)
            .ok_or(ProjectError::MemberNotFound { user_id })?;
        if role == ProjectRole::Owner {
            return Err(ProjectError::CannotRemoveOwner { user_id });
        }
        self.members.retain(|member| member.user_id() != user_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Project, ProjectError, ProjectRole};
    use crate::ids::{ProjectId, UserId};

    fn test_project() -> Project {
        Project::new(
            ProjectId::new(10),
            UserId::new(1),
            String::from("Issue test"),
            String::from("Some description"),
        )
        .expect("failed to create new project")
    }

    #[test]
    fn new_project_contains_its_owner() {
        let project = test_project();
        assert_eq!(project.members_count(), 1);
        assert!(project.contains_member(UserId::new(1)));
        assert_eq!(
            project.member_role(UserId::new(1)),
            Some(ProjectRole::Owner)
        );
    }

    #[test]
    fn empty_project_name_is_invalid() {
        let result = Project::new(
            ProjectId::new(10),
            UserId::new(1),
            String::from("   "),
            String::new(),
        );
        assert!(matches!(result, Err(ProjectError::EmptyName)));
    }

    #[test]
    fn adding_member_store_the_user_as_member() {
        let mut project = test_project();
        let result = project.add_member(UserId::new(2));

        assert_eq!(result, Ok(()));
        assert_eq!(project.members_count(), 2);
        assert_eq!(
            project.member_role(UserId::new(2)),
            Some(ProjectRole::Member)
        );
    }

    #[test]
    fn duplicate_member_is_rejected() {
        let mut project = test_project();
        assert_eq!(project.add_member(UserId::new(2)), Ok(()));
        let result = project.add_member(UserId::new(2));
        assert_eq!(
            result,
            Err(ProjectError::DuplicatedMember {
                user_id: UserId::new(2),
            })
        );
        assert_eq!(project.members_count(), 2);
    }

    #[test]
    fn removing_member_removes_the_user() {
        let mut project = test_project();
        assert_eq!(project.add_member(UserId::new(99)), Ok(()));
        let result = project.remove_member(UserId::new(99));
        assert_eq!(result, Ok(()));
        assert!(!project.contains_member(UserId::new(99)));
        assert_eq!(project.members_count(), 1);
    }

    #[test]
    fn removing_unknown_member_returns_error() {
        let mut project = test_project();
        let result = project.remove_member(UserId::new(99));
        assert_eq!(
            result,
            Err(ProjectError::MemberNotFound {
                user_id: UserId::new(99)
            })
        );
    }

    #[test]
    fn owner_cannot_be_removed() {
        let mut project = test_project();
        let result = project.remove_member(UserId::new(1));

        assert_eq!(
            result,
            Err(ProjectError::CannotRemoveOwner {
                user_id: UserId::new(1),
            })
        );
        assert!(project.contains_member(UserId::new(1)));
    }
}

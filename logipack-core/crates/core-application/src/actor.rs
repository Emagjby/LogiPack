use crate::roles::Role;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ActorContext {
    /// Internal user id (local DB)
    pub user_id: Uuid,

    /// External subject (dev sub / Auth0 sub later)
    pub sub: String,

    /// Roles granted to this user
    pub roles: Vec<Role>,

    /// Employee id if this user is an employee
    pub employee_id: Option<Uuid>,

    /// Office ids this actor will be allowed to operate in
    pub allowed_office_ids: Vec<Uuid>,
}

impl ActorContext {
    pub fn is_admin(&self) -> bool {
        self.roles.contains(&Role::Admin)
    }

    pub fn is_employee(&self) -> bool {
        self.roles.contains(&Role::Employee)
    }
}

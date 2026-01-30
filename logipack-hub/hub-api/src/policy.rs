use axum::http::StatusCode;
use core_application::{actor::ActorContext, roles::Role};

#[inline]
pub fn require_authenticated(_actor: &ActorContext) -> Result<(), StatusCode> {
    Ok(())
}

#[inline]
pub fn require_admin(actor: &ActorContext) -> Result<(), StatusCode> {
    if actor.roles.contains(&Role::Admin) {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

#[inline]
pub fn require_employee(actor: &ActorContext) -> Result<(), StatusCode> {
    if actor.roles.contains(&Role::Employee) || actor.roles.contains(&Role::Admin) {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

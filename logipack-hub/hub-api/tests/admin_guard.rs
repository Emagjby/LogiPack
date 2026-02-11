use axum::{body::Body, extract::Request, http::StatusCode};
use tower::ServiceExt;

use crate::helpers::setup_app_with_employee;

#[allow(dead_code)]
mod helpers;

#[tokio::test]
async fn employee_cannot_access_admin_route() {
    let (app, employee) = setup_app_with_employee().await;

    let res = app
        .oneshot(
            Request::builder()
                .uri("/admin")
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", employee.sub.clone())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    if res.status() != StatusCode::NOT_FOUND {
        println!("TODO: CHANGE THIS TEST ONCE ADMIN ROUTES ARE IMPLEMENTED");
        assert_eq!(res.status(), StatusCode::FORBIDDEN);
    }
}

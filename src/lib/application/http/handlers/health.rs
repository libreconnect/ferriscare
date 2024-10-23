use axum::response::IntoResponse;

pub async fn liveness() -> impl IntoResponse {
    "Liveness check: OK"
}

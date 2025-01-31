use axum::{extract::State, Json};
use std::sync::Arc;
use crate::discord::AppState;
use crate::models::AlertPayload;
use crate::handlers::thread::handle_thread;

pub async fn handle_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AlertPayload>,
) -> &'static str {
    let alerts = payload.alerts.unwrap_or_default();
    let status = payload.status.unwrap_or("unknown".to_string());

    for alert in alerts {
        let thread_name = format!("{} - {}",
            alert.labels
                .as_ref()
                .and_then(|l| l.alertname.as_ref())
                .map(|s| s.as_str())
                .unwrap_or("unknown"),
            alert.labels
                .as_ref()
                .and_then(|l| l.namespace.as_ref())
                .map(|s| s.as_str())
                .unwrap_or("unknown")
        );
        log::debug!("{}", thread_name);
        handle_thread(&state, &thread_name, &alert, alert.status.as_deref().unwrap_or(&status)).await;
    }

    "OK"
}

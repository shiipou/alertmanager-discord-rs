use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertPayload {
    pub alerts: Option<Vec<Alert>>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    pub status: Option<String>,
    pub labels: Option<AlertLabels>,
    pub annotations: Option<AlertAnnotations>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertLabels {
    pub alertname: Option<String>,
    pub namespace: Option<String>,
    pub pod: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertAnnotations {
    pub description: Option<String>,
}

// RemoteSupport: hbbs'in, bağlantı isteklerindeki conn_token'ı
// kendi auth-server'ımıza sorup doğruladığı katman.

use hbb_common::ResultType;

const AUTH_SERVER_BASE: &str = "http://127.0.0.1:4000";

pub async fn verify_conn_token(token: &str, host_peer_id: &str) -> ResultType<bool> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/auth/connections/verify-token", AUTH_SERVER_BASE))
        .json(&serde_json::json!({ "token": token, "hostPeerId": host_peer_id }))
        .timeout(std::time::Duration::from_secs(3))
        .send()
        .await?;

    if !resp.status().is_success() {
        return Ok(false);
    }
    let data: serde_json::Value = resp.json().await?;
    Ok(data.get("valid").and_then(|v| v.as_bool()).unwrap_or(false))
}

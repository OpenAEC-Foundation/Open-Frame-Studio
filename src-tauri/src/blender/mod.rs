use serde_json::{json, Value};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const BONSAI_ADDR: &str = "127.0.0.1:9876";

async fn send_command(command: Value) -> Result<Value, String> {
    let mut stream = TcpStream::connect(BONSAI_ADDR)
        .await
        .map_err(|e| format!("Kan niet verbinden met Blender/Bonsai op {}: {}", BONSAI_ADDR, e))?;

    let msg = serde_json::to_string(&command).map_err(|e| e.to_string())?;
    stream
        .write_all(msg.as_bytes())
        .await
        .map_err(|e| e.to_string())?;
    stream.shutdown().await.map_err(|e| e.to_string())?;

    let mut buf = Vec::new();
    stream
        .read_to_end(&mut buf)
        .await
        .map_err(|e| e.to_string())?;

    let response: Value =
        serde_json::from_slice(&buf).map_err(|e| format!("Ongeldig antwoord van Blender: {}", e))?;

    if response.get("status").and_then(|s| s.as_str()) == Some("error") {
        let msg = response
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("Onbekende fout");
        return Err(format!("Blender fout: {}", msg));
    }

    Ok(response)
}

pub async fn ping() -> Result<Value, String> {
    send_command(json!({"type": "ping"})).await
}

pub async fn execute_code(code: &str) -> Result<String, String> {
    let response = send_command(json!({
        "type": "execute_code",
        "params": { "code": code }
    }))
    .await?;

    Ok(response
        .get("result")
        .and_then(|r| r.as_str())
        .unwrap_or("OK")
        .to_string())
}

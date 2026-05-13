use crate::errors::{AppError, AppResult};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

pub fn check_local_health(host: &str, port: u16, timeout_ms: u64) -> AppResult<bool> {
    wait_for_http_any(
        host,
        port,
        &["/health", "/v1/models", "/props", "/"],
        timeout_ms,
    )
}

pub fn wait_for_http_any(
    host: &str,
    port: u16,
    paths: &[&str],
    timeout_ms: u64,
) -> AppResult<bool> {
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    while Instant::now() < deadline {
        for path in paths {
            if http_get_ok(host, port, path, 1_000)? {
                return Ok(true);
            }
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    Ok(false)
}

pub fn http_get_ok(host: &str, port: u16, path: &str, timeout_ms: u64) -> AppResult<bool> {
    let mut addrs = format!("{host}:{port}")
        .to_socket_addrs()
        .map_err(|error| {
            AppError::with_details(
                "HEALTH_CHECK_FAILED",
                error.to_string(),
                serde_json::json!({"host": host, "port": port}),
            )
        })?;
    let Some(addr) = addrs.next() else {
        return Ok(false);
    };

    let mut stream = match TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms)) {
        Ok(stream) => stream,
        Err(_) => return Ok(false),
    };
    stream.set_read_timeout(Some(Duration::from_millis(timeout_ms)))?;
    stream.set_write_timeout(Some(Duration::from_millis(timeout_ms)))?;

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nAccept: */*\r\n\r\n",
        path, host
    );
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    let _ = stream.read_to_string(&mut response);
    Ok(response.starts_with("HTTP/1.1 200") || response.starts_with("HTTP/1.0 200"))
}

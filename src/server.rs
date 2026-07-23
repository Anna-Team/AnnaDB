use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

use tracing::{debug, error, info};

use crate::storage::main::Storage;

struct Request {
    path: String,
    body: String,
}

fn parse_request(stream: &mut TcpStream) -> Option<Request> {
    let mut reader = BufReader::new(stream.try_clone().ok()?);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;

    let path = {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }
        parts[1].to_string()
    };

    // Read headers
    let mut content_length = 0usize;
    loop {
        line.clear();
        reader.read_line(&mut line).ok()?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        if let Some(len) = trimmed
            .to_lowercase()
            .strip_prefix("content-length:")
            .and_then(|v| v.trim().parse().ok())
        {
            content_length = len;
        }
    }

    // Read body
    let mut body = String::new();
    if content_length > 0 {
        let mut buf = vec![0u8; content_length];
        reader.read_exact(&mut buf).ok()?;
        body = String::from_utf8_lossy(&buf).to_string();
    }

    Some(Request { path, body })
}

fn respond(stream: &mut TcpStream, status: &str, content_type: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status,
        content_type,
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
}

pub fn serve(storage: &mut Storage, port: u16) {
    let addr = format!("0.0.0.0:{}", port);
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            error!(port = port, error = %e, "failed to bind");
            return;
        }
    };
    info!(port = port, "AnnaDB HTTP server listening");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(_) => continue,
        };

        let start = std::time::Instant::now();

        let req = match parse_request(&mut stream) {
            Some(r) => r,
            None => {
                respond(&mut stream, "400 Bad Request", "text/plain", "");
                continue;
            }
        };

        debug!(path = %req.path, len = req.body.len(), "request");

        match req.path.as_str() {
            "/health" => {
                respond(&mut stream, "200 OK", "text/plain", "AnnaDB ok");
            }
            "/tx" => {
                let result = storage.run(&req.body);
                let duration = start.elapsed();
                debug!(duration_ms = duration.as_millis() as u64, "transaction processed");
                respond(&mut stream, "200 OK", "text/plain", &result);
            }
            _ => {
                respond(&mut stream, "404 Not Found", "text/plain", "");
            }
        }
    }
}

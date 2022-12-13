use std::io::BufRead;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Log {
    pub msec: String,
    pub connection: String,
    pub connection_requests: String,
    pub pid: String,
    pub request_id: String,
    pub request_length: String,
    pub remote_addr: String,
    pub remote_user: String,
    pub remote_port: String,
    pub time_local: String,
    pub time_iso8601: String,
    pub request: String,
    pub request_uri: String,
    pub args: String,
    pub status: String,
    pub body_bytes_sent: String,
    pub bytes_sent: String,
    pub http_referer: String,
    pub http_user_agent: String,
    pub http_x_forwarded_for: String,
    pub http_host: String,
    pub server_name: String,
    pub request_time: String,
    pub upstream: String,
    pub upstream_connect_time: String,
    pub upstream_header_time: String,
    pub upstream_response_time: String,
    pub upstream_response_length: String,
    pub upstream_cache_status: String,
    pub ssl_protocol: String,
    pub ssl_cipher: String,
    pub scheme: String,
    pub request_method: String,
    pub server_protocol: String,
    pub pipe: String,
    pub gzip_ratio: String,
    pub http_cf_ray: String,
}

impl Log {
    pub fn new(line: &str) -> Result<Self> {
        let log = match serde_json::from_str(line) {
            Ok(log) => log,
            Err(e) => {
                tracing::error!("parse log failed: {}, line: {}", e, line);
                return Err(anyhow::anyhow!(e));
            }
        };
        Ok(log)
    }

    pub fn parse_file(path: &str) -> Result<Vec<Self>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let mut logs = Vec::new();
        for line in reader.lines() {
            let line = line?;
            // 忽略空行和 '\x' 开头的机器人请求，因为他们会导致 serde 解析失败
            if line.is_empty() || line.contains(r#" "request": "\x"#) {
                continue;
            }
            let log = Log::new(&line)?;
            logs.push(log);
        }
        Ok(logs)
    }
}

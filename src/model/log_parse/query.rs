use anyhow::{Ok, Result};
use chrono::Utc;

use super::{cache::LogCache, log::Log};

pub struct QueryLog {
    pub date: String,
    pub query: String,
    pub result: Vec<Log>,
}

impl QueryLog {
    pub async fn query(query: &str) -> Result<Self> {
        let cache = LogCache::get().await?;
        let result = cache
            .data
            .into_iter()
            .filter(|log| log.request_uri.contains(query))
            .collect();
        Ok(Self {
            date: "all".to_string(),
            query: query.to_string(),
            result,
        })
    }

    pub async fn query_today(query: &str) -> Result<Self> {
        let today = Utc::now().format("%d/%b/%Y").to_string();
        Self::query_with_date(query, &today).await
    }

    pub async fn query_7days(query: &str) -> Result<Vec<Self>> {
        let mut result = Vec::new();
        let days = Self::get_7days()?;
        for day in days {
            let item = Self::query_with_date(query, &day).await?;
            result.push(item);
        }
        Ok(result)
    }

    async fn query_with_date(query: &str, date: &str) -> Result<Self> {
        let cache = LogCache::get().await?;
        let result: Vec<Log> = cache
            .data
            .into_iter()
            .filter(|log| log.request_uri.contains(query) && log.time_local.contains(date))
            .collect();
        Ok(QueryLog {
            date: date.to_string(),
            query: query.to_string(),
            result,
        })
    }

    fn get_7days() -> Result<Vec<String>> {
        let mut days = Vec::new();
        for i in 0..7 {
            let day = Utc::now()
                .checked_sub_signed(chrono::Duration::days(i))
                .ok_or_else(|| anyhow::anyhow!("date time checked sub signed failed"))?
                .format("%d/%b/%Y")
                .to_string();
            days.push(day);
        }
        Ok(days)
    }
}

#[cfg(test)]
mod test {
    use crate::model::log_parse::cache;

    use super::*;

    #[test]
    fn test_log_parse() {
        let line = r#"{"msec": "1669862267.429", "connection": "1", "connection_requests": "1", "pid": "29", "request_id": "164d631b97ff62133da2add13fb9d849", "request_length": "732", "remote_addr": "172.23.0.1", "remote_user": "-", "remote_port": "55248", "time_local": "01/Dec/2022:02:37:47 +0000", "time_iso8601": "2022-12-01T02:37:47+00:00", "request": "GET / HTTP/1.1", "request_uri": "/", "args": "-", "status": "200", "body_bytes_sent": "615", "bytes_sent": "853", "http_referer": "-", "http_user_agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36", "http_x_forwarded_for": "-", "http_host": "localhost:8000", "server_name": "localhost", "request_time": "0.000", "upstream": "-", "upstream_connect_time": "-", "upstream_header_time": "-", "upstream_response_time": "-", "upstream_response_length": "-", "upstream_cache_status": "-", "ssl_protocol": "-", "ssl_cipher": "-", "scheme": "http", "request_method": "GET", "server_protocol": "HTTP/1.1", "pipe": ".", "gzip_ratio": "-", "http_cf_ray": "-r#"}"#;
        let log = Log::new(line).unwrap();
        assert_eq!(log.msec, "1669862267.429");
    }

    #[test]
    fn test_log_parse_file() {
        let logs = Log::parse_file("src/model/test_data/access.log").unwrap();
        assert_eq!(logs.len(), 8);
    }

    async fn init_log_cache() {
        cache::init().await.expect("Failed to cache log");
    }

    #[tokio::test]
    async fn test_query_log() {
        std::env::set_var("PARSE_LOG_FILE", "src/model/test_data/access.log");
        init_log_cache().await;
        let query_log = QueryLog::query("/").await.unwrap();
        assert_eq!(query_log.result.len(), 8);
        let query_log = QueryLog::query("ico").await.unwrap();
        assert_eq!(query_log.result.len(), 4);
    }

    #[tokio::test]
    #[ignore]
    // 该测试与日期有关，今天测试能通过，明天就不行了，所以忽略
    async fn test_query_log_query_today() {
        std::env::set_var("PARSE_LOG_FILE", "src/model/test_data/access.log");
        init_log_cache().await;
        let query_log = QueryLog::query_today("/").await.unwrap();
        assert_eq!(query_log.result.len(), 3);
        let query_log = QueryLog::query_today("ico").await.unwrap();
        assert_eq!(query_log.result.len(), 2);
    }

    #[tokio::test]
    #[ignore]
    // 该测试与日期有关，今天测试能通过，明天就不行了，所以忽略
    async fn test_query_log_query_7days() {
        std::env::set_var("PARSE_LOG_FILE", "src/model/test_data/access.log");
        init_log_cache().await;
        let query_logs = QueryLog::query_7days("/").await.unwrap();
        assert_eq!(query_logs.len(), 7);
        assert_eq!(query_logs[0].result.len(), 3);
        assert_eq!(query_logs[1].result.len(), 0);
        assert_eq!(query_logs[2].result.len(), 0);
        assert_eq!(query_logs[3].result.len(), 0);
        assert_eq!(query_logs[4].result.len(), 0);
        assert_eq!(query_logs[5].result.len(), 0);
        assert_eq!(query_logs[6].result.len(), 0);
    }
}

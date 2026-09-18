#[derive(Debug)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
pub struct LogData {
    pub level: LogLevel,
    pub code: &'static str,
    pub message: &'static str,
    pub error: Option<String>,
}

pub trait ToLog {
    fn log_data(&self) -> LogData;
    fn event_type(&self) -> String;
}

pub fn tracing_log<T: ToLog>(event: &T, user_id: Option<i64>
){
    let log = event.log_data();
    let error = event.event_type();
    let user_id = user_id
        .map(|id| id.to_string())
        .unwrap_or_else(|| "None".to_string());

    tracing::info!(
        user_id = %user_id,
        error_type = ?error,
        code = log.code,
        "{}",
        log.message
    );

}
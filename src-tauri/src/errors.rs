use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: Some(details),
        }
    }

    pub fn developer_mode_required() -> Self {
        Self::new("DEVELOPER_MODE_REQUIRED", "Chế độ nhà phát triển chưa được bật.")
    }

    pub fn safe_generic() -> Self {
        Self::new("UNKNOWN_ERROR", "Local AI hiện không khả dụng.")
    }

    pub fn to_safe(self) -> Self {
        match self.code.as_str() {
            "ENGINE_NOT_FOUND" | "MISSING_BINARY" | "MISSING_MODEL" => {
                Self::new(self.code, "Thiếu các tệp AI bắt buộc.")
            }
            "PORT_IN_USE" => Self::new(self.code, "Không thể khởi động Local AI."),
            "DEVELOPER_MODE_REQUIRED" => self,
            _ => Self::safe_generic(),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::with_details(
            "PROCESS_START_FAILED",
            error.to_string(),
            serde_json::json!({"kind": "io"}),
        )
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        Self::with_details(
            "DATABASE_ERROR",
            error.to_string(),
            serde_json::json!({"kind": "sqlite"}),
        )
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::with_details(
            "CONFIG_PARSE_ERROR",
            error.to_string(),
            serde_json::json!({"kind": "json"}),
        )
    }
}

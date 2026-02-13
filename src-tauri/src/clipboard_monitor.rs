#[cfg(not(target_os = "linux"))]
use clipboard::ClipboardProvider;
#[cfg(not(target_os = "linux"))]
use clipboard::ClipboardContext;
use regex::Regex;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use serde::{Deserialize, Serialize};

lazy_static! {
    // Pattern for API keys (common formats)
    static ref API_KEY_PATTERN: Regex = Regex::new(
        r"(?i)(api[_-]?key|apikey|access[_-]?token|secret[_-]?key|private[_-]?key)[\s:=]+[a-zA-Z0-9_\-]{20,}"
    ).unwrap();
    
    // Pattern for common password indicators
    static ref PASSWORD_PATTERN: Regex = Regex::new(
        r"(?i)(password|passwd|pwd)[\s:=]+.{6,}"
    ).unwrap();
    
    // Pattern for AWS keys
    static ref AWS_KEY_PATTERN: Regex = Regex::new(
        r"(?i)(AKIA[0-9A-Z]{16}|aws_secret_access_key)"
    ).unwrap();
    
    // Pattern for GitHub tokens
    static ref GITHUB_TOKEN_PATTERN: Regex = Regex::new(
        r"(?i)(ghp_[a-zA-Z0-9]{36}|github_token)"
    ).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_type: String,
    pub severity: String,
    pub message: String,
    pub timestamp: u64,
}

pub struct ClipboardMonitor {
    last_content: Arc<Mutex<String>>,
    running: Arc<Mutex<bool>>,
}

impl ClipboardMonitor {
    pub fn new() -> Self {
        ClipboardMonitor {
            last_content: Arc::new(Mutex::new(String::new())),
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self, app_handle: AppHandle) {
        let last_content = Arc::clone(&self.last_content);
        let running = Arc::clone(&self.running);
        
        *running.lock().unwrap() = true;

        #[cfg(not(target_os = "linux"))]
        {
            thread::spawn(move || {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                
                while *running.lock().unwrap() {
                    if let Ok(content) = ctx.get_contents() {
                        let mut last = last_content.lock().unwrap();
                        
                        if content != *last && !content.is_empty() {
                            *last = content.clone();
                            
                            // Check for security issues
                            if let Some(event) = Self::check_security_patterns(&content) {
                                // Emit event to frontend
                                let _ = app_handle.emit("security-event", &event);
                                
                                // Send notification
                                Self::send_notification(&app_handle, &event);
                            }
                        }
                    }
                    
                    thread::sleep(Duration::from_millis(500));
                }
            });
        }

        #[cfg(target_os = "linux")]
        {
            // On Linux, simulate events for demo purposes
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(5));
                
                if *running.lock().unwrap() {
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    
                    let demo_event = SecurityEvent {
                        event_type: "demo_event".to_string(),
                        severity: "low".to_string(),
                        message: "Clipboard monitoring is limited on Linux. Running in demo mode.".to_string(),
                        timestamp,
                    };
                    
                    let _ = app_handle.emit("security-event", &demo_event);
                }
            });
        }
    }

    fn check_security_patterns(content: &str) -> Option<SecurityEvent> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Check for API keys
        if API_KEY_PATTERN.is_match(content) {
            return Some(SecurityEvent {
                event_type: "api_key_detected".to_string(),
                severity: "high".to_string(),
                message: "Potential API key detected in clipboard!".to_string(),
                timestamp,
            });
        }

        // Check for AWS keys
        if AWS_KEY_PATTERN.is_match(content) {
            return Some(SecurityEvent {
                event_type: "aws_key_detected".to_string(),
                severity: "critical".to_string(),
                message: "AWS credentials detected in clipboard!".to_string(),
                timestamp,
            });
        }

        // Check for GitHub tokens
        if GITHUB_TOKEN_PATTERN.is_match(content) {
            return Some(SecurityEvent {
                event_type: "github_token_detected".to_string(),
                severity: "critical".to_string(),
                message: "GitHub token detected in clipboard!".to_string(),
                timestamp,
            });
        }

        // Check for passwords
        if PASSWORD_PATTERN.is_match(content) {
            return Some(SecurityEvent {
                event_type: "password_detected".to_string(),
                severity: "medium".to_string(),
                message: "Potential password detected in clipboard!".to_string(),
                timestamp,
            });
        }

        None
    }

    #[cfg(not(target_os = "linux"))]
    fn send_notification(app_handle: &AppHandle, event: &SecurityEvent) {
        use tauri_plugin_notification::NotificationExt;
        
        let _ = app_handle.notification()
            .builder()
            .title("🔒 Security Warning")
            .body(&event.message)
            .show();
    }

    #[cfg(target_os = "linux")]
    fn send_notification(_app_handle: &AppHandle, _event: &SecurityEvent) {
        // Notifications are not supported on Linux in this implementation
        // due to varying desktop environment requirements
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }
}

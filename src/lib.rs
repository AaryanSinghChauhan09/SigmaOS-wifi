//! SigmaOS-wifi — WiFi stack model.
//!
//! Models WiFi interfaces, scan results, authentication (WPA/WPA2),
//! connection management, and SSID/bssid handling. Deterministic
//! and std-only (no actual hardware access).

pub mod iface;
pub mod scan;
pub mod auth;
pub mod wpa;
pub mod connect;

pub use iface::{WifiInterface, WifiState};
pub use scan::{ScanResult, ScanManager};
pub use auth::{AuthMethod, AuthStatus};
pub use wpa::{WpaConfig, WpaSupplicant};
pub use connect::{Connection, ConnectionManager};

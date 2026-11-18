// SPDX-License-Identifier: GPL-3.0-or-later
//! SigmaOS-wifi — Wi-Fi device discovery, firmware lifecycle, interface control,
//! and link events. Inspired by NetworkManager/iwd/wpa_supplicant patterns.
//!
//! Design principles: single responsibility, fail-fast, defensive programming,
//! no unsafe code, deterministic unit-testable logic.

#![forbid(unsafe_code)]

use std::fmt;
use std::collections::HashMap;

// ── Error type ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WifiError {
    /// Interface name is empty or invalid
    InvalidInterface(String),
    /// SSID is empty or too long (>32 octets)
    InvalidSsid(String),
    /// No interface registered with that name
    InterfaceNotFound(String),
    /// Network not in scan results / known networks
    NetworkNotFound(String),
    /// Interface already connected
    AlreadyConnected(String),
    /// Interface not connected
    NotConnected(String),
    /// General driver/firmware error
    DriverError(String),
}

impl fmt::Display for WifiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInterface(s) => write!(f, "invalid interface: {s}"),
            Self::InvalidSsid(s)      => write!(f, "invalid SSID: {s}"),
            Self::InterfaceNotFound(s) => write!(f, "interface not found: {s}"),
            Self::NetworkNotFound(s)  => write!(f, "network not found: {s}"),
            Self::AlreadyConnected(s) => write!(f, "already connected on {s}"),
            Self::NotConnected(s)     => write!(f, "not connected on {s}"),
            Self::DriverError(s)      => write!(f, "driver error: {s}"),
        }
    }
}

impl std::error::Error for WifiError {}

// ── Types ─────────────────────────────────────────────────────────────────────

/// Security type of a Wi-Fi access point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Security {
    Open,
    Wep,
    Wpa2Personal,
    Wpa3Personal,
    Enterprise,
}

/// A Wi-Fi access point discovered during a scan.
#[derive(Debug, Clone)]
pub struct AccessPoint {
    pub ssid: String,
    pub bssid: String,
    pub signal_dbm: i16,
    pub channel: u8,
    pub security: Security,
}

impl AccessPoint {
    pub fn new(ssid: impl Into<String>, bssid: impl Into<String>,
               signal_dbm: i16, channel: u8, security: Security) -> Result<Self, WifiError> {
        let ssid = ssid.into();
        if ssid.is_empty() || ssid.len() > 32 {
            return Err(WifiError::InvalidSsid(ssid));
        }
        Ok(Self { ssid, bssid: bssid.into(), signal_dbm, channel, security })
    }

    /// Signal quality 0-100 derived from dBm.
    pub fn signal_quality(&self) -> u8 {
        // 2 * (dBm + 100) clamped to 0-100
        let q = 2i32 * (self.signal_dbm as i32 + 100);
        q.clamp(0, 100) as u8
    }
}

// ── Interface state ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceState {
    Down,
    Scanning,
    Disconnected,
    Connecting { ssid: String },
    Connected { ssid: String, ip: String },
    Failed { reason: String },
}

/// A managed Wi-Fi interface.
#[derive(Debug)]
pub struct WifiInterface {
    pub name: String,
    pub state: InterfaceState,
    pub scan_results: Vec<AccessPoint>,
    known_networks: HashMap<String, String>, // ssid -> psk
}

impl WifiInterface {
    pub fn new(name: impl Into<String>) -> Result<Self, WifiError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(WifiError::InvalidInterface(name));
        }
        Ok(Self {
            name,
            state: InterfaceState::Disconnected,
            scan_results: Vec::new(),
            known_networks: HashMap::new(),
        })
    }

    /// Begin a scan (simulated: populates scan_results passed in).
    pub fn scan(&mut self, aps: Vec<AccessPoint>) {
        self.state = InterfaceState::Scanning;
        self.scan_results = aps;
        self.state = InterfaceState::Disconnected;
    }

    /// Add a known network (pre-shared key).
    pub fn add_network(&mut self, ssid: impl Into<String>, psk: impl Into<String>)
        -> Result<(), WifiError> {
        let ssid = ssid.into();
        if ssid.is_empty() || ssid.len() > 32 {
            return Err(WifiError::InvalidSsid(ssid));
        }
        self.known_networks.insert(ssid, psk.into());
        Ok(())
    }

    /// Connect to a network (must be in scan results or known networks).
    pub fn connect(&mut self, ssid: &str, assigned_ip: &str) -> Result<(), WifiError> {
        if matches!(&self.state, InterfaceState::Connected { .. }) {
            return Err(WifiError::AlreadyConnected(self.name.clone()));
        }
        let found = self.scan_results.iter().any(|ap| ap.ssid == ssid)
            || self.known_networks.contains_key(ssid);
        if !found {
            return Err(WifiError::NetworkNotFound(ssid.to_string()));
        }
        self.state = InterfaceState::Connected {
            ssid: ssid.to_string(),
            ip: assigned_ip.to_string(),
        };
        Ok(())
    }

    /// Disconnect from current network.
    pub fn disconnect(&mut self) -> Result<(), WifiError> {
        if !matches!(&self.state, InterfaceState::Connected { .. }) {
            return Err(WifiError::NotConnected(self.name.clone()));
        }
        self.state = InterfaceState::Disconnected;
        Ok(())
    }

    /// Current connected SSID, if any.
    pub fn connected_ssid(&self) -> Option<&str> {
        match &self.state {
            InterfaceState::Connected { ssid, .. } => Some(ssid),
            _ => None,
        }
    }

    /// Current IP address, if connected.
    pub fn ip_address(&self) -> Option<&str> {
        match &self.state {
            InterfaceState::Connected { ip, .. } => Some(ip),
            _ => None,
        }
    }
}

// ── Manager ───────────────────────────────────────────────────────────────────

/// Manages multiple Wi-Fi interfaces on the system.
#[derive(Debug, Default)]
pub struct WifiManager {
    interfaces: HashMap<String, WifiInterface>,
}

impl WifiManager {
    pub fn new() -> Self { Self::default() }

    /// Register a new interface.
    pub fn add_interface(&mut self, iface: WifiInterface) {
        self.interfaces.insert(iface.name.clone(), iface);
    }

    /// Get interface by name.
    pub fn interface(&self, name: &str) -> Option<&WifiInterface> {
        self.interfaces.get(name)
    }

    /// Mutable interface access.
    pub fn interface_mut(&mut self, name: &str) -> Option<&mut WifiInterface> {
        self.interfaces.get_mut(name)
    }

    /// List all interface names.
    pub fn interface_names(&self) -> Vec<&str> {
        self.interfaces.keys().map(|s| s.as_str()).collect()
    }

    /// Connect an interface to a network.
    pub fn connect(&mut self, iface_name: &str, ssid: &str, ip: &str)
        -> Result<(), WifiError> {
        let iface = self.interfaces.get_mut(iface_name)
            .ok_or_else(|| WifiError::InterfaceNotFound(iface_name.to_string()))?;
        iface.connect(ssid, ip)
    }

    /// Disconnect an interface.
    pub fn disconnect(&mut self, iface_name: &str) -> Result<(), WifiError> {
        let iface = self.interfaces.get_mut(iface_name)
            .ok_or_else(|| WifiError::InterfaceNotFound(iface_name.to_string()))?;
        iface.disconnect()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ap(ssid: &str, dbm: i16) -> AccessPoint {
        AccessPoint::new(ssid, "aa:bb:cc:dd:ee:ff", dbm, 6, Security::Wpa2Personal).unwrap()
    }

    #[test]
    fn test_signal_quality() {
        let ap = make_ap("test", -50);
        assert_eq!(ap.signal_quality(), 100); // 2*(-50+100)=100
        let weak = make_ap("weak", -90);
        assert_eq!(weak.signal_quality(), 20); // 2*(-90+100)=20
    }

    #[test]
    fn test_invalid_ssid_rejected() {
        assert!(AccessPoint::new("", "aa:bb", -50, 1, Security::Open).is_err());
        let long = "a".repeat(33);
        assert!(AccessPoint::new(long, "aa:bb", -50, 1, Security::Open).is_err());
    }

    #[test]
    fn test_connect_and_disconnect() {
        let mut iface = WifiInterface::new("wlan0").unwrap();
        iface.scan(vec![make_ap("HomeNet", -60)]);
        iface.connect("HomeNet", "192.168.1.100").unwrap();
        assert_eq!(iface.connected_ssid(), Some("HomeNet"));
        assert_eq!(iface.ip_address(), Some("192.168.1.100"));
        iface.disconnect().unwrap();
        assert!(iface.connected_ssid().is_none());
    }

    #[test]
    fn test_double_connect_fails() {
        let mut iface = WifiInterface::new("wlan0").unwrap();
        iface.scan(vec![make_ap("Net", -55)]);
        iface.connect("Net", "10.0.0.1").unwrap();
        assert!(matches!(iface.connect("Net", "10.0.0.2"), Err(WifiError::AlreadyConnected(_))));
    }

    #[test]
    fn test_connect_unknown_network_fails() {
        let mut iface = WifiInterface::new("wlan0").unwrap();
        assert!(matches!(
            iface.connect("Unknown", "10.0.0.1"),
            Err(WifiError::NetworkNotFound(_))
        ));
    }

    #[test]
    fn test_disconnect_when_not_connected_fails() {
        let mut iface = WifiInterface::new("wlan0").unwrap();
        assert!(matches!(iface.disconnect(), Err(WifiError::NotConnected(_))));
    }

    #[test]
    fn test_known_network_connect() {
        let mut iface = WifiInterface::new("wlan0").unwrap();
        iface.add_network("SavedNet", "password123").unwrap();
        iface.connect("SavedNet", "172.16.0.5").unwrap();
        assert_eq!(iface.connected_ssid(), Some("SavedNet"));
    }

    #[test]
    fn test_manager_multi_interface() {
        let mut mgr = WifiManager::new();
        let mut iface = WifiInterface::new("wlan0").unwrap();
        iface.scan(vec![make_ap("RouterNet", -45)]);
        mgr.add_interface(iface);

        mgr.connect("wlan0", "RouterNet", "192.168.0.50").unwrap();
        assert_eq!(
            mgr.interface("wlan0").unwrap().connected_ssid(),
            Some("RouterNet")
        );

        mgr.disconnect("wlan0").unwrap();
        assert!(mgr.interface("wlan0").unwrap().connected_ssid().is_none());
    }

    #[test]
    fn test_invalid_interface_name() {
        assert!(WifiInterface::new("").is_err());
        assert!(WifiInterface::new("   ").is_err());
    }

    #[test]
    fn test_manager_missing_interface() {
        let mut mgr = WifiManager::new();
        assert!(matches!(
            mgr.connect("wlan99", "X", "1.1.1.1"),
            Err(WifiError::InterfaceNotFound(_))
        ));
    }
}

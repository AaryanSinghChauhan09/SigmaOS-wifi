//! WiFi scan model.

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub ssid: String,
    pub bssid: [u8; 6],
    pub signal_dbm: i16,
    pub channel: u32,
    pub security: String,
}

#[derive(Debug, Default)]
pub struct ScanManager {
    pub results: Vec<ScanResult>,
}

impl ScanManager {
    pub fn new() -> Self { Self::default() }
    pub fn add_result(&mut self, r: ScanResult) { self.results.push(r); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_manager() {
        let mut sm = ScanManager::new();
        sm.add_result(ScanResult { ssid: "TestNet".into(), bssid: [0; 6], signal_dbm: -50, channel: 6, security: "WPA2".into() });
        assert_eq!(sm.results.len(), 1);
    }
}

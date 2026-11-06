//! Connection management model.

#[derive(Debug, Clone)]
pub struct Connection {
    pub ssid: String,
    pub bssid: [u8; 6],
    pub ip: Option<[u8; 4]>,
}

#[derive(Debug, Default)]
pub struct ConnectionManager {
    pub connection: Option<Connection>,
}

impl ConnectionManager {
    pub fn new() -> Self { Self::default() }
    pub fn connect(&mut self, c: Connection) { self.connection = Some(c); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_manager() {
        let mut cm = ConnectionManager::new();
        cm.connect(Connection { ssid: "Test".into(), bssid: [0; 6], ip: Some([192, 168, 1, 100]) });
        assert!(cm.connection.is_some());
    }
}

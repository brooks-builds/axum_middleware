#[derive(Clone, Debug)]
pub struct AppState {
    pub address: [u8; 4],
    pub port: u16,
}

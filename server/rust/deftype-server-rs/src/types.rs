#[derive(Serialize, Deserialize, Debug)]
pub struct ServerTime {
    now: String,
}

impl ServerTime {
    pub fn new(now: String) -> Self {
        ServerTime { now: now }
    }
}

use serde::ser;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerTime {
    now: String,
}

impl ServerTime {
    pub fn new(now: String) -> Self {
        ServerTime { now: now }
    }
}

#[derive(Serialize, Debug)]
pub struct ResultDTO<T: ser::Serialize> {
    pub ok: bool,
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T: ser::Serialize> ResultDTO<T> {
    pub fn ok(data: T) -> Self {
        ResultDTO {
            ok: true,
            code: 0,
            data: data,
            message: "".to_owned(),
        }
    }

    pub fn err(data: T) -> Self {
        ResultDTO {
            ok: false,
            code: 1,
            data: data,
            message: "".to_owned(),
        }
    }

    #[allow(dead_code)]
    pub fn code(mut self, value: i32) -> Self {
        self.code = value;
        self
    }

    #[allow(dead_code)]
    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_owned();
        self
    }
}

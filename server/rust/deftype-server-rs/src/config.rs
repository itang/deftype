use std::fmt;
use std::env;
use std::str::FromStr;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum RunMode {
    Development,
    Production,
}

pub struct RunModeParseError;

static DEVELOPMENT: &'static str = "development";
static PRODUCTION: &'static str = "production";

impl RunMode {
    pub fn is_dev(&self) -> bool {
        *self == RunMode::Development
    }


    #[allow(dead_code)]
    pub fn is_prod(&self) -> bool {
        *self == RunMode::Production
    }

    pub fn to_str(&self) -> &str {
        match *self {
            RunMode::Development => DEVELOPMENT,
            RunMode::Production => PRODUCTION,
        }
    }
}

impl fmt::Display for RunMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}


impl FromStr for RunMode {
    type Err = RunModeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "dev" | "development" => Ok(RunMode::Development),
            "prod" | "production" => Ok(RunMode::Production),
            _ => Err(RunModeParseError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub root_dir: String,
    pub run_mode: RunMode,
}

impl ServerConfig {
    fn new(host: String, port: u16, root_dir: String, run_mode: RunMode) -> Self {
        ServerConfig {
            host: host,
            port: port,
            root_dir: root_dir,
            run_mode: run_mode,
        }
    }

    pub fn load_from_env() -> ServerConfig {
        let d = Self::default();
        let default_port = d.port; // just copy. happy for capture of partially moved value: `d`
        let default_run_mode = d.run_mode;
        let host = env::var("HOST").unwrap_or(d.host);
        let root_dir = env::var("ROOT_DIR").unwrap_or(d.root_dir);
        let port = env::var("PORT")
                       .map(|p| p.parse::<u16>().unwrap_or(default_port))
                       .unwrap_or(default_port);
        let run_mode = env::var("RUN_MODE")
                           .map(|m| m.parse::<RunMode>().unwrap_or(default_run_mode))
                           .unwrap_or(default_run_mode);
        Self::new(host, port, root_dir, run_mode)
    }

    pub fn to_addr(&self) -> (&str, u16) {
        (&self.host, self.port)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self::new("localhost".to_owned(),
                  3000,
                  "public/".to_owned(),
                  RunMode::Development)
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "ServerConfig(host:{} , port: {}, root_dir:{}, run_mode: {})",
               self.host,
               self.port,
               self.root_dir,
               self.run_mode)
    }
}

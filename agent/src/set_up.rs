// set-up config dir and read the config
use config::Config;
use serde::Deserialize;
use std::env;
use std::path::Path;
use tokio::fs;

pub struct ConfSetUp {
    pub conf_dir: String,
    jobs_dir: [&'static str; 2],
}

impl ConfSetUp {
    pub fn new() -> Self {
        Self {
            jobs_dir: ["Web", "HOST"],
            conf_dir: env::var("CONF_DIR").unwrap_or("/etc/ferroscope_agent".to_string()),
        }
    }

    pub async fn set_up(&self) {
        // checks if the path is exist or not if not then create all files and dir
        if self.check_dir().await {
            // dir is there then it's fine
            // in future will also check for the files
            println!("dir path is {}", self.conf_dir)
        } else {
            let parrent_dir = Path::new(&self.conf_dir);
            // no dir found need to create one
            if fs::create_dir_all(parrent_dir).await.is_err() {
                println!("Unable to create CONF dir at {}", self.conf_dir);
                std::process::exit(1)
            };

            for i in self.jobs_dir {
                let _ = fs::create_dir(parrent_dir.join(i)).await;
                // TODO: handel the erros
            }

            // create the base config file
            let bufer_data = br#"

agent_auth_token=""
agent_server_url=""
api_time_out=5
cpu_interval = 60  
uptime_interval=600
ram_interval = 60  
web_interval = 60  
host_interval = 60         
            "#;
            let _ = fs::write(parrent_dir.join("agent.toml"), bufer_data).await;
            println!(
                "Please set-up the config before starting, path => {}",
                self.conf_dir
            );
            std::process::exit(1)
        }
    }

    async fn check_dir(&self) -> bool {
        // returns true if there is a dir else false
        match fs::metadata(&self.conf_dir).await {
            Ok(data) => data.is_dir(),
            Err(_) => false,
        }
    }

    pub fn get_config(&self) -> Result<BaseConFig, config::ConfigError> {
        let parrent_dir = Path::new(&self.conf_dir);
        let c = Config::builder()
            .add_source(config::File::with_name(
                parrent_dir.join("agent.toml").to_str().unwrap(),
            ))
            .build()
            .unwrap();
        c.try_deserialize::<BaseConFig>()
    }
}

#[derive(Debug, Deserialize)]
pub struct BaseConFig {
    agent_auth_token: String,
    agent_server_url: String,
    api_time_out: u8,
    cpu_interval: u64,
    ram_interval: u64,
    web_interval: u64,
    host_interval: u64,
    uptime_interval: u64,
}

impl BaseConFig {
    pub fn get_api_time_out(&self) -> u64 {
        self.api_time_out as u64
    }
    pub fn get_server_url(&self) -> &str {
        &self.agent_server_url.trim_end_matches("/")
    }

    pub fn get_web_interval(&self) -> u64 {
        self.web_interval
    }

    pub fn get_service_url(&self) -> String {
        format!("{}/send_service", self.agent_server_url)
    }
    pub fn get_auth_token(&self) -> &str {
        &self.agent_auth_token
    }

    pub fn get_cpu_interval(&self) -> u64 {
        self.cpu_interval
    }

    pub fn get_ram_interval(&self) -> u64 {
        self.ram_interval
    }

    pub fn get_host_interval(&self) -> u64 {
        self.host_interval
    }

    pub fn get_uptime_interval(&self) -> u64 {
        self.uptime_interval
    }
}

use config::Config;
use std::{path::Path};
use tokio::fs;

pub (super) async  fn file_name_list(dir:&str)-> Result<Vec<String>, std::io::Error>{
    let dir_path = Path::new(dir);
    let mut entries = fs::read_dir(dir).await?;
    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        if entry.file_type().await?.is_file() && entry.file_name().to_string_lossy().to_string().ends_with(".toml") {
            files.push(
                dir_path.join(entry.file_name()
                      .to_string_lossy()
                      .to_string()).to_string_lossy().to_string()
            );
        }
    }
    Ok(files)
}

pub (super) async fn load_config(file:String)->Result<Config, config::ConfigError>{
    Config::builder().add_source(config::File::with_name(&file)).build()
    // let conf: Result<Host, config::ConfigError>=conf.try_deserialize();
    // conf
}
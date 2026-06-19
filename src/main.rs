use whoami::{distro, desktop_env, platform};
use discord_presence::Client;
use std::{fs, thread, time::Duration};
use serde::Deserialize;
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "discord-fetch",
    version = "0.2.0",
    about = "Like fastfetch, but for your Discord status",
    long_about = None
)]

struct Args {
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,
}

#[derive(Deserialize)]
struct Config {
    app_id: u64,
    large_image: Option<String>,
    large_text: Option<String>,
    platform_override: Option<String>,
    os_override: Option<String>,
    desktop_override: Option<String>,
    shell_override: Option<String>,
    separator: Option<String>,
}


fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_dir = match dirs::config_dir() {
        Some(path) => path,
        None => return Err("Failed to find the folder for configs".into())
    };
    let args = Args::parse();
    let config_path = match args.config {
        Some(custom_path) => std::path::PathBuf::from(custom_path),
        None => config_dir.join("discord-fetch").join("config.toml"),
    };
    match config_path.try_exists() {
        Ok(true) => {
            let contents = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&contents)?;
            Ok(config)
        }
        Ok(false) => {
            if let Some(parent) = config_path.parent() {
                if !parent.as_os_str().is_empty() && !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let template = r#"#app_id for discord
app_id = 123456789012345678
"#;
            fs::write(&config_path, template)?;
            return Err("Config created. Fill it out and restart the program.".into());
        }
        Err(e) => return Err(e.into()),
    }
}

fn main() {
    //load config
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {e}");
            std::process::exit(1);
        }
    };


    //Distro
    let distro = config.os_override
        .unwrap_or_else(||distro().unwrap_or_else(|_| "<unknown>".to_string()));
    //Platform
    let platform = config.platform_override
        .unwrap_or_else(||format!("{:?}", platform()));
    //env
    let desktop_env = config.desktop_override
        .unwrap_or_else(||desktop_env()
            .map(|e| e.to_string())
            .unwrap_or_else(|| "<unknown>".to_string()));
    //and shell
    let shell_name = config.shell_override
        .unwrap_or_else(||query_shell::get_shell_name()
            .unwrap_or_else(|_| "<unknown>".to_string()));
    
    let sep = config.separator
        .unwrap_or_else(|| "•".to_string());

    let large_image = config.large_image
        .unwrap_or_else(|| "logo".to_string());

    let large_text = config.large_text
        .unwrap_or_else(|| "OS logo".to_string());

    let full_detail = format!("{distro} {sep} {desktop_env} {sep} {shell_name}");

    let mut drpc = Client::new(config.app_id);

    drpc.on_ready(|_| println!("Discord is ready")).persist();

    drpc.start();
    
    ctrlc::set_handler(move || {
        println!("\n[discord-fetch] Exiting the program... Bye!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl+C handler");

    loop{
        let _ = drpc.set_activity(|act| {
            act
                .details(&platform)
                .state(&full_detail)
                .assets(|assets| {
                    assets
                        .large_image(&large_image)
                        .large_text(&large_text)
                })
        });
        thread::sleep(Duration::from_secs(15));
    }
}

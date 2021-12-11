mod controller;
mod driver;
mod conf;

use std::borrow::Borrow;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use clap::Parser;
use actix_web::{HttpServer, App, web, HttpResponse};
use log::LevelFilter;
use crate::controller::router::{api_router, admin_api_router};
use crate::conf::{AppConfig, LogConfig};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    // The path to the configuration file
    #[clap(short, long, parse(from_os_str), value_name = "PATH")]
    config: Option<PathBuf>,
    // Do an installation
    #[clap(short, long)]
    install: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cmd = Args::parse();
    // Read Configuration file
    if cmd.config.is_none() {
        eprintln!("No configuration file specified. Generated Config.toml for you.");
        eprintln!("Please edit the file and restart the program.");
        generate_config();
        exit(1);
    }
    let conf = cmd.config.unwrap();
    let conf = AppConfig::from_file(&conf).expect("Failed to load config");
    // Setup logger
    if  !setup_logger(conf.log.clone().borrow()) {
        eprintln!("Failed to setup logger");
        exit(1);
    }
    // Setup misc
    // Setup database connection
    let db = driver::db::new(conf.database.clone().borrow());

    // Setup cache
    // Setup global app state
    // If install is true, run the installation
    if cmd.install {
        println!("Installing...");
        exit(0);
    }
    // Start Server
    HttpServer::new(|| {
        App::new()
            .configure(configure)
    })
        .bind("0.0.0.0:8000")?
        .run()
        .await
}

fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/")
                .route(web::get().to(|| HttpResponse::Ok().body("Hello World!")))
        )
        .service(web::scope("/api")
            .configure(api_router)
            .configure(admin_api_router))
    ;
}

fn setup_logger(conf: &Option<LogConfig>) -> Result<(), fern::InitError> {
    let (access_log, error_log, level) = match conf {
        Some(conf) => { (conf.access_log.clone(), conf.error_log.clone(), conf.level.clone()) }
        None => { (None, None, None) }
    };
    let level = match level {
        Some(level) => {
            match level.to_lowercase().as_str() {
                "trace" => log::LevelFilter::Trace,
                "debug" => log::LevelFilter::Debug,
                "info" => log::LevelFilter::Info,
                "warn" => log::LevelFilter::Warn,
                "error" => log::LevelFilter::Error,
                "off" => log::LevelFilter::Off,
                _ => log::LevelFilter::Info
            }
        }
        None => { log::LevelFilter::Info }
    };
    let info_out = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level)
        .filter(|metadata| { metadata.level() <= log::LevelFilter::Info });
    match access_log {
        Some(path) => {
            let path = PathBuf::from(path);
            let file = fern::log_file(path)?;
            info_out.chain(file).apply()?;
        }
        None => {
            info_out.chain(std::io::stdout()).apply()?;
        }
    };
    match level {
        LevelFilter::Error | LevelFilter::Warn => {
            let error_out = fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                .level(level)
                .filter(|metadata| { metadata.level() <= log::LevelFilter::Warn });
            match error_log {
                Some(path) => {
                    let path = PathBuf::from(path);
                    let file = fern::log_file(path)?;
                    error_out.chain(file).apply()?;
                }
                None => {
                    error_out.chain(std::io::stderr()).apply()?;
                }
            };
        }
        _ => {}
    };
    Ok(())
}

fn generate_config() {
    let mut config = std::env::current_dir().unwrap();
    config.push("Config.toml");
    let mut file = std::fs::File::create(config).unwrap();
    file.write_all(include_bytes!("../layout/Config.toml")).unwrap();
}
mod controller;
mod driver;
mod conf;
mod model;

use std::borrow::Borrow;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use actix_session::CookieSession;
use clap::Parser;
use actix_web::{HttpServer, App, web, HttpResponse};
use log::{LevelFilter, info, warn, error};
use sqlx::{Any, Pool};
use crate::controller::router::{api_router, admin_api_router};
use crate::conf::{AppConfig, LogConfig};
use crate::driver::db::{DatabaseDriver};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    // The path to the configuration file
    #[clap(short, long, parse(from_os_str), value_name = "PATH", default_value = "Config.toml")]
    config: PathBuf,
    // Do an installation
    #[clap(short, long)]
    install: bool,
}

struct AppState {
    db: Pool<Any>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cmd: Args = Args::parse();
    // Read Configuration file
    let conf = cmd.config;
    // If the config file doesn't exist, generate one
    let conf_file = conf.as_path();
    if !conf_file.exists() {
        generate_config();
        exit(1);
    }
    let conf = AppConfig::from_file(&conf).expect("Failed to load config");
    // Setup logger
    if setup_logger(conf.log.clone().borrow()).is_err() {
        eprintln!("Failed to setup logger");
        exit(1);
    }
    // Setup misc
    // Setup database connection
    let db = driver::db::new_pool(conf.database.borrow()).await;
    if db.is_err() {
        eprintln!("Failed to connect to database");
        exit(1);
    }
    let db = db.unwrap();
    // Setup cache
    // Setup global app state
    // Start Server
    info!("Starting server at {}:{}", conf.host, conf.port);
    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .app_data(AppState { db: db.clone() })
            .configure(configure)
    })
        .bind((conf.host, conf.port))?
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
    let new_fern = || {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{} [{}] [{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d-%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
    };
    let info_out = new_fern()
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
            let error_out = new_fern()
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
    file.write_all(include_bytes!("../resources/Config.toml")).unwrap();
}
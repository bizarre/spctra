#![crate_name = "spctra"]

#[cfg(feature = "minecraft")]
pub mod minecraft;

pub mod core;

use actix_web::{ web, App, HttpResponse, HttpServer, Responder };
use std::env;
use std::fs;
use crate::core::backend::Backend;
use std::sync::{ Mutex, Arc };
use std::thread;
use std::time;

struct State<B: Backend> {
    pub backend: Arc<Mutex<B>>
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    let config_loc = args.get(1)
        .expect("Configuration file path not provided.");
    
    let _contents = fs::read_to_string(config_loc)
        .expect("Invalid or missing configuration file.");

    if cfg!(feature = "minecraft") {
        #[cfg(feature = "minecraft")]
        start_minecraft(_contents).await?;
    };

    Ok(())
}

async fn index<T : Backend>(data: web::Data<State<T>>) -> impl Responder {
    web::Json(data.backend.lock().unwrap().get_servers())
}

fn start_update_loop<B : Backend + std::marker::Send + 'static>(backend_arc: Arc<Mutex<B>>) {
    thread::spawn(move || {
        loop {
            thread::sleep(time::Duration::from_secs(1));
            
            let backend = backend_arc.lock().unwrap();
            let mut servers = backend.get_servers();

            for server in servers.iter() {
                let result = backend.fetch(server);
                if result.is_ok() {
                    println!("update clients...")
                } else {
                    println!("failed to get server snapshot...")
                }
            }
        }
    });
}

#[cfg(feature = "minecraft")]
async fn start_minecraft(config: String) -> std::io::Result<()> {
    let backend = Arc::new(Mutex::new(minecraft::init(config)));
    
    let state = web::Data::new(State::<minecraft::backend::MinecraftBackend> { 
        backend: Arc::clone(&backend)
    });

    start_update_loop(Arc::clone(&backend));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/servers", web::get().to(index::<minecraft::backend::MinecraftBackend>))
    })
    .bind("127.0.0.1:1337")?
    .run()
    .await
}

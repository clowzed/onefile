use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use futures_util::stream::StreamExt as _;

use log::{debug, error, info};
use std::{env, fs::File, io::Write};

struct KeyFileData {
    data: std::collections::HashMap<String, std::path::PathBuf>,
    upload_folder: std::path::PathBuf,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    info!("Reading env..,");

    debug!("Reading upload folder...");
    let upload_folder =
        std::path::PathBuf::from(std::env::var("UPLOAD_FOLDER").expect("UPLOAD_FOLDER to be set"));

    if !upload_folder.exists() {
        debug!("Creating upload folder...");
        std::fs::create_dir(&upload_folder).expect("Possibility to create upload folder");
    }

    debug!("Reading port...");
    let port = std::env::var("PORT").expect("PORT to be set");

    debug!("Creating server data...");
    let filekeys = actix_web::web::Data::new(std::sync::Mutex::new(KeyFileData {
        data: std::collections::HashMap::new(),
        upload_folder,
    }));

    debug!("Creating server...");
    HttpServer::new(move || {
        App::new()
            .app_data(filekeys.clone())
            .wrap(middleware::Logger::default())
            .service(upload_handler)
            .service(render_file_handler)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

#[actix_web::post("/upload")]
async fn upload_handler(
    mut file: actix_multipart::Multipart,
    data: actix_web::web::Data<std::sync::Mutex<KeyFileData>>,
) -> HttpResponse {
    debug!("Reading file from request...");
    if let Some(Ok(mut file)) = file.next().await {
        debug!("Ok. File exists!");
        debug!("Locking data...");

        if let Ok(mut key_data_file) = data.lock() {
            debug!("Ok. Data locked!");

            let new_key = uuid::Uuid::new_v4();
            info!("New key for file: {}", new_key.to_string());

            let path = key_data_file
                .upload_folder
                .join(format!("{}.file", new_key));

            debug!("New file path: {}", path.to_str().unwrap_or(""));
            debug!("Saving file...");

            if let Ok(mut f) = File::create(&path) {
                while let Some(Ok(chunk)) = file.next().await {
                    f.write(&chunk).unwrap(); // Trust
                }
                info!("File was successfully uploaded!");
            } else {
                error!("Failed to create file!");
                return HttpResponse::InternalServerError().body("Failed to write file!");
            }

            key_data_file.data.insert(new_key.to_string(), path);
            println!("{:?}", key_data_file.data);

            return HttpResponse::Ok().body(new_key.to_string());
        }
        return HttpResponse::InternalServerError().body("Failed to lock data");
    }
    HttpResponse::BadRequest().body("File was not provided!")
}

#[actix_web::get("/get/{key}")]
async fn render_file_handler(
    key: web::Path<String>,
    data: actix_web::web::Data<std::sync::Mutex<KeyFileData>>,
) -> HttpResponse {
    debug!("Locking data...");

    if let Ok(key_data_file) = data.lock() {
        debug!("Ok. Data locked!");
        let key = key.into_inner().clone();

        if key_data_file.data.contains_key(&key) {
            debug!("Key exists!");
            if let Ok(file) = std::fs::read_to_string(key_data_file.data.get(&key).unwrap()) {
                debug!("Sending file...");
                return HttpResponse::Ok().content_type("").body(file);
            } else {
                error!("Failed to open file reading...");
                return HttpResponse::InternalServerError().body("Faile to open file for reading!");
            }
        } else {
            error!("Key not found");
            return HttpResponse::NotFound().body("Key was not found!");
        }
    }
    error!("Failed to lock data!");
    return HttpResponse::InternalServerError().body("Failed to lock data!");
}

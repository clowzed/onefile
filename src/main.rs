use log::debug;

mod handlers;

struct KeyFileData {
    data: std::collections::HashMap<String, std::path::PathBuf>,
    upload_folder: std::path::PathBuf,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    debug!("Reading UPLOAD_FOLDER environment variable...");
    let upload_folder = std::path::PathBuf::from(
        std::env::var("UPLOAD_FOLDER").expect("environment variable UPLOAD_FOLDER should be set!"),
    );

    if !upload_folder.exists() {
        debug!("Creating upload folder: {:?}", &upload_folder);
        std::fs::create_dir(&upload_folder)?;
    }

    debug!("Reading PORT environment variable...");
    let port = std::env::var("PORT").expect("environment variable PORT should be set!");

    debug!("Creating server data...");
    let filekeys = actix_web::web::Data::new(std::sync::Mutex::new(KeyFileData {
        data: std::collections::HashMap::new(),
        upload_folder,
    }));

    debug!("Initializing server...");
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(filekeys.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(handlers::upload::upload_handler)
            .service(handlers::show::render_file_handler)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

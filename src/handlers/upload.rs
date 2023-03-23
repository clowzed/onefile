use futures_util::stream::StreamExt as _;

use log::{debug, error, info};
use std::{fs::File, io::Write};

#[actix_web::post("/upload")]
async fn upload_handler(
    mut file: actix_multipart::Multipart,
    data: actix_web::web::Data<std::sync::Mutex<crate::KeyFileData>>,
) -> actix_web::HttpResponse {
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

            match File::create(&path) {
                Ok(mut f) => {
                    while let Some(Ok(chunk)) = file.next().await {
                        f.write(&chunk).unwrap();
                    }

                    info!("File was successfully uploaded!");
                }
                Err(e) => {
                    error!("Failed to create file! Reason: {:?}", e);
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Failed to write file!");
                }
            }

            key_data_file.data.insert(new_key.to_string(), path);
            return actix_web::HttpResponse::Ok().body(new_key.to_string());
        }
        return actix_web::HttpResponse::InternalServerError().body("Failed to lock server data");
    }
    actix_web::HttpResponse::BadRequest().body("File was not provided!")
}

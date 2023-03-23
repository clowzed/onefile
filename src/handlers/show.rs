use log::{debug, error};


#[actix_web::get("/get/{key}")]
async fn render_file_handler(
    key: actix_web::web::Path<String>,
    data: actix_web::web::Data<std::sync::Mutex<crate::KeyFileData>>,
) -> actix_web::HttpResponse {
    debug!("Locking data...");

    if let Ok(key_data_file) = data.lock() {
        debug!("Ok. Data locked!");

        let key = key.into_inner().clone();
        debug!("Got key: {:?}", key);

        if key_data_file.data.contains_key(&key) {
            debug!("Key exists!");

            //? Using unwrap here is Ok beacause we have mutex on App data.
            //? Last line has checked the existance of key so nothing can happen
            //? The only error which could happen is deleting file

            // TODO add actix rt process scanning upload folder
            // TODO if file does not exist then delete ot from app data
            match std::fs::read_to_string(key_data_file.data.get(&key).unwrap()) {
                Ok(file) => {
                    debug!("Sending file...");
                    return actix_web::HttpResponse::Ok().content_type("").body(file);
                }
                Err(e) => {
                    error!("Failed to open file for reading! Reason: {:?}", e);
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Failed to open file for reading!");
                }
            }
        } else {
            error!("Key: {} was not found!", key);
            return actix_web::HttpResponse::NotFound().body("Key was not found!");
        }
    }
    error!("Failed to lock data!");
    return actix_web::HttpResponse::InternalServerError().body("Failed to lock data!");
}

use actix_web::{get, App, HttpServer, HttpResponse, Result};
use serde_json::json;

#[get("/")]
async fn read_file() -> Result<HttpResponse> {
    let file_contents = tokio::fs::read_to_string("a.txt")
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(json!({
        "file_contents": file_contents
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server on http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(read_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
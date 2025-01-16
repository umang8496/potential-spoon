use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use std::fs::File;
use std::io::Read;


#[get("/video")]
async fn stream_video() -> impl Responder {
    // Path to the video file
    let video_path = "videos/sample.mp4";
    // Open the video file
    match File::open(video_path) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            // Read the entire file into the buffer
            if let Err(err) = file.read_to_end(&mut buffer) {
                eprintln!("Failed to read video file: {}", err);
                return HttpResponse::InternalServerError().body("Failed to read video file");
            }
            // Return the video content with the correct Content-Type
            HttpResponse::Ok()
                .content_type("video/mp4")
                .body(buffer)
        }
        Err(err) => {
            eprintln!("Failed to open video file: {}", err);
            HttpResponse::NotFound().body("Video file not found")
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the server
    HttpServer::new(|| {
        App::new()
            .service(stream_video) // Register the /video endpoint
    })
    // .bind("127.0.0.1:8080")? // Bind to localhost and port 8080
    .bind("0.0.0.0:8080")? // Bind to localhost and port 8080
    .run()
    .await
}

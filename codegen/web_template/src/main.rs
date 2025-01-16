use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
// use async_trait::async_trait;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Weather {
    id: u64,
    name: String,
    description: String,
    temperature: f32,
}

struct AppState {
    client: Mutex<HttpClient>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = HttpClient::new();

    let data: web::Data<AppState> = web::Data::new(AppState {
        client: Mutex::new(client),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(data.clone())
            .route("/weather/{id}", web::get().to(get_weather))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_weather(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let client: std::sync::MutexGuard<HttpClient> = app_state.client.lock().unwrap();
    let response = client.get(&format!("http://api.weatherapi.com/v1/current.json?key=YOUR_API_KEY&q={}", id.into_inner()))
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let weather: Weather = res.json().await.unwrap();
                HttpResponse::Ok().json(weather)
            } else {
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

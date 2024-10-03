use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Define a struct for the cache entry
#[derive(Serialize, Deserialize)]
struct CacheEntry {
    key: String,
    value: String,
    tags: Vec<String>,
}

// Define a type alias for our cache
type Cache = Arc<Mutex<HashMap<String, CacheEntry>>>;

// Define a handler for storing cache data
async fn store_cache(data: web::Json<CacheEntry>, cache: web::Data<Cache>) -> impl Responder {
    let mut cache = cache.lock().unwrap();
    cache.insert(data.key.clone(), data.into_inner());
    HttpResponse::Ok().body("Data stored successfully")
}

// Define a handler for retrieving cache data
async fn get_cache(key: web::Path<String>, cache: web::Data<Cache>) -> impl Responder {
    let cache = cache.lock().unwrap();
    if let Some(entry) = cache.get(&key.into_inner()) {
        HttpResponse::Ok().json(entry)
    } else {
        HttpResponse::NotFound().body("Key not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize a new cache
    let cache: Cache = Arc::new(Mutex::new(HashMap::new()));

    // Start the HTTP server
    HttpServer::new(move || {
        let cache_clone = cache.clone();
        App::new()
            .app_data(web::Data::new(cache_clone))
            .route("/cache", web::post().to(store_cache)) // Handle POST requests
            .route("/cache/{key}", web::get().to(get_cache)) // Handle GET requests
    })
    .bind("127.0.0.1:5000")? // Bind to the address
    .run() // Run the server
    .await // Await completion
}

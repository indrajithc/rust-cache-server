use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use dashmap::DashMap;

// Define a struct for the cache entry
#[derive(Serialize, Deserialize, Clone)]
struct CacheEntry {
    key: String,
    value: String,
    tags: Vec<String>,
}

// Define a struct for the revalidate request
#[derive(Serialize, Deserialize)]
struct RevalidateRequest {
    tags: Vec<String>,
}

// Define a type alias for our cache
type Cache = Arc<DashMap<String, CacheEntry>>;

// Define a handler for storing cache data
async fn store_cache(data: web::Json<CacheEntry>, cache: web::Data<Cache>) -> impl Responder {
    cache.insert(data.key.clone(), data.into_inner());
    HttpResponse::Ok().body("Data stored successfully")
}

// Define a handler for retrieving cache data
async fn get_cache(key: web::Path<String>, cache: web::Data<Cache>) -> impl Responder {
    if let Some(entry) = cache.get(&key.into_inner()) {
        HttpResponse::Ok().json(entry.clone())
    } else {
        HttpResponse::NotFound().body("Key not found")
    }
}

// Define a handler for clearing cache by tags
async fn clear_cache(data: web::Json<RevalidateRequest>, cache: web::Data<Cache>) -> impl Responder {
    // Collect keys to remove based on tags
    let keys_to_remove: Vec<String> = cache
        .iter()
        .filter_map(|entry| {
            if entry.value().tags.iter().any(|tag| data.tags.contains(tag)) {
                Some(entry.key().clone())
            } else {
                None
            }
        })
        .collect();

    // Process removals in batches
    let batch_size = 100;
    for chunk in keys_to_remove.chunks(batch_size) {
        for key in chunk {
            cache.remove(key);
        }
        // Yield control to allow other tasks to run
        tokio::task::yield_now().await;
    }

    HttpResponse::Ok().body("Cache cleared successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize a new cache
    let cache: Cache = Arc::new(DashMap::new());

    // Start the HTTP server
    HttpServer::new(move || {
        let cache_clone = cache.clone();
        App::new()
            .app_data(web::Data::new(cache_clone))
            .route("/cache", web::post().to(store_cache)) // Handle POST requests for storing cache
            .route("/cache/{key}", web::get().to(get_cache)) // Handle GET requests for retrieving cache
            .route("/cache/revalidate", web::post().to(clear_cache)) // Handle POST requests for clearing cache by tags
    })
    .workers(8) // Increase the number of worker threads
    .bind("127.0.0.1:5000")? // Bind to the address
    .run() // Run the server
    .await // Await completion
}

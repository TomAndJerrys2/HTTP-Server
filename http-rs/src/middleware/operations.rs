use std::path::PathBuf;
use std::fs;
use std::path::Path;

// Actix crate for operations
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_multipart::Multipart;

// for parsing filenames easiers :)
use sanitize_filename::sanitize;

// CRUD Operations on the HTTP Server

// Upload data to the HTTP Server via the Protocol
// Using the HTTP Headers we can upload files to the
// Local Server for storage taking advantage of rusts
// crate actix which allows us to take advantage of the already
// written multi-part which is used as a payload

#[post("/upload")]
async fn upload_data(mut data: Multipart) -> impl Responder {
    while let Ok(Some(mut field)) = data.try_next().await {
        let content = field.content_disposition();
        let filename = sanitize(content.get_filename().unwrap());

        if filename.is_empty() {
            return 
        }
    }
}

#[delete("/{filename}")]
async fn delete_data(filename: web::Path<String>) -> Responder {

}

#[get("/download/{filename}")]
async fn download_data(filename: web::Path<String>) -> Responder {

}
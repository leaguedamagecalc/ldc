/*
 * File: champion.rs
 * Copyright: 2024, Alan Fung
 * Description: returns champion.json as an http response
 */
use actix_web::{HttpResponse, Responder};
use reqwest::Client;
use std::fs::{self, File};
use std::io::{Write};
use std::path::Path;

pub async fn fetch_champs() -> impl Responder {
    let cache_path = "champs_cache.json";
    if Path::new(cache_path).exists() {
        match fs::read_to_string(cache_path) {
            Ok(content) => {
                return HttpResponse::Ok().body(content);
            }
            Err(_) => {
                return HttpResponse::InternalServerError().body("Error reading cached file");
            }
        }
    }

    let url = "https://cdn.merakianalytics.com/riot/lol/resources/latest/en-US/champions.json";
    let client = Client::new();
    let response = client.get(url).send().await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let body = resp.text().await.unwrap_or_else(|_| String::from("Failed to read body"));
                match File::create(cache_path) {
                    Ok(mut file) => {
                        if let Err(_) = file.write_all(body.as_bytes()) {
                            return HttpResponse::InternalServerError().body("Failed to write to cache file");
                        }
                    }
                    Err(_) => {
                        return HttpResponse::InternalServerError().body("Failed to create cache file");
                    }
                }

                HttpResponse::Ok().body(body)
            } else {
                HttpResponse::InternalServerError().body("Failed to fetch data")
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Network error while fetching data")
        }
    }
}

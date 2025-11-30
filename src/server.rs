use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post, delete, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

use crate::hscraper::{HScraper, Holiday};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct HolidaysResponse {
    year: u32,
    holidays: Vec<Holiday>,
    count: usize,
}

pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Self {
        Server {
            host: host.to_string(),
            port,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let app = Router::new()
            .route("/health", get(health_check))
            .route("/holiday/:year", post(scrape_holiday))
            .route("/holiday/:year", get(get_holidays))
            .route("/holiday/:year", delete(delete_snapshot))
            .route("/holiday/:year", put(update_snapshot))
            .layer(CorsLayer::permissive());

        let addr: SocketAddr = self.address().parse()?;
        println!("🚀 Server starting at http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        message: "Service is healthy".to_string(),
        data: Some("OK".to_string()),
    })
}

// POST /holiday/{year} - Generate the scraped doc
async fn scrape_holiday(
    Path(year): Path<u32>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, Json<ApiResponse<String>>)> {
    let scraper = HScraper::new(year);

    // Check if snapshot already exists
    if scraper.snapshot_exists() {
        return Err((
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                message: format!("Snapshot for year {} already exists. Use PUT to update.", year),
                data: None,
            }),
        ));
    }

    match scraper.save_snapshot().await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Successfully scraped holidays for year {}", year),
            data: Some(format!("snapshots/holidays_{}.html", year)),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Failed to scrape holidays: {}", e),
                data: None,
            }),
        )),
    }
}

// GET /holiday/{year} - Parse and return the holidays
async fn get_holidays(
    Path(year): Path<u32>,
) -> Result<Json<ApiResponse<HolidaysResponse>>, (StatusCode, Json<ApiResponse<HolidaysResponse>>)> {
    let scraper = HScraper::new(year);

    // Check if snapshot exists
    if !scraper.snapshot_exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: format!("No snapshot found for year {}. Use POST to create one.", year),
                data: None,
            }),
        ));
    }

    // Load and parse the snapshot
    match scraper.load_snapshot() {
        Ok(html) => match scraper.parse_holidays_from_html(&html) {
            Ok(holidays) => {
                let count = holidays.len();
                Ok(Json(ApiResponse {
                    success: true,
                    message: format!("Successfully retrieved {} holidays for year {}", count, year),
                    data: Some(HolidaysResponse {
                        year,
                        holidays,
                        count,
                    }),
                }))
            }
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    message: format!("Failed to parse holidays: {}", e),
                    data: None,
                }),
            )),
        },
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Failed to load snapshot: {}", e),
                data: None,
            }),
        )),
    }
}

// DELETE /holiday/{year} - Delete the scraped doc
async fn delete_snapshot(
    Path(year): Path<u32>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, Json<ApiResponse<String>>)> {
    let scraper = HScraper::new(year);

    if !scraper.snapshot_exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: format!("No snapshot found for year {}", year),
                data: None,
            }),
        ));
    }

    match scraper.delete_snapshot() {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Successfully deleted snapshot for year {}", year),
            data: Some(format!("snapshots/holidays_{}.html", year)),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Failed to delete snapshot: {}", e),
                data: None,
            }),
        )),
    }
}

// PUT /holiday/{year} - Delete and regenerate the scraped doc
async fn update_snapshot(
    Path(year): Path<u32>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, Json<ApiResponse<String>>)> {
    let scraper = HScraper::new(year);

    // Delete existing snapshot if it exists
    if scraper.snapshot_exists() {
        if let Err(e) = scraper.delete_snapshot() {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    message: format!("Failed to delete existing snapshot: {}", e),
                    data: None,
                }),
            ));
        }
    }

    // Create new snapshot
    match scraper.save_snapshot().await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            message: format!("Successfully updated snapshot for year {}", year),
            data: Some(format!("snapshots/holidays_{}.html", year)),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Failed to update snapshot: {}", e),
                data: None,
            }),
        )),
    }
}
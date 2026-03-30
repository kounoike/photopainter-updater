mod app;
mod config;
mod image_pipeline;
mod logging;
mod response;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run().await
}

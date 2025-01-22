pub mod routes;
pub mod error;

use poem::{listener::TcpListener, web::Query, EndpointExt, Route};
use poem_openapi::{OpenApiService, OpenApi};
use db::Db;
use dotenv::dotenv;

struct Api;

#[derive(Clone)]
struct AppState {
    db: Db
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = Db::new().await;
    let api_service = OpenApiService::new(Api, "End user api", "1.0").server("http://localhost:3000");

    let state = AppState {
        db
    };

    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api/v1", api_service).nest("/docs", ui).data(state);
    
    let _ = poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;
}

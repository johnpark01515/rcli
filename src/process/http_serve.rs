use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Debug, Clone)]
struct PathState {
    path: PathBuf,
}

pub async fn http_process_serve(path: PathBuf, port: u16) -> Result<()> {
    info!("{:?},{}", path, port);
    let service = ServeDir::new(path.clone());
    let state = Arc::new(PathState { path });

    let app = Router::new()
        .route("/*path", get(file_handler))
        .nest_service(
            "/tower/",
            service
                .precompressed_br()
                .precompressed_deflate()
                .precompressed_gzip()
                .precompressed_zstd(),
        )
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<PathState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let path = std::path::Path::new(&state.path).join(path);
    if !path.exists() {
        (StatusCode::NOT_FOUND, format!("File:{:?} not exist", path))
    } else {
        match tokio::fs::read_to_string(path).await {
            Ok(res) => (StatusCode::OK, res),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(PathState {
            path: PathBuf::from("."),
        });
        let path = Path("Cargo.toml".to_string());
        let (code, res) = file_handler(State(state), path).await;
        assert_eq!(code, StatusCode::OK);
        assert!(res.contains("[dependencies]"))
    }
}

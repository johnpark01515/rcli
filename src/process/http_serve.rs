use anyhow::Result;
use axum::{
    body::Body,
    extract::{Path, State},
    response::{Html, IntoResponse, Response},
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
) -> Response<Body> {
    let path = std::path::Path::new(&state.path).join(path);
    if !path.exists() {
        format!("File:{:?} not exist", path).into_response()
    } else if path.is_dir() {
        let before: &'static str = r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>My Directory Index</title>
            </head>
            <body>
                <h3>Welcome to fixture Directory</h3>
                <ul>"#;
        let after = r#"
                </ul>
            </body>
            </html>
            "#;
        let res = find_path(path.clone())
            .unwrap()
            .replace(path.as_path().to_str().unwrap(), "");
        Html(before.to_string() + &res + after).into_response()
    } else {
        match tokio::fs::read_to_string(path).await {
            Ok(res) => res.into_response(),
            Err(e) => e.to_string().into_response(),
        }
    }
}

fn find_path(path: PathBuf) -> Result<String> {
    let mut res = String::new();
    if path.is_dir() {
        for entry in std::fs::read_dir(path.to_str().unwrap())? {
            let entry = entry?;
            let new_path = entry.path();
            if new_path.is_dir() {
                let p = find_path(new_path)?;
                res.push_str(&p);
            } else {
                res.push_str(
                    format!(
                        "<li><a href=\"{}\">{}</a></li>\n",
                        new_path.clone().to_str().unwrap(),
                        new_path.to_str().unwrap()
                    )
                    .as_str(),
                );
            }
        }
    } else {
        let path = path.to_str().unwrap();
        res.push_str(format!("<li><a href=\"{}\">{}</a></li>\n", path, path).as_str());
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(PathState {
            path: PathBuf::from("."),
        });
        let path = Path("Cargo.toml".to_string());
        let res = file_handler(State(state), path).await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}

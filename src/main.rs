use axum::{http::{HeaderValue, Method}, response::{Html, IntoResponse}, routing::{get, post}, Json, Router};
use executor::task_executor::TaskExecutor;
use serde::Serialize;
use streaming::task_stream::stream_all_tasks;
use tasks::prime_calculator::handler::create_prime_task;
use tower_http::services::ServeDir;
use tower_http::cors::{CorsLayer, Any};
use std::{net::SocketAddr, sync::Arc};

pub mod types;
pub mod tasks;
pub mod executor;
pub mod api;
pub mod streaming;
pub mod visualisation;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}


fn say_hello() -> String {
    "Hello, world!".to_string()
}

async fn health() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: say_hello(),
    })
}

#[tokio::main]
async fn main() {
    let task_executor = Arc::new(TaskExecutor::new());

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/prime", post(create_prime_task))
        .route("/api/stream", get(stream_all_tasks))
        .route("/visual-test", get(serve_visual_test))
        .nest_service("/", ServeDir::new("static"))
        .layer(cors)
        .with_state(task_executor);

    let addr = SocketAddr::from(([0, 0, 0, 0], 5001));

    println!("Server running on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .unwrap(),
        app
    )
    .await
    .unwrap();
        
}

async fn serve_visual_test() -> impl IntoResponse {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Spiral Visualization</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gray-900 p-8">
    <div class="flex justify-center">
        <div class="w-[800px] h-[600px] bg-gray-800 relative" id="spiralCanvas">
            <h2 class="text-white text-center">Spiral Mapping</h2>
        </div>
    </div>

    <script>
        function createPoint(x, y, hue) {
            const point = document.createElement('div');
            point.style.position = 'absolute';
            point.style.width = '4px';
            point.style.height = '4px';
            point.style.backgroundColor = `hsl(${hue}, 70%, 50%)`;
            point.style.borderRadius = '50%';
            point.style.left = `${x}px`;
            point.style.top = `${y}px`;
            point.style.opacity = '1';
            return point;
        }

        const spiralCanvas = document.getElementById('spiralCanvas');
        let currentNumber = 0;

        function spiralMapping(value) {
            const phi = (1 + Math.sqrt(5)) / 2;
            const angle = 2 * Math.PI * value / phi;
            const radius = Math.sqrt(value) * 2;
            const x = 400 + radius * Math.cos(angle);
            const y = 300 + radius * Math.sin(angle);
            return [x, y];
        }

        function addPoint() {
            const [x, y] = spiralMapping(currentNumber);
            const point = createPoint(x, y, currentNumber % 360);
            spiralCanvas.appendChild(point);
            
            // Fade out and remove after 2 seconds
            setTimeout(() => {
                let opacity = 1;
                const fadeInterval = setInterval(() => {
                    opacity -= 0.1;
                    point.style.opacity = opacity;
                    if (opacity <= 0) {
                        clearInterval(fadeInterval);
                        point.remove();
                    }
                }, 100);
            }, 2000);

            currentNumber++;
        }

        // Add points every 100ms
        setInterval(addPoint, 100);
    </script>
</body>
</html>
    "#)
}

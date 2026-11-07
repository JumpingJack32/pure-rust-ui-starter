use app::app::App;
use axum::Router;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use leptos_meta::*;
use sqlx::SqlitePool;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // 1. Create the in-memory SQLite database pool
    let db_pool = SqlitePool::connect("sqlite::memory:").await?;

    // 2. Run initial schema migration
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL
        )
        "#,
    )
    .execute(&db_pool)
    .await?;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    // 3. Use leptos_routes_with_context to provide db_pool to server functions
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let db_pool = db_pool.clone();
                move || {
                    provide_context(db_pool.clone());
                }
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback_service(ServeDir::new(&*leptos_options.site_root))
        .with_state(leptos_options); // State remains LeptosOptions as required by leptos_routes

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[cfg(any(feature = "hydrate", feature = "ssr"))]
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use worker::*;

#[cfg(any(feature = "hydrate", feature = "ssr"))]
use crate::app::*;

mod api;
mod app;
mod components;
mod utils;

#[cfg(feature = "ssr")]
mod serve_static;

#[cfg(feature = "ssr")]
fn router(env: worker::Env) -> axum::Router {
    use std::sync::Arc;

    use axum::{Extension, routing::post};
    use leptos_axum::{LeptosRoutes, generate_route_list};

    use crate::api::register_server_functions;

    // Match what's in Cargo.toml
    // Doesn't seem to be able to do this automatically
    let mut leptos_options = leptos::config::get_configuration(None)
        .unwrap()
        .leptos_options;
    leptos_options.output_name = "cloudflare_r2_bucket_explorer".into();
    leptos_options.site_root = "public".into();
    leptos_options.not_found_path = "404".into();
    leptos_options.site_addr = "127.0.0.1:8787".parse().unwrap();

    let routes = generate_route_list(|| view! { <App /> });

    register_server_functions();

    let env_cloned = env.clone();
    // build our application with a route
    axum::Router::new()
        .route("/api/{*fn_name}", post(leptos_axum::handle_server_fns))
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(env_cloned.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(serve_static::serve_static)
        .with_state(leptos_options)
        .layer(Extension(Arc::new(env)))
}

#[cfg(feature = "ssr")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
fn start() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
}

#[cfg(feature = "ssr")]
#[event(fetch)]
async fn fetch(
    request: HttpRequest,
    environment: worker::Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    use tower_service::Service;

    let path = request.uri().path();
    let path =
        urlencoding::decode(path).map_err(|err| worker::Error::RustError(err.to_string()))?;
    let key_prefix = path.trim_start_matches('/');
    let readable_key_prefix = if key_prefix.is_empty() {
        "/"
    } else {
        key_prefix
    };

    if readable_key_prefix.starts_with("api/") || readable_key_prefix.ends_with('/') {
        Ok(router(environment).call(request).await?)
    } else {
        let bucket: Bucket = environment.bucket("BUCKET")?;
        match bucket.get(key_prefix).execute().await? {
            Some(object) => Ok(axum::response::Response::builder()
                .status(axum::http::StatusCode::OK)
                .header("content-type", "application/octet-stream")
                .body(axum::body::Body::from_stream(
                    send_wrapper::SendWrapper::new(
                        object.body().expect("must be available").stream()?,
                    ),
                ))?),
            None => Ok(axum::response::Response::builder()
                .status(axum::http::StatusCode::NOT_FOUND)
                .body(axum::body::Body::empty())?),
        }
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(|| view! { <App/> });
}

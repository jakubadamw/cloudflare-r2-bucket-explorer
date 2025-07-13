pub mod list_directory_contents;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    use leptos::server_fn::axum::register_explicit;

    register_explicit::<list_directory_contents::ListDirectoryContents>();
}

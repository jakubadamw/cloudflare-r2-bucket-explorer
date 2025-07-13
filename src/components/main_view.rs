use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::components::root_directory_view::RootDirectoryView;

#[component]
pub fn MainView() -> impl IntoView {
    let location: leptos_router::location::Location = use_location();
    view! {
        <header>
            <h1>{location.pathname.get()}</h1>
        </header>
        <main>
            <RootDirectoryView path=Signal::derive(move || location.pathname.get()) />
        </main>
    }
}

use std::collections::BTreeSet;

use leptos::prelude::*;

use crate::{
    api::list_directory_contents::list_directory_contents, components::entry_view::EntryView,
};

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum DirectoryViewState {
    Unexpanded,
    Loading,
    Loaded,
    Error,
}

impl DirectoryViewState {
    pub(crate) fn to_emoji(self) -> &'static str {
        match self {
            Self::Unexpanded => "📁",
            Self::Loading => "⏳",
            Self::Loaded => "📂",
            Self::Error => "❌",
        }
    }
}

#[component]
pub fn DirectoryView(
    #[prop(into)] path: Signal<String>,
    depth: usize,
    #[prop(into)] expanded_keys: RwSignal<BTreeSet<String>>,
    #[prop(into)] state: RwSignal<DirectoryViewState>,
) -> impl IntoView {
    let entries = Resource::new(
        move || path,
        move |path| async move {
            if state.get() == DirectoryViewState::Unexpanded {
                state.set(DirectoryViewState::Loading);
            }
            let result =
                list_directory_contents(path.get().trim_start_matches('/').to_owned()).await;
            if result.is_ok() {
                state.set(DirectoryViewState::Loaded);
            } else {
                state.set(DirectoryViewState::Error);
            }
            result
        },
    );

    view! {
        <Suspense
            fallback=move || ()
        >
            {move || Suspend::new(async move {
                match entries.await {
                    Ok(entries) => entries
                        .into_iter()
                        .map(|entry| view! { <EntryView entry=entry depth=depth expanded_keys=expanded_keys /> }.into_any())
                        .collect_view()
                        .into_any(),
                    Err(_err) => {
                        ().into_any()
                    }
                }
            })}
        </Suspense>
    }
}

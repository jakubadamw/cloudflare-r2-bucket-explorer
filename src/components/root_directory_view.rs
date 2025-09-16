use std::collections::BTreeSet;

use leptos::prelude::*;
use leptos_router::{
    NavigateOptions,
    hooks::{use_navigate, use_query_map},
};

use crate::components::directory_view::{DirectoryView, DirectoryViewState};

fn hash_set_to_state_string(expanded_keys: &BTreeSet<String>) -> String {
    use base64::Engine;
    use itertools::Itertools;

    base64::engine::general_purpose::STANDARD
        .encode(expanded_keys.iter().sorted().join("|").as_bytes())
}

fn state_string_to_hash_set(state_string: &str) -> BTreeSet<String> {
    use base64::Engine;

    String::from_utf8(
        base64::engine::general_purpose::STANDARD
            .decode(state_string.as_bytes())
            .unwrap_or_default(),
    )
    .unwrap_or_default()
    .split('|')
    .map(str::to_string)
    .collect()
}

#[component]
pub fn RootDirectoryView(#[prop(into)] path: Signal<String>) -> impl IntoView {
    let navigate = use_navigate();
    let query_map = use_query_map();

    let expanded_keys = RwSignal::new(
        query_map
            .read_untracked()
            .get("state")
            .map(|value| state_string_to_hash_set(value.as_str()))
            .unwrap_or_default(),
    );

    Effect::new(move |_| {
        let state_string = hash_set_to_state_string(&expanded_keys.get());
        let new_url = if state_string.is_empty() {
            "".to_owned()
        } else {
            format!("?state={state_string}")
        };
        navigate(&new_url, NavigateOptions::default());
    });

    Effect::new(move |_| {
        expanded_keys.maybe_update(|expanded_keys| {
            let new_expanded_keys = query_map
                .get()
                .get("state")
                .map(|value| state_string_to_hash_set(value.as_str()))
                .unwrap_or_default();
            let changed = new_expanded_keys == *expanded_keys;
            *expanded_keys = new_expanded_keys;
            changed
        });
    });

    view! {
        <table>
            <thead>
                <tr>
                    <th>"Name"</th>
                    <th>"Size"</th>
                    <th>"Uploaded"</th>
                </tr>
            </thead>
            <tbody>
                {
                    path.get().trim_matches('/').rsplit_once('/').map(|(parent_key, _)| parent_key.to_owned()).map(|parent_key| {
                        view! {
                            <tr>
                                <td colspan="3">"📁 "<a rel="external" href=move || format!("/{parent_key}/")>"../"</a></td>
                            </tr>
                        }.into_any()
                    })
                }
            </tbody>
            <DirectoryView path=path depth=0 expanded_keys=expanded_keys state=RwSignal::new(DirectoryViewState::Unexpanded) />
        </table>
    }
}

use std::{collections::BTreeSet, path::Path};

use leptos::prelude::*;

use crate::{
    api::list_directory_contents::{Entry, EntryType},
    components::directory_view::{DirectoryView, DirectoryViewState},
    utils::emoji::get_file_emoji,
};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[component]
pub fn EntryView(
    entry: Entry,
    depth: usize,
    #[prop(into)] expanded_keys: RwSignal<BTreeSet<String>>,
) -> impl IntoView {
    let file_size_format_options =
        humansize::FormatSizeOptions::from(humansize::DECIMAL).decimal_places(2);

    let Entry { key, r#type } = entry;
    let relative_key = key
        .trim_end_matches('/')
        .rsplit_once('/')
        .expect("must be splittable")
        .1
        .to_owned();
    let key_cloned: String = key.clone();
    let key_cloned_cloned = key.clone();
    let key_cloned_cloned_cloned = key.clone();
    let extension = Path::new(&key)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_owned();

    let expanded = Signal::derive(move || {
        expanded_keys
            .try_get()
            .is_some_and(|keys| keys.contains(&key))
    });

    let state = RwSignal::new(DirectoryViewState::Unexpanded);

    view! {
        <tr>
            {
                match r#type {
                    EntryType::File { size, uploaded } => view! {
                        <td data-depth=depth>{get_file_emoji(&extension)}" "<a href=move || format!("/{key_cloned}") download>{relative_key}</a></td>
                        <td>{humansize::format_size(size, file_size_format_options)}</td>
                        <td>{uploaded.format(DATE_FORMAT).to_string()}</td>
                    }.into_any(),
                    EntryType::Directory => {
                        view! {
                            <td colspan="3" data-depth=depth>
                                {move || state.get().to_emoji()}" "<a on:click=move |event| {
                                    expanded_keys.update(|expanded| {
                                        if expanded.contains(&key_cloned) {
                                            expanded.remove(&key_cloned);
                                        } else {
                                            expanded.insert(key_cloned.clone());
                                        }
                                    });
                                    event.prevent_default();
                                } href=move || format!("/{key_cloned_cloned_cloned}")>{relative_key}</a>
                            </td>
                        }.into_any()
                    }
                }
            }
        </tr>
        <Show
            when=move || { expanded.try_get().unwrap_or_default() }
        >
            <DirectoryView path=key_cloned_cloned.clone() depth=depth + 1 expanded_keys=expanded_keys state=state />
        </Show>
    }
}

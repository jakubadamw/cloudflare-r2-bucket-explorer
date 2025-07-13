use leptos::prelude::*;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum EntryType {
    Directory,
    File {
        size: u32,
        uploaded: chrono::DateTime<chrono::Utc>,
    },
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Entry {
    pub(crate) key: String,
    pub(crate) r#type: EntryType,
}

#[server(ListDirectoryContents)]
pub async fn list_directory_contents(prefix: String) -> Result<Vec<Entry>, ServerFnError> {
    use log::debug;

    debug!("Reading path: {prefix}");

    let environment = expect_context::<worker::Env>();
    let bucket = send_wrapper::SendWrapper::new(environment.bucket("BUCKET")?);

    let list_response =
        send_wrapper::SendWrapper::new(bucket.list().delimiter("/").prefix(&prefix).execute())
            .await?;

    let entries: Vec<_> = list_response
        .delimited_prefixes()
        .into_iter()
        .map(|key| Entry {
            key: key.to_owned(),
            r#type: EntryType::Directory,
        })
        .chain(list_response.objects().into_iter().map(|object| {
            Entry {
                key: object.key().to_string(),
                r#type: EntryType::File {
                    size: object.size() as u32,
                    uploaded: chrono::DateTime::<chrono::Utc>::from_timestamp_millis(
                        object.uploaded().as_millis() as i64,
                    )
                    .expect("must be valid"),
                },
            }
        }))
        .collect();

    debug!("Read {} entries.", entries.len());

    Ok(entries)
}

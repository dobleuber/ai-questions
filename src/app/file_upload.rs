use leptos::prelude::*;
use leptos::task::spawn_local;
use server_fn::codec::{MultipartData, MultipartFormData};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

#[server(input = MultipartFormData)]
pub async fn upload_file(data: MultipartData) -> Result<String, ServerFnError> {
    let mut data = data.into_inner().unwrap();
    let mut total_size = 0;

    while let Ok(Some(mut field)) = data.next_field().await {
        let name = field.file_name().unwrap_or_default().to_string();
        println!("Processing file: {}", name);

        while let Ok(Some(chunk)) = field.chunk().await {
            total_size += chunk.len();
        }
    }

    Ok(format!(
        "Archivo subido exitosamente. Tamaño total: {} bytes",
        total_size
    ))
}

#[component]
pub fn FileUpload() -> impl IntoView {
    let (upload_status, set_upload_status) = signal(String::new());

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let form = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        let form_data = FormData::new_with_form(&form).unwrap();

        spawn_local(async move {
            match upload_file(form_data.into()).await {
                Ok(message) => set_upload_status.set(message),
                Err(e) => set_upload_status.set(format!("Error al subir el archivo: {}", e)),
            }
        });
    };

    view! {
        <div class="file-upload-section">
            <h2>"Subir Archivo"</h2>
            <form on:submit=on_submit>
                <input
                    type="file"
                    name="file"
                    id="file"
                    required
                />
                <button type="submit">"Subir"</button>
            </form>
            <p class="upload-status">{upload_status}</p>
        </div>
    }
}

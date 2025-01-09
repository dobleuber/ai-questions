use leptos::prelude::*;

use crate::app::file_upload::FileUpload;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Hola mundo"</h1>

        <div class="file-upload-section">
            <FileUpload />
        </div>
    }
}

use axum::response::IntoResponse;

pub(crate) async fn list_chat_handler() -> impl IntoResponse {
    "List chat"
}

pub(crate) async fn create_chat_handler() -> impl IntoResponse {
    "Create chat"
}

pub(crate) async fn update_chat_handler() -> impl IntoResponse {
    "Update chat"
}

pub(crate) async fn delete_chat_handler() -> impl IntoResponse {
    "Delete chat"
}

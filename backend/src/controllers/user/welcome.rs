use axum::response::Html;

pub async fn index() -> Html<&'static str> {
    Html("<h1>Hello, User!</h1>")
}

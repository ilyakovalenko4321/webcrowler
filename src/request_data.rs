pub async fn request_data(link: &str) -> String {
    let body = reqwest::get(link)
        .await.unwrap()
        .text()
        .await.unwrap();

    body
}
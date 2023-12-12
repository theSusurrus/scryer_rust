pub async fn query(uri: String) -> String {
    let response = reqwest::get(uri).await.unwrap().text().await.unwrap();
    return response;
}

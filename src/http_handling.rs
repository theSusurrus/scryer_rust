use futures::executor::block_on;

pub fn get_http(uri: String) -> String {
    let fut_response = async {
        reqwest::get(uri).await.unwrap().text().await.unwrap()
    };

    let response = block_on(fut_response);

    return response;
}

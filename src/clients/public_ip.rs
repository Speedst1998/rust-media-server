pub async fn get_public_ip() -> String {
    let result = reqwest::get("http://whatismyip.akamai.com/")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    result
}

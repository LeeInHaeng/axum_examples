pub async fn request_with_retry(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client
        ::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    loop {
        let response = client.get(url).send().await?;
        if response.status().is_success() {
            return Ok(response.text().await?);
        }
    }
}
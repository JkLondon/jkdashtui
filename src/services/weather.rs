use crate::Result;

pub async fn get_weather(client: &reqwest::Client) -> Result<String> {
    let resp = client
    .get("https://wttr.in/Gijon?format=4")
    .send()
    .await?
    .text()
    .await?;

    Ok(resp)
}
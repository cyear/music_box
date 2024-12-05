use error_chain::error_chain;
use bytes::Bytes;

error_chain! {
         foreign_links {
             Io(std::io::Error);
             HttpRequest(reqwest::Error);
         }
    }
pub async fn get(target: &str) -> Result<Bytes> {
    let response = reqwest::get(target).await?;
    let content: Bytes = response.bytes().await?;
    Ok(content)
}
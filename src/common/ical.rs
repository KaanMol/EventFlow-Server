use crate::handlers::error::ResourceError;

pub async fn read_ical_uri(
    source_uri: impl Into<String>,
) -> Result<Vec<crate::entity::event::EventEntity>, ResourceError> {
    // Request source
    let client = reqwest::Client::new();
    let ical_body = client
        .get(&source_uri.into())
        .send()
        .await
        .map_err(|e| ResourceError::NetworkError)?
        .text()
        .await
        .map_err(|e| ResourceError::FailedParse("Failed to read source response".to_string()))?;

    let events = parse_ical(ical_body).await?;

    Ok(events)
}

async fn parse_ical(
    ical_body: String,
) -> Result<Vec<crate::entity::event::EventEntity>, ResourceError> {
    todo!()
}

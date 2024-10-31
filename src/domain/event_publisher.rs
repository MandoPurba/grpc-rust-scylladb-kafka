use super::models::Note;

#[async_trait::async_trait]
pub trait EventPublisher: Send + Sync + 'static {
    async fn publish_note_created(&self, note: &Note) -> anyhow::Result<()>;
}

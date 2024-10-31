use super::models::Note;

#[async_trait::async_trait]
pub trait NoteRepository: Send + Sync + 'static {
    async fn save(&self, note: &Note) -> anyhow::Result<()>;
    async fn list_all(&self) -> anyhow::Result<Vec<Note>>;
}

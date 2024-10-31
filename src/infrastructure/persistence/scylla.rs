use anyhow::Ok;
use async_trait::async_trait;
use futures::TryStreamExt;
use scylla::{Session, SessionBuilder};
use uuid::Uuid;

use crate::domain::{models::Note, repository::NoteRepository};

pub struct ScyllaRepository {
    session: Session,
}

impl ScyllaRepository {
    pub async fn new(uri: &str) -> anyhow::Result<Self> {
        let session = SessionBuilder::new().known_node(uri).build().await?;

        // Create keyspace and table
        session.query_unpaged("CREATE KEYSPACE IF NOT EXISTS notes_service WITH REPLICATION = {'class': 'SimpleStrategy', 'replication_factor': 1}",&[]).await?;
        session.query_unpaged("CREATE TABLE IF NOT EXISTS notes_service.notes (id uuid PRIMARY KEY, text text)", &[]).await?;

        Ok(Self { session })
    }
}

#[async_trait]
impl NoteRepository for ScyllaRepository {
    async fn save(&self, note: &Note) -> anyhow::Result<()> {
        self.session
            .query_unpaged(
                "INSERT INTO notes_service.notes (id, text) VALUES (?, ?)",
                (note.id, note.text.clone()),
            )
            .await?;
        Ok(())
    }

    async fn list_all(&self) -> anyhow::Result<Vec<Note>> {
        let mut iter = self
            .session
            .query_iter("SELECT id, text FROM notes_service.notes", &[])
            .await?
            .into_typed::<(Uuid, String)>(); // Sesuaikan tipe ini dengan tipe data yang Anda gunakan

        let mut notes = Vec::new();

        while let Some((id, text)) = iter.try_next().await? {
            notes.push(Note {
                id,
                text
            });
        }

        Ok(notes)
    }
}

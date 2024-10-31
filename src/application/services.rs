use anyhow::Ok;
use uuid::Uuid;

use crate::domain::{event_publisher::EventPublisher, models::Note, repository::NoteRepository};

pub struct NoteService<R, E>
where
    R: NoteRepository,
    E: EventPublisher,
{
    repository: R,
    event_publisher: E,
}

impl<R, E> NoteService<R, E>
where
    R: NoteRepository,
    E: EventPublisher,
{
    pub fn new(repository: R, event_publisher: E) -> Self {
        Self {
            repository,
            event_publisher,
        }
    }

    pub async fn add_note(&self, text: String) -> anyhow::Result<Note> {
        let note = Note {
            id: Uuid::new_v4(),
            text,
        };
        self.repository.save(&note).await?;
        self.event_publisher.publish_note_created(&note).await?;
        Ok(note)
    }

    pub async fn list_notes(&self) -> anyhow::Result<Vec<Note>> {
        self.repository.list_all().await
    }
}

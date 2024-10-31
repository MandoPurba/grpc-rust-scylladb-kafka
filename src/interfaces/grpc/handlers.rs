use tonic::{Request, Response, Status};

use crate::{
    application::services::NoteService,
    domain::{event_publisher::EventPublisher, repository::NoteRepository},
    proto::notes::{
        note_service_server::NoteService as GrpcNoteService, Empty as Nul, ListNotesResponse,
        NoteRequest, NoteResponse,
    },
};

pub struct NoteServiceHandler<R, E>
where
    R: NoteRepository,
    E: EventPublisher,
{
    service: NoteService<R, E>,
}

impl<R, E> NoteServiceHandler<R, E>
where
    R: NoteRepository,
    E: EventPublisher,
{
    pub fn new(service: NoteService<R, E>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl<R, E> GrpcNoteService for NoteServiceHandler<R, E>
where
    R: NoteRepository,
    E: EventPublisher,
{
    async fn add_note(
        &self,
        request: Request<NoteRequest>,
    ) -> Result<Response<NoteResponse>, Status> {
        let text = request.into_inner().text;

        let note = self
            .service
            .add_note(text)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(NoteResponse {
            id: note.id.to_string(),
            text: note.text,
        }))
    }

    async fn list_notes(
        &self,
        _request: Request<Nul>,
    ) -> Result<Response<ListNotesResponse>, Status> {
        let notes = self
            .service
            .list_notes()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let note_responses = notes
            .into_iter()
            .map(|note| NoteResponse {
                id: note.id.to_string(),
                text: note.text,
            })
            .collect();

        Ok(Response::new(ListNotesResponse {
            notes: note_responses,
        }))
    }
}

use dotenv::dotenv;
use rust_scylla_grpc::{
    application::services::NoteService,
    infrastructure::{
        messaging::kafka::KafkaEventPublisher, persistence::scylla::ScyllaRepository,
    },
    interfaces::grpc::handlers::NoteServiceHandler,
    proto::{notes::note_service_server::NoteServiceServer, FILE_DESCRIPTOR_SET},
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Initialize ScyllaDB repository
    let repository = ScyllaRepository::new(
        &std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string()),
    )
    .await?;

    // Initialize Kafka publisher
    let event_publisher = KafkaEventPublisher::new(
        &std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string()),
        &std::env::var("KAFKA_TOPIC").unwrap_or_else(|_| "note-events".to_string()),
    )?;

    // Initialize service
    let service = NoteService::new(repository, event_publisher);

    // Initialize gRPC handler
    let handler = NoteServiceHandler::new(service);

    // reflections
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    // Start gRPC server
    let addr = "[::1]:50051".parse()?;
    println!("NoteService server listening on {}", addr);

    Server::builder()
        .add_service(reflection)
        .add_service(NoteServiceServer::new(handler))
        .serve(addr)
        .await?;

    Ok(())
}

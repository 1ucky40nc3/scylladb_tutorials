use catalytic::scylla::{
    transport::errors::NewSessionError, CachingSession, Session, SessionBuilder,
};
use uuid::Uuid;

use crate::generated::Tutorial;

mod generated;

/// Create a ScyllaDB Rust driver session.
async fn create_scylla_session(uri: &str) -> Result<Session, NewSessionError> {
    SessionBuilder::new().known_node(uri).build().await
}

/// Create a cached Catalytic session
async fn create_catalytic_session(uri: &str) -> Result<CachingSession, NewSessionError> {
    let session = create_scylla_session(&uri).await;

    match session {
        Ok(session) => Ok(CachingSession::from(session, 1_000)),
        Err(error) => Err(error),
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    // Initialize tracing logging
    tracing_subscriber::fmt::init();
    // Try to set environment variables using dotenv
    dotenvy::dotenv().expect("Problem during dotenv environment variable setup!");

    // Create new session
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| String::from("scylla:9042"));
    let session = create_catalytic_session(&uri)
        .await
        .unwrap_or_else(|error| {
            panic!("Problem creating ScyllaDB session: {:?}", error);
        });
    // Set the `tutorial` keyspace for this session
    let keyspace = String::from("tutorial");
    session
        .get_session()
        .use_keyspace(&keyspace, false)
        .await
        .expect(format!("Problem during set operation of keyspace '{:?}'", &keyspace).as_str());

    // Create a new `Tutorial` instance
    let tutorial = Tutorial {
        id: Uuid::new_v4(),
        name: String::from("Using the Rust Driver with ScyllaDB"),
        time: chrono::offset::Utc::now().timestamp(),
    };

    // Insert the tutorial in the database
    tutorial
        .to_ref() // Borrow the original instance
        .insert(&session)
        .await
        .expect("Problem during `tutorial` insert");

    let queried = tutorial
        .primary_key()
        .select_unique_expect(&session)
        .await
        .unwrap()
        .entity;

    assert_eq!(tutorial, queried);

    Ok(())
}

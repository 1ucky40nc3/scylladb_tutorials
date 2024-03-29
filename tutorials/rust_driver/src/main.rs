use core::panic;

use chrono::{DateTime, Utc};
use scylla::{
    cql_to_rust::FromRowError,
    transport::errors::{NewSessionError, QueryError},
    FromRow, QueryResult, SerializeRow, Session, SessionBuilder,
};
use uuid::Uuid;

/// Create a ScyllaDB Rust driver session.
async fn create_session(uri: &str) -> Result<Session, NewSessionError> {
    SessionBuilder::new().known_node(uri).build().await
}

static CREATE_TUTORIAL_KEYSPACE_QUERY: &str = r#"
    CREATE KEYSPACE IF NOT EXISTS tutorial
    WITH REPLICATION = {
        'class': 'SimpleStrategy',
        'replication_factor': 1
    }; 
"#;

static CREATE_TUTORIAL_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS tutorial.tutorial (
        id UUID,
        name TEXT,
        time TIMESTAMP,
        PRIMARY KEY(id)
    );
"#;

static ADD_TUTORIAL_QUERY: &str = r#"
    INSERT INTO tutorial.tutorial (id, name, time)
    VALUES (?, ?, ?);
"#;

static SELECT_TUTORIAL_QUERY: &str = r#"
    SELECT * FROM tutorial.tutorial
    WHERE id = ?
"#;

/// Create the ScyllaDB `tutorial` keyspace
async fn create_tutorial_keyspace(session: &Session) -> Result<QueryResult, QueryError> {
    session.query(CREATE_TUTORIAL_KEYSPACE_QUERY, ()).await
}

/// Create the ScyllaDB `tutorial` keyspace
async fn create_tutorial_table(session: &Session) -> Result<QueryResult, QueryError> {
    session.query(CREATE_TUTORIAL_TABLE_QUERY, ()).await
}

/// Initialize the ScyllaDB database
async fn initialize(session: &Session) -> Result<(), ()> {
    create_tutorial_keyspace(session)
        .await
        .unwrap_or_else(|err| {
            panic!("Problem during `tutorial` keyspace creation! {:?}", err);
        });
    create_tutorial_table(session).await.unwrap_or_else(|err| {
        panic!("Problem during `tutorial` table creation! {:?}", err);
    });
    Ok(())
}

/// Implement a struct for tutorials
#[derive(Debug, Clone, SerializeRow, FromRow)]
struct Tutorial {
    id: Uuid,
    name: String,
    time: DateTime<Utc>,
}

/// Create the ScyllaDB an entry in the `tutorial` table
async fn add_tutorial_table_entry(
    session: &Session,
    tutorial: Tutorial,
) -> Result<QueryResult, QueryError> {
    session.query(ADD_TUTORIAL_QUERY, tutorial).await
}

/// Implement a custom error enum for errors types during the table select query.
#[derive(Debug)]
enum SelectError {
    QueryError(QueryError),
    FromRowError(FromRowError),
}

/// Select an entry from the `tutorials` table
async fn select_tutorial_table_entry(
    session: &Session,
    id: &Uuid,
) -> Result<Vec<Tutorial>, SelectError> {
    let result = session.query(SELECT_TUTORIAL_QUERY, (id,)).await;
    match result {
        Ok(query_result) => {
            let mut tutorials = Vec::new();
            for row in query_result.rows.unwrap_or_default() {
                match row.into_typed::<Tutorial>() {
                    Ok(tutorial) => tutorials.push(tutorial),
                    Err(err) => return Err(SelectError::FromRowError(err)),
                }
            }

            Ok(tutorials)
        }
        Err(err) => Err(SelectError::QueryError(err)),
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Connecting to the ScyllaDB database...");
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| String::from("scylla:9042"));
    let session = create_session(&uri).await.unwrap_or_else(|error| {
        panic!("Problem creating ScyllaDB Rust driver session: {:?}", error);
    });
    initialize(&session).await?;

    // Create a new tutorial entry
    let tutorial = Tutorial {
        id: Uuid::new_v4(),
        name: String::from("Using the Rust Driver with ScyllaDB"),
        time: chrono::offset::Utc::now(),
    };

    // Try to add the tutorial to the database
    println!("Adding `tutorial` table entry: {:?}", &tutorial);
    let _entry = add_tutorial_table_entry(&session, tutorial.clone())
        .await
        .expect("Problem during try to add a new `tutorial` table entry!");

    // Try to find the tutorial in the database
    let uuid = &tutorial.id;
    let entry = select_tutorial_table_entry(&session, &uuid)
        .await
        .expect("Problem during select of `tutorial` table entry!");
    match entry.first() {
        Some(tutorial) => println!("Selected `tutorial` table entry: {:?}", &tutorial),
        None => println!("Can't find tutorial using uuid: {:?}", &uuid),
    }
    Ok(())
}

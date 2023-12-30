use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Employee<'a> {
    title: &'a str,
    name: Name<'a>,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn run_database(title: &str, first_name: &str, last_name: &str) -> surrealdb::Result<()> {
    // Connect to the server
    let db = Surreal::new::<Ws>("0.0.0.0:1101").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("airport").use_db("airport").await?;

    // Create a new employee with a random id
    let created: Vec<Record> = db
        .create("employee")
        .content(Employee {
            title,
            name: Name {
                first: first_name,
                last: last_name,
            },
        })
        .await?;
    dbg!(created);

    Ok(())
}

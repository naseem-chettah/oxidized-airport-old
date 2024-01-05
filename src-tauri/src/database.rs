use serde::Deserialize;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn import_passengers() -> surrealdb::Result<Vec<Record>> {
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

    let passengers: Vec<Record> = db.select("passenger").await?;

    Ok(passengers)
}

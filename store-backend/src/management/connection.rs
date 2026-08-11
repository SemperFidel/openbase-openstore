use mongodb::bson::doc;
use mongodb::Client;
use mongodb::error::Error;
use mongodb::options::ClientOptions;

struct Connection {
    host: String,
    port: u64,
}

async fn connect_to_instance(connection: Connection) -> Result<Client, Error> {
    let uri = format!("mongodb://{}:{}/", connection.host, connection.port);
    let opts = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(opts)?;

    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await?;

    Ok(client)
}

async fn connect_to_replica_set(
    servers: Vec<Connection>,
    replica_set: String,
) -> Result<Client, Error> {
    let servers_uri = servers
        .iter()
        .map(|server| format!("{}:{}", server.host, server.port))
        .collect::<Vec<String>>()
        .join(",");

    let uri = format!("mongodb://{}/?replicaSet={}", servers_uri, replica_set);

    let opts = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(opts)?;

    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await?;

    Ok(client)
}
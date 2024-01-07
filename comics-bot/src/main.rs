use anyhow::Result;
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use fred::{
    clients::RedisClient,
    interfaces::{ClientLike, HashesInterface},
};
use tokio::{fs, net::TcpListener};

struct Error(anyhow::Error);
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl<T: Into<anyhow::Error>> From<T> for Error {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

type RouteResult<T> = Result<T, Error>;

async fn add(
    Path(comic): Path<String>,
    State(db): State<RedisClient>,
    data: Body,
) -> RouteResult<()> {
    let comic_path = format!("comics/{comic}");
    fs::create_dir_all(&comic_path).await?;

    let len = db.hget::<i32, _, _>("comics", &comic).await? + 1;

    // Thanks ItsEthra for adding this handy `to_bytes` function, what could I have done without ya.
    // TODO: Should be limiting and checking if body is valid image
    let png = axum::body::to_bytes(data, usize::MAX).await?;
    fs::write(format!("comics/{comic}/{len}.png"), png).await?;

    db.hset("comics", (comic, len)).await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let client = RedisClient::default();
    client.connect();

    client.wait_for_connect().await?;

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let app = Router::new()
        .route("/add/:comic", post(add))
        .with_state(client);

    axum::serve(listener, app).await?;

    Ok(())
}

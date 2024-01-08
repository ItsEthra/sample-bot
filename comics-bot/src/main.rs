use anyhow::Result;
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, post},
    Router,
};
use fred::{
    clients::RedisClient,
    interfaces::{ClientLike, HashesInterface},
};
use futures_util::StreamExt;
use tokio::{fs, net::TcpListener};
use tokio_stream::wrappers::ReadDirStream;

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

    let len = db
        .hget::<Option<usize>, _, _>("comics", &comic)
        .await?
        .unwrap_or(0)
        + 1;

    // Thanks ItsEthra for adding this handy `to_bytes` function, what could I have done without ya.
    // TODO: Should be limiting body size and checking if body is valid image
    let png = axum::body::to_bytes(data, usize::MAX).await?;
    fs::write(format!("comics/{comic}/{len}.png"), png).await?;

    db.hset("comics", (comic, len)).await?;

    Ok(())
}

async fn remove(
    State(db): State<RedisClient>,
    Path((comic, num)): Path<(String, usize)>,
) -> RouteResult<StatusCode> {
    // dirty, best case scenario would be to have a normal enum error type but oh well
    match db.hget::<Option<usize>, _, _>("comics", &comic).await? {
        None => return Ok(StatusCode::NOT_FOUND),
        Some(max) if num > max => return Ok(StatusCode::NOT_FOUND),
        _ => {}
    }

    let stream = ReadDirStream::new(fs::read_dir(format!("comics/{comic}")).await?);
    let to_move = stream
        .filter_map(|x| async { x.ok() })
        .filter_map(|de| async move {
            let pos = de
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse::<usize>()
                .ok()?;
            if pos == num {
                println!("Removing {:?}", de.path().file_name().unwrap());

                fs::remove_file(de.path()).await.unwrap();
                None
            } else {
                (pos > num).then(|| (de.path(), pos))
            }
        })
        .collect::<Vec<_>>()
        .await;

    if num == 1 && to_move.is_empty() {
        db.hdel("comics", &comic).await?;
    } else {
        db.hincrby("comics", &comic, -1).await?;
    }

    for (path, old) in to_move {
        let new = format!("{}.png", old - 1);
        println!("Moving {:?} to {new}", path.file_name().unwrap());

        fs::rename(path.clone(), path.with_file_name(&new)).await?;
    }

    Ok(StatusCode::OK)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let client = RedisClient::default();
    client.connect();

    client.wait_for_connect().await?;

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let app = Router::new()
        .route("/comic/:comic", post(add))
        .route("/comic/:comic/:num", delete(remove))
        .with_state(client);

    axum::serve(listener, app).await?;

    Ok(())
}

use example_hello_world_types::Description;
use warp::{Filter, Reply};
use anyhow::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let hello = warp::path!("hello" / String / i32)
    .map(|name, n| {
        let body = Description {a: name, b: n};
        Ok(warp::reply::json(&body).into_response())
    });

    warp::serve(hello).run(([0, 0, 0, 0], 3030)).await;
    Ok(())
}

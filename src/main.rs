use warp::Filter;
use std::convert::Infallible;
use warp::http::Uri;
use warp::hyper::client::HttpConnector;
use warp::hyper::Body;
use warp::hyper::Client;
use warp::hyper::Response;
use warp::path::FullPath;

#[tokio::main]
async fn main() {
    // Static file serving
    let static_files = warp::fs::dir("static");

    // API proxy
    let api_proxy = warp::path("api")
        .and(warp::path::full())
        .and_then(proxy_handler);

    // Combine the routes
    let routes = static_files.or(api_proxy);

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Proxy handler function
async fn proxy_handler(full_path: FullPath) -> Result<impl warp::Reply, Infallible> {
    let client = Client::new();
    let uri = format!("http://www.walletexplorer.com{}", full_path.as_str())
        .parse::<Uri>()
        .unwrap();

    let resp = client.get(uri).await.unwrap();
    let body = resp.into_body();

    Ok(Response::new(body))
}
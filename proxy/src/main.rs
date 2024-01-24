use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::{Body, Client, Request, Response, Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    start_proxy_server().await?;

    Ok(())
}

async fn proxy_service(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!(
        "Got request at: {} {:?} {:?} {:?} {:?}",
        req.uri().to_string(),
        req.uri().host(),
        req.uri().port(),
        req.uri().path(),
        req.uri().query(),
    );

    let mut parts = req.uri().clone().into_parts();
    parts.scheme = Some(http::uri::Scheme::HTTPS);

    let new_uri = http::Uri::from_parts(parts).unwrap();
    println!("New URI: {}", new_uri);

    let client = Client::new();
    println!("Forwarding request to: {} {}", new_uri, req.method());

    let forwarded_req = Request::builder()
        .method(req.method())
        .uri(new_uri)
        .version(req.version())
        .body(req.into_body())
        .unwrap();

    client.request(forwarded_req).await
}

async fn start_proxy_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // load the certificates
    // let certs = load_certs("path_to_your_cert.pem")?;
    // let key = load_private_key("path_to_your_key.pem")?;

    // let mut config = ServerConfig::new(NoClientAuth::new());
    // config.set_single_cert(certs, key)?;

    // let acceptor = TlsAcceptor::from(Arc::new(config));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(proxy_service)) });

    // let server = Server::bind(&addr).tls(acceptor).serve(make_svc);
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("proxy server error: {}", e);
    }
    Ok(())
}

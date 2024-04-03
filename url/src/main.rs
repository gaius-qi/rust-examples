use url::Url;

fn main() {
    let url = Url::parse("http://github.com:8080").unwrap();
    println!(
        "{}:{}",
        url.host_str().unwrap(),
        url.port_or_known_default().unwrap()
    );
}

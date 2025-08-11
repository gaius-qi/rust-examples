use std::path::{Path, PathBuf};
use url::Url;

fn main() {
    let url = Url::parse("http://github.com:8080").unwrap();
    println!(
        "{}:{} - {}",
        url.host_str().unwrap(),
        url.port_or_known_default().unwrap(),
        url.scheme(),
    );

    let path = Path::new("/rust/").to_path_buf();
    let path = path.join("url");

    let url = url.join(path.to_str().unwrap()).unwrap();

    println!("{}", url.to_string());
}

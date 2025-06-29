use url_path::UrlPath;

pub fn real_path(p: &str) -> String {
    match p {
        path if path.starts_with("~") => shellexpand::tilde(p).into_owned(),
        path if path.contains("$") => shellexpand::env(path).expect("Could not get path").to_string(),
        _ => UrlPath::new(p).normalize()
    }
}
use url_path::UrlPath;

pub fn real_path(p: &str) -> String {
    match p {
        path if path.starts_with("~") => shellexpand::tilde(p).into_owned(),
        _ => UrlPath::new(p).normalize()
    }
}
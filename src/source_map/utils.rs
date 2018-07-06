use regex::Regex;

pub fn relative(root: &str, path: &str) -> String {
    let mut root = String::from(root);
    if root.is_empty() {
        root.push('.');
    }
    if root.ends_with('/') {
        root.pop();
    }

    let mut level = 0;
    while let Some(_) = path.find(&(root.clone() + "/")) {
        if let Some(index) = root.rfind('/') {
            root.split_off(index);
            lazy_static! {
                static ref RE: Regex = Regex::new("^([^/]+:/)?/*$").unwrap();
            }
            if RE.is_match(&root) {
                return String::from(path);
            }
        } else {
            return String::from(path);
        }
        level += 1;
    }
    "../".repeat(level) + path.split_at(root.len() + 1).1
}

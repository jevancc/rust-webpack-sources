use regex::Regex;

#[inline]
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

#[inline]
pub fn split_string(mut s: String, pos: i32, s_len: Option<usize>) -> (String, String) {
    let s_len = s_len.map_or(s.chars().count(), |l| l);

    if pos <= 0 {
        (String::new(), s)
    } else if pos >= s_len as i32 {
        (s, String::new())
    } else {
        let pos = s.char_indices().skip(pos as usize).next().unwrap().0;
        let off = s.split_off(pos);
        (s, off)
    }
}

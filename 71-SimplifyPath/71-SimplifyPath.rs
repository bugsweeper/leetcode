impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        let mut folder_names_len = 0;
        for dir_name in path.split('/') {
            match dir_name {
                "" | "." => {},
                ".." => folder_names_len += stack.pop().unwrap_or("").len(),
                _ => {
                    folder_names_len += dir_name.len();
                    stack.push(dir_name);
                },
            }
        }
        let mut result = String::with_capacity(folder_names_len + stack.len());
        result = stack.into_iter().fold(result, |mut simple_path, folder_name| {
            simple_path.push('/');
            simple_path.push_str(folder_name);
            simple_path
        });
        if result.is_empty() {
            "/".into()
        } else {
            result
        }
    }
}
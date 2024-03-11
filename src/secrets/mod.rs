use std::collections::HashMap;

pub trait ISecretProvider {
    fn get_secret(&self, key: &str) -> Option<&str>;
    fn new(file_path: &str) -> Self;
    fn change_file(&mut self, file_path: &str);
    fn get_file_path(&self) -> &str;
}

pub struct SecretProvider {
    file_path: String,
    data: HashMap<String, String>,
}

fn load_secrets(file_path: &str) -> HashMap<String, String> {
    let mut data = HashMap::new();
    let file = std::fs::read_to_string(file_path).unwrap();
    for line in file.lines() {
        if line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            continue;
        }
        data.insert(parts[0].to_string(), parts[1].to_string());
    }
    data
}

impl ISecretProvider for SecretProvider {
    fn new(file_path: &str) -> Self {
        SecretProvider {
            file_path: file_path.to_string(),
            data: load_secrets(file_path),
        }
    }

    fn get_secret(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }

    fn get_file_path(&self) -> &str {
        self.file_path.as_str()
    }

    fn change_file(&mut self, file_path: &str) {
        self.file_path = file_path.to_string();
        self.data = load_secrets(file_path);
    }
}

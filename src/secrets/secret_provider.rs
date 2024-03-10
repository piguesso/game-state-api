use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub trait ISecretProvider {
    fn get_secret(&self, key: String) -> Option<String>;
    fn change_source(&mut self, source: String);
    fn get_source(&self) -> String;
}

pub struct SecretProvider {
    source: String,
    data: HashMap<String, String>,
}

fn read_from_file(file_path: String) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

fn extract_secrets(data: Vec<String>) -> HashMap<String, String> {
    let mut secrets = HashMap::new();
    for line in data {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('=').collect();
        secrets.insert(parts[0].to_string(), parts[1].to_string());
    }
    secrets
}

pub fn new_source_provider(source: String) -> SecretProvider {
    let data = read_from_file(source.clone());
    let secrets = extract_secrets(data);
    SecretProvider {
        source,
        data: secrets,
    }
}

impl ISecretProvider for SecretProvider {
    fn get_secret(&self, key: String) -> Option<String> {
        match self.data.get(&key) {
            Some(val) => Some(val.to_string()),
            None => None,
        }
    }

    fn change_source(&mut self, source: String) {
        let data = read_from_file(source.clone());
        let secrets = extract_secrets(data);
        self.source = source;
        self.data = secrets;
    }

    fn get_source(&self) -> String {
        self.source.clone()
    }
}

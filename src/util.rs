//! baobab model — disk usage analyzer.

#[derive(Debug, Clone)]
pub struct Directory {
    pub path: String,
    pub size: u64,
    pub entries: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub directories: Vec<Directory>,
}

impl Default for Model {
    fn default() -> Self { Self { directories: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn scan(&mut self, path: &str) -> bool {
        if std::path::Path::new(path).exists() {
            self.directories.push(Directory { path: path.into(), size: 0, entries: vec![] });
            true
        } else { false }
    }

    pub fn list(&self) -> &[Directory] { &self.directories }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scan_list() {
        let mut b = Model::new();
        assert!(b.scan("/tmp"));
        assert_eq!(b.list().len(), 1);
    }
}

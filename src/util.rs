//! Utility model for imagemagick (legacy compat).

#[derive(Debug, Clone)]
pub struct Image {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub format: String,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub image: String,
    pub op: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub images: Vec<Image>,
    pub operations: Vec<Operation>,
}

impl Default for Model {
    fn default() -> Self { Self { images: vec![], operations: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn load(&mut self, path: &str, width: u32, height: u32, format: &str) -> String {
        let id = format!("{:x}", self.images.len() + 1);
        self.images.push(Image { path: path.into(), width, height, format: format.into() });
        id
    }

    pub fn operate(&mut self, image: &str, op: &str, args: Vec<String>) {
        self.operations.push(Operation { image: image.into(), op: op.into(), args });
    }

    pub fn list(&self) -> &[Image] { &self.images }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_model() {
        let mut m = Model::new();
        m.load("/a.png", 100, 100, "png");
        assert_eq!(m.images.len(), 1);
    }
}

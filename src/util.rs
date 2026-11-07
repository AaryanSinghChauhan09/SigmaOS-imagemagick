//! imagemagick model — image processing.

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

    pub fn load(&mut self, image: Image) { self.images.push(image); }

    pub fn apply(&mut self, image: &str, op: &str, args: Vec<String>) -> bool {
        if self.images.iter().any(|i| i.path == image) {
            self.operations.push(Operation { image: image.into(), op: op.into(), args });
            true
        } else { false }
    }

    pub fn convert(&mut self, src: &str, dst: &str, format: &str) -> bool {
        if self.images.iter().any(|i| i.path == src) {
            self.images.push(Image { path: dst.into(), width: 0, height: 0, format: format.into() });
            true
        } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_apply_convert() {
        let mut m = Model::new();
        m.load(Image { path: "in.png".into(), width: 100, height: 100, format: "png".into() });
        assert!(m.apply("in.png", "resize", vec!["50%".into()]));
        assert!(m.convert("in.png", "out.jpg", "jpg"));
    }
}

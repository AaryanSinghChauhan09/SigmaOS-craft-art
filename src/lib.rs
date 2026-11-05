#[derive(Debug, Clone)]
pub struct Layer { pub name: String, pub opacity: f32, pub visible: bool }
impl Layer {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), opacity: 1.0, visible: true }
    }
}

#[derive(Debug, Default)]
pub struct Canvas { pub width: u32, pub height: u32, layers: Vec<Layer> }
impl Canvas {
    pub fn new(w: u32, h: u32) -> Self { Self { width: w, height: h, layers: Vec::new() } }
    pub fn add_layer(&mut self, l: Layer) { self.layers.push(l); }
    pub fn count(&self) -> usize { self.layers.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_canvas() {
        let mut c = Canvas::new(800, 600);
        c.add_layer(Layer::new("Background"));
        assert_eq!(c.count(), 1);
    }
}

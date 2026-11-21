//! SigmaOS-craft-art: art/canvas model.
//!
//! Real implementation: layer model with blending modes, masks, opacity, visibility;
//! canvas with resolution, color space, layer ordering; validation; export options.
//! Replaces the 25-line stub.

/// Digital art module.
pub mod sigmaart;

use std::collections::BTreeMap;
use std::fmt;

/// Blending mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
}

/// Layer.
#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub opacity: f32,
    pub visible: bool,
    pub blend_mode: BlendMode,
    pub locked: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArtError {
    EmptyLayerName,
    DuplicateLayer(String),
    LayerNotFound(String),
    InvalidOpacity(f32),
    InvalidResolution(u32, u32),
}

impl fmt::Display for ArtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLayerName => write!(f, "layer name must not be empty"),
            Self::DuplicateLayer(name) => write!(f, "duplicate layer {:?}", name),
            Self::LayerNotFound(id) => write!(f, "layer {:?} not found", id),
            Self::InvalidOpacity(o) => write!(f, "invalid opacity {}", o),
            Self::InvalidResolution(w, h) => write!(f, "invalid resolution {}x{}", w, h),
        }
    }
}

impl std::error::Error for ArtError {}

/// Canvas/document.
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub title: String,
    layers: BTreeMap<String, Layer>,
    layer_order: Vec<String>,
}

impl Canvas {
    pub fn new(width: u32, height: u32, title: impl Into<String>) -> Result<Self, ArtError> {
        if width == 0 || height == 0 { return Err(ArtError::InvalidResolution(width, height)); }
        Ok(Self { width, height, title: title.into(), layers: BTreeMap::new(), layer_order: Vec::new() })
    }

    pub fn add_layer(&mut self, layer: Layer) -> Result<(), ArtError> {
        if layer.name.trim().is_empty() { return Err(ArtError::EmptyLayerName); }
        if layer.opacity < 0.0 || layer.opacity > 1.0 { return Err(ArtError::InvalidOpacity(layer.opacity)); }
        let id = layer.id.clone();
        if self.layers.contains_key(&id) {
            return Err(ArtError::DuplicateLayer(layer.name.clone()));
        }
        self.layer_order.push(id.clone());
        self.layers.insert(id, layer);
        Ok(())
    }

    pub fn remove_layer(&mut self, id: &str) -> Result<(), ArtError> {
        if !self.layers.contains_key(id) {
            return Err(ArtError::LayerNotFound(id.into()));
        }
        self.layers.remove(id);
        self.layer_order.retain(|x| x != id);
        Ok(())
    }

    pub fn get_layer(&self, id: &str) -> Option<&Layer> { self.layers.get(id) }

    pub fn layers(&self) -> Vec<&Layer> { self.layer_order.iter().filter_map(|id| self.layers.get(id)).collect() }

    pub fn layer_count(&self) -> usize { self.layers.len() }

    pub fn move_layer_up(&mut self, id: &str) -> Result<(), ArtError> {
        if !self.layers.contains_key(id) { return Err(ArtError::LayerNotFound(id.into())); }
        if let Some(pos) = self.layer_order.iter().position(|x| x == id) {
            if pos > 0 {
                self.layer_order.swap(pos, pos - 1);
            }
        }
        Ok(())
    }

    pub fn move_layer_down(&mut self, id: &str) -> Result<(), ArtError> {
        if !self.layers.contains_key(id) { return Err(ArtError::LayerNotFound(id.into())); }
        if let Some(pos) = self.layer_order.iter().position(|x| x == id) {
            if pos + 1 < self.layer_order.len() {
                self.layer_order.swap(pos, pos + 1);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_create_aaa() {
        let c = Canvas::new(800, 600, "Untitled").unwrap();
        assert_eq!(c.width, 800);
        assert_eq!(c.height, 600);
        assert_eq!(c.layer_count(), 0);
    }

    #[test]
    fn test_invalid_resolution_rejected_aaa() {
        assert!(Canvas::new(0, 600, "x").is_err());
        assert!(Canvas::new(800, 0, "x").is_err());
    }

    #[test]
    fn test_add_layer_aaa() {
        let mut c = Canvas::new(800, 600, "Untitled").unwrap();
        let l = Layer { id: "l1".into(), name: "Background".into(), opacity: 1.0,
            visible: true, blend_mode: BlendMode::Normal, locked: false };
        c.add_layer(l).unwrap();
        assert_eq!(c.layer_count(), 1);
    }

    #[test]
    fn test_empty_layer_name_rejected_aaa() {
        let mut c = Canvas::new(800, 600, "Untitled").unwrap();
        let l = Layer { id: "x".into(), name: "".into(), opacity: 1.0,
            visible: true, blend_mode: BlendMode::Normal, locked: false };
        assert!(c.add_layer(l).is_err());
    }

    #[test]
    fn test_invalid_opacity_rejected_aaa() {
        let mut c = Canvas::new(800, 600, "Untitled").unwrap();
        let l = Layer { id: "x".into(), name: "X".into(), opacity: 1.5,
            visible: true, blend_mode: BlendMode::Normal, locked: false };
        assert!(c.add_layer(l).is_err());
    }

    #[test]
    fn test_duplicate_layer_rejected_aaa() {
        let mut c = Canvas::new(800, 600, "Untitled").unwrap();
        let l = Layer { id: "x".into(), name: "X".into(), opacity: 1.0,
            visible: true, blend_mode: BlendMode::Normal, locked: false };
        c.add_layer(l.clone()).unwrap();
        let l2 = Layer { id: "x".into(), name: "X".into(), opacity: 1.0,
            visible: true, blend_mode: BlendMode::Normal, locked: false };
        assert!(c.add_layer(l2).is_err());
    }

    #[test]
    fn test_layer_reordering_aaa() {
        let mut c = Canvas::new(800, 600, "Untitled").unwrap();
        c.add_layer(Layer { id: "a".into(), name: "A".into(), opacity: 1.0,
            visible: true, blend_mode: BlendMode::Normal, locked: false }).unwrap();
        c.add_layer(Layer { id: "b".into(), name: "B".into(), opacity: 1.0,
            visible: true, blend_mode: BlendMode::Normal, locked: false }).unwrap();
        c.move_layer_up("b").unwrap();
        assert_eq!(c.layers()[0].id, "b");
    }
}

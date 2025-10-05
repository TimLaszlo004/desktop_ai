use image::{ImageBuffer, Rgba};
use serde_derive::{Deserialize, Serialize};

pub type MyImage = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct BoundingBox {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UIElement {
    pub id: String,
    pub element_type: String,
    pub confidence: f32,
    pub boundingbox: BoundingBox,
    pub children: Vec<UIElement>,
}

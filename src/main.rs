use screenshots::{Screen, image::ImageFormat};
use serde_derive::{Deserialize, Serialize};
use serde_json::to_string_pretty;

#[derive(Serialize, Deserialize, Debug)]
struct BoundingBox {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct UiElement {
    id: String,
    element_type: String,
    boundingbox: BoundingBox,
    children: Vec<UiElement>,
}

fn main() {
    let screens = Screen::all().unwrap();

    for (i, screen) in screens.iter().enumerate() {
        let image = screen.capture().unwrap();
        let filename = format!("screenshot_monitor_{}.png", i);
        image.save_with_format(&filename, ImageFormat::Png).unwrap();

        println!("Saved {}", filename);
    }

    let root = UiElement {
        id: "0001".to_string(),
        element_type: "root".to_string(),
        boundingbox: BoundingBox {
            x: 0,
            y: 0,
            w: 1920,
            h: 1080,
        },
        children: vec![UiElement {
            id: "0002".to_string(),
            element_type: "button".to_string(),
            boundingbox: BoundingBox {
                x: 40,
                y: 40,
                w: 200,
                h: 50,
            },
            children: vec![],
        }],
    };

    let json = to_string_pretty(&root).unwrap();
    println!("{}", json);
}

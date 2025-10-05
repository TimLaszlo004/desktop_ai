use image::imageops::resize;
use ndarray::Array4;
use onnxruntime::{
    GraphOptimizationLevel, LoggingLevel, environment::Environment, tensor::OrtOwnedTensor,
};

use crate::types::{BoundingBox, MyImage, UIElement};

fn preprocess_from_buffer(img: &MyImage) -> Array4<f32> {
    // let (width, height) = img.dimensions();
    let target_size = (640, 640);

    let resized_img = resize(
        img,
        target_size.0,
        target_size.1,
        image::imageops::FilterType::Triangle,
    );

    let mut array = Array4::<f32>::zeros((1, 3, target_size.1 as usize, target_size.0 as usize));

    for (x, y, pixel) in resized_img.enumerate_pixels() {
        let [r, g, b, _a] = pixel.0;
        array[[0, 0, y as usize, x as usize]] = r as f32 / 255.0;
        array[[0, 1, y as usize, x as usize]] = g as f32 / 255.0;
        array[[0, 2, y as usize, x as usize]] = b as f32 / 255.0;
    }

    array
}

pub fn run_yolo(img: &MyImage) -> Vec<UIElement> {
    let environment = Environment::builder()
        .with_name("test")
        .with_log_level(LoggingLevel::Verbose)
        .build()
        .expect("Environment building error");
    let mut session = environment
        .new_session_builder()
        .expect("Session builder error")
        .with_optimization_level(GraphOptimizationLevel::Basic)
        .expect("Optimization level error")
        .with_number_threads(1)
        .expect("Number of threads error")
        .with_model_from_file("yolo11n.onnx")
        .expect("Model loading error");
    let array = preprocess_from_buffer(img);
    let input_tensor = vec![array];
    let outputs: Vec<OrtOwnedTensor<f32, _>> = session.run(input_tensor).expect("Output error");

    detections_to_ui(decode_yolo(
        &outputs[0].as_slice().unwrap().to_vec(),
        img.width(),
        img.height(),
    ))
}

struct Detection {
    class_id: usize,
    confidence: f32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

fn decode_yolo(output: &[f32], img_w: u32, img_h: u32) -> Vec<Detection> {
    let num_attrs = 85; // depends on model
    output
        .chunks(num_attrs)
        .filter_map(|chunk| {
            let conf = chunk[4];
            if conf < 0.5 {
                return None;
            }

            let class_scores = &chunk[5..];
            let (class_id, class_conf) =
                class_scores
                    .iter()
                    .enumerate()
                    .fold(
                        (0, 0.0),
                        |acc, (i, &val)| if val > acc.1 { (i, val) } else { acc },
                    );

            let final_conf = conf * class_conf;
            if final_conf < 0.5 {
                return None;
            }

            Some(Detection {
                class_id,
                confidence: final_conf,
                x: chunk[0] * img_w as f32,
                y: chunk[1] * img_h as f32,
                w: chunk[2] * img_w as f32,
                h: chunk[3] * img_h as f32,
            })
        })
        .collect()
}

fn detections_to_ui(detections: Vec<Detection>) -> Vec<UIElement> {
    detections
        .into_iter()
        .enumerate()
        .map(|(i, d)| UIElement {
            id: format!("elem_{:04}", i),
            element_type: format!("class_{}", d.class_id),
            confidence: d.confidence,
            boundingbox: BoundingBox {
                x: d.x as u32,
                y: d.y as u32,
                w: d.w as u32,
                h: d.h as u32,
            },
            children: vec![],
        })
        .collect()
}

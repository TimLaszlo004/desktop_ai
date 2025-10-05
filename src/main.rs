use screenshots::Screen;

mod analizator;
mod types;
use crate::analizator::run_yolo;

fn main() {
    let screens = Screen::all().unwrap();

    for (_, screen) in screens.iter().enumerate() {
        let image = screen.capture().unwrap();
        let result = run_yolo(&image);
        for element in &result {
            println!("{:?}", element);
        }
    }
}

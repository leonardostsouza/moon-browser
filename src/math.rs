// math.rs: Simple useful  math abstractions for Moon Browser

use formality_document::document::*;

pub const PI: f64 = 3.14159265358979323846;

pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx*dx + dy*dy).sqrt()
}

pub fn is_inside(x_click: f64, y_click: f64, elem: &Element) -> bool {
    match elem {
        Element::Circle{x, y, r} => {
            distance(*x as f64, *y as f64, x_click, y_click) < *r as f64
        }
        Element::Square{x, y, r} => {
            (x_click > *x as f64) && (x_click < (*x + *r) as f64) &&
            (y_click > *y as f64) && (y_click < (*y + *r) as f64)
        }
        _ => {panic!("ERROR in function \"is_inside\": Unrecognized Element Type");}
    }
}

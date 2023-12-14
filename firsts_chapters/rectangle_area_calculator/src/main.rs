mod rectangle;
mod utils;

use crate::rectangle::Rectangle;
use crate::utils::area;

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 25
    };

    let rectangle_area = area(&rectangle);

    println!("The rectangle area is: {rectangle_area}");
}
use crate::rectangle::Rectangle;

pub fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
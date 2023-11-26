struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 25
    };

    let rectangle_area = area(&rectangle);

    println!("The rectangle area is: {rectangle_area}");
}
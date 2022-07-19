fn main() {
    area_by_normal_variable();
    area_by_touple();
    area_by_structure();
}

// 独自の方に有用な振る舞いを追加できる
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_by_structure() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixelx.",
        area_structure(&rect1)
    );
    println!("rect is {:?}", rect1);
}

fn area_structure(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area_by_touple() {
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area_touple(rect1)
    );
}

// タプルには要素に名前がつかない
fn area_touple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_by_normal_variable() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of rectrangle is {} squrare pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

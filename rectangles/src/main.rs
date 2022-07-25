fn main() {
    area_by_normal_variable();
    area_by_touple();
    area_by_structure();
    area_by_method();
}

// 独自の方に有用な振る舞いを追加できる
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area_by_method() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rect4 = Rectangle::square(3);
    println!(
        "The area of the rectangle is {} square pixesls.",
        rect1.area()
    );
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("rect4 is {:#?}", rect4);
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

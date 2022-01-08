#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn width(&self) -> u32{
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // an associated function, which doesn't take self as first argument
    // called like:  Rectangle::square(10)
    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    let rect2 = Rectangle{
        width: 20,
        height: 40,
    };

    let rect3=Rectangle{
        width: 40,
        height:50,
    };

    let square = Rectangle::square(10);


    println!("the width of the rectangle is: {}", rect1.width());

    println!("the area of the rectangle is {} square pixles.",
        rect1.area()
    );

    println!("Can rect1 hold rect rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect rect3: {}", rect1.can_hold(&rect3));

    println!("The area of the square is {}", square.area());


}

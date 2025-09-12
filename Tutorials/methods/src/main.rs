struct Rectangle {
    height: usize,
    width: usize
}

impl Rectangle {
    // Instead of using self: &Self, we can just use &self
    fn area(&self) -> usize{
        self.height * self.width
    }

    fn perimeter(self: &Self) -> usize {
        2 * (self.height + self.width)
    }

    
    // Self here equals Rectangle. Using Self will automatically assign the function signature
    fn square(side: usize) -> Rectangle {
        Self {
            height: side,
            width: side
        }
    }
}

impl Rectangle {
    fn can_hold(self: &Self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    // Just curious about this
    fn can_hold_without_self(myself: &Rectangle, other: &Rectangle) -> bool {
        myself.area() > other.area()
    }
}

fn main() {
    let rec: Rectangle = Rectangle {
        height: 20,
        width: 10
    };

    let rec1: Rectangle = Rectangle {
        height: 20,
        width: 10
    };

    let rec2: Rectangle = Rectangle {
        height: 2,
        width: 4
    };

    println!(
        "The area and perimeter of the rectangle with height: {} and width: {} is {} and {}, respectively",
        rec.height,
        rec.width,
        rec.area(),
        rec.perimeter()
    );

    println!(
        "Can rec1 hold rec2? Answer: {}\nCan rec2 hold rec1? Answer: {}",
        rec1.can_hold(&rec2),
        rec2.can_hold(&rec1)
    );

    println!(
        "Can rec1 hold rec2? Answer: {}\nCan rec2 hold rec1? Answer: {}",
        Rectangle::can_hold_without_self(&rec1, &rec2),
        Rectangle::can_hold_without_self(&rec2, &rec1),
    );

    let square: Rectangle = Rectangle::square(10);

    println!(
        "Rectangle square parametes are {} and {}",
        square.height,
        square.width
    );
}
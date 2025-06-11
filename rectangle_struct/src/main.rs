// OOP 

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}



impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self,other: &Rectangle) -> bool {
        let size1 = self.width * self.height;
        let size2 = other.width * other.height;

        self.width > other.width && self.height > other.height

    }
}

// es würde auch gehen für jede funktion
// sein eigenes impl block zu machen wie:

impl Rectangle {
    fn fun(&self) -> u32 {
        69
    }
}



fn main() {
    let rect = Rectangle {
        width: 20,
        height:30

    };

    println!("The area of the rect is {}",rect.area());
    println!("rect is {rect:#?}");

    let rect2 = Rectangle {
        width: 100,
        height: 100,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 5,
    };
    
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    let funny = rect3.fun();
    println!("{}",funny);

}

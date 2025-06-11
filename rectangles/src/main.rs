
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;

    println!("The are of the rectangle is {} square pixels",
    area(width,height)

    );

    let rect = (30,30);

    let result = area_tuple(rect);

    let rect1 = Rectangle  {
        width: 20,
        height: 25,
    };
    
    // bester weg: 
    println!("{}",area_struct(&rect1));

    println!("rect1 is {rect1:#?}");
}


fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
    
}


fn area_tuple(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}


fn area(width: u32,height: u32) -> u32 {

    width * height
}

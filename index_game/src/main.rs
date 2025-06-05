use std::io;


fn main() {

    let a = [1,2,3,4,5];

    println!("Type a number!");

    let mut index = String::new();

    println!("user: {}",index);


    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    
    let value = a[index];

    println!("The value at index {} is {}",index,value);

}




fn main() {
    let y = 5;
    let y = y + 1;

    let number_1: i32 = 33;

    let z = 2.0;
    let x: f32 = 3.0;

    println!("{}",number_1);
    println!("{}",z);
    println!("{}",x);

    {

        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); // 12
    }
    println!("The value of y in the outer scope is: {y}"); // 6

    // numeric operations
    
    let _sum = 5 + 10;

    let _product = 5 * 2;

    let _difference = 24.2 - 32.12;

    let _qoutient = 56.3 / 32.2;

    let _remainder = 43 % 5;
    
    // booleans
    let _t: bool = true;
    
    let _f: bool = false;

    // chars
    let _c = 'z';
    let _z: char = 'U';

    // tuples
    let tup: (i32,f64,u8) = (500,6.4,1);

    let (_x, _y, _z) = tup;

    println!("The value of x is {}",x);

    // acess value with period 

    let one = tup.2;
    let six_f = tup.1;

    println!("The 3 element of the tuple is: {}",one);
    println!("The 2 element of the tuple is: {}",six_f);
    

    // arrays
    
    // type of array 5 means => 5 elements
    let a: [i32;5] = [1, 2, 3, 4, 5];

    // contain same value for each element
    // this is the same way like writing 
    // let a = [3,3,3,3,3];
    let a = [3;5];

    let months = ["January","February","March", "April"];

    // access value in an array

    let first = a[0];
    let second = a[1];

    println!("The first value of the array is {}",first);
    println!("The second value of the array is {}",second);



}



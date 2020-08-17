fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("main: tup {:?}", tup);

    main2();
    main3();
}

fn main2() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("main2: The value of x is: {}", x);
    println!("main2: The value of y is: {}", y);
    println!("main2: The value of z is: {}", z);
}

fn main3() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("main3: five_hundred [x.0]: {}", five_hundred);
    println!("main3: six_point_four [x.1]: {}", six_point_four);
    println!("main3: one [x.2]: {}", one);

}
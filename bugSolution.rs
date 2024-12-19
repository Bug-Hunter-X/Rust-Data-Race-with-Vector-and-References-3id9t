fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    { //Creating a new scope
        let x = &vec[0];
        println!("x = {}", x);
    }
    vec.push(3); //The reference `x` is now out of scope
    println!("vec = {:?}", vec);
}
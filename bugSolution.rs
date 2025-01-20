fn main() {
    let mut x = 5;
    { // Using a block to limit the scope of the mutable borrow.
        let y = &mut x;
        *y += 1;
    }
    { // Using a separate block for the other mutable borrow
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}
Alternatively, using a different approach
fn main() {
    let mut x = 5;
    let y = x; //Clone x into y
    x += 1; //Modify x
    let z = x; //clone x into z
    x +=1; //Modify x
    println!("x = {}, y = {}, z = {}", x, y, z);
}
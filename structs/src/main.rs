fn main() {
    let width = 30;
    let height = 50;
    let rectangle = (30, 50);
    println!("The area of rectangle is {} square pixels. ", area(rectangle));
}
fn area(dimensions: (i32,i32)) -> i32 {
    dimensions.0 * dimensions.1
}
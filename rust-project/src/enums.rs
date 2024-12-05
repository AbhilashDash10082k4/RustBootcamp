//method 2 for calculating area
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}
fn calc_area(shape:Shape) -> f64{
    //pattern matching
    let ans = match shape {
        Shape::Circle(radius) => 3.14*radius*radius,

        Shape::Square(side) => side*side,

        Shape::Triangle(height, base) => 0.5*base*height
    };
    return ans;
}

fn main() {
    
    //method 1 for calculating area
    circle_ar(7.0);
    triangle_area(6.0, 7.8);
    rectangle_area(78.0, 45.5);

    //method 2 of calculating areas
    let circle = Shape::Circle(7.0);
    let area_circle = calc_area(circle);
    println!("Area of circle2 is {}", area_circle);

    let triangle = Shape::Triangle(49.2, 7.5);
    let area_triangle = calc_area(triangle);
    println!("Area of triangle2 is {}", area_triangle);

    let square = Shape::Square(14.8);
    let area_square = calc_area(square);
    println!("Area of square2 is {}", area_square);
}

//method 1 for calculating areas
fn circle_ar(radius: f64) {
    let ar = 3.14*radius*radius;
    println!("Area of circle1 is {}", ar);
}
fn triangle_area(a:f64, b:f64) {
    let ar = 0.5*a*b;
    println!("Area is of triangle1{}", ar);
}
fn rectangle_area(a:f64, b:f64) {
    let ar = a*b;
    println!("Area of rectangle1 is {}", ar)
}
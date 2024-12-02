#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64, f64),
}

impl Shape{
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Square(s) => s * s,
            Shape::Triangle(a,b,c) => {let s = (a+b+c)/2f64; return (s*(s-a)*(s-b)*(s-c)).sqrt(); },
        }
    }
}

fn triangle_area(a: f64, b: f64, c: f64) -> f64{
    let s = (a+b+c)/2f64; 
    (s*(s-a)*(s-b)*(s-c)).sqrt()
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(300000.0), Shape::Triangle(100f64,100f64,100f64)];
    println!("Triangle: {}", shapes[2].area());
    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            _ => 4.0,
        })
        .sum();
    let total_area2: f64 = shapes
        .iter()
        .map(|shape| shape.area())
        .sum();

    println!("Total area: {} sq. units", total_area);
    println!("Total area 2: {} sq. units", total_area2);
    println!("Area: {} sq. units", Shape::Square(5f64).area());
    println!("Shapes size: {}", shapes.len());

    // shapes.iter().map(|thing| println!("Iter Area: {} sq. units", thing.area()));

    for s in &shapes {
        println!("For Area:{:?} -> {} sq. units", s, s.area());
    }


    let largest = largest_shape(&shapes);
    println!("Largest: {:?}", largest);

}


fn largest_shape(vec: &Vec<Shape>) -> &Shape{
    let mut index = 0usize;
    let mut area = -1f64;
    for i in 0..vec.len() {
        if vec[i].area() > area { 
            area = vec[i].area();
            index = i;
         }
    }
    let result = &vec[index];
    return result;

}

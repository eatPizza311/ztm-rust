// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// Use an enum for the box color
enum BoxColor {
    White,
    Brown,
}

impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::White => println!("White"),
            BoxColor::Brown => println!("Brown"),
        }
    }
}

struct Dimensions {
    depth: f64,
    width: f64,
    height: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}
// Use a struct to encapsulate the box characteristics
// Must include dimensions, weight, and color
struct ShippingBox {
    color: BoxColor,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    // Implement functionality on the box struct to create a new box
    fn new(color: BoxColor, weight: f64, dimensions: Dimensions) -> Self {
        Self {
            color,
            weight,
            dimensions,
        }
    }

    // Implement functionality on the box struct to print the characteristics
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let square_dim: Dimensions = Dimensions {
        depth: 10.0,
        width: 10.0,
        height: 10.0,
    };
    let new_box: ShippingBox = ShippingBox::new(BoxColor::Brown, 25.3, square_dim);
    new_box.print();
}

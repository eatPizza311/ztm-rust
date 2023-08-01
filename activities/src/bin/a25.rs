// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

// Use a trait to declare a perimeter calculation function
trait Perimeter {
    fn calc_perimeter(&self) -> i32;
}

struct Square {
    length: i32,
}

impl Perimeter for Square {
    fn calc_perimeter(&self) -> i32 {
        self.length * 4
    }
}

struct Triangle {
    side1: i32,
    side2: i32,
    side3: i32,
}

impl Triangle {
    fn new(side1: i32, side2: i32, side3: i32) -> Option<Triangle> {
        // Check if the side lengths fulfill the triangle property
        if side1 + side2 > side3 && side1 + side3 > side2 && side2 + side3 > side1 {
            Some(Triangle {
                side1,
                side2,
                side3,
            })
        } else {
            None
        }
    }
}

impl Perimeter for Triangle {
    fn calc_perimeter(&self) -> i32 {
        self.side1 + self.side2 + self.side3
    }
}

// Use a single function to print out the perimeter of the shapes
//   The function must utilize impl trait as a function parameter
fn print_perimeter(shape: impl Perimeter) {
    println!("{:?}", shape.calc_perimeter());
}
fn main() {
    print_perimeter(Square { length: 3 });
    print_perimeter(Triangle::new(3, 4, 5).expect("This is a valid triangle!"));
}

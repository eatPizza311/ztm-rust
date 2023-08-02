// Topic: Generics & Functions
//
// Requirements:
// * Create a function that accepts the Priority trait as a generic parameter
//   * The function should print out the guest and their priority
// * Use the function with at least two different guests
//
// Notes:
// * Use the debug token "{:?}" to print out the information
// * Use the compiler to guide you to the correct generic constraints needed

#[derive(Debug)]
enum ServicePriority {
    High,
    Standard,
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGuest;
impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}

#[derive(Debug)]
struct Guest;
impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

// Create a function that accepts the Priority trait as a generic parameter
//   The function should print out the guest and their priority
fn print_priority<T>(guest: T)
where
    T: Priority + std::fmt::Debug,
{
    println!("{:?} is a {:?} priority", guest, guest.get_priority())
}

fn main() {
    let guest1 = Guest {};
    let guest2 = ImportantGuest {};
    print_priority(guest1);
    print_priority(guest2);
}

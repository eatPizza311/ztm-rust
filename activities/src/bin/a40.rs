// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum VehicleType {
    Bus,
    Car,
}

#[derive(Debug, Hash, PartialOrd, PartialEq)]
enum VehicleStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}


#[derive(Debug)]
struct Vehicle {
    vehicle: VehicleType,
    vin: String,
    status: VehicleStatus,
}

type Rental = Rc<RefCell<Vec<Vehicle>>>;

struct Corporate(Rental);

struct StoreFront(Rental);

fn main() {
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_status() {
        let vehicles = vec![
            Vehicle {
                vehicle: VehicleType::Bus,
                status: VehicleStatus::Available,
                vin: "123".to_owned(),
            },
            Vehicle {
                vehicle: VehicleType::Car,
                status: VehicleStatus::Maintenance,
                vin: "abc".to_owned(),
            }
        ];

        let vehicles = Rc::new(RefCell::new(vehicles));

        let corporate = Corporate(Rc::clone(&vehicles));
        let storefront = StoreFront(Rc::clone(&vehicles));

        {
            let mut rentals = storefront.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, VehicleStatus::Available);
                car.status = VehicleStatus::Rented;
            }
        }

        {
            let mut rentals = corporate.0.borrow_mut();
            if let Some(car) = rentals.get_mut(1) {
                assert_eq!(car.status, VehicleStatus::Maintenance);
                car.status = VehicleStatus::Available;
            }
        }

        let rentals = storefront.0.borrow();
        if let Some(car) = rentals.get(0) {
            assert_eq!(car.status, VehicleStatus::Rented)
        }

    } 
}

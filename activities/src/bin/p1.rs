// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
use std::collections::HashMap;
use std::io;

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again")
    }
    let input = buffer.trim().to_owned();
    if input.as_str() != "" {
        Some(input)
    } else {
        None
    }
}

fn get_bill_amunt() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a float!"),
        }
    }
}

mod menu {
    use crate::{get_bill_amunt, get_input, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name: ");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amunt() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill added")
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
    }

    pub fn delete_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
        println!("Bill name to delete:");
        let name: String = match get_input() {
            Some(input) => input,
            None => return,
        };
        if bills.delete(&name) {
            println!("Bill {:?} deleted", name);
        } else {
            println!("The entry doesn't exist");
        }
    }

    pub fn modify_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
        let name: String = match get_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amunt() {
            Some(input) => input,
            None => return,
        };
        if bills.modify(&name, amount) {
            println!("Bill {:?} modified", name);
        } else {
            println!("The entry doesn't exist")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn delete(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn modify(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
    DeleteBill,
    ModifyBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::DeleteBill),
            "4" => Some(Self::ModifyBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("");
        println!("Enter selection:");
    }
}

fn run_program() -> Option<()> {
    let mut all_bills = Bills::new();

    loop {
        MainMenu::show();
        let input: String = get_input()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut all_bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&all_bills),
            Some(MainMenu::DeleteBill) => menu::delete_bill(&mut all_bills),
            Some(MainMenu::ModifyBill) => menu::modify_bill(&mut all_bills),
            None => break,
        }
    }
    None
}

fn main() {
    run_program();
}

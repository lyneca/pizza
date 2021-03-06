extern crate rand;

use std::io::Write;
use std::io;

/// Struct for representing a pizza order
struct Order {
    dbc: u16,  // Double Bacon Cheeseburger
    sup: u16,  // Supreme
    pep: u16,  // Pepperoni
    bao: u16,  // Beef and Onion
    haw: u16,  // Hawaiian
    che: u16,  // Cheese
    mar: u16,  // Margherita
    bpo: u16,  // BBQ Pork and Onion
    bml: u16,  // BBQ Meatlovers
    eme: u16,  // Eight Meats
    veg: u16,  // Vegetarian
    gar: u16,  // Garlic Bread
    dri: u16,  // Drinks
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Order:\n").unwrap();
        write!(f, "  - Double Bacon Cheeseburger: {}\n", self.dbc).unwrap();
        write!(f, "  - Pepperoni: {}\n", self.pep).unwrap();
        write!(f, "  - Supreme: {}\n", self.sup).unwrap();
        write!(f, "  - Beef and Onion: {}\n", self.bao).unwrap();
        write!(f, "  - Hawaiian: {}\n", self.haw).unwrap();
        write!(f, "  - Cheese: {}\n", self.che).unwrap();
        write!(f, "  - Margherita: {}\n", self.mar).unwrap();
        write!(f, "  - BBQ Pork and Onion: {}\n", self.bpo).unwrap();
        write!(f, "  - BBQ Meatlovers: {}\n", self.bml).unwrap();
        write!(f, "  - Eight Meats: {}\n", self.eme).unwrap();
        write!(f, "  - Vegetarian: {}\n", self.veg).unwrap();
        write!(f, "  - Garlic Bread: {}\n", self.gar).unwrap();
        write!(f, "  - Drinks: {}\n", self.dri)
    }
}

impl Order {
    /// Returns the number of pizzas in the order
    fn sum(&self) -> u16 {
        self.dbc
            + self.sup
            + self.pep
            + self.bao
            + self.haw
            + self.che
            + self.mar
            + self.bpo
            + self.bml
            + self.eme
            + self.veg
    }
}

/// Generates an order from the number of people
///  and number of vegetarians
///
/// 1. A number of each pizza type is added to the order
///    in a particular ratio
/// 2. Pizzas are added randomly from the set of pizza types
///    from the lowest ratio (i.e. the pizzas that occur the
///    least frequently in the order)
fn gen_order(people: u16, veg: u16) -> Order {
    // 15 is chosen because it is what the ratios add up to
    let scale = people as f32 / 15.0;

    // 2 is chosen because only cheese and vegetarian are
    //  vegetarian-friendly
    let veg_scale = veg as f32 / 4.0;

    let total: f32 = (people + veg) as f32;

    println!("{} {}", people, veg);

    // Add pizzas to order with arbitrary preferences
    let mut order = Order {
        dbc: (scale  * 3.0).floor() as u16,
        sup: (scale  * 2.0).floor() as u16,
        pep: (scale  * 2.0).floor() as u16,
        bao: (scale  * 2.0).floor() as u16,
        haw: (scale  * 1.0).floor() as u16,
        // Cheese pizza is vegetarian, so half of the vego
        //  order is cheese
        che: ((scale * 1.0).floor() + veg_scale.floor()) as u16,
        mar: (scale  * 1.0).floor() as u16,
        bpo: (scale  * 1.0).floor() as u16,
        bml: (scale  * 1.0).floor() as u16,
        eme: (scale  * 1.0).floor() as u16,

        // Vegetarian
        veg: (veg_scale * 3.0).ceil() as u16,

        // Extras
        gar: (scale       * 4.0).ceil() as u16,
        dri: (total / 5.0 * 2.0).ceil() as u16,
    };

    // Add pizzas randomly until the quota is reached
    //  Currently this only adds from the 1x modifier pizzas
    //  (for more variety)
    while order.sum() + 1 <= people + veg {
        match rand::random::<u8>() % 6 {
            0 => order.haw += 1,
            1 => order.che += 1,
            2 => order.mar += 1,
            3 => order.bpo += 1,
            4 => order.bml += 1,
            5 => order.eme += 1,
            _ => {}
        }
    }
    order
}

/// Generates the number of pizzas per person
fn num_pizzas(people: u16) -> u16 {
    (people as f32 * (2.0 / 3.0)).floor() as u16
}

/// Generates the number of pizzas per vegetarian, rounding up
fn num_veg_pizzas(people: u16) -> u16 {
    (people as f32 * (2.0 / 3.0)).ceil() as u16
}

fn main() {
    // Get number of carnivores
    print!("How many carnivores? ");
    io::stdout().flush().expect("Couldn't flush");
    let mut num_people: String = String::from("");
    io::stdin().read_line(&mut num_people).expect("Could not read input.");
    let num_people: u16 = num_people.trim().parse().expect("Invalid number.");

    // Get number of vegetarians
    print!("How many vegetarians? ");
    io::stdout().flush().expect("Couldn't flush");
    let mut veg: String = String::from("");
    io::stdin().read_line(&mut veg).expect("Could not read input.");
    let num_veg: u16 = veg.trim().parse().expect("Invalid number.");

    // Generate the order
    let order = gen_order(num_pizzas(num_people), num_veg_pizzas(num_veg));

    // Print it
    println!("{}", order);
    println!("Total: {}, {:.2} per person", order.sum(), order.sum() as f32 / (num_people + num_veg) as f32);
}

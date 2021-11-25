//tuples group ogether values of different types
//it has a max of 12 elements

pub fn run() {
let person:(&str, i8, &str) = ("Bukola", 12, "Food");

// println!("(:?)", person);
println!("{} is {} and she likes {}", person.0, person.1, person.2);
}
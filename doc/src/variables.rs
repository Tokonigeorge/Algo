//In rust, unlike js variables are immutable but it is also a block scope

pub fn run() {
//our tuple example from hello
  let long_tuple = (12, true, "yes");
    println!("{:?}",long_tuple);
    let name = "Bukola";
    println!("Name is, {}", name);
    //think of the let in Rst like const in js and one can make a variable mutable by adding the mut
    let mut age = 37;
      println!("My age is {}", age);
    age = 38;
    println!("My age is {}", age);

    //CONST KEYWORD
    //With constants, you have to explicitly define a type and in UPPERCASE :)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assig multiple variables
    let (my_name, my_age) = ("Bukola", 37);
    println!("My name is {}, and I'm {} years old", my_name, my_age);
}
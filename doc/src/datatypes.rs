//Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
//floats: f32, f64
//boolean
//characters
//tuples
//arrays
//Rust is a statically typed language, so it's not required to set the type of all variables as the compiler usually infers the type from how it is used 

pub fn run() {
//setting an explicit type
let _x: i64 = 45;

//to find the max sizes on th integers
// println!("Max i64: {}", std::i64:MAX);
println!("Max i32: {}",std::i32::MAX );
//bool
let is_true = true;
let is_false: bool = false;

let is_greater: bool = 10 > 5;
println!("{:?}", (is_true, is_false, is_greater));

//signifying characters are done in single quotes for a single char
let single_char = 'c';
let smiley = '\u{1F600}'; //works because it is a unicode
// let multiple_char: char = "charr";
println!("{:?}", (single_char, smiley));
}
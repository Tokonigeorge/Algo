//there are two types of strings, the immutable fixed length string and the mutable.
pub fn run () {
    let immutable_string = "Hey, I'm an immutable string";
    let mut mutable_string = String::from("I am mutable ");
    //the push method only works for characters
    mutable_string.push('a');
    //to push a string
    mutable_string.push_str("nd I'm also changing");

    //get length of string
    //get the capacity of a string
    //check if a string is empty
    println!("The length of the string is {} and {}, the first string is {} and the capacity is {}, is empty {}", immutable_string.len(), 
    mutable_string.len(), mutable_string, mutable_string.capacity(), mutable_string.is_empty());
    //strings can be replaced, searched through if it contains, looped through
    for word in mutable_string.split_whitespace() {
        println!("The word is {}", word);
    }
    //create a string with capacity
    let mut capacity_string = String::with_capacity(10);
    capacity_string.push('a');
    capacity_string.push('b');

    //Assertion testing
    assert_eq!(2, capacity_string.len())
    //this wouldn't print a true or false, It'll only faill if the left is != right.
   
}
pub fn run() {
    //"pub" to indicate it is a public function and we run it
    println!("hello world");
    
    //BASIC FORMATTING
    //printing integers are a different case, you need to provide a placeholder
    //with the {} identifier which has to be in a ""
    println!("{}", 1);
    //now for instance, I want to pass on two variables, for that we need placeholders, whether they be string or number variables
    println!("My name is {} and I like {}", "Bukola", "Anime");

    //POSITIONAL ARGUEMENTS
    //Now if "Bukola" is to be repeated in the strings, i.e My name is Bukola and Bukola likes anime, we use positional arguments, instead of
    //defining a variable twice, the first argument position has an index of zero, soo
    println!("My name is {0} and {0} likes {1}", "Bukola", "Anime");

    //NAMED ARGUEMENTS
    //they work a lot like defining variables
    println!("My name is {name} and I like {hobby}", name = "Bukola", hobby = "Anime");

    //Placeholder traits
    //placeholders cam also be converted based on their base system directly through the placeholders
    println!("Binary: {:b}, octal: {:o}, hex: {:x}",10, 10, 10);

    //placeholder for debug, think putting all values into some sort of array but this is called a tuple, a collection of values of
    //different types
    let long_tuple = (12, true, "yes");
    println!("{:?}",long_tuple);
    //extracting a value from the tuple, 
    println!("{}", long_tuple.0);
    //MATHS
    println!("{}, {}, {}", 10 + 10, 10 * 10, 10 % 10);
}
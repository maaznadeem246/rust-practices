// rust have two ways to use string type like one is primitive string which is fixed lenght in memory
// second is which we can change and re-define

pub fn run(){
    // this is primitive string with is fixed length 
    let _hello = "Hello";

    // this string we ca re-define or change the length
    let mut hello = String::from("Hello ");
    
    // Get length
    println!("{}",hello.len());

    //Push char 
    hello.push('w');

    //Push String
    hello.push_str("orld!");
    
    // Capacity in bytes
    println!("Capacity: {}",hello.capacity());

    // Check if Empty
    println!("Is Empty: {}",hello.is_empty());

    // conatains
    println!("Contains 'world' {}",hello.contains("world"));

    // Replace
    println!("Replace: {}",hello.replace( "world","There" ));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10); 
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}",s);



     
}
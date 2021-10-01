
pub fn run(){
    let age: u8 = 22;
    let check_id: bool= true;
    let knows_person_of_age = true;

    // IF/ELESE
    if age>=21 && check_id || knows_person_of_age {
        println!("Bradtender : What would you like to drink?");
    }else if age < 21 && check_id {
        println!("Bradtender : Sorry you have to leave");
    }else{
        println!("Bradtender : I'll need to se your ID");
    }

    // Shorthand IF

    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age: {}",is_of_age)

}
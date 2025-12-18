fn main() {
    // mutability();
    constants();
    shadowing();
}

fn mutability() {
    // All variables all inmutable by default

    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
}

fn constants() {
    // Calculated on compile-time
    // Can be decalred even in global scope

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    {
        println!("Constatn from inner scope: {THREE_HOURS_IN_SECONDS}");
    }
}

fn shadowing() {
    // Usefull to modify the value but maintain the variable inmutable after certain point

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

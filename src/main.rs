fn main() {

    // Data Types in RUST

    //! RUST is statically typd language 
    // i means signed and u means unsigned , i can store negative , u can stored only positive.

    // i8 range  = -128 to 127 , that means we cannot store the number more than it
    // u9 range = 0 to 255 , 
    // let a: i8 = 127;

    // we can also type infer like this,
    let a = 30i8 ;// this will automatic infer a variable as i8
    let s = 30_i8 ;// underscore is for better visualization
    
    println!("Value of S is {s} and a is {a}") ;


    // let add : u8 = random_number() + 56 ; 



    //this is overflowing value concept in RUST
    // For example : If a value is out of range , then it will increment the value respective of it in --release mode
    // However it will be panicked in development mode
    //256 == 0 , 257 == 1 , 258 == 2 and so on

    // in order to run it the same on dev mode, we can explicitely handles this by wrapping_* methods like wrapping_add(),
    

    let wrapped_add :u8 = random_number().wrapping_add(56) ;

    println!("With Wrapping method {wrapped_add}");
}


fn random_number() -> u8 {
 200
}
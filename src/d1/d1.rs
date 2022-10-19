mod variable;
mod format;
mod debug;

pub fn say_hello(){
    println!("hello");
}

pub fn print_var(){
    variable::main();
}

pub fn print_format(){
    format::main();
}

pub fn print_debug(){
    debug::main();
}

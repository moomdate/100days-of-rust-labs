pub fn main() {

    // general
    println!("{} days,{} days,{} days", 1, 2, 3);

    // positional
    println!("{0} days,{1} days,{0} days",1, 99);

    // named arguments
    println!("title is {subject}", subject="text title");

    // format character
    println!("Base 10 repr:               {}",   69420);
    println!("Base 10 repr:               {:b}",   69420);
    println!("Base 10 repr:               {:o}",   69420);

    // right align
    println!("{number:>1}", number=1);
    println!("{number:>2}", number=1);
    println!("{number:>3}", number=1);

    // pad numbers
    println!("{number:0>3}", number=100);


    println!("{number:0>width$}", number=9, width=5);

    // println!("My name is {0}, {1} {0}", "Bond");

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

}

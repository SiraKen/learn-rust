mod chapter1;
mod chapter2;
mod chapter3;
mod chapter4;
mod chapter5;
mod sample_module;

fn main() {
    println!("Hello");
    chapter1::__main__();
    println!("\n==============================\n");
    chapter2::__main__();
    println!("\n==============================\n");
    chapter3::__main__();
    println!("\n==============================\n");
    chapter4::__main__();
    println!("\n==============================\n");
    chapter5::__main__();

    println!("\n==============================\n");
    println!("{}", sample_module::greet::english("John"));
}


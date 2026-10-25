mod processing;

fn main() {
    let notes = vec![12,22,11,2];
    println!("Hello, world!");
    processing::clean();
    println!("la moyeen des not est {}", processing::mean(&notes));
}

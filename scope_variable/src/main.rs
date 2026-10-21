fn main() {
let mut x = "hello";
let  name = &mut x;
*name = "guys";
println!("{}", x);
let name = String::from("Amadou diallo");
fn first_name(name: &String){
    println!("{name}")
}
first_name(&name);
println!("{name}");
{
    let val: i32 = 33;
    reference = &val;
}
println!("{}", reference);
}

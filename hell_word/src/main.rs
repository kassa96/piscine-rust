fn main() {
    let a = 12;
    let a = a +2;
    {
        let a = a +3;
            println!("a scope = {a}");

    }
    println!("a = {a}");
   let spaces = "    ";
   let spaces = spaces.len();
   println!("nombre space: {spaces}");
   let tup: (i32, f64, u8) = (-12, 0.4, 1);
   let (x1, x2, x3) = tup;
   println!("{x1} - {x2} - {x3}");

}


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
   let y = {
    let x = 3;
    x + 6
   };
   println!("y = {y}");
   let number = 12;
   let calcul = if number%2 == 0 {number/2} else {number*3 + 1};
   println!("calcul: {calcul}");
   let mut counter = 1;
   let  somme = loop {
    counter += 1;
    if counter == 10{
        break counter*10
    }

   };
   println!("value counter after loop is {somme}");
   let colors = ["yellow", "green", "red", "black", "white"];
   let mut index = 0;
   while index < colors.len(){
    println!("color: {}", colors[index]);
    index += 1;

   }
}

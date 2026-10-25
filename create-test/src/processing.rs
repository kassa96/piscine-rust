pub fn clean(){
    println!("clean");
}
pub fn mean(tab: &[i32]) -> f64{
    let somme: i32 = tab.iter().sum();
    (somme as f64 / tab.len() as f64) as f64
}
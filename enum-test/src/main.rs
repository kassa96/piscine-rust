enum Status{
    Running,
    Complete(i32),
    Fail(String)
}

fn main() {
let status_complete = Status::Complete(12);
fn display_status(status: &Status) {
    match status {
        Status::Running => {
            println!("running");
        }
        Status::Complete(val)=> {
            println!("complete: {}",val);
        } 
        Status::Fail(val) => {
            println!("fail: {}",val);
        }
    }
}
display_status(&status_complete);
let age: Option<u32> = Some(12);
match age {
    Some(value) => println!("your age is {value}"),
    None => println!("no age is given")
}
fn find_user(user_id: i32) -> Option<String>{
    if user_id == 1 {
        Some(String::from("Kassa"))
    }else{
        None
    }
}
let user = find_user(2);

}

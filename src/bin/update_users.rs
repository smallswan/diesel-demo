use diesel_demo::*;

fn main() {
    let result = update_users();
    assert_eq!(Ok(0), result);
    println!("there isn't post which id eq 1");
}

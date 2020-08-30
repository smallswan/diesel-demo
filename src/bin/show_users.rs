use diesel_demo::*;

fn main() {
    let users = some_users();
    for user in users {
        println!(
            "id:{},name:{},hair color:{:?}, created at :{:?}",
            user.id, user.name, user.hair_color, user.created_at
        );
    }

    println!("---------------------");

    let users = all_users();
    for user in users.unwrap() {
        println!(
            "id:{},name:{},hair color:{:?}, created at :{:?}",
            user.id, user.name, user.hair_color, user.created_at
        );
    }
}

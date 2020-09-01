use diesel::prelude::*;

use diesel_demo::*;
use std::env::args;
fn main() {
    use schema::posts::dsl::*;
    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);
    let connection = establish_connection();

    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);

    let num_deleted = diesel::delete(posts.filter(id.eq(1)))
        .execute(&connection)
        .expect("Error deleting post[id=1]");
    println!("Deleted post which id = 1 , is Ok : {}", num_deleted == 1);
}

extern crate diesel;
extern crate diesel_demo;

use self::diesel_demo::*;

fn main() {
    let connect = establish_connection();
    let result = insert_default_values(&connect);

    match result {
        Ok(row) => {
            println!("affected row : {}", row);
        }
        Err(e) => eprintln!("some error : {:?}", e),
    };

    let affected_row = insert_single_column(&connect).unwrap();
    println!("insert_single_column affected_row = {}", affected_row);

    let affected_row = insert_multiple_columns(&connect).unwrap();
    println!("insert_multiple_columns affected_row = {}", affected_row);

    insert_insertable_struct(&connect).unwrap();

    insert_insertable_struct_option(&connect).unwrap();

    insert_single_column_batch(&connect).unwrap();

    let error = insert_single_column_batch_with_default(&connect).unwrap_err();
    println!("insert_single_column_batch_with_default  {}", error);

    let affected_row = insert_tuple_batch(&connect).unwrap();
    assert_eq!(affected_row, 2);

    let affected_row = insert_tuple_batch_with_default(&connect).unwrap();
    assert_eq!(affected_row, 2);

    insert_insertable_struct_batch(&connect).unwrap();

    let new_id = explicit_returning(&connect).unwrap();
    println!("return id = {}", new_id);
}

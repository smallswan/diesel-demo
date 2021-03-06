#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::{NewPost, Post};
use diesel::debug_query;
use diesel::insert_into;
use diesel::mysql::Mysql;
use diesel::prelude::*;

use chrono::NaiveDateTime;
use schema::users;
use serde_derive::Deserialize;

use dotenv::dotenv;
use std::env;
use std::error::Error;

#[derive(QueryableByName, Queryable, PartialEq, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserForm<'a> {
    name: &'a str,
    hair_color: Option<&'a str>,
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &MysqlConnection, title: &str, body: &str) -> Post {
    use schema::posts;
    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    posts::table.order(posts::id.desc()).first(conn).unwrap()
}

pub fn insert_default_values(conn: &MysqlConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users).default_values().execute(conn)
}

#[test]
fn examine_sql_from_insert_default_values() {
    use schema::users::dsl::*;

    let query = insert_into(users).default_values();
    let sql = "INSERT INTO `users` () VALUES () -- binds: []";
    assert_eq!(sql, debug_query::<Mysql, _>(&query).to_string());
}

pub fn insert_single_column(conn: &MysqlConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users).values(name.eq("Sean")).execute(conn)
}

#[test]
fn examine_sql_from_insert_single_column() {
    use schema::users::dsl::*;

    let query = insert_into(users).values(name.eq("Sean"));
    let sql = "INSERT INTO `users` (`name`) VALUES (?) \
               -- binds: [\"Sean\"]";
    assert_eq!(sql, debug_query::<Mysql, _>(&query).to_string());
}

pub fn insert_multiple_columns(conn: &MysqlConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users)
        .values((name.eq("Tess"), hair_color.eq("Brown")))
        .execute(conn)
}

#[test]
fn examine_sql_from_insert_multiple_columns() {
    use schema::users::dsl::*;

    let query = insert_into(users).values((name.eq("Tess"), hair_color.eq("Brown")));
    let sql = "INSERT INTO `users` (`name`, `hair_color`) VALUES (?, ?) \
               -- binds: [\"Tess\", \"Brown\"]";
    assert_eq!(sql, debug_query::<Mysql, _>(&query).to_string());
}

pub fn insert_insertable_struct(conn: &MysqlConnection) -> Result<(), Box<dyn Error>> {
    use schema::users::dsl::*;

    let json = r#"{ "name": "Sean", "hair_color": "Black" }"#;
    let user_form = serde_json::from_str::<UserForm>(json)?;

    insert_into(users).values(&user_form).execute(conn)?;

    Ok(())
}

#[test]
fn examine_sql_from_insertable_struct() {
    use schema::users::dsl::*;

    let json = r#"{ "name": "Sean", "hair_color": "Black" }"#;
    let user_form = serde_json::from_str::<UserForm>(json).unwrap();
    let query = insert_into(users).values(&user_form);
    let sql = "INSERT INTO `users` (`name`, `hair_color`) VALUES (?, ?) \
               -- binds: [\"Sean\", \"Black\"]";
    assert_eq!(sql, debug_query::<Mysql, _>(&query).to_string());
}

pub fn insert_insertable_struct_option(conn: &MysqlConnection) -> Result<(), Box<dyn Error>> {
    use schema::users::dsl::*;

    let json = r#"{ "name": "Ruby", "hair_color": null }"#;
    let user_form = serde_json::from_str::<UserForm>(json)?;

    insert_into(users).values(&user_form).execute(conn)?;

    Ok(())
}

#[test]
fn examine_sql_from_insertable_struct_option() {
    use schema::users::dsl::*;

    let json = r#"{ "name": "Ruby", "hair_color": null }"#;
    let user_form = serde_json::from_str::<UserForm>(json).unwrap();
    let query = insert_into(users).values(&user_form);
    let sql = "INSERT INTO `users` (`name`, `hair_color`) VALUES (?, DEFAULT) \
               -- binds: [\"Ruby\"]";
    assert_eq!(sql, debug_query::<Mysql, _>(&query).to_string());
}

pub fn insert_single_column_batch(conn: &MysqlConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users)
        .values(&vec![name.eq("Sean"), name.eq("Tess")])
        .execute(conn)
}

#[test]
fn examine_sql_from_insert_single_column_batch() {
    use schema::users::dsl::*;

    let values = vec![name.eq("Sean"), name.eq("Tess")];
    let query = insert_into(users).values(&values);
    let sql = "INSERT INTO `users` (`name`) VALUES (?), (?) \
               -- binds: [\"Sean\", \"Tess\"]";
    assert_eq!(sql, debug_query::<Mysql, _>(&query).to_string());
}

pub fn insert_single_column_batch_with_default(conn: &MysqlConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users)
        .values(&vec![Some(name.eq("Sean")), None])
        .execute(conn)
}

#[test]
fn examine_sql_from_insert_single_column_batch_with_default() {
    use schema::users::dsl::*;

    let values = vec![Some(name.eq("Sean")), None];
    let query = insert_into(users).values(&values);
    let sql = "INSERT INTO `users` (`name`) VALUES (?), (DEFAULT) \
               -- binds: [\"Sean\"]";
    assert_eq!(sql, debug_query::<Mysql, _>(&query).to_string());
}

pub fn insert_tuple_batch(conn: &MysqlConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users)
        .values(&vec![
            (name.eq("Sean"), hair_color.eq("Black")),
            (name.eq("Tess"), hair_color.eq("Brown")),
        ])
        .execute(conn)
}

#[test]
fn examine_sql_from_insert_tuple_batch() {
    use schema::users::dsl::*;

    let values = vec![
        (name.eq("Sean"), hair_color.eq("Black")),
        (name.eq("Tess"), hair_color.eq("Brown")),
    ];
    let query = insert_into(users).values(&values);
    let sql = "INSERT INTO `users` (`name`, `hair_color`) \
               VALUES (?, ?), (?, ?) \
               -- binds: [\"Sean\", \"Black\", \"Tess\", \"Brown\"]";
    assert_eq!(sql, debug_query::<Mysql, _>(&query).to_string());
}

pub fn insert_tuple_batch_with_default(conn: &MysqlConnection) -> QueryResult<usize> {
    use schema::users::dsl::*;

    insert_into(users)
        .values(&vec![
            (name.eq("Sean"), Some(hair_color.eq("Black"))),
            (name.eq("Ruby"), None),
        ])
        .execute(conn)
}

#[test]
fn examine_sql_from_insert_tuple_batch_with_default() {
    use schema::users::dsl::*;

    let values = vec![
        (name.eq("Sean"), Some(hair_color.eq("Black"))),
        (name.eq("Ruby"), None),
    ];
    let query = insert_into(users).values(&values);
    let sql = "INSERT INTO `users` (`name`, `hair_color`) \
               VALUES (?, ?), (?, DEFAULT) \
               -- binds: [\"Sean\", \"Black\", \"Ruby\"]";
    assert_eq!(sql, debug_query::<Mysql, _>(&query).to_string());
}

pub fn insert_insertable_struct_batch(conn: &MysqlConnection) -> Result<(), Box<dyn Error>> {
    use schema::users::dsl::*;

    let json = r#"[
        { "name": "Sean", "hair_color": "Black" },
        { "name": "Tess", "hair_color": "Brown" }
    ]"#;
    let user_form = serde_json::from_str::<Vec<UserForm>>(json)?;

    insert_into(users).values(&user_form).execute(conn)?;

    Ok(())
}

#[test]
fn examine_sql_from_insertable_struct_batch() {
    use schema::users::dsl::*;

    let json = r#"[
        { "name": "Sean", "hair_color": "Black" },
        { "name": "Tess", "hair_color": "Brown" }
    ]"#;
    let user_form = serde_json::from_str::<Vec<UserForm>>(json).unwrap();
    let query = insert_into(users).values(&user_form);
    let sql = "INSERT INTO `users` (`name`, `hair_color`) \
               VALUES (?, ?), (?, ?) \
               -- binds: [\"Sean\", \"Black\", \"Tess\", \"Brown\"]";
    assert_eq!(sql, debug_query::<Mysql, _>(&query).to_string());
}

#[test]
fn insert_get_results_batch() {
    use diesel::result::Error;

    let conn = establish_connection();
    conn.test_transaction::<_, Error, _>(|| {
        use diesel::select;
        use schema::users::dsl::*;

        let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(&conn)?;

        let inserted_users = conn.transaction::<_, Error, _>(|| {
            let inserted_count = insert_into(users)
                .values(&vec![
                    (id.eq(1), name.eq("Sean")),
                    (id.eq(2), name.eq("Tess")),
                ])
                .execute(&conn)?;

            Ok(users
                .order(id.desc())
                .limit(inserted_count as i64)
                .load(&conn)?
                .into_iter()
                .rev()
                .collect::<Vec<_>>())
        })?;

        let expected_users = vec![
            User {
                id: 1,
                name: "Sean".into(),
                hair_color: None,
                created_at: now,
                updated_at: now,
            },
            User {
                id: 2,
                name: "Tess".into(),
                hair_color: None,
                created_at: now,
                updated_at: now,
            },
        ];
        assert_eq!(expected_users, inserted_users);

        Ok(())
    });
}

#[test]
fn examine_sql_from_insert_get_results_batch() {
    use schema::users::dsl::*;

    let values = vec![(id.eq(1), name.eq("Sean")), (id.eq(2), name.eq("Tess"))];
    let insert_query = insert_into(users).values(&values);
    let insert_sql = "INSERT INTO `users` (`id`, `name`) VALUES (?, ?), (?, ?) \
                      -- binds: [1, \"Sean\", 2, \"Tess\"]";
    assert_eq!(
        insert_sql,
        debug_query::<Mysql, _>(&insert_query).to_string()
    );
    let load_query = users.order(id.desc());
    let load_sql = "SELECT `users`.`id`, `users`.`name`, \
                    `users`.`hair_color`, `users`.`created_at`, \
                    `users`.`updated_at` \
                    FROM `users` \
                    ORDER BY `users`.`id` DESC \
                    -- binds: []";
    assert_eq!(load_sql, debug_query::<Mysql, _>(&load_query).to_string());
}

#[test]
fn insert_get_result() {
    use diesel::result::Error;

    let conn = establish_connection();
    conn.test_transaction::<_, Error, _>(|| {
        use diesel::select;
        use schema::users::dsl::*;

        let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(&conn)?;

        let inserted_user = conn.transaction::<_, Error, _>(|| {
            insert_into(users)
                .values((id.eq(3), name.eq("Ruby")))
                .execute(&conn)?;

            users.order(id.desc()).first(&conn)
        })?;

        let expected_user = User {
            id: 3,
            name: "Ruby".into(),
            hair_color: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(expected_user, inserted_user);

        Ok(())
    });
}

#[test]
fn examine_sql_from_insert_get_result() {
    use schema::users::dsl::*;

    let insert_query = insert_into(users).values((id.eq(3), name.eq("Ruby")));
    let insert_sql = "INSERT INTO `users` (`id`, `name`) VALUES (?, ?) -- binds: [3, \"Ruby\"]";
    assert_eq!(
        insert_sql,
        debug_query::<Mysql, _>(&insert_query).to_string()
    );
    let load_query = users.order(id.desc());
    let load_sql = "SELECT `users`.`id`, `users`.`name`, \
                    `users`.`hair_color`, `users`.`created_at`, \
                    `users`.`updated_at` \
                    FROM `users` \
                    ORDER BY `users`.`id` DESC \
                    -- binds: []";
    assert_eq!(load_sql, debug_query::<Mysql, _>(&load_query).to_string());
}

pub fn explicit_returning(conn: &MysqlConnection) -> QueryResult<i32> {
    use diesel::result::Error;
    use schema::users::dsl::*;

    conn.transaction::<_, Error, _>(|| {
        insert_into(users).values(name.eq("Ruby")).execute(conn)?;

        users.select(id).order(id.desc()).first(conn)
    })
}

#[test]
fn examine_sql_from_explicit_returning() {
    use schema::users::dsl::*;

    let insert_query = insert_into(users).values(name.eq("Ruby"));
    let insert_sql = "INSERT INTO `users` (`name`) VALUES (?) -- binds: [\"Ruby\"]";
    assert_eq!(
        insert_sql,
        debug_query::<Mysql, _>(&insert_query).to_string()
    );
    let load_query = users.select(id).order(id.desc());
    let load_sql = "SELECT `users`.`id` FROM `users` ORDER BY `users`.`id` DESC -- binds: []";
    assert_eq!(load_sql, debug_query::<Mysql, _>(&load_query).to_string());
}

pub fn some_users() -> Vec<User> {
    let connection = establish_connection();

    use self::schema::users::dsl::*;
    let all_name = users.select(name).load::<String>(&connection).unwrap();

    println!("all_name : {:?}", all_name);

    let distinct_name = users
        .select(name)
        .distinct()
        .load::<String>(&connection)
        .unwrap();

    println!("distinct_name : {:?}", distinct_name);

    let count = users.count().execute(&connection).unwrap();
    println!("there are {} users ?", count);

    let count2: i64 = users.count().get_result::<i64>(&connection).unwrap();
    println!("there are {} users !", count2);

    users
        .order((created_at.desc(), id.desc()))
        .filter(name.eq("Ruby"))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users")
}

pub fn all_users() -> QueryResult<Vec<User>> {
    use diesel::sql_query;
    let connection = establish_connection();
    let users = sql_query("SELECT * FROM users ORDER BY id").load(&connection);
    users
}

pub fn delete_all_users() -> usize {
    use schema::users::dsl::*;
    let connection = establish_connection();
    let delete_user_num = diesel::delete(users).execute(&connection).unwrap();
    delete_user_num
}

pub fn update_users() -> QueryResult<usize> {
    use schema::users::dsl::*;
    let connection = establish_connection();

    let updated_row = diesel::update(users.filter(name.eq("Rust")))
        .set((name.eq("Ruby"), hair_color.eq(Some("yellow"))))
        .execute(&connection);

    println!("update Ruby to Rust, updated_row : {:?}", updated_row);

    let updated_row = diesel::update(users.filter(id.eq(1)))
        .set(name.eq("James"))
        .execute(&connection);

    updated_row
}

pub fn replace_into_users() {
    use self::schema::users::dsl::*;
    let connection = establish_connection();

    diesel::replace_into(users)
        .values(&vec![
            (id.eq(1), name.eq("Sean2")),
            (id.eq(2), name.eq("Tess2")),
        ])
        .execute(&connection)
        .unwrap();

    diesel::replace_into(users)
        .values((id.eq(1), name.eq("Jim")))
        .execute(&connection)
        .unwrap();
    let names = users.select(name).order(id).load::<String>(&connection);

    println!("{:?}", names);

    diesel::insert_or_ignore_into(users)
        .values((id.eq(1), name.eq("Jim")))
        .execute(&connection)
        .unwrap();

    diesel::insert_or_ignore_into(users)
        .values(&vec![
            (id.eq(1), name.eq("Sean")),
            (id.eq(2), name.eq("Tess")),
        ])
        .execute(&connection)
        .unwrap();

    let names = users
        .select(name)
        .order(id)
        .load::<String>(&connection)
        .unwrap();
    println!("{:?}", names);
}

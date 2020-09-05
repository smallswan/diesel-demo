# diesel-demo
This repository contains some tutorials from Diesel [guide](http://diesel.rs/guides/getting-started/) and [examples](https://github.com/diesel-rs/diesel/tree/master/examples/)

## posts
```
diesel setup
diesel migration generate create_posts
diesel migration run
diesel migration redo

```

```
cargo run --bin write_post

cargo run --bin publish_post

cargo run --bin show_posts

cargo run --bin add_user

cargo test insert_get_results_batch -- --nocapture
```

## Diesel Function
```
debug_query
delete
insert_into
insert_or_ignore_into
replace_into
select
sql_query
update
```
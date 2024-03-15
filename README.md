# Sqlx Sqlite Non-ACID behaviour

Repo for reproducing `sqlx` issue [3120](https://github.com/launchbadge/sqlx/issues/3120)

Run with `cargo run`

The `main.rs` file can be edited to produce different behaviours:
- Edit the journal mode
- Edit the synchronous setting
- Edit the sleep duration/remove it

Often there will still be a case where a `user` cannot be found after being written to the DB.
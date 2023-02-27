use diesel::{r2d2::ConnectionManager, SqliteConnection};

pub type Connection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn connect_pool() -> Pool {
    let database_url = "assets/Cookie";

    // create db connection pool
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}

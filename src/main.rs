extern crate rocket;
#[macro_use] extern crate diesel;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

// An alias to the type for a pool of Diesel SQLite connections.
type PgPool = Pool<ConnectionManager<PgConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &'static str = env!("DATABASE_URL");

/// Initializes a database pool.
fn init_pool() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("db pool")
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .launch();
}

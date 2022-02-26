#[macro_use]
extern crate diesel;
pub mod schema;
pub mod user {
    use crate::schema::*;
    use diesel::{Insertable, Queryable};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Queryable)]
    pub struct User {
        pub id: i32,
        pub email: String,
        pub password: String,
        pub date_created: String,
    }

    #[derive(Debug, Insertable)]
    #[table_name = "users"]
    pub struct UserNew {
        pub email: String,
        pub password: String,
        pub date_created: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UserJson {
        pub email: String,
        pub password: String,
    }
}

pub mod db {
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::SqliteConnection;
    pub fn get_db() -> Pool<ConnectionManager<SqliteConnection>> {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("NOT FOUND");
        Pool::builder()
            .build(ConnectionManager::<SqliteConnection>::new(database_url))
            .unwrap()
    }
}

use rocket_contrib::databases::postgres;
use rocket_contrib::databases::{database};
#[database("packrat")]
pub struct MyPgDatabase(postgres::Client);

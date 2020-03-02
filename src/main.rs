#![feature(proc_macro_hygiene, decl_macro)]
use rocket;
use rocket::routes;
use packybara_rest::{static_rocket_route_info_for_dist, MyPgDatabase};
use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};

fn main() {
    let mut database_config = HashMap::new();
     let mut databases = HashMap::new();
    // database_config.insert("host", Value::from("127.0.0.1"));
    // database_config.insert("user", Value::from("postgres"));
    // database_config.insert("dbname", Value::from("packrat"));
    // database_config.insert("port", Value::from("5432"));
    database_config.insert("url", Value::from("host=127.0.0.1 user=postgres dbname=packrat password=example port=5432"));
    databases.insert("packrat", Value::from(database_config));
    let config = Config::build(Environment::Development)
        .extra("databases", databases)
        .finalize()
        .unwrap();

    rocket::custom(config)
    .attach(MyPgDatabase::fairing())
    .mount("/", routes![dist])
    .launch();
}   

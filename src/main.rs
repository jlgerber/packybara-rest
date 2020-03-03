#![feature(proc_macro_hygiene, decl_macro)]
use rocket;
use rocket::routes;
use packybara_rest::{
    static_rocket_route_info_for_versionpin, 
    static_rocket_route_info_for_versionpins, 
    static_rocket_route_info_for_root,
    static_rocket_route_info_for_packagesxml,
    static_rocket_route_info_for_distributions,
    static_rocket_route_info_for_levels,
    static_rocket_route_info_for_packages,
    static_rocket_route_info_for_platforms,
    static_rocket_route_info_for_roles,
    MyPgDatabase
};
use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};

fn main() {
    let mut database_config = HashMap::new();
     let mut databases = HashMap::new();
    database_config.insert("url", Value::from("host=127.0.0.1 user=postgres dbname=packrat password=example port=5432"));
    databases.insert("packrat", Value::from(database_config));
    let config = Config::build(Environment::Development)
        .extra("databases", databases)
        .finalize()
        .unwrap();

    rocket::custom(config)
    .attach(MyPgDatabase::fairing())
    .mount("/", routes![
        distributions,
        levels,
        packages,
        packagesxml,
        platforms,
        roles,
        root, 
        versionpin, 
        versionpins, 
    ])
    .launch();
}   

#![feature(proc_macro_hygiene, decl_macro)]
use log;
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
    static_rocket_route_info_for_sites,
    static_rocket_route_info_for_pins,
    static_rocket_route_info_for_pkgcoords,
    static_rocket_route_info_for_versionpin_withs,
    static_rocket_route_info_for_withs,
    prefs,
    MyPgDatabase
};
use preferences::{traits::*, DDContext, DDPreferenceFinder, PreferenceName};
use rocket;
use rocket::config::{Config, Environment, Value};
use rocket::routes;
use std::collections::HashMap;
use structopt::*;
use std::path::PathBuf;
use env_logger;

#[derive(Debug, StructOpt)]
#[structopt(name = "packybara-rest", about = "packybara restful server")]
struct Opt {
    /// Specify the path to the preference file
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Test mode
    #[structopt(short = "t", long = "test-mode")]
    test_mode: bool,
}

fn main() {
    let Opt {output, test_mode} = Opt::from_args();
    
    let rest_pref = if let Some(prefs) = output {
        match prefs::PackybaraRestPrefs::load_file(prefs.as_os_str().to_str().unwrap_or("")) {
            Ok(v) => {println!("found prefs");v},
            Err(e) => {
                log::warn!("unable to load preference from supplied file. Falling back to default");
                println!("{:?}",e);
                prefs::PackybaraRestPrefs::default()
            }
        }
    } else {
        let finder = DDPreferenceFinder::from_env(PreferenceName::Main("packybara_rest".to_string()));
        let ctx = if test_mode {
            DDContext::TestEqUser
        } else {
            DDContext::Normal
        };
        match prefs::PackybaraRestPrefs::load(&finder, ctx) {
            Ok(v) => v,
            Err(e) => {
                log::warn!("problem loading prefs from environment: {:?}. Loading default prefs", e);
                prefs::PackybaraRestPrefs::default()
            }
        }
     };
    
    let db_url = rest_pref.as_connectparams(prefs::Mode::Prod).to_string();
    let mut database_config = HashMap::new();
     let mut databases = HashMap::new();
    database_config.insert("url", Value::from(db_url.as_str()));
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
        pins,
        pkgcoords,
        platforms,
        roles,
        root, 
        sites,
        versionpin, 
        versionpins, 
        versionpin_withs,
        withs,
    ])
    .launch();
}   

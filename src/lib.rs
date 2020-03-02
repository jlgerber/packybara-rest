#![feature(proc_macro_hygiene, decl_macro)]
use rocket::get;
use packybara::packrat::PackratDb;
//use packybara::packrat::{Client, NoTls};
use packybara::db::find::versionpins::FindVersionPinsRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
pub mod errors;
pub use errors::ApiResult;
use rocket_contrib::databases::postgres;
use rocket_contrib::databases::{database};



#[database("packrat")]
pub struct MyPgDatabase(postgres::Client);



#[get("/dist/<package>?<level>&<role>&<platform>&<site>")]             
pub fn dist(
    package: String, 
    level: Option<String>, 
    role: Option<String>,
    platform: Option<String>, 
    site: Option<String> , 
    mut client: MyPgDatabase,
) -> ApiResult<FindVersionPinsRow> { 
    let level = level.unwrap_or("facility".to_string());
    let role = role.unwrap_or("any".to_string());
    let platform = platform.unwrap_or("any".to_string());
    let site = site.unwrap_or("any".to_string());

    let mut pb = PackratDb::new(&mut client.0);
    let result = pb
    .find_versionpin(package.as_str())
    .level(level.as_str())
    .role(role.as_str())
    .platform(platform.as_str())
    .site(site.as_str())
    .query()?;

    Ok(Json(result))
}



#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use packybara::packrat::PackratDb;
use packybara::packrat::{Client, NoTls};
use packybara::db::find::versionpins::FindVersionPinsRow;
use packybara::db::find::versionpin::FindVersionPinError;
use packybara::db::traits::*;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::Serialize;
use rocket::response::{self, Response,Responder};
use rocket::request::Request;
use log;
use std::io::Cursor;
use rocket::http::ContentType;

#[derive(Debug)]
pub enum PackybaraRestError {
    FindVersionPinError(FindVersionPinError)
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub type ApiResult<T> = Result<Json<T>, PackybaraRestError>;

#[derive(Debug,Serialize)]
pub struct PbError {
    status: u16,
    error: &'static str,
    msg: String,
}
impl From<PackybaraRestError> for PbError {
    fn from(error: PackybaraRestError) ->  PbError{
        match error {
            PackybaraRestError::FindVersionPinError(e) => {
                PbError {
                    status: 400,
                    error: "FindVersionpinError",
                    msg: e.to_string()
                }
            }
        }
    }
}

impl From<FindVersionPinError> for PackybaraRestError {
    fn from(error: FindVersionPinError) -> PackybaraRestError {
        PackybaraRestError::FindVersionPinError(error)
    }
}

impl<'r> Responder<'r> for PackybaraRestError {
    fn respond_to(self,  _: &Request) -> response::Result<'r> {
       
        let err: PbError = self.into();
        let msg = serde_json::to_string(&err).expect("counldnt convert error to json");
            
    
        log::error!("PackybaraRestError::FindVersionpinError {}", &msg);
        //Err(Status::new(433, "FindVersionPin failure. Check"))
        
        Response::build()
        .header(ContentType::JSON)
                .status(Status::BadRequest)
                .sized_body(Cursor::new(msg))
                .ok()
    }
}

#[get("/dist/<package>?<level>&<role>&<platform>&<site>")]             
fn dist(
    package: String, 
    level: Option<String>, 
    role: Option<String>,
    platform: Option<String>, 
    site: Option<String> 
) -> ApiResult<FindVersionPinsRow> { 
    let level = level.unwrap_or("facility".to_string());
    let role = role.unwrap_or("any".to_string());
    let platform = platform.unwrap_or("any".to_string());
    let site = site.unwrap_or("any".to_string());
    let  client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
        NoTls,
    ).unwrap();

    let mut pb = PackratDb::new(client);
    let result = pb
    .find_versionpin(package.as_str())
    .level(level.as_str())
    .role(role.as_str())
    .platform(platform.as_str())
    .site(site.as_str())
    .query()?;

    Ok(Json(result))
}

fn main() {
    rocket::ignite().mount("/", routes![dist]).launch();
}   

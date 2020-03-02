use packybara::db::find::versionpin::FindVersionPinError;
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
        
        Response::build()
        .header(ContentType::JSON)
                .status(Status::BadRequest)
                .sized_body(Cursor::new(msg))
                .ok()
    }
}

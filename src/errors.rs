use packybara::db::{
    find::versionpin::FindVersionPinError,
    find_all::versionpins::FindAllVersionPinsError
};
use rocket::http::Status;
use serde::Serialize;
use rocket::response::{self, Response,Responder};
use rocket::request::Request;
use log;
use std::io::Cursor;
use rocket::http::ContentType;

#[derive(Debug)]
pub enum PackybaraRestError {
    FindVersionPinError(FindVersionPinError),
    FindAllVersionPinsError(FindAllVersionPinsError)
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}


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
                    error: "FindVersionPinError",
                    msg: e.to_string()
                }
            }
            PackybaraRestError::FindAllVersionPinsError(e) => {
                PbError {
                    status: 400,
                    error: "FindAllVersionPinsError",
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

impl From<FindAllVersionPinsError> for PackybaraRestError {
    fn from(error: FindAllVersionPinsError) -> PackybaraRestError {
        PackybaraRestError::FindAllVersionPinsError(error)
    }
}

impl<'r> Responder<'r> for PackybaraRestError {
    fn respond_to(self,  _: &Request) -> response::Result<'r> {
       
        let err: PbError = self.into();
        let msg = serde_json::to_string(&err).expect("counldnt convert error to json");
            
    
        log::error!("PackybaraRestError {}", &msg);
        
        Response::build()
        .header(ContentType::JSON)
                .status(Status::BadRequest)
                .sized_body(Cursor::new(msg))
                .ok()
    }
}

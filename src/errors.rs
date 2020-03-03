use packybara::db::{
    find::versionpin::FindVersionPinError,
    find_all::versionpins::FindAllVersionPinsError,
    find_all::distributions::FindAllDistributionsError,
    find_all::levels::FindAllLevelsError,
    find_all::packages::FindAllPackagesError,
    find_all::platforms::FindAllPlatformsError,
    find_all::roles::FindAllRolesError,
    find_all::sites::FindAllSitesError,
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
    FindAllVersionPinsError(FindAllVersionPinsError),
    FindAllDistributionsError(FindAllDistributionsError),
    FindAllLevelsError(FindAllLevelsError),
    FindAllPackagesError(FindAllPackagesError),
    FindAllPlatformsError(FindAllPlatformsError),
    FindAllRolesError(FindAllRolesError),
    FindAllSitesError(FindAllSitesError),
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
            PackybaraRestError::FindAllDistributionsError(e) => {
                PbError {
                    status: 400,
                    error: "FindAllDistributionsError",
                    msg: e.to_string()
                }
            }
            PackybaraRestError::FindAllLevelsError(e) => {
                PbError {
                    status: 400,
                    error: "FindAllLevelsError",
                    msg: e.to_string()
                }
            }
            PackybaraRestError::FindAllPackagesError(e) => {
                PbError {
                    status: 400,
                    error: "FindAllPackagesError",
                    msg: e.to_string()
                }
            }
            PackybaraRestError::FindAllPlatformsError(e) => {
                PbError {
                    status: 400,
                    error: "FindAllPlatformsError",
                    msg: e.to_string()
                }
            }
            PackybaraRestError::FindAllRolesError(e) => {
                PbError {
                    status: 400,
                    error: "FindAllRolesError",
                    msg: e.to_string()
                }
            }
            PackybaraRestError::FindAllSitesError(e) => {
                PbError {
                    status: 400,
                    error: "FindAllSitesError",
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

impl From<FindAllDistributionsError> for PackybaraRestError {
    fn from(error: FindAllDistributionsError) -> PackybaraRestError {
        PackybaraRestError::FindAllDistributionsError(error)
    }
}

impl From<FindAllLevelsError> for PackybaraRestError {
    fn from(error: FindAllLevelsError) -> PackybaraRestError {
        PackybaraRestError::FindAllLevelsError(error)
    }
}

impl From<FindAllPackagesError> for PackybaraRestError {
    fn from(error: FindAllPackagesError) -> PackybaraRestError {
        PackybaraRestError::FindAllPackagesError(error)
    }
}

impl From<FindAllPlatformsError> for PackybaraRestError {
    fn from(error: FindAllPlatformsError) -> PackybaraRestError {
        PackybaraRestError::FindAllPlatformsError(error)
    }
}

impl From<FindAllRolesError> for PackybaraRestError {
    fn from(error: FindAllRolesError) -> PackybaraRestError {
        PackybaraRestError::FindAllRolesError(error)
    }
}

impl From<FindAllSitesError> for PackybaraRestError {
    fn from(error: FindAllSitesError) -> PackybaraRestError {
        PackybaraRestError::FindAllSitesError(error)
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

use packybara::db::{
    find::versionpin::FindVersionPinError,
    find_all::versionpins::FindAllVersionPinsError,
    find_all::distributions::FindAllDistributionsError,
    find_all::levels::FindAllLevelsError,
    find_all::packages::FindAllPackagesError,
    find_all::platforms::FindAllPlatformsError,
    find_all::roles::FindAllRolesError,
    find_all::sites::FindAllSitesError,
    find_all::versionpin_withs::FindAllWithsError,
    find::pins::FindPinsError,
    find::withs::FindWithsError,
    search_attribute::SearchModeError,
    find_all::pkgcoords::FindAllPkgCoordsError,
};
use strum;
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
    FindPinsError(FindPinsError),
    StrumParseError(strum::ParseError),
    SearchModeError(SearchModeError),
    FindAllPkgCoordsError(FindAllPkgCoordsError),
    FindWithsError(FindWithsError),
    FindAllWithsError(FindAllWithsError),
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
            PackybaraRestError::FindPinsError(e) => {
                PbError {
                    status: 400,
                    error: "FindPinsError",
                    msg: e.to_string()
                }
            }
            PackybaraRestError::StrumParseError(e) => {
                PbError {
                    status: 400,
                    error: "StrumParseError",
                    msg: e.to_string()
                }
            }
            PackybaraRestError::SearchModeError(e) => {
                PbError {
                    status: 400,
                    error: "SearchModeError",
                    msg: e.to_string()
                }
            }
            PackybaraRestError::FindAllPkgCoordsError(e) => {
                PbError {
                    status: 400,
                    error: "FindAllPkgCoordsError",
                    msg: e.to_string()
                }
            }
            PackybaraRestError::FindWithsError(e) => {
                PbError {
                    status: 400,
                    error: "FindWithsError",
                    msg: e.to_string()
                }
            }
            PackybaraRestError::FindAllWithsError(e) => {
                PbError {
                    status: 400,
                    error: "FindAllWithsError",
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

impl From<FindPinsError> for PackybaraRestError {
    fn from(error: FindPinsError) -> PackybaraRestError {
        PackybaraRestError::FindPinsError(error)
    }
}

impl From<strum::ParseError> for PackybaraRestError {
    fn from(error: strum::ParseError) -> PackybaraRestError {
        PackybaraRestError::StrumParseError(error)
    }
}

impl From<SearchModeError> for PackybaraRestError {
    fn from(error: SearchModeError) -> PackybaraRestError {
        PackybaraRestError::SearchModeError(error)
    }
}

impl From<FindAllPkgCoordsError> for PackybaraRestError {
    fn from(error: FindAllPkgCoordsError) -> PackybaraRestError {
        PackybaraRestError::FindAllPkgCoordsError(error)
    }
}

impl From<FindWithsError> for PackybaraRestError {
    fn from(error: FindWithsError) -> PackybaraRestError {
        PackybaraRestError::FindWithsError(error)
    }
}
impl From<FindAllWithsError> for PackybaraRestError {
    fn from(error: FindAllWithsError) -> PackybaraRestError {
        PackybaraRestError::FindAllWithsError(error)
    }
}
//
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

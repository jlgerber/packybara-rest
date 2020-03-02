#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::json::Json;

pub type ApiResult<T> = Result<Json<T>, PackybaraRestError>;

pub mod errors;
pub use errors::PackybaraRestError;
pub mod database;
pub use database::MyPgDatabase;
pub mod crud;
pub use crud::versionpin::static_rocket_route_info_for_versionpin;
pub use crud::versionpins::static_rocket_route_info_for_versionpins;
pub use crud::root::static_rocket_route_info_for_root;

pub mod route_desc;
pub use route_desc::RouteDesc;

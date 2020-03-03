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
pub use crud::packages_xml::static_rocket_route_info_for_packagesxml;
pub use crud::distributions::static_rocket_route_info_for_distributions;
pub use crud::levels::static_rocket_route_info_for_levels;
pub use crud::packages::static_rocket_route_info_for_packages;
pub use crud::platforms::static_rocket_route_info_for_platforms;
pub use crud::roles::static_rocket_route_info_for_roles;
pub use crud::sites::static_rocket_route_info_for_sites;
pub use crud::pins::static_rocket_route_info_for_pins;
pub use crud::pkgcoords::static_rocket_route_info_for_pkgcoords;
pub use crud::withs::static_rocket_route_info_for_withs;
pub mod route_desc;
pub use route_desc::RouteDesc;

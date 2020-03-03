use rocket::get;
use rocket_contrib::json::Json;
use crate::ApiResult;
use serde::Serialize;

use super::versionpin::versionpin_get_rd;
use super::versionpins::versionpins_get_rd;
use super::distributions::distributions_get_rd;
use super::levels::levels_get_rd;
use super::packages::packages_get_rd;
use super::packages_xml::packagesxml_post_rd;
use super::platforms::platforms_get_rd;
use super::roles::roles_get_rd;

use crate::route_desc::RouteDesc;

#[derive(Debug,Serialize)]
pub struct Root {
    routes: Vec<RouteDesc>
}

#[get("/")]             
pub fn root(
) -> ApiResult<Root> { 
    let root = Root {
        routes: vec![
            distributions_get_rd(),
            levels_get_rd(),
            packages_get_rd(),
            packagesxml_post_rd(),
            versionpin_get_rd(),
            versionpins_get_rd(),
            platforms_get_rd(),
            roles_get_rd(),
        ]
    };
    Ok(Json(root))
}
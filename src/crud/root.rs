use rocket::get;
use rocket_contrib::json::Json;
use crate::ApiResult;
use serde::Serialize;
use super::versionpin::versionpin_get_rd;
use super::versionpins::versionpins_get_rd;
use super::packages_xml::packagesxml_post_rd;
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
            versionpin_get_rd(),
            versionpins_get_rd(),
            packagesxml_post_rd()
        ]
    };
    Ok(Json(root))
}
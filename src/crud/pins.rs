use rocket::get;
use packybara::packrat::PackratDb;
use packybara::db::find::pins::FindPinsRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
use crate::MyPgDatabase;
use crate::ApiResult;
use crate::RouteDesc;
use std::ops::Deref;
pub use packybara::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
use std::str::FromStr;

/// create the RouteDesc (route description object)
pub(crate) fn pins_get_rd() -> RouteDesc {
    RouteDesc::new(
        "pins",
        "/v1/pins?<level>&<role>&<platform>&<site>&<search_mode>&<order_by>",
        "get",
        "Retrieve pins "
    )
    .parameter("level", "The level to search for pins")
    .parameter("role", "The role to search for pins")
    .parameter("platform", "The platform to search for pins")
    .parameter("site", "The location to search for pins")
    .parameter("search_mode", "The mode - ancestor, exact, descendant")
    .parameter("order_by", "The parameter or parameters to order the results by")
    .build()
}

#[get("/v1/pins?<level>&<role>&<platform>&<site>&<search_mode>&<order_by>")]             
pub fn pins(
    level: Option<String>,
    role: Option<String>,
    platform: Option<String>,
    site: Option<String>,
    search_mode: Option<String>,
    order_by: Option<String>,
    mut client: MyPgDatabase,
) -> ApiResult<Vec<FindPinsRow>> { 
    let mut pb = PackratDb::new(&mut client.0);
    let mut results = pb.find_pins();
    results
        .role_opt(role.as_ref().map(Deref::deref))
        .level_opt(level.as_ref().map(Deref::deref))
        .platform_opt(platform.as_ref().map(Deref::deref))
        .site_opt(site.as_ref().map(Deref::deref));
    if let Some(ref mode) = search_mode {
        results.search_mode(LtreeSearchMode::from_str(mode)?);
    }
    if let Some(ref order) = order_by {
        let orders = order
            .split(",")
            .map(|x| SearchAttribute::from_str(x).unwrap_or(SearchAttribute::Unknown))
            .collect::<Vec<SearchAttribute>>();
        results.order_by(orders);
    }
    let results = results.query()?;

    Ok(Json(results))
}

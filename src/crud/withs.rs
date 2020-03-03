use rocket::get;
use packybara::packrat::PackratDb;
use packybara::db::find::withs::FindWithsRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
use crate::MyPgDatabase;
use crate::ApiResult;
use crate::RouteDesc;
use std::ops::Deref;
use packybara::SearchAttribute;
use std::str::FromStr;

/// create the RouteDesc (route description object)
pub(crate) fn withs_get_rd() -> RouteDesc {
    RouteDesc::new(
        "/v1/withs?<package>&<level>&<role>&<platform>&<site>&<order_by>",
        "get",
        "Retrieve withs matching the supplied parameters"
    )
    .parameter("package", "The package name")
    .parameter("level", "The level (eg facility) as a levelspec")
    .parameter("role", "The role or subrole (eg model or model_beta)")
    .parameter("platform", "The os to search for (eg cent7_64)")
    .parameter("site", "The location (eg portland)")
    .parameter("order_by", "comma separated field names to order query results by")
    .build()
}

#[get("/v1/withs/<package>?<level>&<role>&<platform>&<site>&<order_by>")]             
pub fn withs(
    package: String, 
    level: Option<String>, 
    role: Option<String>,
    platform: Option<String>, 
    site: Option<String>,
    order_by: Option<String>,
    mut client: MyPgDatabase,
) -> ApiResult<Vec<FindWithsRow>> { 
    
    let mut pb = PackratDb::new(&mut client.0);
    let mut results = pb.find_withs(package.as_str());
    results
        .role_opt(role.as_ref().map(Deref::deref))
        .level_opt(level.as_ref().map(Deref::deref))
        .platform_opt(platform.as_ref().map(Deref::deref))
        .site_opt(site.as_ref().map(Deref::deref));
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

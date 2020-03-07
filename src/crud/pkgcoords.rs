use rocket::get;
use packybara::packrat::PackratDb;
use packybara::db::find_all::pkgcoords::FindAllPkgCoordsRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
use crate::MyPgDatabase;
use crate::ApiResult;
use crate::RouteDesc;
use std::ops::Deref;
use packybara::SearchMode;

/// create the RouteDesc (route description object)
pub(crate) fn pkgcoords_get_rd() -> RouteDesc {
    RouteDesc::new(
        "pkgcoords",
        "/v1/pkgcoords?<package>&<level>&<role>&<platform>&<site>&<mode>&<order_by>",
        "get",
        "Retrieve pkgcoords matching the supplied parameters"
    )
    .parameter("package", "The package name")
    .parameter("level", "The level (eg facility) as a levelspec")
    .parameter("role", "The role or subrole (eg model or model_beta)")
    .parameter("platform", "The os to search for (eg cent7_64)")
    .parameter("site", "The location (eg portland)")
    .parameter("mode", "The direction of the search - ancestors, exact, descendants")
    .parameter("order_by", "comma separated field names to order query results by")
    .build()
}

#[get("/v1/pkgcoords?<package>&<level>&<role>&<platform>&<site>&<mode>&<order_by>")]             
pub fn pkgcoords(
    package: Option<String>, 
    level: Option<String>, 
    role: Option<String>,
    platform: Option<String>, 
    site: Option<String>,
    mode: Option<String>,
    order_by: Option<String>,
    mut client: MyPgDatabase,
) -> ApiResult<Vec<FindAllPkgCoordsRow>> { 
    
    let mut pb = PackratDb::new(&mut client.0);
    let mut results = pb.find_pkgcoords();
    results
        .package_opt(package.as_ref().map(Deref::deref))
        .role_opt(role.as_ref().map(Deref::deref))
        .level_opt(level.as_ref().map(Deref::deref))
        .platform_opt(platform.as_ref().map(Deref::deref))
        .site_opt(site.as_ref().map(Deref::deref))
        .order_by_opt(order_by.as_ref().map(Deref::deref));
    if let Some(ref mode) = mode {
        results.search_mode(SearchMode::try_from_str(mode)?);
    }
    let results = results.query()?;

    Ok(Json(results))
}


use rocket::get;
use packybara::packrat::PackratDb;
use packybara::db::find_all::versionpins::FindAllVersionPinsRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
use crate::MyPgDatabase;
use crate::ApiResult;
use packybara::db::search_attribute::{SearchAttribute, OrderDirection};
use std::str::FromStr;
use crate::RouteDesc;

/// create the RouteDesc (route description object)
pub(crate) fn versionpins_get_rd() -> RouteDesc {
    RouteDesc::new(
        "versionpins",
        "/v1/versionpins?<package>&<level>&<role>&<platform>&<site>&<order_by>&<order_dir>&<limit>",
        "get",
        "Retrieve versionpins matching the supplied parameters"
    ).parameter("package", "the name of the package")
    .parameter("level", "The level (eg facility) as a levelspec")
    .parameter("role", "The role or subrole (eg model or model_beta)")
    .parameter("platform", "The os to search for (eg cent7_64)")
    .parameter("site", "The location (eg portland)")
    .parameter("order_by", "comma separated field names to order query results by")
    .parameter("order_dir", "The direction to order the results - asc or desc")
    .parameter("limit", "The max number of results to return")
    .build()
}

#[get("/v1/versionpins?<package>&<level>&<role>&<platform>&<site>&<order_by>&<order_dir>&<limit>")]             
pub fn versionpins(
    package: Option<String>, 
    level: Option<String>, 
    role: Option<String>,
    platform: Option<String>, 
    site: Option<String>,
    order_by: Option<String>,
    order_dir: Option<String>,
    limit: Option<i32>,
    mut client: MyPgDatabase,
) -> ApiResult<Vec<FindAllVersionPinsRow>> { 
    
    let result = PackratDb::new(&mut client.0)
        .find_all_versionpins()
        .package_opt(package.as_ref().map(String::as_str))
        .level_opt(level.as_ref().map(String::as_str))
        .role_opt(role.as_ref().map(String::as_str))
        .platform_opt(platform.as_ref().map(String::as_str))
        .site_opt(site.as_ref().map(String::as_str))
        .order_by_opt(
            order_by.map(|x| x.split(',')
                .filter_map(|x| 
                    SearchAttribute::from_str(x)
                    .ok()
                ).collect::<Vec<_>>()
            )
        )
        .order_direction_opt(
            order_dir.map(|x| 
                OrderDirection::from_str(x.as_str())
                .unwrap_or(OrderDirection::Asc) 
            )
        )
        .limit(limit.unwrap_or(0))
        .query()?;

    Ok(Json(result))
}

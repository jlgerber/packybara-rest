use rocket::get;
use packybara::packrat::PackratDb;
use packybara::db::find_all::distributions::FindAllDistributionsRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
use crate::MyPgDatabase;
use crate::ApiResult;
use packybara::db::search_attribute::{SearchAttribute, OrderDirection};
use std::str::FromStr;
use crate::RouteDesc;
use std::ops::Deref;

/// create the RouteDesc (route description object)
pub(crate) fn distributions_get_rd() -> RouteDesc {
    RouteDesc::new(
        "/v1/distributions?<package>&<version>&<order_dir>",
        "get",
        "Retrieve distributions matching the supplied parameters"
    )
    .parameter("package", "the distribution's package name")
    .parameter("version", "The version of the distribution")
    .parameter("order_dir", "The direction to order the results - asc or desc")
    .build()
}

#[get("/v1/distributions?<package>&<version>&<order_dir>")]             
pub fn versionpins(
    package: Option<String>, 
    version: Option<String>, 
    order_dir: Option<String>,
    mut client: MyPgDatabase,
) -> ApiResult<Vec<FindAllDistributionsRow>> { 
    
    let result = PackratDb::new(&mut client.0)
        .find_all_distributions()
        .package_opt(package.as_ref().map(Deref::deref))
        .version_opt(version.as_ref().map(Deref::deref))
        .order_direction_opt(
            order_dir.map(|x| 
                OrderDirection::from_str(x.as_str())
                .unwrap_or(OrderDirection::Asc) 
            )
        )
        .query()?;

    Ok(Json(result))
}

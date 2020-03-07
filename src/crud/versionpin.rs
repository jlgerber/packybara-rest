
use rocket::get;
use packybara::packrat::PackratDb;
use packybara::db::find::versionpins::FindVersionPinsRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
use crate::MyPgDatabase;
use crate::ApiResult;
use crate::RouteDesc;

/// create the RouteDesc (route description object)
pub(crate) fn versionpin_get_rd() -> RouteDesc {
    RouteDesc::new(
        "versionpin",
        "/v1/versionpin/<package>?<level>&<role>&<platform>&<site>",
        "get",
        "Retrieve the closest versionpin and withs to the supplied parameters"
    ).parameter("package", "the name of the package")
    .parameter("level", "The level (eg facility) as a levelspec")
    .parameter("role", "The role or subrole (eg model or model_beta)")
    .parameter("platform", "The os to search for (eg cent7_64)")
    .parameter("site", "The location (eg portland)")
    .build()
}

#[get("/v1/versionpin/<package>?<level>&<role>&<platform>&<site>")]             
pub fn versionpin(
    package: String, 
    level: Option<String>, 
    role: Option<String>,
    platform: Option<String>, 
    site: Option<String> , 
    mut client: MyPgDatabase,
) -> ApiResult<FindVersionPinsRow> { 
   
    let result = PackratDb::new(&mut client.0)
        .find_versionpin(package.as_str())
        .level_opt(level.as_ref().map(String::as_str))
        .role_opt(role.as_ref().map(String::as_str))
        .platform_opt(platform.as_ref().map(String::as_str))
        .site_opt(site.as_ref().map(String::as_str))
        .query()?;

    Ok(Json(result))
}



use rocket::get;
use packybara::packrat::PackratDb;
use packybara::db::find_all::packages::FindAllPackagesRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
use crate::MyPgDatabase;
use crate::ApiResult;
use crate::RouteDesc;

/// create the RouteDesc (route description object)
pub(crate) fn packages_get_rd() -> RouteDesc {
    RouteDesc::new(
        "/v1/packages>",
        "get",
        "Retrieve packages "
    )
    .build()
}

#[get("/v1/packages")]             
pub fn packages(
    mut client: MyPgDatabase,
) -> ApiResult<Vec<FindAllPackagesRow>> { 
    let results = PackratDb::new(&mut client.0).find_all_packages().query()?;
    Ok(Json(results))
}

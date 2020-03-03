use rocket::get;
use packybara::packrat::PackratDb;
use packybara::db::find_all::platforms::FindAllPlatformsRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
use crate::MyPgDatabase;
use crate::ApiResult;
use crate::RouteDesc;

/// create the RouteDesc (route description object)
pub(crate) fn platforms_get_rd() -> RouteDesc {
    RouteDesc::new(
        "/v1/platforms",
        "get",
        "Retrieve platforms "
    )
    .build()
}

#[get("/v1/platforms")]             
pub fn platforms(
    mut client: MyPgDatabase,
) -> ApiResult<Vec<FindAllPlatformsRow>> { 
    let results = PackratDb::new(&mut client.0).find_all_platforms().query()?;
    let results = results.into_iter().filter(|x| x.name != "any").collect::<Vec<_>>();
    Ok(Json(results))
}

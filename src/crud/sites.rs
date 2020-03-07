use rocket::get;
use packybara::packrat::PackratDb;
use packybara::db::find_all::sites::FindAllSitesRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
use crate::MyPgDatabase;
use crate::ApiResult;
use crate::RouteDesc;

/// create the RouteDesc (route description object)
pub(crate) fn sites_get_rd() -> RouteDesc {
    RouteDesc::new("sites",
        "/v1/sites?<category>",
        "get",
        "Retrieve sites "
    )
    .build()
}

#[get("/v1/sites?")]             
pub fn sites(
    mut client: MyPgDatabase,
) -> ApiResult<Vec<FindAllSitesRow>> { 
    let results = PackratDb::new(&mut client.0)
        .find_all_sites()
        .query()?;
    let results = results.into_iter().filter(|x| x.name != "any").collect::<Vec<_>>();
    Ok(Json(results))
}

use rocket::get;
use packybara::packrat::PackratDb;
use packybara::db::find_all::versionpin_withs::FindAllWithsRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
use crate::MyPgDatabase;
use crate::ApiResult;
use crate::RouteDesc;
use packybara::types::IdType;

/// create the RouteDesc (route description object)
pub(crate) fn versionpin_withs_get_rd() -> RouteDesc {
    RouteDesc::new(
        "versionpin_withs",
        "/v1/vpin-withs/<vpin_id>",
        "get",
        "Retrieve withs matching the supplied parameters"
    )
    .parameter("vpin_id", "The versionpin's database id")
    .build()
}

#[get("/v1/vpin-withs/<vpin_id>")]             
pub fn versionpin_withs(
    vpin_id: IdType, 
    mut client: MyPgDatabase,
) -> ApiResult<Vec<FindAllWithsRow>> { 
    
    let mut pb = PackratDb::new(&mut client.0);
    let results = pb.find_all_versionpin_withs(vpin_id).query()?;

    Ok(Json(results))
}

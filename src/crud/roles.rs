use rocket::get;
use packybara::packrat::PackratDb;
use packybara::db::find_all::roles::FindAllRolesRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
use crate::MyPgDatabase;
use crate::ApiResult;
use crate::RouteDesc;
use std::ops::Deref;

/// create the RouteDesc (route description object)
pub(crate) fn roles_get_rd() -> RouteDesc {
    RouteDesc::new(
        "roles",
        "/v1/roles?<category>",
        "get",
        "Retrieve roles "
    )
    .parameter("category", "Either \"role\" or \"subrole\"")
    .build()
}

#[get("/v1/roles?<category>")]             
pub fn roles(
    category: Option<String>,
    mut client: MyPgDatabase,
) -> ApiResult<Vec<FindAllRolesRow>> { 
    let results = PackratDb::new(&mut client.0)
        .find_all_roles()
        .category_opt(category.as_ref().map(Deref::deref))
        .query()?;
    let results = results.into_iter().filter(|x| x.role != "any").collect::<Vec<_>>();
    Ok(Json(results))
}

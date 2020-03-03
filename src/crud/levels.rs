use rocket::get;
use packybara::packrat::PackratDb;
use packybara::db::find_all::levels::FindAllLevelsRow;
use packybara::db::traits::*;
use rocket_contrib::json::Json;
use crate::MyPgDatabase;
use crate::ApiResult;
use crate::RouteDesc;
use std::ops::Deref;
use packybara::OrderLevelBy;
use std::str::FromStr;

/// create the RouteDesc (route description object)
pub(crate) fn levels_get_rd() -> RouteDesc {
    RouteDesc::new(
        "/v1/levels?<level>&<show>&<depth>&<order_by>",
        "get",
        "Retrieve levels matching the supplied parameters"
    )
    .parameter("level", "The name of a level")
    .parameter("show", "The show to which the level belongs")
    .parameter("depth", "The depth that the level lies at")
    .parameter("order_by", "The parameter or parameters to order the results by")
    .build()
}

#[get("/v1/levels?<level>&<show>&<depth>&<order_by>")]             
pub fn levels(
    level: Option<String>, 
    show: Option<String>, 
    depth: Option<u8>,
    order_by: Option<String>,
    mut client: MyPgDatabase,
) -> ApiResult<Vec<FindAllLevelsRow>> { 
    
    let mut results = PackratDb::new(&mut client.0);
    let mut results = results.find_all_levels();
    results
        .level_opt(level.as_ref().map(Deref::deref))
        .show_opt(show.as_ref().map(Deref::deref))
        .depth_opt(depth);
    if let Some(ref order) = order_by {
        let orders = order
            .split(",")
            .map(|x| {
                OrderLevelBy::from_str(x).unwrap_or_else(|y| {
                    log::warn!("invalid order-by argument:'{}'. {}", x, y);
                    OrderLevelBy::Name
                })
            })
            .collect::<Vec<OrderLevelBy>>();
        results.order_by(orders);
    }
    let results = results.query()?;
    Ok(Json(results))
}

//! Generate am xml file which stores the versionpins for a specific
//! show. Historically, this file has been known as a packages.xml
use rocket::post;
use serde::{Serialize,Deserialize};
use rocket_contrib::json::Json;

use packybara::packrat::PackratDb;
use packybara::io::packages_xml::xml;

use crate::MyPgDatabase;
use crate::RouteDesc;


/// create the RouteDesc (route description object)
pub(crate) fn packagesxml_post_rd() -> RouteDesc {
    RouteDesc::new(
        "/v1/packages.xml",
        "post",
        "Generate a pakages.xml given an appropriate posted document"
    )
    .body("{ \"show\": \"<name of show>\", \"path\": \"path to output packages.xml\" }")
    .build()
}


/// Body of request to create a packages.xml for a show 
/// at the location defined by path 
#[derive(Debug,Serialize,Deserialize)]
pub struct PackagesXml {
    pub show: String,
    pub path: String,
}


#[post("/v1/packages.xml",data = "<output>")]             
pub fn packagesxml(
    output: Json<PackagesXml>,
    mut client: MyPgDatabase,
) { 
   
    let mut db = PackratDb::new(&mut client.0);
    xml::write_xml(&mut db, output.show.clone(), output.path.clone()).expect("unable to write out pakages.xml");
}



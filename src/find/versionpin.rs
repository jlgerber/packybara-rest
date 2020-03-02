
#[get("/version-pin/<package>?<level>&<role>&<platform>&<site>")]             
pub fn version_pin(
    package: String, 
    level: Option<String>, 
    role: Option<String>,
    platform: Option<String>, 
    site: Option<String> 
) -> ApiResult<FindVersionPinsRow> { 
    let level = level.unwrap_or("facility".to_string());
    let role = role.unwrap_or("any".to_string());
    let platform = platform.unwrap_or("any".to_string());
    let site = site.unwrap_or("any".to_string());
    let  client = Client::connect(
        "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
        NoTls,
    ).unwrap();

    let mut pb = PackratDb::new(client);
    let result = pb
    .find_versionpin(package.as_str())
    .level(level.as_str())
    .role(role.as_str())
    .platform(platform.as_str())
    .site(site.as_str())
    .query()?;

    Ok(Json(result))
}
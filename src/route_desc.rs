use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RouteDesc {
    name: String,
    route: String,
    verb: String,
    description: String, 
    parameters: HashMap<String,String>,
    body: Option<String>
}

impl RouteDesc {
    /// New up a RouteDesc
    pub fn new<I>(name: I, route: I, verb: I, description: I) -> Self where I: Into<String> {
        Self {
            name: name.into(),
            route: route.into(),
            verb: verb.into(),
            description: description.into(),
            parameters: HashMap::new(),
            body: None
        }
    }

    /// Add a parameter to the RouteDesc
    pub fn parameter<I>(&mut self, name: I, value: I) -> &mut Self where I: Into<String> {
        self.parameters.insert(name.into(), value.into());
        self
    }
    /// Add a body to the RouteDesc
    pub fn body<I>(&mut self,  value: I) -> &mut Self where I: Into<String> {
        self.body = Some(value.into());
        self
    }
    
    /// Supports the builder pattern. Chain after  one or more add_parameter calls. 
    /// 
    /// # Example
    /// 
    /// ```
    /// let rd = RouteDesc::new("/v1/foo/bar?<a>&<b>", "foo the bar")
    ///            .parameter("a", "the a value")
    ///            .parameter("b", "The b value")
    ///            .build();
    /// ```
    pub fn build(&mut self) -> Self {
        let mut new_route = RouteDesc::new("","","","");
        std::mem::swap(&mut new_route, self);
        new_route
    }

}
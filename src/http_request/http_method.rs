use std::str::FromStr;

#[derive(Debug)]
pub enum HttpMethod {
    //Post { body: String },
    GET,
}

impl FromStr for HttpMethod {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            //"POST" => Ok(HttpMethod::Post {
            //    body: String::new(),
            //}),
            "GET" => Ok(HttpMethod::GET),
            _ => Err(()),
        }
    }

    type Err = ();
}

use std::fs;
use serde::ser::Serialize;
use handlebars::Handlebars;
use serde_json::Value;

lazy_static! {
    static ref HANDLEBARS: Handlebars<'static> = Handlebars::new();
}

type Endpoint = fn() -> serde_json::Value;

fn index_endpoint() -> serde_json::Value {
    static mut myint: u32 = 0;
    unsafe {
        myint += 1;
        json!({"name": myint })

    }
}

pub struct Response {
    contents: String,
}

impl Response {
    pub fn as_bytes(&self) -> &[u8] {
        return self.contents.as_bytes();
    }
}

pub fn match_route<'a>(path: &str) -> Response {
    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let contents: String;
    match path {
        "/" => {
            contents = register::<Value>("index", index_endpoint)
        }
        _ => {
            contents = register::<Value>("404", index_endpoint)
        }
    }
    let payload = format!("{}{}", status_line, contents);
    Response{ contents: payload }
}

pub fn get_404() -> Response {
    Response { contents: String::from("404") }
}

fn register<T>(path: &str, endpoint: Endpoint) -> String where T: Serialize {
    let mut file_path = String::from("templates/");
    file_path.push_str(path);
    file_path.push_str(".html");
    let file_data = fs::read_to_string(file_path).unwrap();
    let contents = HANDLEBARS.render_template(file_data.as_str(), &endpoint()).unwrap();
    contents
}
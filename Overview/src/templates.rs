use askama::Template;

#[derive(Template)]
#[template(path = "template.html")]
pub struct MyTemplate {
    pub usrimg: String,
    pub usrname: String,
}


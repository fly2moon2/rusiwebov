#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::request::FromRequest;
use rocket::request::{Form, LenientForm};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/login")]
fn login() -> String {
    format!("Login   Hello, world!")
}


#[get("/login/<name>")]
fn login_name(name: String) -> String {
    format!("Login Hello, world {}", name)
}

#[get("/login/<userid>", rank=2)]
fn login_userid(userid: &RawStr) -> String {
    format!("Login useridHello, world {}", userid.as_str())
}

// http://localhost:8000/hello?wave&name=watson
#[get("/hello?wave&<name>")]
fn hello(name: Option<String>) -> String {
    name.map(|name| format!("Hi optional, {}!", name))
        .unwrap_or_else(|| "Hello optional none!".into())
}
/* fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
} */

#[get("/hello/<name>/<age>/<cool>", rank=2)]
fn hello2(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

// Multiple segments.  FromForm (can set default)
// query parameters
// option or lenient form
// note: option for user account
#[derive(FromForm)]
struct User {
    name: String,
    account: Option<usize>,
}

// echohttp://localhost:8000/item?name=mose&account=1000
#[get("/item?<user..>")]
fn item(user: Option<Form<User>>) -> String {
    if let Some(user) = user {
        if let Some(account) = user.account {
            format!("Hello, {} year old named {}!", account, user.name)
        } else {
            format!("Hello {}!", user.name)
        }
    } else {
        "We're gonna need a name, and only a name.".into()
    }
}
#[get("/item?<id>&<user..>", rank=2)]
fn item2(id: u8, user: LenientForm<User>) -> String{ 
    format!("item id: {}, user: {}",id, user.name)
}
/* #[get("/item?<id>&<user..>")]
fn item(id: u8, user: Form<User>) -> String{ 
    format!("item id: {} user:",id)
} */


fn main() {
    rocket::ignite().mount("/", routes![index, login, login_name, login_userid, hello, hello2, item, item2]).launch();
}
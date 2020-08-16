#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::request::FromRequest;
use rocket::request::{Form, LenientForm, FromFormValue};




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

// Multiple segments.  FromForm trait(can set default)
// dynamic query parameters implements FromFormValue
// option or lenient form
// note: option for user account
// https://api.rocket.rs/v0.4/rocket/request/trait.FromFormValue.html#method.default
// FromFormValue - custom validation for AdultAge
struct AdultAge(usize);

impl<'v> FromFormValue<'v> for AdultAge {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<AdultAge, &'v RawStr> {
        match form_value.parse::<usize>() {
            Ok(age) if age >= 21 => Ok(AdultAge(age)),
            _ => Err(form_value),
        }
    }
}

#[derive(FromForm)]
struct User {
    name: String,
    account: Option<usize>,
    age: Option<AdultAge>,
}

// echohttp://localhost:8000/item?name=mose&account=1000
#[get("/item?<user..>")]
fn item(user: Option<Form<User>>) -> String {
    if let Some(user) = user {
        if let Some(account) = user.account {
            format!("Hello, {} account of old named {}!", account, user.name)
        } else if let Some(age) = &user.age {
            format!("Hello, year old named {}!", user.name)
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
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::request::FromRequest;
use rocket::request::{Form, LenientForm, FromFormValue};
use rocket::response::Redirect;

use std::fmt;
use std::fmt::{Display};

#[derive(Debug)]
struct StrongPassword<'r>(&'r str);

#[derive(Debug)]
struct BoomAge(u8);

#[derive(FromForm)]
struct UserLogin<'r> {
    username: &'r RawStr,
    password: Result<StrongPassword<'r>, &'static str>,
    age: Result<BoomAge, &'static str>,
}

impl<'v> FromFormValue<'v> for StrongPassword<'v> {
    type Error = &'static str;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        if v.len() < 8 {
            Err("too short!")
        } else {
            Ok(StrongPassword(v.as_str()))
        }
    }
}

impl<'v> FromFormValue<'v> for BoomAge {
    type Error = &'static str;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        let age = match u8::from_form_value(v) {
            Ok(v) => v,
            Err(_) => return Err("value is not a number."),
        };

        match age >= 16 {
            true => Ok(BoomAge(age)),
            false => Err("must be at least 16."),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/login", data = "<user>")]
fn login(user: Form<UserLogin>) -> Result<Redirect, String> {
    if let Err(e) = user.age {
        return Err(format!("Age is invalid: {}", e));
    }

    if let Err(e) = user.password {
        return Err(format!("Password is invalid: {}", e));
    }

    if user.username == "Sergio" {
        if let Ok(StrongPassword("password")) = user.password {
            Ok(Redirect::to("/user/Sergio"))
        } else {
            Err("Wrong password!".to_string())
        }
    } else {
        Err(format!("Unrecognized user, '{}'.", user.username))
    }
}

#[get("/user/<username>")]
fn user_page(username: &RawStr) -> String {
    format!("This is {}'s page.", username)
}

#[get("/logon")]
fn logon() -> String {
    format!("logon Hello, world!")
}


#[get("/logon/<name>")]
fn logon_name(name: String) -> String {
    format!("logon Hello, world {}", name)
}

#[get("/logon/<userid>", rank=2)]
fn logon_userid(userid: &RawStr) -> String {
    format!("logon useridHello, world {}", userid.as_str())
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
// needs to derive Debug trait to enable display fmt
// impl Display for AdultAge to customise the Display format
#[derive(Debug)]
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

impl Display for AdultAge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Age: {}", self.0)
    }
 }

#[derive(FromForm)]
struct User {
    name: String,
    account: Option<usize>,
    age: Option<AdultAge>,
}

// http://localhost:8000/item?name=mose&account=1000
// http://localhost:8000/item?name=mose&age=10
#[get("/item?<user..>")]
fn item(user: Option<Form<User>>) -> String {
    if let Some(user) = user {
        if let Some(account) = user.account {
            format!("Hello, {} account of old named {}!", account, user.name)
        } else if let Some(age) = &user.age {
            format!("Hello, year old named {} age {:?}, age display {}!", user.name, age, age)
        } else {
            format!("Hello {}! Your are no adult {:?}", user.name, user.age)
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
    rocket::ignite().mount("/", routes![index, login, user_page, logon, logon_name, logon_userid, hello, hello2, item, item2]).launch();
}
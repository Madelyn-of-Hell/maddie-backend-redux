mod generator;

use generator::{compile_facade, compile_real_site};
use rocket::{
    Request, get,
    http::Status,
    request::{FromRequest, Outcome},
    response::content::RawHtml,
    routes,
};
use std::fs;

#[rocket::launch]
fn launch() -> _ {
    fs::write("facade.html", compile_facade()).unwrap();
    fs::write("real.html", compile_real_site()).unwrap();
    rocket::build().mount("/", routes![real_site, facade])
}

#[get("/")]
fn real_site(_cookie_guard: CookieGuard) -> Result<RawHtml<String>, Status> {
    let site_read = fs::read_to_string("real.html");

    if let Ok(site) = site_read {
        Ok(RawHtml(site))
    } else {
        Err(Status::new(500))
    }
}

#[get("/", rank = 2)]
fn facade() -> Result<RawHtml<String>, Status> {
    let facade_read = fs::read_to_string("facade.html");

    if let Ok(facade) = facade_read {
        Ok(RawHtml(facade))
    } else {
        Err(Status::new(500))
    }
}

struct CookieGuard();

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CookieGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(cookie) = dbg!(req.cookies().get("those who know"))
            && cookie.value() == "meee :3"
        {
            Outcome::Success(CookieGuard())
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

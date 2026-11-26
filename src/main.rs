mod generator;

use generator::{compile_facade, compile_real_site};
use rocket::{
    Request, get,
    http::{ContentType, Status},
    request::{FromRequest, Outcome},
    response::content::RawHtml,
    routes,
    time::Error,
};
use std::{fs, path::PathBuf};

#[rocket::launch]
fn launch() -> _ {
    fs::write("facade.html", compile_facade()).unwrap();
    fs::write("real.html", compile_real_site()).unwrap();
    rocket::build().mount("/", routes![real_site, facade, serve_content])
}

#[get("/")]
fn real_site(_cookie_guard: CookieGuard) -> Result<RawHtml<String>, Status> {
    let site_read = fs::read_to_string("real.html");

    if let Ok(site) = site_read {
        Ok(RawHtml(site))
    } else {
        Err(Status::InternalServerError)
    }
}

#[get("/", rank = 2)]
fn facade() -> Result<RawHtml<String>, Status> {
    let facade_read = fs::read_to_string("facade.html");

    if let Ok(facade) = facade_read {
        Ok(RawHtml(facade))
    } else {
        Err(Status::InternalServerError)
    }
}

#[get("/<file..>", rank = 3)]
fn serve_content(file: PathBuf) -> Result<(ContentType, Vec<u8>), Status> {
    const SERVABLE_FILES: [&str; 7] = [
        "main.css",
        "main.js",
        "files/88x31.gif",
        "files/Faceplate Round.png",
        "files/Normal Round.png",
        "files/Jetbrains Mono.woff2",
        "files/robots.txt",
    ];
    let file = file.to_string_lossy().to_string();
    if SERVABLE_FILES.contains(&file.as_str()) {
        (match file.as_str() {
            "main.css" => fs::read("main.css").map(|data| (ContentType::CSS, data)),
            "main.js" => fs::read("main.js").map(|data| (ContentType::JavaScript, data)),
            "files/88x31.gif" => fs::read("files/88x31.gif").map(|data| (ContentType::GIF, data)),
            "files/Faceplate Round.png" => {
                fs::read("files/Faceplate Round.png").map(|data| (ContentType::PNG, data))
            }
            "files/Normal Round.png" => {
                fs::read("files/Normal Round.png").map(|data| (ContentType::PNG, data))
            }
            "files/Jetbrains Mono.woff2" => {
                fs::read("files/Jetbrains Mono.woff2").map(|data| (ContentType::WOFF2, data))
            }
            "files/robots.txt" => {
                fs::read("files/robots.txt").map(|data| (ContentType::Text, data))
            }
            _ => std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Nope")),
        })
        .map_err(|_| Status::InternalServerError)
    } else {
        println!("{file}");
        Err(Status::NotFound)
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

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Error};
use actix_files::NamedFile;

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hi! You can get the zodiac sign of your input date via /check/<month>/<day>!")
}

// check the zodiac sign of an input date
pub fn get_zodiac_sign(month: i32, day: i32) -> &'static str {
    if (month == 3 && day >=21) || (month==4 && day <= 19) {
        return "aries";
    }
    if (month == 4 && day >=20) || (month==5 && day <= 20) {
        return "taurus";
    }
    if (month == 5 && day >=21) || (month==6 && day <= 21) {
        return "gemini";
    }
    if (month == 6 && day >=22) || (month==7 && day <= 22) {
        return "cancer";
    }
    if (month == 7 && day >=23) || (month==8 && day <= 22) {
        return "leo";
    }
    if (month == 8 && day >=23) || (month==9 && day <= 22) {
        return "virgo";
    }
    if (month == 9 && day >=23) || (month==10 && day <= 23) {
        return "libra";
    }
    if (month == 10 && day >=24) || (month==11 && day <= 22) {
        return "scorpio";
    }
    if (month == 11 && day >=23) || (month==12 && day <= 21) {
        return "sagittarius";
    }
    if (month == 12 && day >=22) || (month==1 && day <= 19) {
        return "capricorn";
    }
    if (month == 1 && day >=20) || (month==2 && day <= 18) {
        return "aquarius";
    }
    if (month == 2 && day >=19) || (month==3 && day <= 20) {
        return "pisces";
    }
    return "invalid";
}

#[get("/check/{month}/{day}")]
async fn check(info: web::Path<(i32, i32)>) -> Result<NamedFile, Error> {
    let result = get_zodiac_sign(info.0, info.1);
    let name = result.to_string();
    let file_name = format!("./src/images/{}.png", name);
    Ok(NamedFile::open(file_name)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running the service");
    HttpServer::new(|| App::new().service(hello).service(check))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

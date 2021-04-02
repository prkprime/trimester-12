pub mod register;
use register::Register;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use fancy_regex::Regex;
use lazy_static::{lazy_static, LazyStatic};
use tera::{Context, Tera};

async fn render_register(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctxt = Context::new();
    let errors: Vec<&str> = Vec::new(); //check_password(&form.password);
    ctxt.insert("errors", &errors);
    ctxt.insert("title", "Password Checker");
    ctxt.insert("success", &-1i32);

    let resp = tmpl.render("register.html", &ctxt).unwrap();
    HttpResponse::Ok().content_type("text/html").body(resp)
}

fn check_password(passwd: &String) -> Vec<&str> {
    lazy_static! {
        static ref ONE_CAP_LET: Regex = Regex::new(r"^(?=.*[A-Z])").unwrap();
        static ref ONE_SML_LET: Regex = Regex::new(r"^(?=.*[a-z])").unwrap();
        static ref ONE_NUM: Regex = Regex::new(r"^(?=.*\d)").unwrap();
        static ref ONE_SPL_LET: Regex = Regex::new(r"^(?=.*[@$!%*?&])").unwrap();
        static ref LENGTH: Regex = Regex::new(r"^{8,20}$").unwrap();
        static ref RE: Regex =
            Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,20}$")
                .unwrap();
    }
    let mut errors: Vec<&str> = Vec::new();
    if ONE_CAP_LET.is_match(passwd).unwrap() == false {
        errors.push("add at least one upper case letter");
    }
    if ONE_SML_LET.is_match(passwd).unwrap() == false {
        errors.push("add at least one lower case letter");
    }
    if ONE_NUM.is_match(passwd).unwrap() == false {
        errors.push("add at least numeric character");
    }
    if ONE_SPL_LET.is_match(passwd).unwrap() == false {
        errors.push("add at least one Special Character");
    }
    if LENGTH.is_match(passwd).unwrap() == false {
        errors.push("keep password length between 8-20");
    }
    errors
}

async fn register(tmpl: web::Data<Tera>, form: web::Form<Register>) -> impl Responder {
    let errors: Vec<&str> = check_password(&form.password);
    let mut ctxt = Context::new();
    ctxt.insert("title", "Password Checker");
    let mut status = 0;
    if errors.len() == 0 {
        status = 1;
    }
    ctxt.insert("success", &status);
    ctxt.insert("errors", &errors);
    let resp = tmpl.render("register.html", &ctxt).unwrap();
    HttpResponse::Ok().content_type("text/html").body(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/*")).unwrap();
        App::new()
            .wrap(Logger::default()) // enable logger
            .data(tera)
            .service(
                web::resource("/register")
                    .route(web::get().to(render_register))
                    .route(web::post().to(register)),
            )
    })
    .workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

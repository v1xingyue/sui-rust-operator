use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::hook::HookCaller;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[post("/myinfo")]
async fn share_info(data: web::Data<HookData>) -> impl Responder {
    println!("{}", data.get_ref().msg);
    data.get_ref().to_string()
}

struct HookData {
    msg: String,
}

impl HookData {
    pub fn to_string(&self) -> String {
        String::from(format!("{}", self.msg))
    }
}

pub async fn start(bind_host: &str, port: u16, _hook: HookCaller) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(HookData {
                msg: String::from("hello world"),
            }))
            .service(hello)
            .service(echo)
            .service(share_info)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((bind_host, port))?
    .run()
    .await
}

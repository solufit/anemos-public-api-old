use actix_web::web;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            //.route("/", web::get().to(index))
    );
}

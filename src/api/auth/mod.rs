mod login;
mod logout;

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.configure(login::configure).configure(logout::configure);
}

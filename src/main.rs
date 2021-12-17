use lib::run;

mod lib;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}

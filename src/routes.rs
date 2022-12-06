#[derive(serde::Deserialize)]
pub struct CreateUserBody {
    name: String,
}

#[actix_web::post("/user")]
pub async fn create_user(
    state: actix_web::web::Data<super::AppState>,
    user: actix_web::web::Json<CreateUserBody>,
) -> impl actix_web::Responder {
    println!("Registering user {:?}", user.name);

    let new_user = super::database::Database::create_user(&state.conn, user.name.clone()).await;
    println!("{:?}", new_user);

    ""
}

#[actix_rt::test]
pub async fn create_user() {
    let state = crate::AppState {
        db: crate::common::database::connect().await,
    };

    let user = crate::handlers::user::create_user(
        crate::entity::user::User {
            id: None,
            name: "John Doe".to_string(),
            sources: vec![],
        },
        actix_web::web::Data::new(state),
    )
    .await;

    match user {
        Ok(user) => {
            println!("{}", user.id.unwrap().to_string());
            // Adding the user ID to env variable so the user isnt going to get created over and over again.
            // TODO: Maybe search for a better method to achieve the same
            // std::env::set_var("user_id", user.id.unwrap().to_string());

            assert_eq!(user.name, "John Doe")
        }
        Err(error) => panic!("{}", error),
    }
}

#[actix_rt::test]
pub async fn get_user() {
    let state = crate::AppState {
        db: crate::common::database::connect().await,
    };

    let user = crate::handlers::user::get_user(
        crate::routes::parse_id(&"63c53s0ee0a74a9e466187037".to_string()).unwrap(),
        actix_web::web::Data::new(state),
    )
    .await;

    match user {
        Ok(user) => assert_eq!(user.name, "John Doe"),
        Err(error) => panic!("{}", error),
    }
}

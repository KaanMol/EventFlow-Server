use actix_web::{dev::ServiceRequest, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use base64::Engine;

pub async fn auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    // TODO: move this to documentation
    // generate correct pem with:
    // openssl rsa -pubout -in rsa-private.pem -out rsa-public.pem
    let env_bytes_base64 = match std::env::var("TOKEN_PEM") {
        Ok(val) => val.into_bytes(),
        Err(_) => {
            tracing::error!("Could not find TOKEN_PEM in environment variables");
            return Err((
                actix_web::error::ErrorInternalServerError("Authorization check failed"),
                req,
            ));
        }
    };

    let base64_engine = base64::engine::GeneralPurpose::new(
        &base64::alphabet::URL_SAFE,
        base64::engine::general_purpose::NO_PAD,
    );

    let decoded_bytes = &base64_engine.decode(&env_bytes_base64).unwrap();

    let key = &jsonwebtoken::DecodingKey::from_rsa_pem(decoded_bytes).unwrap();

    let result = jsonwebtoken::decode::<crate::app::Claims>(
        credentials.token(),
        key,
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256),
    )
    .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()));

    match result {
        Ok(user_claims) => {
            // TODO: Implement permissions in Identity Provider to make Authoriation possible
            // req.attach(claims.permissions);

            // Inject user claims in requests
            req.extensions_mut().insert(user_claims.claims);
            Ok(req)
        }
        // required by `actix-web-httpauth` validator signature
        Err(e) => Err((e, req)),
    }
}

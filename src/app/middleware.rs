use actix_web::{dev::ServiceRequest, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let result = jsonwebtoken::decode::<crate::app::Claims>(
        credentials.token(),
        // TODO: move this to documentation
        // generate correct pem with:
        // openssl rsa -pubout -in rsa-private.pem -out rsa-public.pem
        &jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("../certs/token.pem")).unwrap(),
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

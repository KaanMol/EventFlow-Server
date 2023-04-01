use std::fmt;

use actix_web::{body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use super::error::ResourceError;

// Struct that holds the data of a response
#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub error: Option<ResponseError>,
    pub data: Option<T>,
}

// Struct that holds the data of an error in a response
pub struct ResponseError {
    code: StatusCode,
    message: String,
}

// Implementation of the Serialize trait for the ResponseError struct
// This allows us to convert any ResponseError to a JSON object
// Cannot be derived using a macro because `StatusCode` does not implement Serialize :(
impl Serialize for ResponseError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ResponseError", 2)?;
        state.serialize_field("code", &self.code.as_u16())?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

// Implementation of the Deserialize trait for the ResponseError struct
// This allows us to convert any JSON object to a ResponseError
// Cannot be derived using a macro because `StatusCode` does not implement Deserialize :(
impl<'de> Deserialize<'de> for ResponseError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Code,
            Message,
        }

        struct ResponseErrorVisitor;

        impl<'de> serde::de::Visitor<'de> for ResponseErrorVisitor {
            type Value = ResponseError;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ResponseError")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<ResponseError, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let code: u16 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let message = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                let status_code = StatusCode::from_u16(code).expect("StatusCode is invalid");

                Ok(ResponseError {
                    code: status_code,
                    message,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<ResponseError, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut code = None;
                let mut message = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Code => {
                            if code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code = Some(map.next_value()?);
                        }
                        Field::Message => {
                            if message.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message = Some(map.next_value()?);
                        }
                    }
                }
                let code = code.ok_or_else(|| serde::de::Error::missing_field("code"))?;
                let message = message.ok_or_else(|| serde::de::Error::missing_field("message"))?;

                let status_code = StatusCode::from_u16(code).expect("StatusCode is invalid");

                Ok(ResponseError {
                    code: status_code,
                    message,
                })
            }
        }

        const FIELDS: &[&str] = &["code", "message"];
        deserializer.deserialize_struct("ResponseError", FIELDS, ResponseErrorVisitor)
    }
}

// Implementation of the from_data and from_error methods for an ApiResponse
// These methods allow us to create an ApiResponse for data or errors
impl<T> ApiResponse<T>
where
    T: Serialize,
{
    // Create an ApiResponse based on data
    pub fn from_data(data: T) -> Self {
        Self {
            success: true,
            error: None,
            data: Some(data),
        }
    }

    // Create an ApiResponse based on an error
    pub fn from_error(code: StatusCode, error: ResourceError) -> Self {
        Self {
            success: false,
            error: Some(ResponseError {
                code,
                message: error.to_string(),
            }),
            data: None,
        }
    }
}

//  ApiResponse<T> -> HttpResponse<BoxBody> conversion
//  Implementing the From trait here allows us to reuse this conversion for
//  the Responder trait and ResponseError trait using the into() method
impl<T> From<ApiResponse<T>> for HttpResponse<BoxBody>
where
    T: Serialize,
{
    fn from(response: ApiResponse<T>) -> Self {
        // Determine the status code based on the error status code
        let status = match response.error.as_ref() {
            Some(error) => error.code,
            None => StatusCode::OK, // TODO: Also allow for a status code of 201 CREATED, etc.
        };

        // Build the response using the status code
        let mut http_response = HttpResponse::build(status);

        // Add the ApiResponse as the body of the response
        http_response.json(response)
    }
}

// Implementation of the Responder trait for the ApiResponse struct
// This allows us to use ApiResponse as a response type in a handler
impl<T> Responder for ApiResponse<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        // Convert the ApiResponse to an HttpResponse using the implementation of the From trait
        self.into()
    }
}

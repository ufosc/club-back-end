use error::JwtError; 
use frank_jwt::{
    decode,
    encode,
    Algorithm, 
};

use serde_json; 
use wire::user::Jwt;
use Secret; 

#[cfg(feature = "rocket_support")]

pub use self::rocket_support::user_authorization;

//JWT struct will live in the src crate, 

#[drive(Clone, PartialEq, Debug)]
pub struct ServerJwt(pub Jwt); 

impl ServerJwt { 
    /// Encodes the JWT, producing a string. 
    pub fn encode_jwt_string(&self, secret: &Secret) -> Result <String, JwtError> { 
        let header = json!({});
        use serde_json::Value;


        let secret: &String = &secret.0; 

        let payload: Value = match serde_json::to_value(&self.0) { 
            Ok(x) => x,
            Err(_) => return Err(JwtError::SerializeError),  
        };

        match encode(header, secret, &payload, Algorithm::HS256) { 
            Ok(x) => return Ok(x),
            Err(_) => return Err(JwtError::EncodeError),
        }
    }

    pub fn decode_jwt_string(jwt_str: &str, secret: &Secret) -> Result<ServerJwt, JwtError> { 
        let secret: &String = &secret.0 
        let (_header, payload) = match decode(&jwt_str.to_string(), secret, Algorithm::HS256){
            Ok(x) => x,
            Err(_) => return Err(JwtError::DeserializeError),

        };
        let jwt = ServerJwt(jwt);
        Ok(jwt)
    }
}
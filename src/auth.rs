use crate::model::IdTokenPayload;
use jsonwebtoken::errors::Result as JwtResult;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

pub fn decode_id_token(id_token: &str, secret: &str) -> JwtResult<IdTokenPayload> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();
    Ok(jsonwebtoken::decode(id_token, &key, &validation)?.claims)
}

pub fn encode_id_token(payload: &IdTokenPayload, secret: &str) -> JwtResult<String> {
    let header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());
    Ok(jsonwebtoken::encode(&header, &payload, &key)?)
}

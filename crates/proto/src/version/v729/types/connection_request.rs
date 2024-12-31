use std::collections::BTreeMap;
use std::io::{Cursor, Read};

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use byteorder::{LittleEndian, ReadBytesExt};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use serde_json::Value;
use varint_rs::VarintReader;
use p384::pkcs8::spki;
use uuid::Uuid;

pub const MOJANG_PUBLIC_KEY: &str = "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAECRXueJeTDqNRRgJi/vlRufByu/2G0i2Ebt6YMar5QX/R0DIIyrJMcUpruK4QveTfJSTp3Shlq4Gk34cD/4GUWwkv0DVuzeuB+tXija7HBxii03NHDbPAD0AKnLr2wdAp";

#[derive(Debug, Clone)]
pub struct ConnectionRequest {
    /// Array of Base64 encoded JSON Web Token certificates to authenticate the player.
    ///
    /// The last certificate in the chain will have a property 'extraData' that contains player identity information including the XBL XUID (if the player was signed in to XBL at the time of the connection).
    pub certificate_chain: Vec<BTreeMap<String, Value>>,
    /// Base64 encoded JSON Web Token that contains other relevant client properties.
    ///
    /// Properties Include:
    /// - SelfSignedId
    /// - ServerAddress = (unresolved url if applicable)
    /// - ClientRandomId
    /// - SkinId
    /// - SkinData
    /// - SkinImageWidth
    /// - SkinImageHeight
    /// - CapeData
    /// - CapeImageWidth
    /// - CapeImageHeight
    /// - SkinResourcePatch
    /// - SkinGeometryData
    /// - SkinGeometryDataEngineVersion
    /// - SkinAnimationData
    /// - PlayFabId
    /// - AnimatedImageData = Array of:
    ///   - Type
    ///   - Image
    ///   - ImageWidth
    ///   - ImageHeight
    ///   - Frames
    ///   - AnimationExpression
    /// - ArmSize
    /// - SkinColor
    /// - PersonaPieces = Array of:
    ///   - PackId
    ///   - PieceId
    ///   - IsDefault
    ///   - PieceType
    ///   - ProductId
    /// - PieceTintColors = Array of:
    ///   - PieceType
    ///   - Colors = Array of color hexstrings
    /// - IsEduMode (if edu mode)
    /// - TenantId (if edu mode)
    /// - ADRole (if edu mode)
    /// - IsEditorMode
    /// - GameVersion
    /// - DeviceModel
    /// - DeviceOS = (see enumeration: BuildPlatform)
    /// - DefaultInputMode = (see enumeration: InputMode)
    /// - CurrentInputMode = (see enumeration: InputMode)
    /// - UIProfile = (see enumeration: UIProfile)
    /// - GuiScale
    /// - LanguageCode
    /// - PlatformUserId
    /// - ThirdPartyName
    /// - ThirdPartyNameOnly
    /// - PlatformOnlineId
    /// - PlatformOfflineId
    /// - DeviceId
    /// - TrustedSkin
    /// - PremiumSkin
    /// - PersonaSkin
    /// - OverrideSkin
    /// - CapeOnClassicSkin
    /// - CapeId
    /// - CompatibleWithClientSideChunkGen
    pub raw_token: BTreeMap<String, Value>,
}

#[derive(Deserialize, Debug)]
struct CertChain {
    pub chain: Vec<String>
}

#[derive(Deserialize, Debug)]
struct KeyPayload {
    #[serde(rename = "identityPublicKey")]
    pub public_key: String
}

fn parse_first_token(token: &str) -> Result<bool, ProtoCodecError> {
    let header = jsonwebtoken::decode_header(token)?;
    let Some(base64_x5u) = header.x5u else {
        todo!();
    };
    let bytes = BASE64_STANDARD.decode(base64_x5u)?;

    let public_key = spki::SubjectPublicKeyInfoRef::try_from(bytes.as_ref()).map_err(|e| {
        unimplemented!("{e:?}");
        ProtoCodecError::LeftOvers(0)
    })?;

    let decoding_key = DecodingKey::from_ec_der(public_key.subject_public_key.raw_bytes());
    let mut validation = Validation::new(Algorithm::ES384);
    validation.validate_exp = true;
    validation.validate_nbf = true;

    // Decode token
    let payload = jsonwebtoken::decode::<KeyPayload>(token, &decoding_key, &validation)?;
    Ok(payload.claims.public_key.eq(&MOJANG_PUBLIC_KEY))
}

fn parse_mojang_token(token: &str) -> Result<String, ProtoCodecError> {
    let bytes = BASE64_STANDARD.decode(MOJANG_PUBLIC_KEY)?;
    let public_key = spki::SubjectPublicKeyInfoRef::try_from(bytes.as_ref()).map_err(|e| {
        unimplemented!("{e:?}");
        ProtoCodecError::LeftOvers(0)
    })?;

    let decoding_key = DecodingKey::from_ec_der(public_key.subject_public_key.raw_bytes());
    let mut validation = Validation::new(Algorithm::ES384);
    validation.set_issuer(&["Mojang"]);
    validation.validate_nbf = true;
    validation.validate_exp = true;

    let payload = jsonwebtoken::decode::<KeyPayload>(token, &decoding_key, &validation)?;
    Ok(payload.claims.public_key)
}

#[derive(Deserialize, Debug)]
pub struct RawIdentity {
    #[serde(rename = "XUID")]
    pub xuid: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "identity")]
    pub uuid: Uuid
}

#[derive(Deserialize, Debug)]
struct IdentityPayload {
    #[serde(rename = "extraData")]
    pub client_data: RawIdentity,
    #[serde(rename = "identityPublicKey")]
    pub public_key: String
}

fn parse_identity_token(token: &str, key: &str) -> Result<IdentityPayload, ProtoCodecError> {
    let bytes = BASE64_STANDARD.decode(key)?;
    let public_key = spki::SubjectPublicKeyInfoRef::try_from(bytes.as_ref()).map_err(|e| {
        unimplemented!("{e:?}");
        ProtoCodecError::LeftOvers(0)
    })?;

    let decoding_key = DecodingKey::from_ec_der(public_key.subject_public_key.raw_bytes());
    let mut validation = Validation::new(Algorithm::ES384);
    validation.set_issuer(&["Mojang"]);
    validation.validate_exp = true;
    validation.validate_nbf = true;

    let payload = jsonwebtoken::decode::<IdentityPayload>(token, &decoding_key, &validation)?;
    Ok(payload.claims)
}

impl ProtoCodec for ConnectionRequest {
    fn proto_serialize(&self, _stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        todo!()
    }

    // TODO: Add microsoft auth
    // TODO: Validate jwts (This is hard, Zuri nor Vincent could help me)
    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        stream.read_u32_varint()?;

        let cert_chain_json_len = stream.read_i32::<LittleEndian>()?;
        let mut cert_chain_json = Vec::with_capacity(cert_chain_json_len as usize);
        cert_chain_json.resize(cert_chain_json_len as usize, 0);

        stream.read_exact(&mut cert_chain_json)?;

        let cert_chain = serde_json::from_slice::<CertChain>(&cert_chain_json)?;

        let identity;
        match cert_chain.chain.len() {
            // User is offline
            1 => {
                todo!();
            },
            // Authenticated with Microsoft services
            3 => {
                // Verify the first token and use its public key for the next token.

                let mut valid = parse_first_token(&cert_chain.chain[0])?;
                if !valid {
                    // Login attempted using forged token chain.
                    todo!()
                }

                let key = parse_mojang_token(&cert_chain.chain[1])?;
                identity = parse_identity_token(&cert_chain.chain[2], &key)?;
            },
            // This should not happen...
            _ => {
                todo!()
            }
        }

        dbg!(identity);

        todo!()

        // let mut certificate_chain: Vec<BTreeMap<String, Value>> = vec![];
        //
        // // Read the ConnectionRequests length, Mojang stores it as a String
        // // (certificate_chain len + raw_token len + 8)
        // // 8 = i32 len + i32 len (length of certificate_chain's len and raw_token's len)
        // // can be ignored, other lengths are provided
        // stream.read_u32_varint()?;
        //
        // let certificate_chain_string = read_i32_string(stream)?;
        //
        // // parse certificate chain string into json
        // let mut certificate_chain_json = serde_json::from_str(&certificate_chain_string)?;
        //
        // let certificate_chain_json_jwts = match certificate_chain_json {
        //     Value::Object(ref mut v) => {
        //         let chain = v.get_mut("chain").ok_or(ProtoCodecError::FormatMismatch(
        //             "Missing element chain in JWT certificate_chain",
        //         ))?;
        //
        //         match chain {
        //             Value::Array(v) => v,
        //             _ => {
        //                 // the certificate chain should always be an object with just an
        //                 // array of JWTs called "chain"
        //                 return Err(ProtoCodecError::FormatMismatch(
        //                     "Expected chain in JWT certificate_chain to be of type Array",
        //                 ));
        //             }
        //         }
        //     }
        //     _ => {
        //         // the certificate chain should always be an object with just an array of
        //         // JWTs called "chain"
        //         return Err(ProtoCodecError::FormatMismatch(
        //             "Expected Object in base of JWT certificate_chain",
        //         ));
        //     }
        // };
        //
        // let mut key_data = vec![];
        //
        // for jwt_json in certificate_chain_json_jwts {
        //     let jwt_string = match jwt_json {
        //         Value::String(str) => str,
        //         _ => {
        //             // the certificate chain's should always be a jwt string
        //             return Err(ProtoCodecError::FormatMismatch(
        //                 "Expected chain array in certificate_chain to just contain Strings",
        //             ));
        //         }
        //     };
        //
        //     // Extract header
        //     let jwt_header =
        //         jsonwebtoken::decode_header(jwt_string).map_err(ProtoCodecError::JwtError)?;
        //
        //     let mut jwt_validation = Validation::new(jwt_header.alg);
        //     // TODO: This definitely is not right. Even Zuri-MC doesn't understand this.. I may understand it.. I do understand it, update I don't. But I now know someone that does, I hope
        //     // TODO: Someone else should find out how this works
        //     jwt_validation.insecure_disable_signature_validation();
        //     jwt_validation.set_required_spec_claims::<&str>(&[]);
        //
        //     // Is first jwt, use self-signed header from x5u
        //     if key_data.is_empty() {
        //         let x5u = jwt_header.x5u.ok_or(ProtoCodecError::FormatMismatch(
        //             "Expected x5u in JWT header",
        //         ))?;
        //
        //         let x5u = x5u.as_bytes();
        //
        //         key_data = BASE64_STANDARD
        //             .decode(x5u)
        //             .map_err(ProtoCodecError::Base64DecodeError)?;
        //     }
        //
        //     // Decode the jwt string into a jwt object
        //     let jwt = jsonwebtoken::decode::<BTreeMap<String, Value>>(
        //         &jwt_string,
        //         &DecodingKey::from_ec_der(&key_data),
        //         &jwt_validation,
        //     )
        //     .map_err(ProtoCodecError::JwtError)?;
        //
        //     let identity_field =
        //         jwt.claims
        //             .get("identityPublicKey")
        //             .ok_or(ProtoCodecError::FormatMismatch(
        //                 "Missing identityPublicKey field in JWT for validation",
        //             ))?;
        //
        //     key_data = match identity_field {
        //         Value::String(str) => BASE64_STANDARD
        //             .decode(str.as_bytes())
        //             .map_err(ProtoCodecError::Base64DecodeError)?,
        //         _ => {
        //             return Err(ProtoCodecError::FormatMismatch(
        //                 "Expected identityPublicKey field in JWT to be of type String",
        //             ))
        //         }
        //     };
        //
        //     certificate_chain.push(jwt.claims);
        // }
        //
        // let raw_token_string = read_i32_string(stream)?;
        //
        // // Extract header
        // let raw_token_jwt_header =
        //     jsonwebtoken::decode_header(&raw_token_string).map_err(ProtoCodecError::JwtError)?;
        //
        // let mut jwt_validation = Validation::new(raw_token_jwt_header.alg);
        // // TODO: This definitely is not right. Even Zuri-MC doesn't understand this.. I may understand it.. I do understand it, update I don't.
        // // TODO: Someone else should find out how this works
        // jwt_validation.insecure_disable_signature_validation();
        // jwt_validation.set_required_spec_claims::<&str>(&[]);
        //
        // // Decode the jwt string into a jwt object
        // let raw_token = jsonwebtoken::decode::<BTreeMap<String, Value>>(
        //     &raw_token_string,
        //     &DecodingKey::from_ec_der(&[]),
        //     &jwt_validation,
        // )
        // .map_err(ProtoCodecError::JwtError)?
        // .claims;
        //
        // Ok(Self {
        //     certificate_chain,
        //     raw_token,
        // })
    }

    fn get_size_prediction(&self) -> usize {
        // TODO
        1
    }
}

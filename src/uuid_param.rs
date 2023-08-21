use rocket::request::FromParam;
use std::str::FromStr;
use uuid::Uuid;
pub struct UuidParam(pub Uuid);

impl<'r> FromParam<'r> for UuidParam {
    type Error = String;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Uuid::from_str(param)
            .map(UuidParam)
            .map_err(|e| e.to_string())
    }
}

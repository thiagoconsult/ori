use serde::Serialize;

#[derive(Serialize)]
pub struct Description(String);
impl Description {
    pub fn new(value: &str) -> Result<Self, String> {
        let value_in = value.trim();

        if value_in.chars().count() < 10 {
            Err("Tamanho Inválido.".to_owned())
        } else {
            Ok(Self(value_in.to_owned()))
        }
    }
}

#[derive(Serialize)]
pub struct AssetId(String);
impl TryFrom<&str> for AssetId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value_in = value.trim();

        if value_in.is_empty() || value_in.chars().count() < 5 {
            Err("Asset Id Inválido".to_owned())
        } else {
            Ok(Self(value_in.to_owned()))
        }
    }
}

#[derive(Serialize)]
#[serde(transparent)]
pub struct ReportedBy {
    value: String,
}
impl ReportedBy {
    pub fn new(value: &Option<String>) -> Self {
        let value_in = match value {
            Some(v) => {
                if v.trim().chars().count() > 5 {
                    v.trim().to_owned()
                } else {
                    "anonymous".to_owned()
                }
            }
            None => "anonymous".to_owned(),
        };

        Self { value: value_in }
    }
}

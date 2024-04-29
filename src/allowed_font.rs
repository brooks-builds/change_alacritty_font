pub enum AllowedFont {
    Iosevka { useable: bool },
    Wingdings { useable: bool },
    Unknown
}

impl AllowedFo bnt {
    pub fn new(name: &str) -> Self {
        match name {
            "iosevka" => Self::Iosevka { useable: true }
        }
    }
}

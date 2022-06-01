use juniper::GraphQLEnum;

#[derive(Display, GraphQLEnum, EnumString)]
pub enum Language {
    #[strum(serialize="en")]
    EN,
    #[strum(serialize="es")]
    ES,
    #[strum(default)]
    UNKNOWN,
}

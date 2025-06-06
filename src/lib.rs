use specta::{Type, ts};
use sha3::{Sha3_256, Digest};
use serde::{Serialize, Deserialize as SerdeDeserialize};
use time::Time;

use uuid::Uuid;
use std::{fmt::Display, ops::Deref};
use spacetimedb::{
    sats::{impl_deserialize, impl_serialize, impl_st},
    ReducerContext,
};


// st_uuid implementation
#[derive(Debug, Serialize, SerdeDeserialize, Type)]
pub struct StUuid(pub Uuid);

// spacetimedb impls
impl_st!([] StUuid, spacetimedb::sats::AlgebraicType::String);
impl_serialize!([] StUuid, (self, ser) => {
    ser.serialize_str(self.hyphenated().encode_upper(&mut Uuid::encode_buffer()))
});
impl_deserialize!([] StUuid, de => {
    let s = <std::string::String as spacetimedb::Deserialize>::deserialize(de).map(|s| s.into_boxed_str())?;
    Ok(Uuid::parse_str(&s).map(|u| u.into()).expect("Failed to Deserialize to UUID"))
});

impl StUuid {
    pub fn new(ctx: &ReducerContext) -> Self {
        StUuid(Uuid::from_bytes(ctx.random()))
    }
}

impl Display for StUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl Deref for StUuid {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Uuid> for StUuid {
    fn from(val: Uuid) -> Self {
        Self(val)
    }
}

// Seed and related types

#[derive(Type)]
pub struct Seed {
    pub seed: String,
    pub s: String,
    pub f: String,
    pub r: Rarity,
    pub v: String,
    pub t: Time,
    pub u: StUuid,
}

#[derive(Type)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Type)]
pub struct Color {
    pub hex: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Type)]
pub enum IrisShape {
    Round,
    Almond,
    Heart,
    Dot,
    Star,
}

#[derive(Type)]
pub struct Iris {
    pub color: Color,
    pub shape: IrisShape
}

#[derive(Type)]
pub struct Eye {
    pub color: Color,
    pub iris: Iris,
}

#[derive(Type)]
pub struct Eyes {
    pub left: Eye,
    pub light: Eye,
}

#[derive(Type)]
pub struct HairColor {
    pub gradient: bool,
    pub color1: Color,
    pub color2: Color,
}

#[derive(Type)]
pub enum HairStyle {
    Variant1,
    Variant2,
    Variant3,
}

#[derive(Type)]
pub struct Hair {
    pub color: HairColor,
    pub length: u16,
    pub style: HairStyle,
}

#[derive(Type)]
pub struct Physical {
    pub race: Race,
    pub gender: Gender,
    pub age: u16, // in years
    pub height: u16, // in cm
    pub weight: u16, // in kg
    pub eyes: Eyes,
    pub hair: Hair,
    pub skin_color: Color,
}

#[derive(Type)]
pub struct Name {
    pub first: String,
    pub middle: Option<String>,
    pub last: String,
}

#[derive(Type)]
pub enum MaritalStatus {
    Single,
    Married,
    Divorced,
    Widowed,
}

#[derive(Type)]
pub struct Parents {
    pub mother: Option<Seed>,
    pub father: Option<Seed>,
}

#[derive(Type)]
pub struct Siblings {
    pub count: u8,
    pub sibling1: Option<Seed>,
    pub sibling2: Option<Seed>,
    pub sibling3: Option<Seed>,
    pub sibling4: Option<Seed>,
    pub sibling5: Option<Seed>,
}

#[derive(Type)]
pub struct Children {
    pub count: u8,
    pub children1: Option<Seed>,
    pub children2: Option<Seed>,
    pub children3: Option<Seed>,
    pub children4: Option<Seed>,
    pub children5: Option<Seed>,
}

#[derive(Type)]
pub struct Relationship {
    pub status: MaritalStatus,
    pub parents: Parents,
    pub siblings: Siblings,
    pub children: Children,
}

#[derive(Type)]
pub enum Rarity {
    Common,
    Rare,
    SuperRare,
    SuperSuperRare,
    UltraRare,
    Epic,
    Legendary,
    Mythic,
    Unique,
}

#[derive(Type)]
pub enum Race {
    Human,
}

#[derive(Type)]
pub enum Class {
    Villager,
}

#[derive(Type)]
pub struct Metadata {
    pub seed: Seed,
    pub name: Name,
    pub class: Class,
    pub physical: Physical,
    pub relationship: Relationship,
    pub money: u32,
}

pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input);
    let result = hasher.finalize();
    return result.iter().map(|b| format!("{:02x}", b)).collect();
}

pub fn convert() {
    let out_stuuid = ts::export::<StUuid>(&Default::default()).unwrap();
    let out_seed = ts::export::<Seed>(&Default::default()).unwrap();
    let out_gender = ts::export::<Gender>(&Default::default()).unwrap();
    let out_color = ts::export::<Color>(&Default::default()).unwrap();
    let out_iris_shape = ts::export::<IrisShape>(&Default::default()).unwrap();
    let out_iris = ts::export::<Iris>(&Default::default()).unwrap();
    let out_eye = ts::export::<Eye>(&Default::default()).unwrap();
    let out_eyes = ts::export::<Eyes>(&Default::default()).unwrap();
    let out_hair_color = ts::export::<HairColor>(&Default::default()).unwrap();
    let out_hair_style = ts::export::<HairStyle>(&Default::default()).unwrap();
    let out_hair = ts::export::<Hair>(&Default::default()).unwrap();
    let out_physical = ts::export::<Physical>(&Default::default()).unwrap();
    let out_name = ts::export::<Name>(&Default::default()).unwrap();
    let out_marital_status = ts::export::<MaritalStatus>(&Default::default()).unwrap();
    let out_parents = ts::export::<Parents>(&Default::default()).unwrap();
    let out_siblings = ts::export::<Siblings>(&Default::default()).unwrap();
    let out_children = ts::export::<Children>(&Default::default()).unwrap();
    let out_relationship = ts::export::<Relationship>(&Default::default()).unwrap();
    let out_rarity = ts::export::<Rarity>(&Default::default()).unwrap();
    let out_race = ts::export::<Race>(&Default::default()).unwrap();
    let out_class = ts::export::<Class>(&Default::default()).unwrap();
    let out_metadata = ts::export::<Metadata>(&Default::default()).unwrap();

    let lines = [
        out_stuuid,
        out_seed,
        out_gender,
        out_color,
        out_iris_shape,
        out_iris,
        out_eye,
        out_eyes,
        out_hair_color,
        out_hair_style,
        out_hair,
        out_physical,
        out_name,
        out_marital_status,
        out_parents,
        out_siblings,
        out_children,
        out_relationship,
        out_rarity,
        out_race,
        out_class,
        out_metadata,
    ];

    let lines_to_write: Vec<_> = lines
        .iter()
        .flat_map(|s| s.lines())
        .collect();

    std::fs::write("lib.ts", lines_to_write.join("\n")).unwrap();
}
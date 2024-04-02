use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct JosaaItem {
    pub name: String,
    pub course: String,
    pub quota: String,
    pub seat: String,
    pub gender: String,
    pub or: u32,
    pub cr: u32,
    pub year: u16,
    pub round: u8
}

#[derive(FromRow, Debug, Clone, Serialize)]
//TOGGLE COMMENT the Attributes to be displayed.
//Attributes are displayed in order of declaration here.
pub struct Ranking {
    // pub id: u32,
    // pub app_no: String,
    pub name: String,
    // pub dob: String,
    // pub comm: String,
    pub category: String,
    pub cutoff: f32,
    pub rank: u32,
    pub college: u16,
    pub college_name: String,
    pub branch_name: String
}

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct College {
    pub id: u32,
    pub name: String,
    pub city: String
}

#[allow(dead_code)]
pub struct Branch {
    pub id: String,
    pub name: String
}
use serde::Serialize;
use sqlx::FromRow;

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
    // pub branch: String,
    pub branch_name: String
}
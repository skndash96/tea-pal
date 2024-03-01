use csv::ReaderBuilder;
use serde::{ Serialize, Deserialize };
use std::boxed::Box;
use std::fs::read_to_string;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ranking {
    idx: u32,
    // app_no: u64,
    name: String,
    // dob: String,
    cutoff: f32,
    rank: u64,
    // comm: String,
    college: String,
    branch: String,
    category: String,

    #[serde(default = "default_string")]
    college_name: String,
    #[serde(default = "default_string")]
    college_city: String,
    
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Options {
    pub cutoff: Option<f32>,
    pub rank: Option<usize>,
    pub category: Option<String>,
    pub branch: Option<String>,

    #[serde(default = "default_limit")]
    pub limit: usize,
}
fn default_limit() -> usize { 10 }
fn default_string() -> String { String::new() }

#[derive(Clone)]
pub struct Ranklist {
    list: Vec<Ranking>
}

impl Ranklist {
    pub fn add(&mut self, item: Ranking) {
        self.list.push(item);
    }

    pub fn new() -> Ranklist {
        Ranklist {
            list: vec![]
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn query(&self, q: Options) -> Option<&[Ranking]> {
        if let Some(mark) = q.cutoff {
            if let Some(start) = self.list.iter().position(|x| x.cutoff <= mark) {
                return Some(
                    &self.list[start..start + q.limit]
                );
            }
        }

        None
    }
}

pub fn init_db() -> Result<Ranklist, Box<csv::Error>> {
    let coll = get_coll_db();
    
    let mut db: Ranklist = Ranklist::new();

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("src/tnea2023_1.csv")?;

    for res in rdr.deserialize::<Ranking>() {
        let mut item = res?;
        
        if let Some((name, city)) = coll.get(&item.college) {
            item.college_name = name.clone();
            item.college_city = city.clone();
        }

        db.add(item);
    }

    Ok(db.clone())
}



pub type CollMap = HashMap<String, (String, String)>;

pub fn get_coll_db() -> CollMap {
    let content: String = read_to_string("./src/tnea_college-id.json").unwrap();
    
    let mapping : CollMap = serde_json::from_str(&content).unwrap();

    mapping
}
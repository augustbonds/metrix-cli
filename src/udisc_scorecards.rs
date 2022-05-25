use csv::StringRecord;
use futures::StreamExt;
use serde::Deserialize;


#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UDiscScorecard {
    pub player_name: String,
    pub course_name: String,
    pub layout_name: String,
    pub date: String,
    pub total: u32,
    pub plus_minus: Option<i32>,
    pub hole1: Option<u32>,
    pub hole2: Option<u32>,
    pub hole3: Option<u32>,
    pub hole4: Option<u32>,
    pub hole5: Option<u32>,
    pub hole6: Option<u32>,
    pub hole7: Option<u32>,
    pub hole8: Option<u32>,
    pub hole9: Option<u32>,
    pub hole10: Option<u32>,
    pub hole11: Option<u32>,
    pub hole12: Option<u32>,
    pub hole13: Option<u32>,
    pub hole14: Option<u32>,
    pub hole15: Option<u32>,
    pub hole16: Option<u32>,
    pub hole17: Option<u32>,
    pub hole18: Option<u32>,
}

pub fn parse_scorecards(path: &str) -> Vec<UDiscScorecard> {
    let mut reader = csv::Reader::from_path(path).unwrap();

    // Headers that do not match fields in the struct will cause an exception
    // We modify the headers in the file to match what the struct expects.
    let headers = reader.headers().unwrap();
    let corrected_udisc_headers = correct_udisc_headers(headers);
    reader.set_headers(corrected_udisc_headers);

    let mut scorecards : Vec<UDiscScorecard> = Vec::new();
    for result in reader.deserialize() {
        match result {
            Ok(scorecard) => scorecards.push(scorecard),
            Err(e) => {
                eprintln!("Failed to read scorecard: {}", e);
                std::process::exit(-1);
            }
        }
    }
    scorecards
}

fn correct_udisc_headers(p0: &StringRecord) -> StringRecord {
    p0.iter().map(|sr|{
        match sr {
            "PlayerName" => "player_name".to_owned(),
            "CourseName" => "course_name".to_owned(),
            "LayoutName" => "layout_name".to_owned(),
            "Date" => "date".to_owned(),
            "Total" => "total".to_owned(),
            "+/-" => "plus_minus".to_owned(),
            s if s.contains("Hole") => s.to_lowercase(),
            s => s.to_owned()
        }
    }).collect()
}

pub trait FrontNine {
    fn front_nine(&self) -> [u32;9];
}

impl FrontNine for UDiscScorecard {
    fn front_nine(&self) -> [u32; 9] {
        let mut scores = [0;9];
        scores[0] = self.hole1.unwrap_or(0);
        scores[1] = self.hole2.unwrap_or(0);
        scores[2] = self.hole3.unwrap_or(0);
        scores[3] = self.hole4.unwrap_or(0);
        scores[4] = self.hole5.unwrap_or(0);
        scores[5] = self.hole6.unwrap_or(0);
        scores[6] = self.hole7.unwrap_or(0);
        scores[7] = self.hole8.unwrap_or(0);
        scores[8] = self.hole9.unwrap_or(0);
        scores
    }
}
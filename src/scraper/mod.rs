mod sdsu;
pub use crate::scraper::sdsu::SdsuSpider;

//trait Parser {
//    fn parse(&self) -> Result<Vec<Course>>;
//}

pub struct Options {
    pub courses: Vec<String>,
    pub period: String,
    pub period_num: String,
    pub season: String,
    pub year: String,
}

impl Options {
    pub fn from_params(params: &crate::scheduler::Parameters) -> Self {
        let period = params.period.to_owned();
        let p = period.clone();
        let mut p = p.split(' ');
        let season = p.next().unwrap();
        let year = p.next().unwrap();
        let period_num = format!(
            "{}{}",
            year,
            match season {
                "Spring" => "2",
                "Summer" => "3",
                "Fall" => "4",
                _ => "2",
            }
        );

        Options {
            courses: params.courses.to_owned(),
            period,
            period_num,
            season: season.to_owned(),
            year: year.to_owned(),
        }
    }
}

use ::domain::LessonResult;
use chrono::prelude::*;
use time::Duration;


impl LessonResult {
    pub fn time(&self) -> DateTime<Utc> {
        match self {
            &LessonResult::Hard(time) => time.clone(),
            &LessonResult::Good(time) => time.clone(),
            &LessonResult::Easy(time) => time.clone(),
        }
    }

    pub fn rest_duration(&self) -> Duration {
        match self {
            &LessonResult::Hard(_) => Duration::seconds(0),
            &LessonResult::Good(_) => Duration::days(2),
            &LessonResult::Easy(_) => Duration::days(5),
        }
    }

    pub fn due_time(&self) -> DateTime<Utc> {
        self.time() + self.rest_duration()
    }
}

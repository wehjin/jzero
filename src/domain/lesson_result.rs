use ::domain::LessonResult;
use chrono::prelude::*;
use time::Duration;


impl LessonResult {
    pub fn time(&self) -> DateTime<Utc> {
        match self {
            &LessonResult::Hard(time) => time.clone()
        }
    }

    pub fn rest_duration(&self) -> Duration {
        match self {
            &LessonResult::Hard(_) => Duration::seconds(0),
        }
    }

    pub fn due_time(&self) -> DateTime<Utc> {
        self.time() + self.rest_duration()
    }
}

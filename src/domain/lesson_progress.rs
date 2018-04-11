#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum LessonProgress {
    Start,
    Learn,
    Review,
}

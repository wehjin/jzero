#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum LessonProgress {
    Test,
    Learn,
    Review,
}

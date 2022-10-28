pub enum TestResult {
    Accepted,
    WrongAnswer(String, String),
    Timeout,
}

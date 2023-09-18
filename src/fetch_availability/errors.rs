use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Raw names string is empty, cannot parse")]
    EmptyRaw,
}

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Failed to evaluate JS")]
    FailedEval,
    #[error("Evaluated JS returned no value")]
    EvalNoValue,
}

#[derive(Error, Debug, PartialEq)]
pub enum ProcessResultError {
    #[error("Failed to get next part of availability matrix: {section}")]
    AvailMatrixNoNext { section: Box<str> },
    #[error("Failed to parse timestamp from availability matrix: {timestamp}")]
    AvailMatrixFailedTimestampParse { timestamp: String },
}

#[derive(Error, Debug)]
pub enum HeadlessChromeError {
    #[error("Failed to launch headless chrome")]
    FailedToLaunch,
    #[error("Failed to create new tab")]
    FailedToNewTab,
    #[error("Failed to navigate to URL")]
    FailedToNavigate,
    #[error("Failed to wait until navigated")]
    FailedToWaitUntilNavigated,
}

#[derive(Error, Debug)]
pub enum ParseWhen2MeetError {
    #[error("Fetch error: {0}")]
    Fetch(FetchError),
    #[error("Parse error: {0}")]
    Parse(ParseError),
    #[error("Process result error: {0}")]
    ProcessResult(ProcessResultError),
    #[error("Headless Chrome error: {0}")]
    HeadlessChrome(HeadlessChromeError),
}

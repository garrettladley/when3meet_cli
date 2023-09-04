#[derive(Debug)]
pub enum ParseError {
    EmptyRaw,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyRaw => {
                write!(f, "Raw names string is empty, cannot parse")
            }
        }
    }
}

#[derive(Debug)]
pub enum FetchError {
    FailedEval,
    EvalNoValue,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::FailedEval => write!(f, "Failed to evaluate JS"),
            FetchError::EvalNoValue => write!(f, "Evaluated JS returned no value"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ProcessResultError {
    AvailMatrixNoNext { section: String },
    AvailMatrixFailedTimestampParse { timestamp: String },
}

impl std::fmt::Display for ProcessResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessResultError::AvailMatrixNoNext { section } => {
                write!(
                    f,
                    "Failed to get next part of availability matrix: {:?}",
                    section
                )
            }
            ProcessResultError::AvailMatrixFailedTimestampParse { timestamp } => write!(
                f,
                "Failed to parse timestamp from availability matrix: {}",
                timestamp
            ),
        }
    }
}

#[derive(Debug)]
pub enum HeadlessChromeError {
    FailedToLaunch,
    FailedToNewTab,
    FailedToNavigate,
    FailedToWaitUntilNavigated,
}

impl std::fmt::Display for HeadlessChromeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeadlessChromeError::FailedToLaunch => write!(f, "Failed to launch headless chrome"),
            HeadlessChromeError::FailedToNewTab => write!(f, "Failed to create new tab"),
            HeadlessChromeError::FailedToNavigate => write!(f, "Failed to navigate to URL"),
            HeadlessChromeError::FailedToWaitUntilNavigated => {
                write!(f, "Failed to wait until navigated")
            }
        }
    }
}

#[derive(Debug)]
pub enum ParseWhen2MeetError {
    Fetch(FetchError),
    Parse(ParseError),
    ProcessResult(ProcessResultError),
    HeadlessChrome(HeadlessChromeError),
}

impl std::fmt::Display for ParseWhen2MeetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseWhen2MeetError::Fetch(fetch_error) => write!(f, "Fetch error: {}", fetch_error),
            ParseWhen2MeetError::Parse(parse_error) => write!(f, "Parse error: {}", parse_error),
            ParseWhen2MeetError::ProcessResult(process_result_error) => {
                write!(f, "Process result error: {}", process_result_error)
            }
            ParseWhen2MeetError::HeadlessChrome(headless_chrome_error) => {
                write!(f, "Headless chrome error: {}", headless_chrome_error)
            }
        }
    }
}

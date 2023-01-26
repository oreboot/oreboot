pub mod apollolake;
pub mod common;

#[derive(Debug, PartialEq)]
pub enum Error {
    MissingCommunityPad,
    MissingLockAction,
    SkipGpioPad,
    NoSmm,
    NullGpioPads,
    SbiFailure,
    PcrTimeout,
    UntrustedMode,
    UndefinedOffset,
}

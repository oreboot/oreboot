pub mod apollolake;

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

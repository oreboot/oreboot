use std::path::PathBuf;

pub struct BuildConfig<T> {
    payload: PathBuf,
    config: T,
}

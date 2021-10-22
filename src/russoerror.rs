pub enum SSOErrorKind {
    SSOConnectionError,
}

pub struct SSOError {
    kind: SSOErrorKind,
    message: String,
}

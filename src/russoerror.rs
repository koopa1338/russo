pub(crate) enum SSOErrorKind {
    SSOConnectionError,
}

pub(crate) struct SSOError {
    kind: SSOErrorKind,
    message: String,
}

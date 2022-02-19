pub enum SSOErrorKind {
    SSOConnectionError,
    SSOUnreachableError,
    SSOUnsopportedError,
    SSOMalformedRequestError,
}

/// Error that is returned if communication with the sso server fails. See SSOErrorKind for
/// possible erros.
#[allow(dead_code)]
pub struct SSOError {
    kind: SSOErrorKind,
    message: String,
}

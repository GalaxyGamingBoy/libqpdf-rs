#[derive(Debug, Default)]
pub struct QPDFReadParams {
    pub(crate) attempt_recovery: bool,
    pub(crate) ignore_xref: bool,
}

impl QPDFReadParams {
    pub fn with_attempt_recovery(mut self) -> Self {
        self.attempt_recovery = true;
        self
    }

    pub fn with_ignore_xref(mut self) -> Self {
        self.ignore_xref = true;
        self
    }
}

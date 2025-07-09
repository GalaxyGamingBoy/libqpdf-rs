#[derive(Debug, Default)]
pub struct QPDFWriteParams {
    pub(crate) object_stream: QPDFWriteObjectStream,
    pub(crate) stream_data: QPDFWriteStreamData,
    pub(crate) compress_stream: bool,
    pub(crate) decode_level: QPDFWriteDecodeLevel,
    pub(crate) preserve_unreferenced_objects: bool,
    pub(crate) newline_before_endstream: bool,
    pub(crate) content_normalization: bool,
    pub(crate) qdf_mode: bool,
    pub(crate) static_id: bool,
    pub(crate) suppress_original_object_ids: bool,
    pub(crate) preserve_encryption: bool,
    pub(crate) linearization: bool,
    pub(crate) version: QPDFWriteVersion,
}

#[derive(Debug, Default)]
pub enum QPDFWriteObjectStream {
    #[default]
    Disable,
    Preserve,
    Generate,
}

#[derive(Debug, Default)]
pub enum QPDFWriteStreamData {
    #[default]
    Uncompress,
    Preserve,
    Compress,
}

#[derive(Debug, Default)]
pub enum QPDFWriteDecodeLevel {
    #[default]
    None,
    Generalized,
    Specialized,
    ALl,
}

#[derive(Default, Debug)]
pub enum QPDFWriteVersion {
    #[default]
    None,
    MinVersion(String),
    MinVersionAndExtension(String, i32),
    ForceVersion(String),
    ForceVersionAndExtension(String, i32),
}

impl QPDFWriteParams {
    pub fn with_object_stream(mut self, obj: QPDFWriteObjectStream) -> Self {
        self.object_stream = obj;
        self
    }

    pub fn with_stream_data(mut self, str: QPDFWriteStreamData) -> Self {
        self.stream_data = str;
        self
    }

    pub fn with_compress_stream(mut self) -> Self {
        self.compress_stream = true;
        self
    }

    pub fn with_decode_level(mut self, lvl: QPDFWriteDecodeLevel) -> Self {
        self.decode_level = lvl;
        self
    }

    pub fn with_preserve_unreferenced_objects(mut self) -> Self {
        self.preserve_unreferenced_objects = true;
        self
    }

    pub fn with_newline_before_endstream(mut self) -> Self {
        self.newline_before_endstream = true;
        self
    }

    pub fn with_content_normalization(mut self) -> Self {
        self.content_normalization = true;
        self
    }

    pub fn with_qdf_mode(mut self) -> Self {
        self.qdf_mode = true;
        self
    }

    pub fn with_static_id(mut self) -> Self {
        self.static_id = true;
        self
    }

    pub fn with_suppress_original_object_ids(mut self) -> Self {
        self.suppress_original_object_ids = true;
        self
    }

    pub fn with_preserve_encryption(mut self) -> Self {
        self.preserve_encryption = true;
        self
    }

    pub fn with_linearization(mut self) -> Self {
        self.linearization = true;
        self
    }

    pub fn with_version(mut self, ver: QPDFWriteVersion) -> Self {
        self.version = ver;
        self
    }
}

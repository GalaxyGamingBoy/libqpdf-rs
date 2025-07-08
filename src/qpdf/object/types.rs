pub enum QPDFIsObjectType {
    Initialized,
    Bool,
    Null,
    Integer,
    Real,
    Name,
    String,
    Operator,
    InlineImage,
    Array,
    Dictionary,
    Stream,
    Indirect,
    Scalar,
    NameEquals(String),
    DictionaryOfType(String, String),
}

pub enum QPDFModifyObjectTypes {
    Uninitialized,
    Null,
    Bool(bool),
    Integer(i32),
    Real(String),
    RealFromDouble(f32, i32),
    Name(String),
    String(String),
    BinaryString(String),
    Array,
    Dictionary,
    Stream,
}

pub type Generation = i32;
pub type ObjectId = i32;

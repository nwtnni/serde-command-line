use core::fmt::Display;

use serde::ser;
use serde::Serialize;

mod error;

pub use error::Error;
pub type Result<T> = core::result::Result<T, Error>;

pub fn to_vec<T: Serialize>(r#struct: &T) -> Result<Vec<String>> {
    let mut serializer = Serializer::default();
    r#struct.serialize(&mut serializer)?;
    Ok(serializer.args)
}

#[derive(Default)]
struct Serializer {
    args: Vec<String>,
}

impl ser::Serializer for &'_ mut Serializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = ser::Impossible<Value, Error>;
    type SerializeTuple = ser::Impossible<Value, Error>;
    type SerializeTupleStruct = ser::Impossible<Value, Error>;
    type SerializeTupleVariant = ser::Impossible<Value, Error>;
    type SerializeMap = ser::Impossible<Value, Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> std::result::Result<Self::Ok, Self::Error> {
        match v {
            true => Ok(Value::Flag),
            false => Ok(Value::None),
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        serialize_value(&v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        serialize_value(&v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        serialize_value(&v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        serialize_value(&v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        serialize_value(&v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        serialize_value(&v)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        Err(Error::Unsupported("bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(Value::None)
    }

    fn serialize_some<T>(self, v: &T) -> Result<Self::Ok>
    where
        T: ?Sized + serde::Serialize,
    {
        v.serialize(self)
    }

    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("unit"))
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("unit struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        serialize_value(&variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Unsupported("newtype struct"))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Unsupported("newtype variant"))
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        Err(Error::Unsupported("seq"))
    }

    fn serialize_tuple(
        self,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Unsupported("tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Unsupported("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Unsupported("tuple variant"))
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        Err(Error::Unsupported("map"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        self.args.push(variant.to_string());
        Ok(self)
    }
}

impl ser::SerializeStruct for &'_ mut Serializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        let arg = match value.serialize(&mut **self)? {
            Value::Flag => format!("--{key}"),
            Value::Value(value) => format!("--{key}={value}"),
            Value::None | Value::Subcommand => return Ok(()),
        };

        self.args.push(arg);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Value::Subcommand)
    }
}

impl ser::SerializeStructVariant for &'_ mut Serializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        <Self as ser::SerializeStruct>::serialize_field(self, key, value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeStruct>::end(self)
    }
}

enum Value {
    Flag,
    None,
    #[allow(clippy::enum_variant_names)]
    Value(String),
    Subcommand,
}

fn serialize_value<T: Display>(value: &T) -> Result<Value> {
    Ok(Value::Value(value.to_string()))
}

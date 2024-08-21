use std::io::Cursor;

use serde::ser;
use serde::Serialize;

use crate::{SquashCursor, Error, Result, Vlq};

macro_rules! impl_serialize {
    ($($fn_name:ident, $t:ty),*) => {
        $(
            fn $fn_name(self, v: $t) -> Result<Self::Ok> {
                self.output.push(v).map(|_| ())
            }
        )*
    };
}

pub fn serialize<T>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize {
    let output = Cursor::new(Vec::new());
    let mut serializer = Serializer::new(output);
    value.serialize(&mut serializer)?;
    Ok(serializer.into_inner())
}

#[derive(Debug)]
pub struct Serializer {
    output: Cursor<Vec<u8>>,
    counter: u64,
}
impl Serializer {
    pub fn new(output: Cursor<Vec<u8>>) -> Self {
        Self {
            output,
            counter: 0,
        }
    }

    pub fn get_ref(&self) -> &Vec<u8> {
        self.output.get_ref()
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.output.into_inner()
    }
}
impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    impl_serialize!(
        serialize_bool, bool,
        serialize_i8, i8,
        serialize_i16, i16,
        serialize_i32, i32,
        serialize_i64, i64,
        serialize_u8, u8,
        serialize_u16, u16,
        serialize_u32, u32,
        serialize_u64, u64,
        serialize_f32, f32,
        serialize_f64, f64,
        serialize_char, char
    );

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.output.push(v.to_string())?;
        Ok(())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.output.push(v.to_vec())?;
        Ok(())
    }
    fn serialize_none(self) -> Result<Self::Ok> {
        self.serialize_unit()
    }
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
        where
            T: ?Sized + Serialize {
        value.serialize(&mut *self)
    }
    fn serialize_unit(self) -> Result<Self::Ok> {
        self.output.push(0_u8)?;
        Ok(())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }
    fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }
    fn serialize_newtype_struct<T>(
            self,
            _name: &'static str,
            value: &T,
        ) -> Result<Self::Ok>
        where
            T: ?Sized + Serialize {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T>(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            value: &T,
        ) -> Result<Self::Ok>
        where
            T: ?Sized + Serialize {
        value.serialize(self)
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(
            self,
            _name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_tuple(len)
    }
    fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleVariant> {
        Ok(self)
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }
    fn serialize_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStruct> {
        Ok(self)
    }
    fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStructVariant> {
        Ok(self)
    }
}
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
        self.counter += 1;
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<Self::Ok> {
        self.output.push(Vlq(self.counter))?;
        self.counter = 0;
        Ok(())
    }
}
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = <Self as ser::SerializeTuple>::Ok;
    type Error = <Self as ser::SerializeTuple>::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
        ser::SerializeTuple::serialize_element(self, value)
    }
    fn end(self) -> Result<Self::Ok> {
        ser::SerializeTuple::end(self)
    }
}
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
        ser::SerializeTuple::serialize_element(self, value)
    }
    fn end(self) -> Result<Self::Ok> {
        ser::SerializeTuple::end(self)
    }
}
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
        key.serialize(&mut **self)
    }
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
        value.serialize(&mut **self)
    }
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<()>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize, {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;
        self.counter += 1;
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        self.output.push(Vlq(self.counter))?;
        self.counter = 0;
        Ok(())
    }
}
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
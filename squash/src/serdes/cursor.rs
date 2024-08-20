use std::{collections::HashMap, io::{self, Cursor, Read, Seek, Write}};

use ux::{i24, i40, i48, i56, u24, u40, u48, u56};

use super::Vlq;

pub trait LeBytes<const N: usize> {
    fn to_le_bytes(self) -> [u8; N];
    fn from_le_bytes(bytes: [u8; N]) -> Self;
}
macro_rules! impl_le_bytes {
    ($($type:ty, $size:expr, $from_ty:ty),*) => {
        $(
            impl LeBytes<$size> for $type {
                fn to_le_bytes(self) -> [u8; $size] {
                    <$from_ty>::from(self).to_le_bytes()[..$size].try_into().unwrap()
                }
                fn from_le_bytes(bytes: [u8; $size]) -> Self {
                    <$from_ty>::from_le_bytes({
                        let mut arr = [0; std::mem::size_of::<$from_ty>()];
                        arr[..$size].copy_from_slice(&bytes);
                        arr
                    }).try_into().unwrap()
                }
            }
        )*
    };
}
impl_le_bytes!(
    u24, 3, u32,
    u40, 5, u64,
    u48, 6, u64,
    u56, 7, u64,
    i24, 3, i32,
    i40, 5, i64,
    i48, 6, i64,
    i56, 7, i64
);

pub trait SquashCursor: Read + Write + Seek {
    fn realloc(&mut self, size: u64);
    fn print_cursor(&self);

    fn seek_end(&mut self) -> io::Result<u64> {
        self.seek(io::SeekFrom::End(0))
    }
    fn pop_read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let ret = -(buf.len() as i64);
        self.seek(io::SeekFrom::Current(ret))?;
        let x = self.read(buf)? as i64;
        self.seek(io::SeekFrom::Current(ret))?;
        Ok(x as usize)
    }

    fn push<'a, T>(&mut self, value: T) -> crate::Result<usize>
    where
        T: SquashObject,
        Self: Sized,
    {
        value.push_obj(self)
    }

    fn pop<T>(&mut self) -> crate::Result<T>
    where
        T: SquashObject,
        Self: Sized
    {
        T::pop_obj(self)
    }
}

impl<A> SquashCursor for Cursor<A>
where 
    A: AsMut<Vec<u8>> + AsRef<Vec<u8>> + core::fmt::Debug,
    Cursor<A>: Read + Write + Seek
{
    fn realloc(&mut self, size: u64) {
        let position = self.position();
        let buf = self.get_mut().as_mut();
        let len = buf.len() as u64;
        if len < position + size {
            buf.resize((position + size) as usize, 0);
        } 
    }

    fn print_cursor(&self) {
        let buf = self.get_ref();
        println!("{:?} - {}", buf, self.position());
    }
}

pub trait SquashObject {
    fn push_obj<T: SquashCursor>(self, cursor: &mut T) -> crate::Result<usize>;
    fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
        where
            T: SquashCursor,
            Self: Sized;
}

macro_rules! impl_squash_object {
    ($t:ty, $size:expr) => {
        impl SquashObject for $t {
            fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
                where
                    T: SquashCursor,
                    Self: Sized {
                let mut buf = [0; $size];
                cursor.pop_read(&mut buf)?;
                Ok(<$t>::from_le_bytes(buf))
            }
            fn push_obj<T: SquashCursor>(self, cursor: &mut T) -> crate::Result<usize> {
                cursor.realloc($size);
                Ok(cursor.write(&self.to_le_bytes()[..$size])?)
            }
        }
    };
}

impl_squash_object!(u8, 1);
impl_squash_object!(u16, 2);
impl_squash_object!(u24, 3);
impl_squash_object!(u32, 4);
impl_squash_object!(u40, 5);
impl_squash_object!(u48, 6);
impl_squash_object!(u56, 7);
impl_squash_object!(u64, 8);
impl_squash_object!(i8, 1);
impl_squash_object!(i16, 2);
impl_squash_object!(i24, 3);
impl_squash_object!(i32, 4);
impl_squash_object!(i40, 5);
impl_squash_object!(i48, 6);
impl_squash_object!(i56, 7);
impl_squash_object!(i64, 8);
impl_squash_object!(f32, 4);
impl_squash_object!(f64, 8);

impl SquashObject for bool {
    fn push_obj<T: SquashCursor>(self, cursor: &mut T) -> crate::Result<usize> {
        let value = if self { 1_u8 } else { 0_u8 };
        cursor.push(value)
    }
    fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
            where
                T: SquashCursor,
                Self: Sized {
        let value = cursor.pop::<u8>()?;
        Ok(value == 1)
    }
}

macro_rules! impl_squash_object_for_bool_tuple {
    ($type:ident, $($idx:tt),*) => {
        impl SquashObject for $crate::$type {
            fn push_obj<T: SquashCursor>(self, cursor: &mut T) -> $crate::Result<usize> {
                let mut value = 0_u8;
                value |= (self.0 as u8) << 0;
                $(
                    value |= (self.$idx as u8) << $idx;
                )*
                cursor.push(value)
            }

            fn pop_obj<T>(cursor: &mut T) -> $crate::Result<Self>
            where
                T: SquashCursor,
                Self: Sized {
                let x = cursor.pop::<u8>()?;
                Ok($crate::$type(
                    (x >> 0) & 1 != 0,
                    $(
                        (x >> $idx) & 1 != 0
                    ),*
                ))
            }
        }
    };
}

impl_squash_object_for_bool_tuple!(BoolTuple8, 1, 2, 3, 4, 5, 6, 7);
impl_squash_object_for_bool_tuple!(BoolTuple7, 1, 2, 3, 4, 5, 6);
impl_squash_object_for_bool_tuple!(BoolTuple6, 1, 2, 3, 4, 5);
impl_squash_object_for_bool_tuple!(BoolTuple5, 1, 2, 3, 4);
impl_squash_object_for_bool_tuple!(BoolTuple4, 1, 2, 3);
impl_squash_object_for_bool_tuple!(BoolTuple3, 1, 2);
impl_squash_object_for_bool_tuple!(BoolTuple2, 1);

impl<T> SquashObject for Option<T>
    where
        T: SquashObject
{
    fn pop_obj<U>(cursor: &mut U) -> crate::Result<Self>
            where
                U: SquashCursor,
                Self: Sized {
        let is_some = cursor.pop::<u8>()? == 1;
        if is_some {
            Ok(Some(T::pop_obj(cursor)?))
        } else {
            Ok(None)
        }
    }
    fn push_obj<U: SquashCursor>(self, cursor: &mut U) -> crate::Result<usize> {
        let mut count = 0;
        if let Some(x) = self {
            count += cursor.push(x)?;
        }
        count += cursor.push(1_u8)?;
        Ok(count)
    }
}
impl<T> SquashObject for Vec<T>
    where
        T: SquashObject
{
    fn pop_obj<U>(cursor: &mut U) -> crate::Result<Self>
            where
                U: SquashCursor,
                Self: Sized {
        let len = cursor.pop::<Vlq>()?.0;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            Vec::push(&mut vec, T::pop_obj(cursor)?);
        }
        Ok(vec)
    }
    fn push_obj<U: SquashCursor>(self, cursor: &mut U) -> crate::Result<usize> {
        let mut count = 0;
        count += cursor.push(Vlq(self.len() as u64))?;
        for item in self {
            count += cursor.push(item)?;
        }
        Ok(count)
    }
}
impl<K, V> SquashObject for HashMap<K, V>
where
    K: SquashObject + Eq + std::hash::Hash,
    V: SquashObject,
{
    fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
            where
                T: SquashCursor,
                Self: Sized {
        let len = cursor.pop::<Vlq>()?.0;
        let mut map = HashMap::with_capacity(len as usize);
        for _ in 0..len {
            let key = K::pop_obj(cursor)?;
            let value = V::pop_obj(cursor)?;
            map.insert(key, value);
        }
        Ok(map)
    }
    fn push_obj<T: SquashCursor>(self, cursor: &mut T) -> crate::Result<usize> {
        let mut count = 0;
        count += cursor.push(Vlq(self.len() as u64))?;
        for (key, value) in self {
            count += cursor.push(key)?;
            count += cursor.push(value)?;
        }
        Ok(count)
    }
}

impl SquashObject for Vlq {
    fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
            where
                T: SquashCursor,
                Self: Sized {
        let mut x = 0;
        let mut counter = 0;

        for _ in 0..=7 {
            let b = cursor.pop::<u8>()? as u64;
            if b >= 128 {
                return Ok(Self(b as u64 - 128) * 128_u64.pow(counter));
            }

            counter += 1;

            let b = cursor.pop::<u8>()? as u64;
            x += b * 128_u64.pow(counter);
        }
        
        Err(crate::error::Error::Io(io::Error::new(io::ErrorKind::InvalidData, format!("Not a valid vlq: {}", x))))
    }
    fn push_obj<T: SquashCursor>(self, cursor: &mut T) -> crate::Result<usize> {
        let value = self.0;
        let x0 = value % 128;
        let x1 = value / 128 % 128;
        let x2 = value / 128 / 128 % 128;
        let x3 = value / 128 / 128 / 128 % 128;
        let x4 = value / 128 / 128 / 128 / 128 % 128;
        let x5 = value / 128 / 128 / 128 / 128 / 128 % 128;
        let x6 = value / 128 / 128 / 128 / 128 / 128 / 128 % 128;
        let x7 = value / 128 / 128 / 128 / 128 / 128 / 128 / 128 % 128;

        if x7 != 0 {
            let y = x0 * 256_u64.pow(7) + x1 * 256_u64.pow(6) + x2 * 256_u64.pow(5) + x3 * 256_u64.pow(4) + x4 * 256_u64.pow(3) + x5 * 256_u64.pow(2) + x6 * 256 + x7 + 128;
            cursor.push(y)
        } else if x6 != 0 {
            let y = u56::try_from(x0 * 256_u64.pow(6) + x1 * 256_u64.pow(5) + x2 * 256_u64.pow(4) + x3 * 256_u64.pow(3) + x4 * 256_u64.pow(2) + x5 * 256 + x6 + 128).unwrap();
            cursor.push(y)
        } else if x5 != 0 {
            let y = u48::try_from(x0 * 256_u64.pow(5) + x1 * 256_u64.pow(4) + x2 * 256_u64.pow(3) + x3 * 256_u64.pow(2) + x4 * 256 + x5 + 128).unwrap();
            cursor.push(y)
        } else if x4 != 0 {
            let y = u40::try_from(x0 * 256_u64.pow(4) + x1 * 256_u64.pow(3) + x2 * 256_u64.pow(2) + x3 * 256 + x4 + 128).unwrap();
            cursor.push(y)
        } else if x3 != 0 {
            let y = u32::try_from(x0 * 256_u64.pow(3) + x1 * 256_u64.pow(2) + x2 * 256 + x3 + 128).unwrap();
            cursor.push(y)
        } else if x2 != 0 {
            let y = u24::try_from(x0 * 256_u64.pow(2) + x1 * 256 + x2 + 128).unwrap();
            cursor.push(y)
        } else if x1 != 0 {
            let y = u16::try_from(x0 * 256 + x1 + 128).unwrap();
            cursor.push(y)
        } else {
            let y = u8::try_from(x0 + 128).unwrap();
            cursor.push(y)
        }
    }
}

impl SquashObject for String {
    fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
            where
                T: SquashCursor,
                Self: Sized {
        let len: Vlq = cursor.pop()?;
        let mut buf = vec![0; len.0 as usize];
        cursor.pop_read(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }
    fn push_obj<T: SquashCursor>(self, cursor: &mut T) -> crate::Result<usize> {
        let len = Vlq(self.len() as u64);
        let mut count = cursor.push(len)?;
        count += cursor.write(self.as_bytes())?;
        Ok(count)
    }
}

impl SquashObject for char {
    fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
            where
                T: SquashCursor,
                Self: Sized {
        let str = String::pop_obj(cursor)?;
        str.chars().next().ok_or(crate::error::Error::Io(io::Error::new(io::ErrorKind::InvalidData, "Empty string")).into())
    }
    fn push_obj<T: SquashCursor>(self, cursor: &mut T) -> crate::Result<usize> {
        self.to_string().push_obj(cursor)
    }
}
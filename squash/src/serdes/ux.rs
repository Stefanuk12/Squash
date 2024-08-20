use serde::Deserialize;
use crate::{SquashObject, Zero};

macro_rules! impl_custom_int {
    ($name:ident, $int_type:ident, $byte_count:expr) => {
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Deserialize)]
        #[allow(non_camel_case_types)]
        pub struct $name([u8; $byte_count]);
        impl $name {
            pub fn new(val: $int_type) -> Result<Self, crate::Error> {
                Self::try_from(val)
            }

            pub fn convert(self) -> $int_type {
                $int_type::from(self)
            }
        }

        impl From<$name> for $int_type {
            fn from(val: $name) -> $int_type {
                let mut bytes = [0u8; std::mem::size_of::<$int_type>()];
                bytes[..$byte_count].copy_from_slice(&val.0);
                $int_type::from_le_bytes(bytes)
            }
        }
        
        impl TryFrom<$int_type> for $name {
            type Error = crate::Error;
        
            fn try_from(val: $int_type) -> crate::Result<$name> {
                let bytes = val.to_le_bytes();
                if bytes[$byte_count..].iter().any(|&x| x != 0) {
                    return Err(crate::Error::ValueTooLarge);
                }
                let mut arr = [0u8; $byte_count];
                arr.copy_from_slice(&bytes[..$byte_count]);
                Ok($name(arr))
            }
        }

        impl From<$name> for ux::$name {
            fn from(val: $name) -> ux::$name {
                ux::$name::new($int_type::from(val))
            }
        }

        impl SquashObject for $name {
            fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
            where
                T: super::SquashCursor,
                Self: Sized,
            {
                let mut arr = [0u8; $byte_count];
                for i in 0..$byte_count {
                    arr[i] = cursor.pop()?;
                }
                Ok($name(arr))
            }

            fn push_obj<T: super::SquashCursor>(self, cursor: &mut T) -> crate::Result<usize> {
                for i in (0..$byte_count).rev() {
                    cursor.push(self.0[i])?;
                }
                Ok($byte_count)
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                let mut bytes = self.0;
                bytes.reverse();
                serializer.serialize_bytes(&bytes)
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                write!(f, "{}", $int_type::from(*self))
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                write!(f, "{}", $int_type::from(*self))
            }
        }

        impl Zero for $name {
            const ZERO: Self = $name([0u8; $byte_count]);
        }
    };
}

// Generate implementations for unsigned types
impl_custom_int!(u24, u32, 3);
impl_custom_int!(u40, u64, 5);
impl_custom_int!(u48, u64, 6);
impl_custom_int!(u56, u64, 7);

// Generate implementations for signed types
impl_custom_int!(i24, i32, 3);
impl_custom_int!(i40, i64, 5);
impl_custom_int!(i48, i64, 6);
impl_custom_int!(i56, i64, 7);
macro_rules! import {
    ($($module:ident),*) => {
        $(
            pub mod $module;
            pub use $module::*;
        )*
    };
}

import!(error, serdes);

pub use squash_derive::*;

#[cfg(feature = "roblox")]
import!(roblox);

#[macro_export]
macro_rules! impl_number {
    ($aa:ident, $t:ty, $serialize_fn:ident) => {
        impl ::serde::Serialize for $aa<$t> {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.$serialize_fn(self.0)
            }
        }
        impl<'de> ::serde::Deserialize<'de> for $aa<$t> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                <$t>::deserialize(deserializer).map($aa)
            }
        }
        impl $crate::SquashObject for $aa<$t> {
            fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
            where
                T: crate::SquashCursor,
                Self: Sized,
            {
                Ok(cursor.pop::<$t>().map($aa)?)
            }
            fn push_obj<T: crate::SquashCursor>(self, cursor: &mut T) -> crate::Result<usize> {
                Ok(cursor.push(self.0)?)
            }
        }
        impl From<$t> for $aa<$t> {
            fn from(t: $t) -> Self {
                $aa(t)
            }
        }
        impl From<$aa<$t>> for $t {
            fn from(t: $aa<$t>) -> Self {
                t.0
            }
        }
        impl ::std::ops::Deref for $aa<$t> {
            type Target = $t;
            fn deref(&self) -> &$t {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for $aa<$t> {
            fn deref_mut(&mut self) -> &mut $t {
                &mut self.0
            }
        }
    };
    ($aa:ident, $t:ty) => {
        impl ::serde::Serialize for $aa<$t> {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let bytes = self.0.to_le_bytes();
                serializer.serialize_bytes(&bytes)
            }
        }
        impl<'de> ::serde::Deserialize<'de> for $aa<$t> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let bytes = Vec::<u8>::deserialize(deserializer)?;
                Ok($aa(<$t>::from_le_bytes(bytes.try_into().unwrap())))
            }
        }
        impl $crate::SquashObject for $aa<$t> {
            fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
            where
                T: crate::SquashCursor,
                Self: Sized,
            {
                Ok(cursor.pop::<$t>().map(|x| $aa(x as $t))?)
            }
            fn push_obj<T: crate::SquashCursor>(self, cursor: &mut T) -> crate::Result<usize> {
                Ok(cursor.push(self.0)?)
            }
        }
        impl From<$t> for $aa<$t> {
            fn from(t: $t) -> Self {
                $aa(t)
            }
        }
        impl From<$aa<$t>> for $t {
            fn from(t: $aa<$t>) -> Self {
                t.0
            }
        }
        impl ::std::ops::Deref for $aa<$t> {
            type Target = $t;
            fn deref(&self) -> &$t {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for $aa<$t> {
            fn deref_mut(&mut self) -> &mut $t {
                &mut self.0
            }
        }
    };
}

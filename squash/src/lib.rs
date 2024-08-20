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

#[macro_export]
macro_rules! impl_reverse_deserialize {
    ($ident:ident<$($gen:ident $(: $bound:path)?),*>, $($field:tt),*) => {
        impl<'de, $($gen $(: $bound)?),*> ::serde::Deserialize<'de> for $ident<$($gen),*> {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(field_identifier, rename_all = "lowercase")]
                #[allow(non_camel_case_types)]
                enum Field {
                    $($field,)*
                }
                struct MainVisitor<$($gen $(: $bound)?),*>(::core::marker::PhantomData<($($gen,)*)>);

                impl<'de, $($gen $(: $bound)?),*> ::serde::de::Visitor<'de> for MainVisitor<$($gen),*> {
                    type Value = $ident<$($gen),*>;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(stringify!("struct {}", $ident))
                    }
                    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
                    where
                        A: ::serde::de::SeqAccess<'de>,
                    {
                        $(
                            let $field = seq
                                .next_element()?
                                .ok_or_else(|| ::serde::de::Error::invalid_length(0, &self))?;
                        )*
                        Ok(Self::Value { $($field,)* })
                    }
                }
                const FIELDS: &[&str] = &[$(stringify!($field),)*];
                deserializer.deserialize_struct(
                    stringify!($ident),
                    FIELDS,
                    MainVisitor(::core::marker::PhantomData),
                )
            }
        }
    };
    ($ident:ident, $($field:tt),*) => {
        impl<'de> ::serde::Deserialize<'de> for $ident {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(field_identifier, rename_all = "lowercase")]
                #[allow(non_camel_case_types)]
                enum Field {
                    $($field,)*
                }
                struct MainVisitor(::core::marker::PhantomData<()>);

                impl<'de> ::serde::de::Visitor<'de> for MainVisitor {
                    type Value = $ident;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(stringify!("struct {}", $ident))
                    }
                    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
                    where
                        A: ::serde::de::SeqAccess<'de>,
                    {
                        $(
                            let $field = seq
                                .next_element()?
                                .ok_or_else(|| ::serde::de::Error::invalid_length(0, &self))?;
                        )*
                        Ok(Self::Value { $($field,)* })
                    }
                }
                const FIELDS: &[&str] = &[$(stringify!($field),)*];
                deserializer.deserialize_struct(
                    stringify!($ident),
                    FIELDS,
                    MainVisitor(::core::marker::PhantomData),
                )
            }
        }
    };
    ($ident:ident<$($gen:ident $(: $bound:path)?),*>) => {
        impl<'de, $($gen $(: $bound)?),*> ::serde::Deserialize<'de> for $ident<$($gen),*> {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(field_identifier, rename_all = "lowercase")]
                #[allow(non_camel_case_types)]
                enum Field {
                    _0,
                }
                struct MainVisitor<$($gen $(: $bound)?),*>(::core::marker::PhantomData<($($gen,)*)>);

                impl<'de, $($gen $(: $bound)?),*> ::serde::de::Visitor<'de> for MainVisitor<$($gen),*> {
                    type Value = $ident<$($gen),*>;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(stringify!("struct {}", $ident))
                    }
                    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
                    where
                        A: ::serde::de::SeqAccess<'de>,
                    {
                        let _0 = seq
                            .next_element()?
                            .ok_or_else(|| ::serde::de::Error::invalid_length(0, &self))?;
                        Ok(Self(_0))
                    }
                }
                const FIELDS: &[&str] = &["_0"];
                deserializer.deserialize_struct(
                    stringify!($ident),
                    FIELDS,
                    MainVisitor(::core::marker::PhantomData),
                )
            }
        }
    };
    ($ident:ident) => {
        impl<'de> ::serde::Deserialize<'de> for $ident {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(field_identifier, rename_all = "lowercase")]
                #[allow(non_camel_case_types)]
                enum Field {
                    _0,
                }
                struct MainVisitor(::core::marker::PhantomData<()>);

                impl<'de> ::serde::de::Visitor<'de> for MainVisitor {
                    type Value = $ident;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(stringify!("struct {}", $ident))
                    }
                    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
                    where
                        A: ::serde::de::SeqAccess<'de>,
                    {
                        let _0 = seq
                            .next_element()?
                            .ok_or_else(|| ::serde::de::Error::invalid_length(0, &self))?;
                        Ok(Self::Value(_0))
                    }
                }
                const FIELDS: &[&str] = &["_0"];
                deserializer.deserialize_struct(
                    stringify!($ident),
                    FIELDS,
                    MainVisitor(::core::marker::PhantomData),
                )
            }
        }
    };
}
// #[doc(hidden)]
// #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
// const _: () = {
//     #[allow(unused_extern_crates, clippy::useless_attribute)]
//     extern crate serde as _serde;
//     #[automatically_derived]
//     impl<'de, T: SquashNumber> _serde::Deserialize<'de> for NumberSequence<T>
//     where
//         T: SquashNumber,
//     {
//         fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
//         where
//             __D: _serde::Deserializer<'de>,
//         {
//             #[doc(hidden)]
//             struct __Visitor<'de, T: SquashNumber>
//             where
//                 T: SquashNumber,
//             {
//                 marker: _serde::__private::PhantomData<NumberSequence<T>>,
//                 lifetime: _serde::__private::PhantomData<&'de ()>,
//             }
//             impl<'de, T: SquashNumber> _serde::de::Visitor<'de> for __Visitor<'de, T>
//             where
//                 T: SquashNumber,
//             {
//                 type Value = NumberSequence<T>;
//                 fn expecting(
//                     &self,
//                     __formatter: &mut _serde::__private::Formatter,
//                 ) -> _serde::__private::fmt::Result {
//                     _serde::__private::Formatter::write_str(
//                         __formatter,
//                         "tuple struct NumberSequence",
//                     )
//                 }
//                 #[inline]
//                 fn visit_newtype_struct<__E>(
//                     self,
//                     __e: __E,
//                 ) -> _serde::__private::Result<Self::Value, __E::Error>
//                 where
//                     __E: _serde::Deserializer<'de>,
//                 {
//                     let __field0: Vec<NumberSequenceKeypoint<T>> =
//                         <Vec<NumberSequenceKeypoint<T>> as _serde::Deserialize>::deserialize(__e)?;
//                     _serde::__private::Ok(NumberSequence(__field0))
//                 }
//                 #[inline]
//                 fn visit_seq<__A>(
//                     self,
//                     mut __seq: __A,
//                 ) -> _serde::__private::Result<Self::Value, __A::Error>
//                 where
//                     __A: _serde::de::SeqAccess<'de>,
//                 {
//                     let __field0 = match _serde::de::SeqAccess::next_element::<
//                         Vec<NumberSequenceKeypoint<T>>,
//                     >(&mut __seq)?
//                     {
//                         _serde::__private::Some(__value) => __value,
//                         _serde::__private::None => {
//                             return _serde::__private::Err(_serde::de::Error::invalid_length(
//                                 0usize,
//                                 &"tuple struct NumberSequence with 1 element",
//                             ))
//                         }
//                     };
//                     _serde::__private::Ok(NumberSequence(__field0))
//                 }
//             }
//             _serde::Deserializer::deserialize_newtype_struct(
//                 __deserializer,
//                 "NumberSequence",
//                 __Visitor {
//                     marker: _serde::__private::PhantomData::<NumberSequence<T>>,
//                     lifetime: _serde::__private::PhantomData,
//                 },
//             )
//         }
//     }
// };

#[macro_export]
macro_rules! impl_squash_object_a {
    ($ident:ident<$($gen:ident $(: $bound:path)?),*>, $($field:tt),*; $($backward_field:tt),*) => {
        impl<$($gen $(: $bound)?),*> $crate::SquashObject for $ident<$($gen),*> {
            fn pop_obj<Obj>(cursor: &mut Obj) -> $crate::Result<Self>
            where
                Obj: $crate::SquashCursor,
                Self: Sized,
            {
                Ok($ident {
                    $(
                        $backward_field: cursor.pop()?,
                    )*
                })
            }
            fn push_obj<Obj: $crate::SquashCursor>(self, cursor: &mut Obj) -> $crate::Result<usize> {
                let mut count = 0;
                $(
                    count += cursor.push(self.$field.clone())?;
                )*
                Ok(count)
            }
        }
    };
    ($ident:ident, $($field:tt),*; $($backward_field:tt),*) => {
        impl $crate::SquashObject for $ident {
            fn pop_obj<Obj>(cursor: &mut Obj) -> $crate::Result<Self>
            where
                Obj: $crate::SquashCursor,
                Self: Sized,
            {
                Ok($ident {
                    $(
                        $backward_field: cursor.pop()?,
                    )*
                })
            }
            fn push_obj<Obj: $crate::SquashCursor>(self, cursor: &mut Obj) -> $crate::Result<usize> {
                let mut count = 0;
                $(
                    count += cursor.push(self.$field.clone())?;
                )*
                Ok(count)
            }
        }
    };
    ($ident:ident<$($gen:ident $(: $bound:path)?),*>) => {
        impl<$($gen $(: $bound)?),*> $crate::SquashObject for $ident<$($gen),*> {
            fn pop_obj<Obj>(cursor: &mut Obj) -> $crate::Result<Self>
            where
                Obj: $crate::SquashCursor,
                Self: Sized,
            {
                Ok($ident(cursor.pop()?))
            }
            fn push_obj<Obj: $crate::SquashCursor>(self, cursor: &mut Obj) -> $crate::Result<usize> {
                Ok(cursor.push(self.0)?)
            }
        }
    };
    ($ident:ident) => {
        impl $crate::SquashObject for $ident {
            fn pop_obj<Obj>(cursor: &mut Obj) -> $crate::Result<Self>
            where
                Obj: $crate::SquashCursor,
                Self: Sized,
            {
                Ok($ident(cursor.pop()?))
            }
            fn push_obj<Obj: $crate::SquashCursor>(self, cursor: &mut Obj) -> $crate::Result<usize> {
                Ok(cursor.push(self.0)?)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_squash {
    ($ident:ident<$($gen:ident $(: $bound:path)?),*>, $($field:tt),*; $($backward_field:tt),*) => {
        $crate::impl_reverse_deserialize!($ident<$($gen $(: $bound)?),*>, $($field),*);
        $crate::impl_squash_object_a!($ident<$($gen $(: $bound)?),*>, $($field),*; $($backward_field),*);
    };
    ($ident:ident, $($field:tt),*; $($backward_field:tt),*) => {
        $crate::impl_reverse_deserialize!($ident, $($field),*);
        $crate::impl_squash_object_a!($ident, $($field),*; $($backward_field),*);
    };
    ($ident:ident<$($gen:ident $(: $bound:path)?),*>) => {
        $crate::impl_squash_object_a!($ident<$($gen $(: $bound)?),*>);
    };
    ($ident:ident) => {
        $crate::impl_squash_object_a!($ident);
    };
}

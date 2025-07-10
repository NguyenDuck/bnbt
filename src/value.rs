/**
 * Copyright © 2025 NguyenDuck
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
////////////////////////////////////////////////////////////////////////
use std::collections::HashMap;

macro_rules! impl_nbt_accessors {
    ($name:ident {
        $($variant:ident),*
    }) => {
        impl $name {
            pub const fn get_type_id(&self) -> u8 {
                unsafe { *(self as *const Self as *const u8) }
            }
        }
    };

    ($name:ident {
        $($no_data_variant:ident),*
    }) => {
        impl $name {
            $(
                #[doc = "Checks if value matches variant"]
                paste::paste! {
                    pub fn [<is_ $no_data_variant:snake>](&self) -> bool {
                        matches!(self, Self::$no_data_variant)
                    }
                }
            )*
        }
    };

    ($name:ident {
        $($with_data_variant:ident($ty:ty)),*
    }) => {
        impl $name {
            $(
                paste::paste! {
                    #[doc = "Checks if value matches variant"]
                    pub fn [<is_ $with_data_variant:snake>](&self) -> bool {
                        matches!(self, Self::$with_data_variant(_))
                    }

                    #[doc = "Returns reference to inner value if matches variant"]
                    pub fn [<as_ $with_data_variant:snake>](&self) -> Result<&$ty, &'static str> {
                        match self {
                            Self::$with_data_variant(v) => Ok(v),
                            _ => Err(concat!("Not a ", stringify!($with_data_variant), " tag")),
                        }
                    }

                    #[doc = "Try to convert into inner value"]
                    pub fn [<try_into_ $with_data_variant:snake>](self) -> Result<$ty, Self> {
                        match self {
                            Self::$with_data_variant(v) => Ok(v),
                            _ => Err(self),
                        }
                    }

                    pub fn [<into_ $with_data_variant:snake>](self) -> $ty {
                        match self {
                            Self::$with_data_variant(v) => v,
                            _ => panic!("Not a {} tag", stringify!($with_data_variant)),
                        }
                    }
                }
            )*
        }

        $(
            impl From<$ty> for $name {
                fn from(value: $ty) -> Self {
                    Self::$with_data_variant(value)
                }
            }
        )*

        $(
            impl Into<$ty> for $name {
                fn into(self) -> $ty {
                    match self {
                        Self::$with_data_variant(v) => v,
                        _ => panic!("Not a {} tag", stringify!($with_data_variant)),
                    }
                }
            }
        )*
    };
}

macro_rules! impl_nbt_accessors_mut {
    ($name:ident {
        $($with_data_variant:ident($ty:ty)),*
    }) => {
        impl $name {
            $(
                paste::paste! {
                    #[doc = "Returns reference to mutable inner value if matches variant"]
                    pub fn [<as_ $with_data_variant:snake _mut>](&mut self) -> Result<&mut $ty, &'static str> {
                        match self {
                            Self::$with_data_variant(v) => Ok(v),
                            _ => Err(concat!("Not a ", stringify!($with_data_variant), " tag")),
                        }
                    }
                }
            )*
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum NBTTagValue {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<NBTTagValue>),
    Compound(HashMap<String, NBTTagValue>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl_nbt_accessors!(NBTTagValue { End });
impl_nbt_accessors!(NBTTagValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<NBTTagValue>),
    Compound(HashMap<String, NBTTagValue>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>)
});

impl_nbt_accessors_mut!(NBTTagValue {
    ByteArray(Vec<u8>),
    List(Vec<NBTTagValue>),
    Compound(HashMap<String, NBTTagValue>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>)
});

// Special implements
impl From<&str> for NBTTagValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl std::ops::Index<&str> for NBTTagValue {
    type Output = NBTTagValue;

    fn index(&self, index: &str) -> &Self::Output {
        self.as_compound().unwrap().get(index).unwrap()
    }
}

impl std::ops::IndexMut<&str> for NBTTagValue {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.as_compound_mut().unwrap().get_mut(index).unwrap()
    }
}

impl std::ops::Index<usize> for NBTTagValue {
    type Output = NBTTagValue;

    fn index(&self, index: usize) -> &Self::Output {
        self.as_list().unwrap().get(index).unwrap()
    }
}

impl std::ops::IndexMut<usize> for NBTTagValue {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_list_mut().unwrap().get_mut(index).unwrap()
    }
}

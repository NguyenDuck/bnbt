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

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::value::NBTTagValue;

#[derive(Debug, Clone, PartialEq)]
pub struct NBTTag {
    pub name: String,
    pub value: NBTTagValue,
}

impl NBTTag {
    pub fn new(name: String, value: NBTTagValue) -> Self {
        Self { name, value }
    }

    pub fn new_unnamed(value: NBTTagValue) -> Self {
        NBTTag::new("".to_string(), value)
    }

    pub fn get_type_id(&self) -> u8 {
        self.value.get_type_id()
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut bytes = BytesMut::new();

        self.write_to(&mut bytes);

        bytes.freeze()
    }

    fn write_to(&self, bytes: &mut BytesMut) {
        bytes.put_u8(self.value.get_type_id());

        if let NBTTagValue::End = self.value {
            return;
        }

        bytes.put_u16_le(self.name.len() as u16);
        bytes.put_slice(self.name.as_bytes());

        self.write_payload_to(bytes);
    }

    fn write_payload_to(&self, bytes: &mut BytesMut) {
        match &self.value {
            NBTTagValue::Byte(v) => bytes.put_i8(*v),
            NBTTagValue::Short(v) => bytes.put_i16_le(*v),
            NBTTagValue::Int(v) => bytes.put_i32_le(*v),
            NBTTagValue::Long(v) => bytes.put_i64_le(*v),
            NBTTagValue::Float(v) => bytes.put_f32_le(*v),
            NBTTagValue::Double(v) => bytes.put_f64_le(*v),
            NBTTagValue::ByteArray(v) => {
                bytes.put_i32_le(v.len() as i32);
                for &b in v {
                    bytes.put_u8(b);
                }
            }
            NBTTagValue::String(v) => {
                bytes.put_u16_le(v.len() as u16);
                bytes.put_slice(v.as_bytes());
            }
            NBTTagValue::List(items) => {
                let type_id = items.first().map_or(0, |item| item.get_type_id());
                bytes.put_u8(type_id);

                if type_id == 0 {
                    return;
                }

                bytes.put_u32_le(items.len() as u32);

                for item in items {
                    NBTTag::new_unnamed(item.clone()).write_payload_to(bytes);
                }

                bytes.put_u8(0);
            }
            NBTTagValue::Compound(map) => {
                for (key, value) in map {
                    bytes.put_u8(value.get_type_id());
                    bytes.put_u16_le(key.len() as u16);
                    bytes.put_slice(key.as_bytes());
                    NBTTag::new_unnamed(value.clone()).write_payload_to(bytes);
                }

                bytes.put_u8(0);
            }
            NBTTagValue::IntArray(v) => {
                bytes.put_u32_le(v.len() as u32);
                for &i in v {
                    bytes.put_i32_le(i);
                }
            }
            NBTTagValue::LongArray(v) => {
                bytes.put_u32_le(v.len() as u32);
                for &l in v {
                    bytes.put_i64_le(l);
                }
            }
            NBTTagValue::End => {}
        }
    }

    pub fn from_bytes(mut bytes: Bytes) -> Self {
        let tag_type = bytes.get_u8();

        let name_len = bytes.get_u16_le() as usize;
        let name = String::from_utf8_lossy(&bytes[..name_len]).into();
        bytes.advance(name_len);

        let value = Self::from_bytes_payload(tag_type, &mut bytes);

        Self { name, value }
    }

    fn from_bytes_payload(tag_type: u8, bytes: &mut Bytes) -> NBTTagValue {
        match tag_type {
            1 => NBTTagValue::Byte(bytes.get_i8()),
            2 => NBTTagValue::Short(bytes.get_i16_le()),
            3 => NBTTagValue::Int(bytes.get_i32_le()),
            4 => NBTTagValue::Long(bytes.get_i64_le()),
            5 => NBTTagValue::Float(bytes.get_f32_le()),
            6 => NBTTagValue::Double(bytes.get_f64_le()),
            7 => {
                let len = bytes.get_i32_le() as usize;
                let mut byte_array = Vec::with_capacity(len);
                for _ in 0..len {
                    byte_array.push(bytes.get_u8());
                }
                NBTTagValue::ByteArray(byte_array)
            }
            8 => {
                let len = bytes.get_u16_le() as usize;
                let string = String::from_utf8_lossy(&bytes[..len]).into_owned();
                bytes.advance(len);
                NBTTagValue::String(string)
            }
            9 => {
                let list_type = bytes.get_u8();

                if list_type == 0 {
                    return NBTTagValue::List([].into());
                }

                let len = bytes.get_u32_le() as usize;
                let mut list = Vec::with_capacity(len);

                for _ in 0..len {
                    list.push(Self::from_bytes_payload(list_type, bytes));
                }

                bytes.get_u8();

                NBTTagValue::List(list)
            }
            10 => {
                let mut compound = HashMap::new();

                loop {
                    let tag_type = bytes.get_u8();
                    if tag_type == 0 {
                        break;
                    }

                    let key_len = bytes.get_u16_le() as usize;
                    let key = String::from_utf8_lossy(&bytes[..key_len]).into_owned();
                    bytes.advance(key_len);

                    let value = Self::from_bytes_payload(tag_type, bytes);
                    compound.insert(key, value);
                }

                NBTTagValue::Compound(compound)
            }
            11 => {
                let len = bytes.get_u32_le() as usize;
                let mut int_array = Vec::with_capacity(len);
                for _ in 0..len {
                    int_array.push(bytes.get_i32_le());
                }
                NBTTagValue::IntArray(int_array)
            }
            12 => {
                let len = bytes.get_u32_le() as usize;
                let mut long_array = Vec::with_capacity(len);
                for _ in 0..len {
                    long_array.push(bytes.get_i64_le());
                }
                NBTTagValue::LongArray(long_array)
            }
            _ => NBTTagValue::End,
        }
    }
}

impl std::ops::Index<&str> for NBTTag {
    type Output = NBTTagValue;

    fn index(&self, index: &str) -> &Self::Output {
        &self.value[index]
    }
}

impl std::ops::IndexMut<&str> for NBTTag {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        &mut self.value[index]
    }
}

impl std::ops::Index<usize> for NBTTag {
    type Output = NBTTagValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl std::ops::IndexMut<usize> for NBTTag {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}

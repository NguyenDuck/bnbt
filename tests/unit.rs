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
#[cfg(test)]
mod tests {

    mod simple {

        mod byte {
            use bnbt::{NBTTag, NBTTagValue, nbt};

            #[test]
            fn simple_tag() {
                let tag = NBTTag::new("test".into(), NBTTagValue::Byte(8));

                assert!(tag.value.is_byte());
                assert_eq!(tag.name, "test");
                assert_eq!(tag.value.as_byte().unwrap(), &8);
            }

            #[test]
            fn value_from_variable_using_macro() {
                let value = 8i8;
                let tag = nbt!("test", value);

                assert!(tag.value.is_byte());
                assert_eq!(tag.name, "test");
                assert_eq!(tag.value.as_byte().unwrap(), &value);
            }

            #[test]
            fn value_from_literal_using_macro() {
                let tag = nbt!("test", 8i8);

                assert!(tag.value.is_byte());
                assert_eq!(tag.name, "test");
                assert_eq!(tag.value.as_byte().unwrap(), &8);
            }
        }

        mod array_type {
            use bnbt::{NBTTag, NBTTagValue, nbt};

            #[test]
            fn byte_array_tag() {
                let tag = NBTTag::new_unnamed(NBTTagValue::ByteArray(vec![1, 2, 3]));

                assert!(tag.value.is_byte_array());
                assert_eq!(tag.value.as_byte_array().unwrap(), &[1, 2, 3]);
            }

            #[test]
            fn int_array_tag() {
                let tag = NBTTag::new_unnamed(NBTTagValue::IntArray(vec![1, 2, 3]));

                assert!(tag.value.is_int_array());
                assert_eq!(tag.value.as_int_array().unwrap(), &[1, 2, 3]);
            }

            #[test]
            fn long_array_tag() {
                let tag = NBTTag::new_unnamed(NBTTagValue::LongArray(vec![1, 2, 3]));

                assert!(tag.value.is_long_array());
                assert_eq!(tag.value.as_long_array().unwrap(), &[1, 2, 3]);
            }

            #[test]
            fn value_from_literal_using_macro() {
                let tag = nbt!("test", [B, 1, 2, 3]);

                assert!(tag.value.is_byte_array());
                assert_eq!(tag.value.as_byte_array().unwrap(), &[1, 2, 3]);
            }
        }

        mod list {
            use bnbt::{NBTTag, NBTTagValue, nbt};

            #[test]
            fn simple_tag() {
                let tag = NBTTag::new_unnamed(NBTTagValue::List(vec![
                    NBTTagValue::Byte(8),
                    NBTTagValue::Byte(16),
                ]));

                assert!(tag.value.is_list());
                assert_eq!(tag.value.as_list().unwrap().len(), 2);
                assert_eq!(tag.value.as_list().unwrap()[0].as_byte().unwrap(), &8);
                assert_eq!(tag.value.as_list().unwrap()[1].as_byte().unwrap(), &16);
            }

            #[test]
            fn value_from_literal_using_macro() {
                let tag = nbt!("test", [i8; 8, 16]);

                assert!(tag.value.is_list());
                assert_eq!(tag.value.as_list().unwrap().len(), 2);
                assert_eq!(tag.value.as_list().unwrap()[0].as_byte().unwrap(), &8);
                assert_eq!(tag.value.as_list().unwrap()[1].as_byte().unwrap(), &16);
            }

            #[test]
            fn access_nested_list_tag_with_index() {
                let tag = nbt!("", [[i8; 8, 16], [i8; 16, 32]]);

                assert!(tag.value.is_list());
                assert_eq!(tag.value.as_list().unwrap().len(), 2);
                assert_eq!(tag[0].as_list().unwrap().len(), 2);
                assert_eq!(tag[1].as_list().unwrap().len(), 2);
                assert_eq!(tag[0][0].as_byte().unwrap(), &8);
                assert_eq!(tag[0][1].as_byte().unwrap(), &16);
                assert_eq!(tag[1][0].as_byte().unwrap(), &16);
                assert_eq!(tag[1][1].as_byte().unwrap(), &32);
            }
        }

        mod compound {
            use std::collections::HashMap;

            use bnbt::{NBTTag, NBTTagValue, nbt};

            #[test]
            fn simple_tag() {
                let mut tag = NBTTag::new_unnamed(NBTTagValue::Compound(HashMap::new()));

                let compound = tag.value.as_compound_mut().unwrap();

                compound.insert("test".into(), 1i8.into());

                assert_eq!(tag["test"].as_byte().unwrap(), &1);
            }

            #[test]
            fn value_from_literal_using_macro() {
                let tag = nbt!("test", {});

                assert!(tag.value.is_compound());
                assert_eq!(tag.name, "test");
                assert_eq!(tag.value.as_compound().unwrap().len(), 0);
            }

            #[test]
            fn empty_nested_compound_tag() {
                let tag = nbt!("test", {
                    "test": {},
                });

                assert!(tag.value.is_compound());
                assert_eq!(tag.name, "test");
                assert_eq!(tag.value.as_compound().unwrap().len(), 1);
                assert_eq!(
                    *tag.value.as_compound().unwrap().get("test").unwrap(),
                    nbt!("", {}).value
                );
            }

            #[test]
            fn access_nested_compound_tag_with_index() {
                let tag = nbt!("", {
                    "test": {},
                });

                assert!(tag.value.is_compound());
                assert_eq!(tag.name, "");
                assert_eq!(tag.value.as_compound().unwrap().len(), 1);
                assert_eq!(tag["test"].as_compound().unwrap().len(), 0);
            }
        }
    }
}

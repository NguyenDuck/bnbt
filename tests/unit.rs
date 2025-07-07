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
    use bnbt::{tag::NBTTag, value::NBTTagValue};

    use std::collections::HashMap;

    #[test]
    fn test_simple_tag() {
        let tag = NBTTag::new("test".to_string(), NBTTagValue::Byte(8));

        assert!(tag.value.is_byte());
        assert_eq!(tag.name, "test");
        assert_eq!(tag.value.as_byte().unwrap(), &8);
    }

    #[test]
    fn test_list_tag() {
        let tag = NBTTag::new(
            "test".to_string(),
            NBTTagValue::List(vec![NBTTagValue::Byte(8), NBTTagValue::Byte(16)]),
        );
        assert!(tag.value.is_list());
        assert_eq!(tag.name, "test");
        assert_eq!(tag.value.as_list().unwrap().len(), 2);
        assert_eq!(tag.value.as_list().unwrap()[0].as_byte().unwrap(), &8);
        assert_eq!(tag.value.as_list().unwrap()[1].as_byte().unwrap(), &16);
    }

    #[test]
    fn test_compound_tag() {
        let mut map = HashMap::new();

        let list_value: Vec<NBTTagValue> = vec![8i8.into(), 16i8.into()];

        let list_tag = NBTTagValue::List(list_value);

        map.insert("list_tag".to_owned(), list_tag.clone());

        let tag = NBTTag::new("test".to_string(), NBTTagValue::Compound(map));

        assert!(tag.value.is_compound());
        assert_eq!(tag.name, "test");
        assert_eq!(tag.value.as_compound().unwrap().len(), 1);
        assert_eq!(
            *tag.value.as_compound().unwrap().get("list_tag").unwrap(),
            list_tag
        );
    }

    #[test]
    fn test_compound_index_operator() {
        let mut tag = NBTTag::new_unnamed(NBTTagValue::Compound(HashMap::new()));

        let compound = tag.value.as_compound_mut().unwrap();

        compound.insert("test".into(), 1i8.into());

        assert_eq!(tag["test"].as_byte().unwrap(), &1);
    }

    #[test]
    fn test_compound_index_mut_operator() {
        let mut tag = NBTTag::new_unnamed(NBTTagValue::Compound(HashMap::new()));

        let compound = tag.value.as_compound_mut().unwrap();

        compound.insert("test".into(), 1i8.into());

        *compound.get_mut("test").unwrap() = 2i8.into();

        assert_eq!(tag["test"].as_byte().unwrap(), &2);
    }

    #[test]
    fn test_list_index_operator() {
        let mut tag = NBTTag::new_unnamed(NBTTagValue::List(Vec::new()));

        let list = tag.value.as_list_mut().unwrap();

        list.push(1i8.into());

        assert_eq!(tag[0].as_byte().unwrap(), &1);
    }

    #[test]
    fn test_list_index_mut_operator() {
        let mut tag = NBTTag::new_unnamed(NBTTagValue::List(Vec::new()));

        let list = tag.value.as_list_mut().unwrap();

        list.push(1i8.into());

        *list.get_mut(0).unwrap() = 2i8.into();

        assert_eq!(tag[0].as_byte().unwrap(), &2);
    }
}

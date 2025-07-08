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
#[macro_export]
#[doc(hidden)]
macro_rules! nbt_spec_array_inner {
    (L, $($value:expr),* $(,)?) => {
        $crate::nbt_spec_array_inner!(Long, $($value),*)
    };

    (Long, $($value:expr),* $(,)?) => {
        $crate::NBTTagValue::LongArray(vec![$($value),*])
    };

    (I, $($value:expr),* $(,)?) => {
        $crate::nbt_spec_array_inner!(Int, $($value),*)
    };

    (Int, $($value:expr),* $(,)?) => {
        $crate::NBTTagValue::IntArray(vec![$($value),*])
    };

    (B, $($value:expr),* $(,)?) => {
        $crate::nbt_spec_array_inner!(Byte, $($value),*)
    };

    (Byte, $($value:expr),* $(,)?) => {
        $crate::NBTTagValue::ByteArray(vec![$($value),*])
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! nbt_inner {
    ({ $($key:tt : $value:tt),* $(,)? }) => {{
        use std::collections::HashMap;

        let mut map: HashMap<String, $crate::NBTTagValue> = HashMap::new();

        $(map.insert($key.into(), $crate::nbt_inner!(@value $value)))*;

        $crate::NBTTagValue::Compound(map)
    }};

    ([$type:ident, $($value:expr),* $(,)?]) => {
        $crate::nbt_spec_array_inner!($type, $($value),*)
    };

    ([$type:ty; $($value:expr),* $(,)?]) => {
        $crate::NBTTagValue::List(vec![
            $({
                let val: $type = $value;
                $crate::nbt_inner!(@value val)
            }),*
        ])
    };

    ([$($value:tt),* $(,)?]) => {
        $crate::NBTTagValue::List(vec![
            $(
                $crate::nbt_inner!($value)
            ),*
        ])
    };

    ($value:ident) => {
        $crate::nbt_inner!(@value $value)
    };

    ($value:literal) => {
        $crate::nbt_inner!(@value $value)
    };

    (@value $ident:ident) => {
        $crate::NBTTagValue::from($ident)
    };
    (@value $lit:literal) => {
        $crate::NBTTagValue::from($lit)
    };
    (@value $other:tt) => {
        $crate::NBTTagValue::from($crate::nbt_inner!($other))
    };
}

/// \/\/Examples
/// ```rust
/// use bnbt::nbt;
///
/// let string_tag = nbt!("test", "test");
/// let byte_tag = nbt!("test", 1i8);
/// let int_tag = nbt!("test", 2);
///
/// let byte_array_tag = nbt!("test", [B, 1, 2, 3]);
/// let int_array_tag = nbt!("test", [I, 1, 2, 3]);
/// let long_array_tag = nbt!("test", [L, 1, 2, 3]);
///
/// let empty_list_tag = nbt!("test", []);
/// let list_of_byte_tag = nbt!("test", [i8; 1, 2]);
/// let list_of_compound_tag = nbt!("test", [
///     {"byte": 1i8},
///     {"int": 2},
/// ]);
///
/// let compound_tag = nbt!("test", {});
/// ```
#[macro_export]
macro_rules! nbt {
    ($name:literal, $value:tt) => {{ $crate::NBTTag::new($name.to_string(), $crate::nbt_inner!($value)) }};
    ($name:expr, $value:tt) => {{ $crate::NBTTag::new($name, $crate::nbt_inner!($value)) }};
}

#[test]
fn nbt_macro_simple_test() {
    {
        let byte_tag = nbt!("test", 1i8);

        assert_eq!(byte_tag.get_type_id(), 1);
        assert_eq!(byte_tag.name, "test");
        assert_eq!(byte_tag.value.as_byte().unwrap(), &1);
    }
    {
        let short_tag = nbt!("test", 2i16);

        assert_eq!(short_tag.get_type_id(), 2);
        assert_eq!(short_tag.name, "test");
        assert_eq!(short_tag.value.as_short().unwrap(), &2);
    }
    {
        let int_tag = nbt!("test", 3i32);

        assert_eq!(int_tag.get_type_id(), 3);
        assert_eq!(int_tag.name, "test");
        assert_eq!(int_tag.value.as_int().unwrap(), &3);
    }
    {
        let long_tag = nbt!("test", 4i64);

        assert_eq!(long_tag.get_type_id(), 4);
        assert_eq!(long_tag.name, "test");
        assert_eq!(long_tag.value.as_long().unwrap(), &4);
    }
    {
        let float_tag = nbt!("test", 5f32);

        assert_eq!(float_tag.get_type_id(), 5);
        assert_eq!(float_tag.name, "test");
        assert_eq!(float_tag.value.as_float().unwrap(), &5f32);
    }
    {
        let double_tag = nbt!("test", 6f64);

        assert_eq!(double_tag.get_type_id(), 6);
        assert_eq!(double_tag.name, "test");
        assert_eq!(double_tag.value.as_double().unwrap(), &6f64);
    }
    {
        let byte_array_tag = nbt!("test", [B, 1, 2, 3]);

        assert_eq!(byte_array_tag.get_type_id(), 7);
        assert_eq!(byte_array_tag.name, "test");
        assert_eq!(byte_array_tag.value.as_byte_array().unwrap(), &[1, 2, 3]);
    }
    {
        let string_tag = nbt!("test", "test");

        assert_eq!(string_tag.get_type_id(), 8);
        assert_eq!(string_tag.name, "test");
        assert_eq!(string_tag.value.as_string().unwrap(), "test");
    }
    {
        let list_tag = nbt!("test", [i8; 1, 2,]);

        assert_eq!(list_tag.get_type_id(), 9);
        assert_eq!(list_tag.name, "test");
        assert_eq!(list_tag.value.as_list().unwrap().len(), 2);
        assert_eq!(list_tag.value.as_list().unwrap()[0].as_byte().unwrap(), &1);
        assert_eq!(list_tag.value.as_list().unwrap()[1].as_byte().unwrap(), &2);
    }
    {
        let compound_tag = nbt!("test", {
            "test": 1i8,
        });

        assert_eq!(compound_tag.get_type_id(), 10);
        assert_eq!(compound_tag.name, "test");
        assert_eq!(compound_tag.value.as_compound().unwrap().len(), 1);
        assert_eq!(
            *compound_tag
                .value
                .as_compound()
                .unwrap()
                .get("test")
                .unwrap(),
            1i8.into()
        );
    }
    {
        let int_array_tag = nbt!("test", [I, 1, 2, 3]);

        assert_eq!(int_array_tag.get_type_id(), 11);
        assert_eq!(int_array_tag.name, "test");
        assert_eq!(int_array_tag.value.as_int_array().unwrap(), &[1, 2, 3]);
    }
    {
        let long_array_tag = nbt!("test", [L, 1, 2, 3]);

        assert_eq!(long_array_tag.get_type_id(), 12);
        assert_eq!(long_array_tag.name, "test");
        assert_eq!(long_array_tag.value.as_long_array().unwrap(), &[1, 2, 3]);
    }
}

#[test]
fn nbt_macro_nested_test() {
    {
        let compound_tag = nbt!("test", {
            "test_nested_compound": {
                "test_byte": 1i8,
            },
        });

        assert_eq!(compound_tag.get_type_id(), 10);
        assert_eq!(compound_tag.name, "test");
        assert_eq!(compound_tag.value.as_compound().unwrap().len(), 1);
        assert_eq!(
            *compound_tag
                .value
                .as_compound()
                .unwrap()
                .get("test_nested_compound")
                .unwrap(),
            nbt!("", {
                "test_byte": 1i8,
            })
            .value
        );
    }
}

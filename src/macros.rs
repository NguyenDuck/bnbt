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

        $(map.insert($key.into(), $crate::nbt_inner!(@value $value));)*

        $crate::NBTTagValue::Compound(map)
    }};

    ([$type:ident, $($value:expr),* $(,)?]) => {
        $crate::nbt_spec_array_inner!($type, $($value),*)
    };

    ([$($value:tt),* $(,)?]) => {
        $crate::NBTTagValue::List(vec![
            $(
                $crate::nbt_inner!(@value $value)
            ),*
        ])
    };

    ([$type:ty; $($value:expr),* $(,)?]) => {
        $crate::NBTTagValue::List(vec![
            $({
                let val: $type = $value;
                $crate::nbt_inner!(@value val)
            }),*
        ])
    };

    ($value:tt) => {
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
    ($name:literal, $value:tt) => {
        $crate::NBTTag::new($name.to_string(), $crate::nbt_inner!($value))
    };
    ($name:expr, $value:tt) => {
        $crate::NBTTag::new($name, $crate::nbt_inner!($value))
    };
}

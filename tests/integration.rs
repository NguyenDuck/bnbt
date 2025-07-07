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
    use std::collections::HashMap;

    use bnbt::{NBTSerializer, NBTTag, NBTTagValue};
    use tempfile::NamedTempFile;

    #[test]
    fn test_write_read() {
        let mut tag = NBTTag::new_unnamed(NBTTagValue::Compound(HashMap::new()));

        let compound = tag.value.as_compound_mut().unwrap();

        compound.insert("test".into(), 1i8.into());

        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_str().unwrap();

        NBTSerializer::write_to_file(&tag, path).unwrap();

        let read_tag = NBTSerializer::read_from_file(path).unwrap();

        assert_eq!(tag, read_tag);
    }
}

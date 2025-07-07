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
use crate::tag::NBTTag;
use bytes::Bytes;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::io::{Read, Write};

pub struct NBTSerializer;

impl NBTSerializer {
    pub fn write_to_file(tag: &NBTTag, path: &str) -> std::io::Result<std::fs::File> {
        let file = std::fs::File::create(path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(&tag.to_bytes())?;

        Ok(encoder.finish()?)
    }

    pub fn read_from_file(path: &str) -> std::io::Result<NBTTag> {
        let file = std::fs::File::open(path)?;
        let mut decoder = GzDecoder::new(file);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)?;

        let tag = NBTTag::from_bytes(Bytes::from(buffer));

        Ok(tag)
    }
}

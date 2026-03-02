// Inline mbox iterator — replaces the `mbox-reader` crate.
//
// The upstream `mbox-reader` 0.2.0 (djc/mbox-reader, archived) has a bug where
// the iterator only emits an entry when it encounters the *next* "From " separator
// line, which means the very last message in every mbox file is silently dropped.
//
// This module is a minimal replacement that fixes that bug. It should be replaced
// if/when the upstream crate is fixed. See: https://github.com/djc/mbox-reader/issues
//
// The implementation reads the file into memory and scans for "\nFrom " boundaries,
// yielding slices between them. The final slice (last message) is correctly
// emitted when the iterator is exhausted.

use std::io;
use std::path::Path;

/// An mbox file loaded into memory.
pub struct MboxFile {
    data: Vec<u8>,
}

impl MboxFile {
    pub fn from_file(path: &Path) -> io::Result<MboxFile> {
        let data = std::fs::read(path)?;
        Ok(MboxFile { data })
    }

    pub fn iter(&self) -> MboxIterator<'_> {
        MboxIterator {
            data: &self.data,
            pos: 0,
            done: false,
        }
    }
}

/// Iterator over mbox entries. Each entry is the raw message bytes between
/// two "From " separator lines.
pub struct MboxIterator<'a> {
    data: &'a [u8],
    pos: usize,
    done: bool,
}

impl<'a> Iterator for MboxIterator<'a> {
    type Item = MboxEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done || self.pos >= self.data.len() {
            return None;
        }

        // Skip the "From " envelope line at current position
        let msg_start = if self.data[self.pos..].starts_with(b"From ") {
            match self.data[self.pos..].iter().position(|&b| b == b'\n') {
                Some(nl) => self.pos + nl + 1,
                None => {
                    self.done = true;
                    return None;
                }
            }
        } else {
            self.pos
        };

        // Scan for the next "\nFrom " boundary
        let remaining = &self.data[msg_start..];
        let mut i = 0;
        while i + 5 < remaining.len() {
            if remaining[i] == b'\n'
                && remaining[i + 1] == b'F'
                && remaining[i + 2] == b'r'
                && remaining[i + 3] == b'o'
                && remaining[i + 4] == b'm'
                && remaining[i + 5] == b' '
            {
                // Found next "From " — emit everything up to here
                let msg_end = msg_start + i + 1;
                self.pos = msg_end;
                let bytes = &self.data[msg_start..msg_end];
                if !bytes.is_empty() {
                    return Some(MboxEntry { bytes });
                }
            }
            i += 1;
        }

        // No more "From " found — emit the rest as the last message
        self.done = true;
        let bytes = &self.data[msg_start..];
        if bytes.is_empty() {
            return None;
        }
        Some(MboxEntry { bytes })
    }
}

/// A single message entry from an mbox file.
pub struct MboxEntry<'a> {
    bytes: &'a [u8],
}

impl<'a> MboxEntry<'a> {
    /// Returns the raw message bytes (without the "From " envelope line).
    pub fn message(&self) -> &'a [u8] {
        self.bytes
    }
}

//! Utility functions for the regex engine.

/// Safe bytecode buffer backed by Vec<u8>.
#[derive(Clone, Default)]
pub struct ByteBuffer {
    data: Vec<u8>,
    error: bool,
}

impl ByteBuffer {
    /// Create a new empty buffer.
    pub fn new() -> Self {
        ByteBuffer { data: Vec::new(), error: false }
    }

    /// Create a buffer with pre-allocated capacity.
    pub fn with_capacity(cap: usize) -> Self {
        ByteBuffer { data: Vec::with_capacity(cap), error: false }
    }

    /// Get raw pointer to the data (for legacy code).
    #[inline]
    pub fn as_ptr(&self) -> *const u8 { self.data.as_ptr() }

    /// Get mutable raw pointer to the data (for legacy code).
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 { self.data.as_mut_ptr() }

    /// Get the current length.
    #[inline]
    pub fn len(&self) -> usize { self.data.len() }

    /// Check if the buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool { self.data.is_empty() }

    /// Check if an error occurred.
    #[inline]
    pub fn has_error(&self) -> bool { self.error }

    /// Set error flag.
    #[inline]
    pub fn set_error(&mut self) { self.error = true; }

    /// Append a single byte.
    #[inline]
    pub fn push(&mut self, byte: u8) {
        self.data.push(byte);
    }

    /// Append a u16 in little-endian format.
    #[inline]
    pub fn push_u16(&mut self, val: u16) {
        self.data.extend_from_slice(&val.to_le_bytes());
    }

    /// Append a u32 in little-endian format.
    #[inline]
    pub fn push_u32(&mut self, val: u32) {
        self.data.extend_from_slice(&val.to_le_bytes());
    }

    /// Append a slice of bytes.
    #[inline]
    pub fn extend(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    /// Insert bytes at a given position, shifting existing bytes.
    pub fn insert_bytes(&mut self, pos: usize, len: usize) -> bool {
        if pos > self.data.len() {
            return false;
        }
        let old_len = self.data.len();
        self.data.resize(old_len + len, 0);
        self.data.copy_within(pos..old_len, pos + len);
        true
    }

    /// Truncate the buffer to the given length.
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.data.truncate(len);
    }

    /// Reserve capacity for at least `additional` more bytes.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Resize the buffer to `new_len`, filling new bytes with zeros.
    #[inline]
    pub fn resize(&mut self, new_len: usize) {
        self.data.resize(new_len, 0);
    }

    /// Consume and return the underlying Vec.
    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }

    /// Convert to a libc-allocated raw pointer.
    /// The caller is responsible for freeing this memory using libc::free.
    pub fn into_raw(self) -> *mut u8 {
        if self.data.is_empty() {
            return std::ptr::null_mut();
        }
        unsafe {
            let ptr = libc::malloc(self.data.len()) as *mut u8;
            if !ptr.is_null() {
                std::ptr::copy_nonoverlapping(self.data.as_ptr(), ptr, self.data.len());
            }
            ptr
        }
    }

    /// Clear the buffer (reset to empty).
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Return a slice of the data.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Return a mutable slice of the data.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Read a u32 from the buffer at the given offset (little-endian).
    #[inline]
    pub fn get_u32_at(&self, offset: usize) -> u32 {
        u32::from_le_bytes([
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ])
    }

    /// Write a u32 to the buffer at the given offset (little-endian).
    #[inline]
    pub fn set_u32_at(&mut self, offset: usize, val: u32) {
        let bytes = val.to_le_bytes();
        self.data[offset] = bytes[0];
        self.data[offset + 1] = bytes[1];
        self.data[offset + 2] = bytes[2];
        self.data[offset + 3] = bytes[3];
    }

    /// Read a u16 from the buffer at the given offset (little-endian).
    #[inline]
    pub fn get_u16_at(&self, offset: usize) -> u16 {
        u16::from_le_bytes([self.data[offset], self.data[offset + 1]])
    }

    /// Write a u16 to the buffer at the given offset (little-endian).
    #[inline]
    pub fn set_u16_at(&mut self, offset: usize, val: u16) {
        let bytes = val.to_le_bytes();
        self.data[offset] = bytes[0];
        self.data[offset + 1] = bytes[1];
    }
}

impl std::ops::Index<usize> for ByteBuffer {
    type Output = u8;
    #[inline]
    fn index(&self, index: usize) -> &u8 {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for ByteBuffer {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.data[index]
    }
}

impl std::ops::Index<std::ops::Range<usize>> for ByteBuffer {
    type Output = [u8];
    #[inline]
    fn index(&self, range: std::ops::Range<usize>) -> &[u8] {
        &self.data[range]
    }
}

impl std::ops::IndexMut<std::ops::Range<usize>> for ByteBuffer {
    #[inline]
    fn index_mut(&mut self, range: std::ops::Range<usize>) -> &mut [u8] {
        &mut self.data[range]
    }
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for ByteBuffer {
    type Output = [u8];
    #[inline]
    fn index(&self, range: std::ops::RangeFrom<usize>) -> &[u8] {
        &self.data[range]
    }
}

impl std::ops::IndexMut<std::ops::RangeFrom<usize>> for ByteBuffer {
    #[inline]
    fn index_mut(&mut self, range: std::ops::RangeFrom<usize>) -> &mut [u8] {
        &mut self.data[range]
    }
}

/// Minimum valid codepoint for each UTF-8 sequence length.
static UTF8_MIN_CODE: [u32; 5] = [0x80, 0x800, 0x10000, 0x200000, 0x4000000];

/// Mask for extracting data bits from the first byte of a multi-byte UTF-8 sequence.
static UTF8_FIRST_CODE_MASK: [u8; 5] = [0x1f, 0x0f, 0x07, 0x03, 0x01];

/// Encodes a Unicode codepoint to UTF-8.
/// Returns the number of bytes written, or 0 if the codepoint is invalid.
/// The buffer must have space for at least 6 bytes.
pub fn unicode_to_utf8(buf: &mut [u8], c: u32) -> usize {
    if c < 0x80 {
        buf[0] = c as u8;
        1
    } else if c < 0x800 {
        buf[0] = ((c >> 6) | 0xc0) as u8;
        buf[1] = ((c & 0x3f) | 0x80) as u8;
        2
    } else if c < 0x10000 {
        buf[0] = ((c >> 12) | 0xe0) as u8;
        buf[1] = (((c >> 6) & 0x3f) | 0x80) as u8;
        buf[2] = ((c & 0x3f) | 0x80) as u8;
        3
    } else if c < 0x200000 {
        buf[0] = ((c >> 18) | 0xf0) as u8;
        buf[1] = (((c >> 12) & 0x3f) | 0x80) as u8;
        buf[2] = (((c >> 6) & 0x3f) | 0x80) as u8;
        buf[3] = ((c & 0x3f) | 0x80) as u8;
        4
    } else if c < 0x4000000 {
        buf[0] = ((c >> 24) | 0xf8) as u8;
        buf[1] = (((c >> 18) & 0x3f) | 0x80) as u8;
        buf[2] = (((c >> 12) & 0x3f) | 0x80) as u8;
        buf[3] = (((c >> 6) & 0x3f) | 0x80) as u8;
        buf[4] = ((c & 0x3f) | 0x80) as u8;
        5
    } else if c < 0x80000000 {
        buf[0] = ((c >> 30) | 0xfc) as u8;
        buf[1] = (((c >> 24) & 0x3f) | 0x80) as u8;
        buf[2] = (((c >> 18) & 0x3f) | 0x80) as u8;
        buf[3] = (((c >> 12) & 0x3f) | 0x80) as u8;
        buf[4] = (((c >> 6) & 0x3f) | 0x80) as u8;
        buf[5] = ((c & 0x3f) | 0x80) as u8;
        6
    } else {
        0
    }
}

/// Decodes a UTF-8 sequence to a Unicode codepoint.
/// Returns `Some((codepoint, bytes_consumed))` on success, or `None` on invalid UTF-8.
pub fn unicode_from_utf8(buf: &[u8]) -> Option<(u32, usize)> {
    if buf.is_empty() {
        return None;
    }

    let c = buf[0] as i32;
    if c < 0x80 {
        return Some((c as u32, 1));
    }

    let l = match c {
        0xc0..=0xdf => 1,
        0xe0..=0xef => 2,
        0xf0..=0xf7 => 3,
        0xf8..=0xfb => 4,
        0xfc..=0xfd => 5,
        _ => return None,
    };

    if buf.len() < l + 1 {
        return None;
    }

    let mut c = (c & UTF8_FIRST_CODE_MASK[l - 1] as i32) as u32;

    for i in 0..l {
        let b = buf[1 + i] as i32;
        if b < 0x80 || b >= 0xc0 {
            return None;
        }
        c = (c << 6) | (b & 0x3f) as u32;
    }

    if c < UTF8_MIN_CODE[l - 1] {
        return None;
    }

    Some((c, l + 1))
}

/// Panics with an assertion failure message (for legacy C code compatibility).
pub fn assert_fail(assertion: &str, file: &str, line: u32, function: &str) -> ! {
    panic!(
        "Assertion failed: {}, function {}, file {}, line {}",
        assertion, function, file, line
    );
}

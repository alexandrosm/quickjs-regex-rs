use ::c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;

use super::util::ByteBuffer;
use super::util::unicode_to_utf8;
use super::util::unicode_from_utf8;

// Import from unicode module
use super::unicode::{
    lre_ctype_bits, lre_is_space_non_ascii, lre_is_id_start, lre_is_id_continue,
    lre_canonicalize, cr_init, cr_free, cr_realloc, cr_op1, cr_invert,
    cr_regexp_canonicalize, unicode_script, unicode_general_category,
    unicode_prop, unicode_sequence_prop, CharRange, DynBufReallocFunc,
};

// Use libc for memory operations
use libc::{memcpy, memmove, memset, memcmp, strcmp, strlen};

// Runtime callbacks - Rust implementations for pure Rust engine
/// Check for stack overflow. Returns 0 (no overflow) since Rust manages its own stack.
#[inline]
fn lre_check_stack_overflow(_opaque: *mut std::ffi::c_void, _alloca_size: size_t) -> i32 {
    0 // No stack overflow check needed - Rust handles this
}

/// Check for execution timeout. Returns 0 (no timeout) by default.
#[inline]
fn lre_check_timeout(_opaque: *mut std::ffi::c_void) -> i32 {
    0 // No timeout by default
}

/// Reallocate memory. Uses libc realloc.
#[inline]
fn lre_realloc(
    _opaque: *mut std::ffi::c_void,
    ptr: *mut std::ffi::c_void,
    size: size_t,
) -> *mut std::ffi::c_void {
    // SAFETY: ptr is either null or a valid allocation from malloc/realloc
    unsafe {
        if size == 0 {
            libc::free(ptr);
            std::ptr::null_mut()
        } else {
            libc::realloc(ptr, size)
        }
    }
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type BOOL = i32;
pub type C2RustUnnamed = u32;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
// packed_u32 and packed_u16 structs removed - now using std::ptr::read_unaligned/write_unaligned
// DynBuf struct removed - REParseState now uses ByteBuffer (Vec<u8>-backed)

/// Parser state for regex compilation.
/// Uses ByteBuffer (Vec<u8>-backed) instead of DynBuf for memory safety.
pub struct REParseState {
    pub byte_code: ByteBuffer,
    pub buf_ptr: *const uint8_t,
    pub buf_end: *const uint8_t,
    pub buf_start: *const uint8_t,
    pub re_flags: i32,
    pub is_unicode: BOOL,
    pub unicode_sets: BOOL,
    pub ignore_case: BOOL,
    pub multi_line: BOOL,
    pub dotall: BOOL,
    pub group_name_scope: uint8_t,
    pub capture_count: i32,
    pub total_capture_count: i32,
    pub has_named_captures: i32,
    pub opaque: *mut std::ffi::c_void,
    pub group_names: ByteBuffer,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub error_msg: [i8; 128],
    pub tmp_buf: [i8; 128],
}
pub const REOP_backward_back_reference_i: C2RustUnnamed_4 = 35;
pub const REOP_backward_back_reference: C2RustUnnamed_4 = 34;
pub const REOP_back_reference_i: C2RustUnnamed_4 = 33;
pub const REOP_back_reference: C2RustUnnamed_4 = 32;
pub const REOP_range32_i: C2RustUnnamed_4 = 39;
pub const REOP_range32: C2RustUnnamed_4 = 38;
pub const REOP_range_i: C2RustUnnamed_4 = 37;
pub const REOP_range: C2RustUnnamed_4 = 36;
pub const REOP_loop_check_adv_split_next_first: C2RustUnnamed_4 = 26;
pub const REOP_loop_check_adv_split_goto_first: C2RustUnnamed_4 = 25;
pub const REOP_loop_split_next_first: C2RustUnnamed_4 = 24;
pub const REOP_loop_split_goto_first: C2RustUnnamed_4 = 23;
pub const REOP_loop: C2RustUnnamed_4 = 22;
pub const REOP_check_advance: C2RustUnnamed_4 = 43;
pub const REOP_set_char_pos: C2RustUnnamed_4 = 42;
pub const REOP_set_i32: C2RustUnnamed_4 = 27;
pub const REOP_COUNT: C2RustUnnamed_4 = 45;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REOpCode {
    pub size: uint8_t,
}
pub const REOP_match: C2RustUnnamed_4 = 16;
pub const REOP_save_end: C2RustUnnamed_4 = 20;
pub const REOP_split_next_first: C2RustUnnamed_4 = 15;
pub const REOP_split_goto_first: C2RustUnnamed_4 = 14;
pub const REOP_goto: C2RustUnnamed_4 = 13;
pub const REOP_save_reset: C2RustUnnamed_4 = 21;
pub const REOP_save_start: C2RustUnnamed_4 = 19;
pub const REOP_prev: C2RustUnnamed_4 = 44;
pub const REOP_not_word_boundary_i: C2RustUnnamed_4 = 31;
pub const REOP_not_word_boundary: C2RustUnnamed_4 = 30;
pub const REOP_word_boundary_i: C2RustUnnamed_4 = 29;
pub const REOP_word_boundary: C2RustUnnamed_4 = 28;
pub const REOP_line_end_m: C2RustUnnamed_4 = 12;
pub const REOP_line_end: C2RustUnnamed_4 = 11;
pub const REOP_line_start_m: C2RustUnnamed_4 = 10;
pub const REOP_line_start: C2RustUnnamed_4 = 9;
pub const REOP_not_space: C2RustUnnamed_4 = 8;
pub const REOP_space: C2RustUnnamed_4 = 7;
pub const REOP_any: C2RustUnnamed_4 = 6;
pub const REOP_dot: C2RustUnnamed_4 = 5;
pub const REOP_char32_i: C2RustUnnamed_4 = 4;
pub const REOP_char32: C2RustUnnamed_4 = 3;
pub const REOP_char_i: C2RustUnnamed_4 = 2;
pub const REOP_char: C2RustUnnamed_4 = 1;
#[derive(Clone)]
#[repr(C)]
pub struct REStringList {
    pub cr: CharRange,
    pub n_strings: uint32_t,
    pub hash_size: uint32_t,
    pub hash_bits: i32,
    pub hash_table: *mut *mut REString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REString {
    pub next: *mut REString,
    pub hash: uint32_t,
    pub len: uint32_t,
    pub buf: [uint32_t; 0],
}
// CharRange is imported from unicode module
pub const CHAR_RANGE_S: C2RustUnnamed_5 = 3;
pub const CHAR_RANGE_s: C2RustUnnamed_5 = 2;
pub const CR_OP_UNION: C2RustUnnamed_2 = 0;
pub type UnicodeSequencePropCB = fn(
    *mut std::ffi::c_void,
    *const uint32_t,
    i32,
) -> ();
pub const CHAR_RANGE_W: C2RustUnnamed_5 = 5;
pub const CHAR_RANGE_w: C2RustUnnamed_5 = 4;
pub const CHAR_RANGE_D: C2RustUnnamed_5 = 1;
pub const CHAR_RANGE_d: C2RustUnnamed_5 = 0;
pub const CR_OP_SUB: C2RustUnnamed_2 = 3;
pub const CR_OP_INTER: C2RustUnnamed_2 = 1;
pub const UNICODE_C_DIGIT: C2RustUnnamed_3 = 2;
pub const UNICODE_C_DOLLAR: C2RustUnnamed_3 = 32;
pub const UNICODE_C_UNDER: C2RustUnnamed_3 = 16;
pub const UNICODE_C_LOWER: C2RustUnnamed_3 = 8;
pub const UNICODE_C_UPPER: C2RustUnnamed_3 = 4;
pub const REOP_lookahead_match: C2RustUnnamed_4 = 17;
pub const REOP_lookahead: C2RustUnnamed_4 = 40;
#[derive(Copy, Clone)]
#[repr(C)]
pub union StackElem {
    pub ptr: *mut uint8_t,
    pub val: intptr_t,
    pub bp: C2RustUnnamed_1,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    #[bitfield(name = "val", ty = "u64", bits = "0..=60")]
    #[bitfield(name = "type_0", ty = "u64", bits = "61..=63")]
    pub val_type_0: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REExecContext {
    pub cbuf: *const uint8_t,
    pub cbuf_end: *const uint8_t,
    pub cbuf_type: i32,
    pub capture_count: i32,
    pub is_unicode: BOOL,
    pub interrupt_counter: i32,
    pub opaque: *mut std::ffi::c_void,
    pub stack_buf: *mut StackElem,
    pub stack_size: size_t,
    pub static_stack_buf: [StackElem; 32],
}
pub const RE_EXEC_STATE_LOOKAHEAD: REExecStateEnum = 1;
pub type REExecStateEnum = u32;
pub const RE_EXEC_STATE_NEGATIVE_LOOKAHEAD: REExecStateEnum = 2;
pub const RE_EXEC_STATE_SPLIT: REExecStateEnum = 0;
pub const UNICODE_C_SPACE: C2RustUnnamed_3 = 1;
pub const REOP_negative_lookahead: C2RustUnnamed_4 = 41;
pub const REOP_negative_lookahead_match: C2RustUnnamed_4 = 18;
pub type C2RustUnnamed_2 = u32;
pub const CR_OP_XOR: C2RustUnnamed_2 = 2;
pub type C2RustUnnamed_3 = u32;
pub const UNICODE_C_XDIGIT: C2RustUnnamed_3 = 64;
pub type C2RustUnnamed_4 = u32;
pub const REOP_invalid: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = u32;
pub const NULL: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;
pub const INT32_MAX: i32 = 2147483647 as i32;
pub const UINT32_MAX: u32 = 4294967295 as u32;
pub const LRE_FLAG_IGNORECASE: i32 = (1 as i32)
    << 1 as i32;
/// Returns the maximum of two integers.
/// Converted from unsafe extern "C" fn to safe idiomatic Rust.
#[inline]
fn max_int(a: i32, b: i32) -> i32 {
    a.max(b)
}
/// Reads a u32 from a byte slice (little-endian, safe version).
#[inline]
fn get_u32_safe(data: &[u8]) -> u32 {
    u32::from_le_bytes([data[0], data[1], data[2], data[3]])
}

/// Reads a u16 from a byte slice (little-endian, safe version).
#[inline]
fn get_u16_safe(data: &[u8]) -> u32 {
    u16::from_le_bytes([data[0], data[1]]) as u32
}

/// Reads a u32 from a potentially unaligned byte pointer.
///
/// # Safety
/// The pointer must be valid for reading 4 bytes.
#[inline]
fn get_u32(tab: *const u8) -> u32 {
    // SAFETY: tab must point to at least 4 valid bytes
    unsafe {
        let slice = std::slice::from_raw_parts(tab, 4);
        get_u32_safe(slice)
    }
}

/// Reads a u16 from a potentially unaligned byte pointer.
#[inline]
fn get_u16(tab: *const u8) -> u32 {
    // SAFETY: tab must point to at least 2 valid bytes
    unsafe {
        let slice = std::slice::from_raw_parts(tab, 2);
        get_u16_safe(slice)
    }
}

// ByteBuffer helper methods - replacements for old dbuf_* functions
// These are now safe and work with the new ByteBuffer type

/// Copy a null-terminated string to a buffer with size limit.
fn pstrcpy(buf: *mut i8, buf_size: i32, src: *const i8) {
    if buf_size <= 0 {
        return;
    }
    // SAFETY: buf and src must be valid pointers, enforced by caller
    unsafe {
        let max_copy = (buf_size - 1) as usize;
        let mut i = 0;
        while i < max_copy && *src.add(i) != 0 {
            *buf.add(i) = *src.add(i);
            i += 1;
        }
        *buf.add(i) = 0;
    }
}

/// Push a single byte to the buffer (safe).
#[inline]
fn bb_putc(buf: &mut ByteBuffer, val: u8) {
    buf.push(val);
}

/// Push a u16 to the buffer in little-endian format (safe).
#[inline]
fn bb_put_u16(buf: &mut ByteBuffer, val: u16) {
    buf.push_u16(val);
}

/// Push a u32 to the buffer in little-endian format (safe).
#[inline]
fn bb_put_u32(buf: &mut ByteBuffer, val: u32) {
    buf.push_u32(val);
}

/// Extend the buffer with a slice of bytes (safe).
#[inline]
fn bb_put(buf: &mut ByteBuffer, data: &[u8]) {
    buf.extend(data);
}

/// Check if the buffer has an error (safe).
/// Returns BOOL (i32) for compatibility: 0 = no error, non-zero = error.
#[inline]
fn bb_error(buf: &ByteBuffer) -> BOOL {
    buf.has_error() as BOOL
}

/// Insert `len` zero bytes at position `pos`, shifting existing content (safe).
/// Returns 0 on success, -1 on failure (for compatibility with dbuf_insert).
#[inline]
fn bb_insert(buf: &mut ByteBuffer, pos: i32, len: i32) -> i32 {
    if buf.insert_bytes(pos as usize, len as usize) {
        0
    } else {
        -1
    }
}
pub const UTF8_CHAR_LEN_MAX: i32 = 6 as i32;
/// Checks if a code point is a UTF-16 high surrogate (0xD800-0xDBFF).
/// Converted from unsafe extern "C" fn to safe Rust.
#[inline]
fn is_hi_surrogate(c: u32) -> i32 {
    // High surrogates: 0xD800-0xDBFF (c >> 10 == 0x36)
    (c >> 10 == 0x36) as i32
}

/// Checks if a code point is a UTF-16 low surrogate (0xDC00-0xDFFF).
/// Converted from unsafe extern "C" fn to safe Rust.
#[inline]
fn is_lo_surrogate(c: u32) -> i32 {
    // Low surrogates: 0xDC00-0xDFFF (c >> 10 == 0x37)
    (c >> 10 == 0x37) as i32
}

/// Converts a UTF-16 surrogate pair to a Unicode code point.
/// Converted from unsafe extern "C" fn to safe Rust.
#[inline]
fn from_surrogate(hi: u32, lo: u32) -> u32 {
    // Formula: 0x10000 + (hi - 0xD800) * 0x400 + (lo - 0xDC00)
    0x10000_u32
        .wrapping_add(0x400_u32.wrapping_mul(hi.wrapping_sub(0xD800)))
        .wrapping_add(lo.wrapping_sub(0xDC00))
}

/// Converts a hex character to its numeric value (0-15), or -1 if invalid.
/// Converted from unsafe extern "C" fn to safe Rust.
#[inline]
fn from_hex(c: i32) -> i32 {
    match c as u8 as char {
        '0'..='9' => c - '0' as i32,
        'A'..='F' => c - 'A' as i32 + 10,
        'a'..='f' => c - 'a' as i32 + 10,
        _ => -1,
    }
}
pub const LRE_FLAG_MULTILINE: i32 = (1 as i32)
    << 2 as i32;
pub const CAPTURE_COUNT_MAX: i32 = 255 as i32;
pub const REGISTER_COUNT_MAX: i32 = 255 as i32;
pub const INTERRUPT_COUNTER_INIT: i32 = 10000 as i32;
pub const CP_LS: i32 = 0x2028 as i32;
pub const CP_PS: i32 = 0x2029 as i32;
static reopcode_info: [REOpCode; 45] = [
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 3 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 3 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 5 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 5 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 5 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 5 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 5 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 2 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 2 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 3 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 6 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 10 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 10 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 10 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 10 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 6 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 2 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 2 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 2 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 2 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 3 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 3 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 3 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 3 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 5 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 5 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 2 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 2 as uint8_t };
        init
    },
    {
        let mut init = REOpCode { size: 1 as uint8_t };
        init
    },
];
pub const RE_HEADER_FLAGS: i32 = 0 as i32;
pub const RE_HEADER_CAPTURE_COUNT: i32 = 2 as i32;
pub const RE_HEADER_REGISTER_COUNT: i32 = 3 as i32;
pub const RE_HEADER_BYTECODE_LEN: i32 = 4 as i32;
pub const RE_HEADER_LEN: i32 = 8 as i32;
/// Checks if a character is an ASCII digit ('0'-'9').
/// Converted from unsafe extern "C" fn to safe Rust.
#[inline]
fn is_digit(c: i32) -> i32 {
    (c >= '0' as i32 && c <= '9' as i32) as i32
}
/// Computes a hash value for a string buffer.
fn re_string_hash(buf: &[u32]) -> u32 {
    let mut h: u32 = 1;
    for &val in buf {
        h = h.wrapping_mul(263).wrapping_add(val);
    }
    h.wrapping_mul(0x61c88647)
}
fn re_string_list_init(
    s1: *mut REParseState,
    s: *mut REStringList,
) {
    // SAFETY: s1 and s are valid pointers from the parser
    unsafe {
        cr_init(
            &mut (*s).cr,
            (*s1).opaque,
            Some(
                lre_realloc
                    as fn(
                        *mut std::ffi::c_void,
                        *mut std::ffi::c_void,
                        size_t,
                    ) -> *mut std::ffi::c_void,
            ),
        );
        (*s).n_strings = 0 as uint32_t;
        (*s).hash_size = 0 as uint32_t;
        (*s).hash_bits = 0 as i32;
        (*s).hash_table = 0 as *mut *mut REString;
    }
}
/// Frees all strings in a string list and deallocates memory.
/// Converted outer loop from c2rust while to for loop.
fn re_string_list_free(s: *mut REStringList) {
    // SAFETY: s is a valid REStringList pointer from the parser
    unsafe {
        let mut p: *mut REString;
        let mut p_next: *mut REString;
        // Iterate through hash buckets
        for i in 0..(*s).hash_size {
            p = *((*s).hash_table).offset(i as isize);
            // Free linked list in each bucket (must remain while loop for linked list traversal)
            while !p.is_null() {
                p_next = (*p).next as *mut REString;
                lre_realloc((*s).cr.mem_opaque, p as *mut std::ffi::c_void, 0 as size_t);
                p = p_next;
            }
        }
        lre_realloc(
            (*s).cr.mem_opaque,
            (*s).hash_table as *mut std::ffi::c_void,
            0 as size_t,
        );
        cr_free(&mut (*s).cr);
    }
}
fn re_string_find2(
    s: *mut REStringList,
    len: i32,
    buf: *const uint32_t,
    h0: uint32_t,
    add_flag: BOOL,
) -> i32 {
    // SAFETY: s, buf are valid pointers from the parser
    unsafe {
        let mut h: uint32_t = 0 as uint32_t;
        let mut p: *mut REString = 0 as *mut REString;
        if (*s).n_strings != 0 as uint32_t {
            h = h0 >> 32 as i32 - (*s).hash_bits;
            p = *((*s).hash_table).offset(h as isize);
            while !p.is_null() {
                if (*p).hash == h0 && (*p).len == len as uint32_t
                    && memcmp(
                        ((*p).buf).as_mut_ptr() as *const std::ffi::c_void,
                        buf as *const std::ffi::c_void,
                        (len as size_t)
                            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                    ) == 0
                {
                    return 1 as i32;
                }
                p = (*p).next as *mut REString;
            }
        }
        if add_flag == 0 {
            return 0 as i32;
        }
        if (((*s).n_strings).wrapping_add(1 as uint32_t) > (*s).hash_size)
            as i32 as i64 != 0
        {
            let mut new_hash_table: *mut *mut REString = 0 as *mut *mut REString;
            let mut p_next: *mut REString = 0 as *mut REString;
            let mut new_hash_bits: i32 = 0;
            let mut new_hash_size: uint32_t = 0;
            new_hash_bits = max_int(
                (*s).hash_bits + 1 as i32,
                4 as i32,
            );
            new_hash_size = ((1 as i32) << new_hash_bits) as uint32_t;
            new_hash_table = lre_realloc(
                (*s).cr.mem_opaque,
                NULL,
                (::core::mem::size_of::<*mut REString>() as size_t)
                    .wrapping_mul(new_hash_size as size_t),
            ) as *mut *mut REString;
            if new_hash_table.is_null() {
                return -(1 as i32);
            }
            memset(
                new_hash_table as *mut std::ffi::c_void,
                0 as i32,
                (::core::mem::size_of::<*mut REString>() as size_t)
                    .wrapping_mul(new_hash_size as size_t),
            );
            // Rehash all existing entries into the new table
            for i in 0..(*s).hash_size {
                p = *((*s).hash_table).offset(i as isize);
                // Traverse linked list in bucket (must remain while loop)
                while !p.is_null() {
                    p_next = (*p).next as *mut REString;
                    h = (*p).hash >> 32 as i32 - new_hash_bits;
                    (*p).next = *new_hash_table.offset(h as isize) as *mut REString;
                    let ref mut fresh31 = *new_hash_table.offset(h as isize);
                    *fresh31 = p;
                    p = p_next;
                }
            }
            lre_realloc(
                (*s).cr.mem_opaque,
                (*s).hash_table as *mut std::ffi::c_void,
                0 as size_t,
            );
            (*s).hash_bits = new_hash_bits;
            (*s).hash_size = new_hash_size;
            (*s).hash_table = new_hash_table;
            h = h0 >> 32 as i32 - (*s).hash_bits;
        }
        p = lre_realloc(
            (*s).cr.mem_opaque,
            NULL,
            (::core::mem::size_of::<REString>() as size_t)
                .wrapping_add(
                    (len as size_t)
                        .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                ),
        ) as *mut REString;
        if p.is_null() {
            return -(1 as i32);
        }
        (*p).next = *((*s).hash_table).offset(h as isize) as *mut REString;
        let ref mut fresh32 = *((*s).hash_table).offset(h as isize);
        *fresh32 = p;
        (*s).n_strings = ((*s).n_strings).wrapping_add(1);
        (*p).hash = h0;
        (*p).len = len as uint32_t;
        memcpy(
            ((*p).buf).as_mut_ptr() as *mut std::ffi::c_void,
            buf as *const std::ffi::c_void,
            (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(len as size_t),
        );
        return 1 as i32;
    }
}
fn re_string_find(
    s: *mut REStringList,
    len: i32,
    buf: *const uint32_t,
    add_flag: BOOL,
) -> i32 {
    // SAFETY: buf is a valid pointer from the parser
    unsafe {
        let slice = std::slice::from_raw_parts(buf, len as usize);
        let h0 = re_string_hash(slice);
        return re_string_find2(s, len, buf, h0, add_flag);
    }
}
fn re_string_add(
    s: *mut REStringList,
    len: i32,
    buf: *const uint32_t,
) -> i32 {
    // SAFETY: s, buf are valid pointers from the parser
    unsafe {
        if len == 1 as i32 {
            return cr_union_interval(
                &mut (*s).cr,
                *buf.offset(0 as i32 as isize),
                *buf.offset(0 as i32 as isize),
            );
        }
        if re_string_find(s, len, buf, TRUE as i32 as BOOL)
            < 0 as i32
        {
            return -(1 as i32);
        }
        return 0 as i32;
    }
}
fn re_string_list_op(
    a: *mut REStringList,
    b: *mut REStringList,
    op: i32,
) -> i32 {
    // SAFETY: a, b are valid REStringList pointers from the parser
    unsafe {
        let mut i: i32 = 0;
        let mut ret: i32 = 0;
        let mut p: *mut REString = 0 as *mut REString;
        let mut pp: *mut *mut REString = 0 as *mut *mut REString;
        if cr_op1(&mut (*a).cr, (*b).cr.points, (*b).cr.len, op) != 0 {
            return -(1 as i32);
        }
        match op {
            0 => {
                if (*b).n_strings != 0 as uint32_t {
                    i = 0 as i32;
                    while (i as uint32_t) < (*b).hash_size {
                        p = *((*b).hash_table).offset(i as isize);
                        while !p.is_null() {
                            if re_string_find2(
                                a,
                                (*p).len as i32,
                                ((*p).buf).as_mut_ptr(),
                                (*p).hash,
                                TRUE as i32 as BOOL,
                            ) < 0 as i32
                            {
                                return -(1 as i32);
                            }
                            p = (*p).next as *mut REString;
                        }
                        i += 1;
                    }
                }
            }
            1 | 3 => {
                i = 0 as i32;
                while (i as uint32_t) < (*a).hash_size {
                    pp = &mut *((*a).hash_table).offset(i as isize) as *mut *mut REString;
                    loop {
                        p = *pp;
                        if p.is_null() {
                            break;
                        }
                        ret = re_string_find2(
                            b,
                            (*p).len as i32,
                            ((*p).buf).as_mut_ptr(),
                            (*p).hash,
                            FALSE as i32 as BOOL,
                        );
                        if op == CR_OP_SUB as i32 {
                            ret = (ret == 0) as i32;
                        }
                        if ret == 0 {
                            *pp = (*p).next as *mut REString;
                            (*a).n_strings = ((*a).n_strings).wrapping_sub(1);
                            lre_realloc(
                                (*a).cr.mem_opaque,
                                p as *mut std::ffi::c_void,
                                0 as size_t,
                            );
                        } else {
                            pp = &mut (*p).next as *mut *mut REString;
                        }
                    }
                    i += 1;
                }
            }
            _ => {
                std::process::abort();
            }
        }
        return 0 as i32;
    }
}
fn re_string_list_canonicalize(
    s1: *mut REParseState,
    s: *mut REStringList,
    is_unicode: BOOL,
) -> i32 {
    // SAFETY: s1, s are valid pointers from the parser
    unsafe {
        if cr_regexp_canonicalize(&mut (*s).cr, is_unicode as i32) != 0 {
            return -(1 as i32);
        }
        if (*s).n_strings != 0 as uint32_t {
            let mut a_s: REStringList = REStringList {
                cr: CharRange {
                    len: 0,
                    size: 0,
                    points: 0 as *mut uint32_t,
                    mem_opaque: 0 as *mut std::ffi::c_void,
                    realloc_func: None,
                },
                n_strings: 0,
                hash_size: 0,
                hash_bits: 0,
                hash_table: 0 as *mut *mut REString,
            };
            let mut a: *mut REStringList = &mut a_s;
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut p: *mut REString = 0 as *mut REString;
            re_string_list_init(s1, a);
            (*a).n_strings = (*s).n_strings;
            (*a).hash_size = (*s).hash_size;
            (*a).hash_bits = (*s).hash_bits;
            (*a).hash_table = (*s).hash_table;
            (*s).n_strings = 0 as uint32_t;
            (*s).hash_size = 0 as uint32_t;
            (*s).hash_bits = 0 as i32;
            (*s).hash_table = 0 as *mut *mut REString;
            i = 0 as i32;
            while (i as uint32_t) < (*a).hash_size {
                p = *((*a).hash_table).offset(i as isize);
                while !p.is_null() {
                    j = 0 as i32;
                    while (j as uint32_t) < (*p).len {
                        *((*p).buf).as_mut_ptr().offset(j as isize) = lre_canonicalize(
                            *((*p).buf).as_mut_ptr().offset(j as isize),
                            is_unicode as i32,
                        ) as uint32_t;
                        j += 1;
                    }
                    if re_string_add(
                        s,
                        (*p).len as i32,
                        ((*p).buf).as_mut_ptr(),
                    ) != 0
                    {
                        re_string_list_free(a);
                        return -(1 as i32);
                    }
                    p = (*p).next as *mut REString;
                }
                i += 1;
            }
            re_string_list_free(a);
        }
        return 0 as i32;
    }
}
static char_range_d: [uint16_t; 3] = [
    1 as i32 as uint16_t,
    0x30 as i32 as uint16_t,
    (0x39 as i32 + 1 as i32) as uint16_t,
];
static char_range_s: [uint16_t; 21] = [
    10 as i32 as uint16_t,
    0x9 as i32 as uint16_t,
    (0xd as i32 + 1 as i32) as uint16_t,
    0x20 as i32 as uint16_t,
    (0x20 as i32 + 1 as i32) as uint16_t,
    0xa0 as i32 as uint16_t,
    (0xa0 as i32 + 1 as i32) as uint16_t,
    0x1680 as i32 as uint16_t,
    (0x1680 as i32 + 1 as i32) as uint16_t,
    0x2000 as i32 as uint16_t,
    (0x200a as i32 + 1 as i32) as uint16_t,
    0x2028 as i32 as uint16_t,
    (0x2029 as i32 + 1 as i32) as uint16_t,
    0x202f as i32 as uint16_t,
    (0x202f as i32 + 1 as i32) as uint16_t,
    0x205f as i32 as uint16_t,
    (0x205f as i32 + 1 as i32) as uint16_t,
    0x3000 as i32 as uint16_t,
    (0x3000 as i32 + 1 as i32) as uint16_t,
    0xfeff as i32 as uint16_t,
    (0xfeff as i32 + 1 as i32) as uint16_t,
];
static char_range_w: [uint16_t; 9] = [
    4 as i32 as uint16_t,
    0x30 as i32 as uint16_t,
    (0x39 as i32 + 1 as i32) as uint16_t,
    0x41 as i32 as uint16_t,
    (0x5a as i32 + 1 as i32) as uint16_t,
    0x5f as i32 as uint16_t,
    (0x5f as i32 + 1 as i32) as uint16_t,
    0x61 as i32 as uint16_t,
    (0x7a as i32 + 1 as i32) as uint16_t,
];
pub const CLASS_RANGE_BASE: i32 = 0x40000000 as i32;

// Helper function to get char_range_table entry at runtime
/// Returns a pointer to a built-in character range table.
#[inline]
fn get_char_range_table(idx: usize) -> *const uint16_t {
    match idx {
        0 => char_range_d.as_ptr(),
        1 => char_range_s.as_ptr(),
        2 => char_range_w.as_ptr(),
        _ => core::ptr::null(),
    }
}
/// Initializes a character range from a built-in class.
fn cr_init_char_range(s: *mut REParseState, cr: *mut REStringList, c: u32) -> i32 {
    // SAFETY: s, cr are valid pointers from the parser; c_pt is a static table
    unsafe {
        let invert = (c & 1) as BOOL;
        let mut c_pt = get_char_range_table((c >> 1) as usize);
        let len = {
            let fresh = c_pt;
            c_pt = c_pt.offset(1);
            *fresh as i32
        };
        re_string_list_init(s, cr);

        for i in 0..(len * 2) {
            if cr_add_point(&mut (*cr).cr, *c_pt.offset(i as isize) as u32) != 0 {
                re_string_list_free(cr);
                return -1;
            }
        }

        if invert != 0 {
            if cr_invert(&mut (*cr).cr) != 0 {
                re_string_list_free(cr);
                return -1;
            }
        }
        0
    }
}
fn re_emit_op(s: &mut REParseState, op: i32) {
    bb_putc(&mut s.byte_code, op as u8);
}
fn re_emit_op_u32(s: &mut REParseState, op: i32, val: u32) -> i32 {
    bb_putc(&mut s.byte_code, op as u8);
    let pos = s.byte_code.len() as i32;
    bb_put_u32(&mut s.byte_code, val);
    pos
}
fn re_emit_goto(s: &mut REParseState, op: i32, val: u32) -> i32 {
    bb_putc(&mut s.byte_code, op as u8);
    let pos = s.byte_code.len() as i32;
    bb_put_u32(&mut s.byte_code, val.wrapping_sub((pos + 4) as u32));
    pos
}
fn re_emit_goto_u8(s: &mut REParseState, op: i32, arg: u32, val: u32) -> i32 {
    bb_putc(&mut s.byte_code, op as u8);
    bb_putc(&mut s.byte_code, arg as u8);
    let pos = s.byte_code.len() as i32;
    bb_put_u32(&mut s.byte_code, val.wrapping_sub((pos + 4) as u32));
    pos
}
fn re_emit_goto_u8_u32(s: &mut REParseState, op: i32, arg0: u32, arg1: u32, val: u32) -> i32 {
    bb_putc(&mut s.byte_code, op as u8);
    bb_putc(&mut s.byte_code, arg0 as u8);
    bb_put_u32(&mut s.byte_code, arg1);
    let pos = s.byte_code.len() as i32;
    bb_put_u32(&mut s.byte_code, val.wrapping_sub((pos + 4) as u32));
    pos
}
fn re_emit_op_u8(s: &mut REParseState, op: i32, val: u32) {
    bb_putc(&mut s.byte_code, op as u8);
    bb_putc(&mut s.byte_code, val as u8);
}
fn re_emit_op_u16(s: &mut REParseState, op: i32, val: u32) {
    bb_putc(&mut s.byte_code, op as u8);
    bb_put_u16(&mut s.byte_code, val as u16);
}
// STUB: re_parse_error uses C variadic arguments
// The function sets an error message and returns -1
// Note: Originally variadic, but we only use the format string
fn re_parse_error(s: &mut REParseState, fmt: *const i8) -> i32 {
    // Copy the format string as-is (better than nothing)
    // SAFETY: Accessing union field error_msg which is always valid when parsing
    unsafe {
        let dst = s.u.error_msg.as_mut_ptr();
        let mut i = 0;
        let mut p = fmt;
        while i < 126 && *p != 0 {
            *dst.add(i) = *p;
            i += 1;
            p = p.add(1);
        }
        *dst.add(i) = 0;
    }
    -1
}

fn re_parse_out_of_memory(s: &mut REParseState) -> i32 {
    re_parse_error(&mut *s, b"out of memory\0" as *const u8 as *const i8)
}
/// Parses decimal digits from a byte slice (safe version).
/// Returns (value, bytes_consumed). Returns -1 for value on overflow if allow_overflow is false.
#[inline]
fn parse_digits_safe(data: &[u8], allow_overflow: bool) -> (i32, usize) {
    let mut v: u64 = 0;
    let mut pos = 0;

    while pos < data.len() {
        let c = data[pos] as i32;
        if c < '0' as i32 || c > '9' as i32 {
            break;
        }
        v = v
            .wrapping_mul(10)
            .wrapping_add(c as u64)
            .wrapping_sub('0' as u64);
        if v >= INT32_MAX as u64 {
            if allow_overflow {
                v = INT32_MAX as u64;
            } else {
                return (-1, pos);
            }
        }
        pos += 1;
    }

    (v as i32, pos)
}

/// Parses decimal digits from a byte pointer, advancing the pointer.
fn parse_digits(
    pp: *mut *const uint8_t,
    allow_overflow: BOOL,
) -> i32 {
    // SAFETY: pp points to a valid pointer within the input buffer
    unsafe {
        let p = *pp;
        // Create a slice up to the end of a reasonable buffer (we don't know the actual length,
        // but parse_digits_safe will stop at non-digit characters)
        // We use a large upper bound since the actual stop is at non-digit chars
        let slice = std::slice::from_raw_parts(p, 256);
        let (value, consumed) = parse_digits_safe(slice, allow_overflow != 0);
        *pp = p.offset(consumed as isize);
        value
    }
}
/// Expects a specific character at the current position.
fn re_parse_expect(s: &mut REParseState, pp: *mut *const uint8_t, c: i32) -> i32 {
    // SAFETY: pp points to a valid pointer within the input buffer
    unsafe {
        let p = *pp;
        if *p as i32 != c {
            return re_parse_error(s, b"expecting character\0" as *const u8 as *const i8);
        }
        *pp = p.offset(1);
        0
    }
}
#[no_mangle]
pub fn lre_parse_escape(
    mut pp: *mut *const uint8_t,
    mut allow_utf16: i32,
) -> i32 {
    // SAFETY: pp points to a valid pointer, all pointer arithmetic stays in bounds
    unsafe {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut c: uint32_t = 0;
    p = *pp;
    let fresh26 = p;
    p = p.offset(1);
    c = *fresh26 as uint32_t;
    match c {
        98 => {
            c = '\u{8}' as i32 as uint32_t;
        }
        102 => {
            c = '\u{c}' as i32 as uint32_t;
        }
        110 => {
            c = '\n' as i32 as uint32_t;
        }
        114 => {
            c = '\r' as i32 as uint32_t;
        }
        116 => {
            c = '\t' as i32 as uint32_t;
        }
        118 => {
            c = '\u{b}' as i32 as uint32_t;
        }
        120 => {
            let mut h0: i32 = 0;
            let mut h1: i32 = 0;
            let fresh27 = p;
            p = p.offset(1);
            h0 = from_hex(*fresh27 as i32);
            if h0 < 0 as i32 {
                return -(1 as i32);
            }
            let fresh28 = p;
            p = p.offset(1);
            h1 = from_hex(*fresh28 as i32);
            if h1 < 0 as i32 {
                return -(1 as i32);
            }
            c = (h0 << 4 as i32 | h1) as uint32_t;
        }
        117 => {
            let mut h: i32 = 0;
            let mut i: i32 = 0;
            let mut c1: uint32_t = 0;
            if *p as i32 == '{' as i32 && allow_utf16 != 0 {
                p = p.offset(1);
                c = 0 as uint32_t;
                loop {
                    let fresh29 = p;
                    p = p.offset(1);
                    h = from_hex(*fresh29 as i32);
                    if h < 0 as i32 {
                        return -(1 as i32);
                    }
                    c = c << 4 as i32 | h as uint32_t;
                    if c > 0x10ffff as uint32_t {
                        return -(1 as i32);
                    }
                    if *p as i32 == '}' as i32 {
                        break;
                    }
                }
                p = p.offset(1);
            } else {
                c = 0 as uint32_t;
                i = 0 as i32;
                while i < 4 as i32 {
                    let fresh30 = p;
                    p = p.offset(1);
                    h = from_hex(*fresh30 as i32);
                    if h < 0 as i32 {
                        return -(1 as i32);
                    }
                    c = c << 4 as i32 | h as uint32_t;
                    i += 1;
                }
                if is_hi_surrogate(c) != 0 && allow_utf16 == 2 as i32
                    && *p.offset(0 as i32 as isize) as i32
                        == '\\' as i32
                    && *p.offset(1 as i32 as isize) as i32
                        == 'u' as i32
                {
                    c1 = 0 as uint32_t;
                    i = 0 as i32;
                    while i < 4 as i32 {
                        h = from_hex(
                            *p.offset((2 as i32 + i) as isize)
                                as i32,
                        );
                        if h < 0 as i32 {
                            break;
                        }
                        c1 = c1 << 4 as i32 | h as uint32_t;
                        i += 1;
                    }
                    if i == 4 as i32 && is_lo_surrogate(c1) != 0 {
                        p = p.offset(6 as i32 as isize);
                        c = from_surrogate(c, c1);
                    }
                }
            }
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
            c = (c as u32).wrapping_sub('0' as i32 as u32)
                as uint32_t as uint32_t;
            if allow_utf16 == 2 as i32 {
                if c != 0 as uint32_t || is_digit(*p as i32) != 0 {
                    return -(1 as i32);
                }
            } else {
                let mut v: uint32_t = 0;
                v = (*p as i32 - '0' as i32) as uint32_t;
                if !(v > 7 as uint32_t) {
                    c = c << 3 as i32 | v;
                    p = p.offset(1);
                    if !(c >= 32 as uint32_t) {
                        v = (*p as i32 - '0' as i32) as uint32_t;
                        if !(v > 7 as uint32_t) {
                            c = c << 3 as i32 | v;
                            p = p.offset(1);
                        }
                    }
                }
            }
        }
        _ => return -(2 as i32),
    }
    *pp = p;
    return c as i32;
    } // close unsafe block
}
/// Returns true if the character is a Unicode property name character.
fn is_unicode_char(c: i32) -> BOOL {
    (c >= '0' as i32 && c <= '9' as i32 || c >= 'A' as i32 && c <= 'Z' as i32
        || c >= 'a' as i32 && c <= 'z' as i32 || c == '_' as i32) as i32
}
fn seq_prop_cb(
    mut opaque: *mut std::ffi::c_void,
    mut seq: *const uint32_t,
    mut seq_len: i32,
) {
    let mut sl: *mut REStringList = opaque as *mut REStringList;
    re_string_add(sl, seq_len, seq);
}
fn parse_unicode_property(
    s: *mut REParseState,
    cr: *mut REStringList,
    pp: *mut *const uint8_t,
    is_inv: BOOL,
    allow_sequence_prop: BOOL,
) -> i32 {
    // SAFETY: all pointers are valid from the parser
    unsafe {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut name: [i8; 64] = [0; 64];
    let mut value: [i8; 64] = [0; 64];
    let mut q: *mut i8 = 0 as *mut i8;
    let mut script_ext: BOOL = 0;
    let mut ret: i32 = 0;
    p = *pp;
    if *p as i32 != '{' as i32 {
        return re_parse_error(
            &mut *s,
            b"expecting '{' after \\p\0" as *const u8 as *const i8,
        );
    }
    p = p.offset(1);
    q = name.as_mut_ptr();
    loop {
        if !(is_unicode_char(*p as i32) != 0) {
            current_block = 14523784380283086299;
            break;
        }
        if q.offset_from(name.as_mut_ptr()) as i64 as usize
            >= (::core::mem::size_of::<[i8; 64]>() as usize)
                .wrapping_sub(1 as usize)
        {
            current_block = 15312645817252085012;
            break;
        }
        let fresh33 = p;
        p = p.offset(1);
        let fresh34 = q;
        q = q.offset(1);
        *fresh34 = *fresh33 as i8;
    }
    match current_block {
        14523784380283086299 => {
            *q = '\0' as i32 as i8;
            q = value.as_mut_ptr();
            if *p as i32 == '=' as i32 {
                p = p.offset(1);
                while is_unicode_char(*p as i32) != 0 {
                    if q.offset_from(value.as_mut_ptr()) as i64 as usize
                        >= (::core::mem::size_of::<[i8; 64]>() as usize)
                            .wrapping_sub(1 as usize)
                    {
                        return re_parse_error(
            &mut *s,
                            b"unknown unicode property value\0" as *const u8
                                as *const i8,
                        );
                    }
                    let fresh35 = p;
                    p = p.offset(1);
                    let fresh36 = q;
                    q = q.offset(1);
                    *fresh36 = *fresh35 as i8;
                }
            }
            *q = '\0' as i32 as i8;
            if *p as i32 != '}' as i32 {
                return re_parse_error(
            &mut *s,
                    b"expecting '}'\0" as *const u8 as *const i8,
                );
            }
            p = p.offset(1);
            if strcmp(
                name.as_mut_ptr(),
                b"Script\0" as *const u8 as *const i8,
            ) == 0
                || strcmp(
                    name.as_mut_ptr(),
                    b"sc\0" as *const u8 as *const i8,
                ) == 0
            {
                script_ext = FALSE as i32 as BOOL;
                current_block = 12375508261711692781;
            } else if strcmp(
                name.as_mut_ptr(),
                b"Script_Extensions\0" as *const u8 as *const i8,
            ) == 0
                || strcmp(
                    name.as_mut_ptr(),
                    b"scx\0" as *const u8 as *const i8,
                ) == 0
            {
                script_ext = TRUE as i32 as BOOL;
                current_block = 12375508261711692781;
            } else if strcmp(
                name.as_mut_ptr(),
                b"General_Category\0" as *const u8 as *const i8,
            ) == 0
                || strcmp(
                    name.as_mut_ptr(),
                    b"gc\0" as *const u8 as *const i8,
                ) == 0
            {
                re_string_list_init(s, cr);
                ret = unicode_general_category(&mut (*cr).cr, value.as_mut_ptr());
                if ret != 0 {
                    re_string_list_free(cr);
                    if ret == -(2 as i32) {
                        return re_parse_error(
            &mut *s,
                            b"unknown unicode general category\0" as *const u8
                                as *const i8,
                        )
                    } else {
                        current_block = 11105379909962262667;
                    }
                } else {
                    current_block = 11793792312832361944;
                }
            } else if value[0 as i32 as usize] as i32
                == '\0' as i32
            {
                re_string_list_init(s, cr);
                ret = unicode_general_category(&mut (*cr).cr, name.as_mut_ptr());
                if ret == -(1 as i32) {
                    re_string_list_free(cr);
                    current_block = 11105379909962262667;
                } else {
                    if ret < 0 as i32 {
                        ret = unicode_prop(&mut (*cr).cr, name.as_mut_ptr());
                        if ret == -(1 as i32) {
                            re_string_list_free(cr);
                            current_block = 11105379909962262667;
                        } else {
                            current_block = 572715077006366937;
                        }
                    } else {
                        current_block = 572715077006366937;
                    }
                    match current_block {
                        11105379909962262667 => {}
                        _ => {
                            if ret < 0 as i32 && is_inv == 0
                                && allow_sequence_prop != 0
                            {
                                let mut cr_tmp: CharRange = CharRange {
                                    len: 0,
                                    size: 0,
                                    points: 0 as *mut uint32_t,
                                    mem_opaque: 0 as *mut std::ffi::c_void,
                                    realloc_func: None,
                                };
                                cr_init(
                                    &mut cr_tmp,
                                    (*s).opaque,
                                    Some(
                                        lre_realloc
                                            as fn(
                                                *mut std::ffi::c_void,
                                                *mut std::ffi::c_void,
                                                size_t,
                                            ) -> *mut std::ffi::c_void,
                                    ),
                                );
                                ret = unicode_sequence_prop(
                                    name.as_mut_ptr(),
                                    Some(
                                        seq_prop_cb
                                            as fn(
                                                *mut std::ffi::c_void,
                                                *const uint32_t,
                                                i32,
                                            ) -> (),
                                    ),
                                    cr as *mut std::ffi::c_void,
                                    &mut cr_tmp,
                                );
                                cr_free(&mut cr_tmp);
                                if ret == -(1 as i32) {
                                    re_string_list_free(cr);
                                    current_block = 11105379909962262667;
                                } else {
                                    current_block = 3160140712158701372;
                                }
                            } else {
                                current_block = 3160140712158701372;
                            }
                            match current_block {
                                11105379909962262667 => {}
                                _ => {
                                    if ret < 0 as i32 {
                                        current_block = 15312645817252085012;
                                    } else {
                                        current_block = 11793792312832361944;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                current_block = 15312645817252085012;
            }
            match current_block {
                15312645817252085012 => {}
                _ => {
                    match current_block {
                        12375508261711692781 => {
                            re_string_list_init(s, cr);
                            ret = unicode_script(
                                &mut (*cr).cr,
                                value.as_mut_ptr(),
                                script_ext as i32,
                            );
                            if ret != 0 {
                                re_string_list_free(cr);
                                if ret == -(2 as i32) {
                                    return re_parse_error(
            &mut *s,
                                        b"unknown unicode script\0" as *const u8
                                            as *const i8,
                                    )
                                } else {
                                    current_block = 11105379909962262667;
                                }
                            } else {
                                current_block = 11793792312832361944;
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        11793792312832361944 => {
                            if (*s).ignore_case != 0 && (*s).unicode_sets != 0 {
                                if re_string_list_canonicalize(s, cr, (*s).is_unicode) != 0
                                {
                                    re_string_list_free(cr);
                                    current_block = 11105379909962262667;
                                } else {
                                    current_block = 7018308795614528254;
                                }
                            } else {
                                current_block = 7018308795614528254;
                            }
                            match current_block {
                                11105379909962262667 => {}
                                _ => {
                                    if is_inv != 0 {
                                        if cr_invert(&mut (*cr).cr) != 0 {
                                            re_string_list_free(cr);
                                            current_block = 11105379909962262667;
                                        } else {
                                            current_block = 2520131295878969859;
                                        }
                                    } else {
                                        current_block = 2520131295878969859;
                                    }
                                    match current_block {
                                        11105379909962262667 => {}
                                        _ => {
                                            if (*s).ignore_case != 0 && (*s).unicode_sets == 0 {
                                                if re_string_list_canonicalize(s, cr, (*s).is_unicode) != 0
                                                {
                                                    re_string_list_free(cr);
                                                    current_block = 11105379909962262667;
                                                } else {
                                                    current_block = 7252614138838059896;
                                                }
                                            } else {
                                                current_block = 7252614138838059896;
                                            }
                                            match current_block {
                                                11105379909962262667 => {}
                                                _ => {
                                                    *pp = p;
                                                    return 0 as i32;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    return re_parse_out_of_memory(&mut *s);
                }
            }
        }
        _ => {}
    }
    return re_parse_error(
            &mut *s,
        b"unknown unicode property name\0" as *const u8 as *const i8,
    );
    } // close unsafe block
}
fn parse_class_string_disjunction(
    s: *mut REParseState,
    cr: *mut REStringList,
    pp: *mut *const uint8_t,
) -> i32 {
    // SAFETY: all pointers are valid from the parser
    unsafe {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut str_buf: ByteBuffer = ByteBuffer::new();
    let mut c: i32 = 0;
    p = *pp;
    if *p as i32 != '{' as i32 {
        return re_parse_error(
            &mut *s,
            b"expecting '{' after \\q\0" as *const u8 as *const i8,
        );
    }
    // ByteBuffer is already initialized
    re_string_list_init(s, cr);
    p = p.offset(1);
    's_31: loop {
        str_buf.clear();
        while *p as i32 != '}' as i32
            && *p as i32 != '|' as i32
        {
            c = get_class_atom(
                s,
                0 as *mut REStringList,
                &mut p,
                FALSE as i32 as BOOL,
            );
            if c < 0 as i32 {
                current_block = 4849670670732935458;
                break 's_31;
            }
            bb_put_u32(&mut str_buf, c as uint32_t);
        }
        if re_string_add(
            cr,
            (str_buf.len()).wrapping_div(4) as i32,
            str_buf.as_mut_ptr() as *mut uint32_t,
        ) != 0
        {
            re_parse_out_of_memory(&mut *s);
            current_block = 4849670670732935458;
            break;
        } else {
            if *p as i32 == '}' as i32 {
                current_block = 8831408221741692167;
                break;
            }
            p = p.offset(1);
        }
    }
    match current_block {
        8831408221741692167 => {
            if (*s).ignore_case != 0 {
                if re_string_list_canonicalize(s, cr, TRUE as i32 as BOOL)
                    != 0
                {
                    current_block = 4849670670732935458;
                } else {
                    current_block = 15904375183555213903;
                }
            } else {
                current_block = 15904375183555213903;
            }
            match current_block {
                4849670670732935458 => {}
                _ => {
                    p = p.offset(1);
                    // ByteBuffer drops automatically
                    *pp = p;
                    return 0 as i32;
                }
            }
        }
        _ => {}
    }
    // ByteBuffer drops automatically
    re_string_list_free(cr);
    return -(1 as i32);
    } // close unsafe block
}
fn get_class_atom(
    s: *mut REParseState,
    cr: *mut REStringList,
    pp: *mut *const uint8_t,
    inclass: BOOL,
) -> i32 {
    // SAFETY: all pointers are valid from the parser
    unsafe {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut c: uint32_t = 0;
    let mut ret: i32 = 0;
    p = *pp;
    c = *p as uint32_t;
    match c {
        92 => {
            p = p.offset(1);
            if p >= (*s).buf_end {
                current_block = 13671070048700312155;
            } else {
                let fresh25 = p;
                p = p.offset(1);
                c = *fresh25 as uint32_t;
                match c {
                    100 => {
                        current_block = 5512075495849833569;
                        match current_block {
                            11152951584060771545 => {
                                if (*s).unicode_sets != 0 && !cr.is_null() && inclass != 0 {
                                    if parse_class_string_disjunction(s, cr, &mut p) != 0 {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7338405858205321868 => {
                                if (*s).is_unicode != 0 && !cr.is_null() {
                                    if parse_unicode_property(
                                        s,
                                        cr,
                                        &mut p,
                                        (c == 'P' as i32 as uint32_t) as i32,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7105743787279032682 => {
                                c = *p as uint32_t;
                                if c >= 'a' as i32 as uint32_t
                                    && c <= 'z' as i32 as uint32_t
                                    || c >= 'A' as i32 as uint32_t
                                        && c <= 'Z' as i32 as uint32_t
                                    || (c >= '0' as i32 as uint32_t
                                        && c <= '9' as i32 as uint32_t
                                        || c == '_' as i32 as uint32_t) && inclass != 0
                                        && (*s).is_unicode == 0
                                {
                                    c = (c as u32 & 0x1f as u32)
                                        as uint32_t;
                                    p = p.offset(1);
                                    current_block = 8834769789432328951;
                                } else if (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    p = p.offset(-1);
                                    c = '\\' as i32 as uint32_t;
                                    current_block = 8834769789432328951;
                                }
                            }
                            5512075495849833569 => {
                                c = CHAR_RANGE_d as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17801949008645005255 => {
                                if inclass == 0 && (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    current_block = 8834769789432328951;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            8834769789432328951 => {}
                            _ => {
                                match current_block {
                                    8572234933149657763 => {
                                        if cr.is_null() {
                                            current_block = 11173639622526169465;
                                        } else {
                                            if cr_init_char_range(s, cr, c) != 0 {
                                                return -(1 as i32);
                                            }
                                            c = (c as u32)
                                                .wrapping_add(CLASS_RANGE_BASE as u32)
                                                as uint32_t as uint32_t;
                                            current_block = 8834769789432328951;
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    8834769789432328951 => {}
                                    _ => {
                                        match current_block {
                                            11173639622526169465 => {
                                                p = p.offset(-1);
                                                ret = lre_parse_escape(
                                                    &mut p,
                                                    (*s).is_unicode as i32 * 2 as i32,
                                                );
                                                if ret >= 0 as i32 {
                                                    c = ret as uint32_t;
                                                    current_block = 8834769789432328951;
                                                } else if (*s).is_unicode != 0 {
                                                    current_block = 7227556340696114278;
                                                } else {
                                                    current_block = 16251750946745332477;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            16251750946745332477 => {}
                                            8834769789432328951 => {}
                                            _ => {
                                                return re_parse_error(
            &mut *s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    68 => {
                        current_block = 17770042771771916326;
                        match current_block {
                            11152951584060771545 => {
                                if (*s).unicode_sets != 0 && !cr.is_null() && inclass != 0 {
                                    if parse_class_string_disjunction(s, cr, &mut p) != 0 {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7338405858205321868 => {
                                if (*s).is_unicode != 0 && !cr.is_null() {
                                    if parse_unicode_property(
                                        s,
                                        cr,
                                        &mut p,
                                        (c == 'P' as i32 as uint32_t) as i32,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7105743787279032682 => {
                                c = *p as uint32_t;
                                if c >= 'a' as i32 as uint32_t
                                    && c <= 'z' as i32 as uint32_t
                                    || c >= 'A' as i32 as uint32_t
                                        && c <= 'Z' as i32 as uint32_t
                                    || (c >= '0' as i32 as uint32_t
                                        && c <= '9' as i32 as uint32_t
                                        || c == '_' as i32 as uint32_t) && inclass != 0
                                        && (*s).is_unicode == 0
                                {
                                    c = (c as u32 & 0x1f as u32)
                                        as uint32_t;
                                    p = p.offset(1);
                                    current_block = 8834769789432328951;
                                } else if (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    p = p.offset(-1);
                                    c = '\\' as i32 as uint32_t;
                                    current_block = 8834769789432328951;
                                }
                            }
                            5512075495849833569 => {
                                c = CHAR_RANGE_d as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17801949008645005255 => {
                                if inclass == 0 && (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    current_block = 8834769789432328951;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            8834769789432328951 => {}
                            _ => {
                                match current_block {
                                    8572234933149657763 => {
                                        if cr.is_null() {
                                            current_block = 11173639622526169465;
                                        } else {
                                            if cr_init_char_range(s, cr, c) != 0 {
                                                return -(1 as i32);
                                            }
                                            c = (c as u32)
                                                .wrapping_add(CLASS_RANGE_BASE as u32)
                                                as uint32_t as uint32_t;
                                            current_block = 8834769789432328951;
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    8834769789432328951 => {}
                                    _ => {
                                        match current_block {
                                            11173639622526169465 => {
                                                p = p.offset(-1);
                                                ret = lre_parse_escape(
                                                    &mut p,
                                                    (*s).is_unicode as i32 * 2 as i32,
                                                );
                                                if ret >= 0 as i32 {
                                                    c = ret as uint32_t;
                                                    current_block = 8834769789432328951;
                                                } else if (*s).is_unicode != 0 {
                                                    current_block = 7227556340696114278;
                                                } else {
                                                    current_block = 16251750946745332477;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            16251750946745332477 => {}
                                            8834769789432328951 => {}
                                            _ => {
                                                return re_parse_error(
            &mut *s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    115 => {
                        current_block = 802061165721266012;
                        match current_block {
                            11152951584060771545 => {
                                if (*s).unicode_sets != 0 && !cr.is_null() && inclass != 0 {
                                    if parse_class_string_disjunction(s, cr, &mut p) != 0 {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7338405858205321868 => {
                                if (*s).is_unicode != 0 && !cr.is_null() {
                                    if parse_unicode_property(
                                        s,
                                        cr,
                                        &mut p,
                                        (c == 'P' as i32 as uint32_t) as i32,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7105743787279032682 => {
                                c = *p as uint32_t;
                                if c >= 'a' as i32 as uint32_t
                                    && c <= 'z' as i32 as uint32_t
                                    || c >= 'A' as i32 as uint32_t
                                        && c <= 'Z' as i32 as uint32_t
                                    || (c >= '0' as i32 as uint32_t
                                        && c <= '9' as i32 as uint32_t
                                        || c == '_' as i32 as uint32_t) && inclass != 0
                                        && (*s).is_unicode == 0
                                {
                                    c = (c as u32 & 0x1f as u32)
                                        as uint32_t;
                                    p = p.offset(1);
                                    current_block = 8834769789432328951;
                                } else if (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    p = p.offset(-1);
                                    c = '\\' as i32 as uint32_t;
                                    current_block = 8834769789432328951;
                                }
                            }
                            5512075495849833569 => {
                                c = CHAR_RANGE_d as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17801949008645005255 => {
                                if inclass == 0 && (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    current_block = 8834769789432328951;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            8834769789432328951 => {}
                            _ => {
                                match current_block {
                                    8572234933149657763 => {
                                        if cr.is_null() {
                                            current_block = 11173639622526169465;
                                        } else {
                                            if cr_init_char_range(s, cr, c) != 0 {
                                                return -(1 as i32);
                                            }
                                            c = (c as u32)
                                                .wrapping_add(CLASS_RANGE_BASE as u32)
                                                as uint32_t as uint32_t;
                                            current_block = 8834769789432328951;
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    8834769789432328951 => {}
                                    _ => {
                                        match current_block {
                                            11173639622526169465 => {
                                                p = p.offset(-1);
                                                ret = lre_parse_escape(
                                                    &mut p,
                                                    (*s).is_unicode as i32 * 2 as i32,
                                                );
                                                if ret >= 0 as i32 {
                                                    c = ret as uint32_t;
                                                    current_block = 8834769789432328951;
                                                } else if (*s).is_unicode != 0 {
                                                    current_block = 7227556340696114278;
                                                } else {
                                                    current_block = 16251750946745332477;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            16251750946745332477 => {}
                                            8834769789432328951 => {}
                                            _ => {
                                                return re_parse_error(
            &mut *s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    83 => {
                        current_block = 10822250284037535193;
                        match current_block {
                            11152951584060771545 => {
                                if (*s).unicode_sets != 0 && !cr.is_null() && inclass != 0 {
                                    if parse_class_string_disjunction(s, cr, &mut p) != 0 {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7338405858205321868 => {
                                if (*s).is_unicode != 0 && !cr.is_null() {
                                    if parse_unicode_property(
                                        s,
                                        cr,
                                        &mut p,
                                        (c == 'P' as i32 as uint32_t) as i32,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7105743787279032682 => {
                                c = *p as uint32_t;
                                if c >= 'a' as i32 as uint32_t
                                    && c <= 'z' as i32 as uint32_t
                                    || c >= 'A' as i32 as uint32_t
                                        && c <= 'Z' as i32 as uint32_t
                                    || (c >= '0' as i32 as uint32_t
                                        && c <= '9' as i32 as uint32_t
                                        || c == '_' as i32 as uint32_t) && inclass != 0
                                        && (*s).is_unicode == 0
                                {
                                    c = (c as u32 & 0x1f as u32)
                                        as uint32_t;
                                    p = p.offset(1);
                                    current_block = 8834769789432328951;
                                } else if (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    p = p.offset(-1);
                                    c = '\\' as i32 as uint32_t;
                                    current_block = 8834769789432328951;
                                }
                            }
                            5512075495849833569 => {
                                c = CHAR_RANGE_d as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17801949008645005255 => {
                                if inclass == 0 && (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    current_block = 8834769789432328951;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            8834769789432328951 => {}
                            _ => {
                                match current_block {
                                    8572234933149657763 => {
                                        if cr.is_null() {
                                            current_block = 11173639622526169465;
                                        } else {
                                            if cr_init_char_range(s, cr, c) != 0 {
                                                return -(1 as i32);
                                            }
                                            c = (c as u32)
                                                .wrapping_add(CLASS_RANGE_BASE as u32)
                                                as uint32_t as uint32_t;
                                            current_block = 8834769789432328951;
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    8834769789432328951 => {}
                                    _ => {
                                        match current_block {
                                            11173639622526169465 => {
                                                p = p.offset(-1);
                                                ret = lre_parse_escape(
                                                    &mut p,
                                                    (*s).is_unicode as i32 * 2 as i32,
                                                );
                                                if ret >= 0 as i32 {
                                                    c = ret as uint32_t;
                                                    current_block = 8834769789432328951;
                                                } else if (*s).is_unicode != 0 {
                                                    current_block = 7227556340696114278;
                                                } else {
                                                    current_block = 16251750946745332477;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            16251750946745332477 => {}
                                            8834769789432328951 => {}
                                            _ => {
                                                return re_parse_error(
            &mut *s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    119 => {
                        current_block = 1814055369555096573;
                        match current_block {
                            11152951584060771545 => {
                                if (*s).unicode_sets != 0 && !cr.is_null() && inclass != 0 {
                                    if parse_class_string_disjunction(s, cr, &mut p) != 0 {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7338405858205321868 => {
                                if (*s).is_unicode != 0 && !cr.is_null() {
                                    if parse_unicode_property(
                                        s,
                                        cr,
                                        &mut p,
                                        (c == 'P' as i32 as uint32_t) as i32,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7105743787279032682 => {
                                c = *p as uint32_t;
                                if c >= 'a' as i32 as uint32_t
                                    && c <= 'z' as i32 as uint32_t
                                    || c >= 'A' as i32 as uint32_t
                                        && c <= 'Z' as i32 as uint32_t
                                    || (c >= '0' as i32 as uint32_t
                                        && c <= '9' as i32 as uint32_t
                                        || c == '_' as i32 as uint32_t) && inclass != 0
                                        && (*s).is_unicode == 0
                                {
                                    c = (c as u32 & 0x1f as u32)
                                        as uint32_t;
                                    p = p.offset(1);
                                    current_block = 8834769789432328951;
                                } else if (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    p = p.offset(-1);
                                    c = '\\' as i32 as uint32_t;
                                    current_block = 8834769789432328951;
                                }
                            }
                            5512075495849833569 => {
                                c = CHAR_RANGE_d as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17801949008645005255 => {
                                if inclass == 0 && (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    current_block = 8834769789432328951;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            8834769789432328951 => {}
                            _ => {
                                match current_block {
                                    8572234933149657763 => {
                                        if cr.is_null() {
                                            current_block = 11173639622526169465;
                                        } else {
                                            if cr_init_char_range(s, cr, c) != 0 {
                                                return -(1 as i32);
                                            }
                                            c = (c as u32)
                                                .wrapping_add(CLASS_RANGE_BASE as u32)
                                                as uint32_t as uint32_t;
                                            current_block = 8834769789432328951;
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    8834769789432328951 => {}
                                    _ => {
                                        match current_block {
                                            11173639622526169465 => {
                                                p = p.offset(-1);
                                                ret = lre_parse_escape(
                                                    &mut p,
                                                    (*s).is_unicode as i32 * 2 as i32,
                                                );
                                                if ret >= 0 as i32 {
                                                    c = ret as uint32_t;
                                                    current_block = 8834769789432328951;
                                                } else if (*s).is_unicode != 0 {
                                                    current_block = 7227556340696114278;
                                                } else {
                                                    current_block = 16251750946745332477;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            16251750946745332477 => {}
                                            8834769789432328951 => {}
                                            _ => {
                                                return re_parse_error(
            &mut *s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    87 => {
                        current_block = 10254712216801151959;
                        match current_block {
                            11152951584060771545 => {
                                if (*s).unicode_sets != 0 && !cr.is_null() && inclass != 0 {
                                    if parse_class_string_disjunction(s, cr, &mut p) != 0 {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7338405858205321868 => {
                                if (*s).is_unicode != 0 && !cr.is_null() {
                                    if parse_unicode_property(
                                        s,
                                        cr,
                                        &mut p,
                                        (c == 'P' as i32 as uint32_t) as i32,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7105743787279032682 => {
                                c = *p as uint32_t;
                                if c >= 'a' as i32 as uint32_t
                                    && c <= 'z' as i32 as uint32_t
                                    || c >= 'A' as i32 as uint32_t
                                        && c <= 'Z' as i32 as uint32_t
                                    || (c >= '0' as i32 as uint32_t
                                        && c <= '9' as i32 as uint32_t
                                        || c == '_' as i32 as uint32_t) && inclass != 0
                                        && (*s).is_unicode == 0
                                {
                                    c = (c as u32 & 0x1f as u32)
                                        as uint32_t;
                                    p = p.offset(1);
                                    current_block = 8834769789432328951;
                                } else if (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    p = p.offset(-1);
                                    c = '\\' as i32 as uint32_t;
                                    current_block = 8834769789432328951;
                                }
                            }
                            5512075495849833569 => {
                                c = CHAR_RANGE_d as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17801949008645005255 => {
                                if inclass == 0 && (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    current_block = 8834769789432328951;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            8834769789432328951 => {}
                            _ => {
                                match current_block {
                                    8572234933149657763 => {
                                        if cr.is_null() {
                                            current_block = 11173639622526169465;
                                        } else {
                                            if cr_init_char_range(s, cr, c) != 0 {
                                                return -(1 as i32);
                                            }
                                            c = (c as u32)
                                                .wrapping_add(CLASS_RANGE_BASE as u32)
                                                as uint32_t as uint32_t;
                                            current_block = 8834769789432328951;
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    8834769789432328951 => {}
                                    _ => {
                                        match current_block {
                                            11173639622526169465 => {
                                                p = p.offset(-1);
                                                ret = lre_parse_escape(
                                                    &mut p,
                                                    (*s).is_unicode as i32 * 2 as i32,
                                                );
                                                if ret >= 0 as i32 {
                                                    c = ret as uint32_t;
                                                    current_block = 8834769789432328951;
                                                } else if (*s).is_unicode != 0 {
                                                    current_block = 7227556340696114278;
                                                } else {
                                                    current_block = 16251750946745332477;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            16251750946745332477 => {}
                                            8834769789432328951 => {}
                                            _ => {
                                                return re_parse_error(
            &mut *s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    99 => {
                        current_block = 7105743787279032682;
                        match current_block {
                            11152951584060771545 => {
                                if (*s).unicode_sets != 0 && !cr.is_null() && inclass != 0 {
                                    if parse_class_string_disjunction(s, cr, &mut p) != 0 {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7338405858205321868 => {
                                if (*s).is_unicode != 0 && !cr.is_null() {
                                    if parse_unicode_property(
                                        s,
                                        cr,
                                        &mut p,
                                        (c == 'P' as i32 as uint32_t) as i32,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7105743787279032682 => {
                                c = *p as uint32_t;
                                if c >= 'a' as i32 as uint32_t
                                    && c <= 'z' as i32 as uint32_t
                                    || c >= 'A' as i32 as uint32_t
                                        && c <= 'Z' as i32 as uint32_t
                                    || (c >= '0' as i32 as uint32_t
                                        && c <= '9' as i32 as uint32_t
                                        || c == '_' as i32 as uint32_t) && inclass != 0
                                        && (*s).is_unicode == 0
                                {
                                    c = (c as u32 & 0x1f as u32)
                                        as uint32_t;
                                    p = p.offset(1);
                                    current_block = 8834769789432328951;
                                } else if (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    p = p.offset(-1);
                                    c = '\\' as i32 as uint32_t;
                                    current_block = 8834769789432328951;
                                }
                            }
                            5512075495849833569 => {
                                c = CHAR_RANGE_d as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17801949008645005255 => {
                                if inclass == 0 && (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    current_block = 8834769789432328951;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            8834769789432328951 => {}
                            _ => {
                                match current_block {
                                    8572234933149657763 => {
                                        if cr.is_null() {
                                            current_block = 11173639622526169465;
                                        } else {
                                            if cr_init_char_range(s, cr, c) != 0 {
                                                return -(1 as i32);
                                            }
                                            c = (c as u32)
                                                .wrapping_add(CLASS_RANGE_BASE as u32)
                                                as uint32_t as uint32_t;
                                            current_block = 8834769789432328951;
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    8834769789432328951 => {}
                                    _ => {
                                        match current_block {
                                            11173639622526169465 => {
                                                p = p.offset(-1);
                                                ret = lre_parse_escape(
                                                    &mut p,
                                                    (*s).is_unicode as i32 * 2 as i32,
                                                );
                                                if ret >= 0 as i32 {
                                                    c = ret as uint32_t;
                                                    current_block = 8834769789432328951;
                                                } else if (*s).is_unicode != 0 {
                                                    current_block = 7227556340696114278;
                                                } else {
                                                    current_block = 16251750946745332477;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            16251750946745332477 => {}
                                            8834769789432328951 => {}
                                            _ => {
                                                return re_parse_error(
            &mut *s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    45 => {
                        current_block = 17801949008645005255;
                        match current_block {
                            11152951584060771545 => {
                                if (*s).unicode_sets != 0 && !cr.is_null() && inclass != 0 {
                                    if parse_class_string_disjunction(s, cr, &mut p) != 0 {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7338405858205321868 => {
                                if (*s).is_unicode != 0 && !cr.is_null() {
                                    if parse_unicode_property(
                                        s,
                                        cr,
                                        &mut p,
                                        (c == 'P' as i32 as uint32_t) as i32,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7105743787279032682 => {
                                c = *p as uint32_t;
                                if c >= 'a' as i32 as uint32_t
                                    && c <= 'z' as i32 as uint32_t
                                    || c >= 'A' as i32 as uint32_t
                                        && c <= 'Z' as i32 as uint32_t
                                    || (c >= '0' as i32 as uint32_t
                                        && c <= '9' as i32 as uint32_t
                                        || c == '_' as i32 as uint32_t) && inclass != 0
                                        && (*s).is_unicode == 0
                                {
                                    c = (c as u32 & 0x1f as u32)
                                        as uint32_t;
                                    p = p.offset(1);
                                    current_block = 8834769789432328951;
                                } else if (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    p = p.offset(-1);
                                    c = '\\' as i32 as uint32_t;
                                    current_block = 8834769789432328951;
                                }
                            }
                            5512075495849833569 => {
                                c = CHAR_RANGE_d as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17801949008645005255 => {
                                if inclass == 0 && (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    current_block = 8834769789432328951;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            8834769789432328951 => {}
                            _ => {
                                match current_block {
                                    8572234933149657763 => {
                                        if cr.is_null() {
                                            current_block = 11173639622526169465;
                                        } else {
                                            if cr_init_char_range(s, cr, c) != 0 {
                                                return -(1 as i32);
                                            }
                                            c = (c as u32)
                                                .wrapping_add(CLASS_RANGE_BASE as u32)
                                                as uint32_t as uint32_t;
                                            current_block = 8834769789432328951;
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    8834769789432328951 => {}
                                    _ => {
                                        match current_block {
                                            11173639622526169465 => {
                                                p = p.offset(-1);
                                                ret = lre_parse_escape(
                                                    &mut p,
                                                    (*s).is_unicode as i32 * 2 as i32,
                                                );
                                                if ret >= 0 as i32 {
                                                    c = ret as uint32_t;
                                                    current_block = 8834769789432328951;
                                                } else if (*s).is_unicode != 0 {
                                                    current_block = 7227556340696114278;
                                                } else {
                                                    current_block = 16251750946745332477;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            16251750946745332477 => {}
                                            8834769789432328951 => {}
                                            _ => {
                                                return re_parse_error(
            &mut *s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    94 | 36 | 92 | 46 | 42 | 43 | 63 | 40 | 41 | 91 | 93 | 123 | 125
                    | 124 | 47 => {
                        current_block = 8834769789432328951;
                    }
                    112 | 80 => {
                        current_block = 7338405858205321868;
                        match current_block {
                            11152951584060771545 => {
                                if (*s).unicode_sets != 0 && !cr.is_null() && inclass != 0 {
                                    if parse_class_string_disjunction(s, cr, &mut p) != 0 {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7338405858205321868 => {
                                if (*s).is_unicode != 0 && !cr.is_null() {
                                    if parse_unicode_property(
                                        s,
                                        cr,
                                        &mut p,
                                        (c == 'P' as i32 as uint32_t) as i32,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7105743787279032682 => {
                                c = *p as uint32_t;
                                if c >= 'a' as i32 as uint32_t
                                    && c <= 'z' as i32 as uint32_t
                                    || c >= 'A' as i32 as uint32_t
                                        && c <= 'Z' as i32 as uint32_t
                                    || (c >= '0' as i32 as uint32_t
                                        && c <= '9' as i32 as uint32_t
                                        || c == '_' as i32 as uint32_t) && inclass != 0
                                        && (*s).is_unicode == 0
                                {
                                    c = (c as u32 & 0x1f as u32)
                                        as uint32_t;
                                    p = p.offset(1);
                                    current_block = 8834769789432328951;
                                } else if (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    p = p.offset(-1);
                                    c = '\\' as i32 as uint32_t;
                                    current_block = 8834769789432328951;
                                }
                            }
                            5512075495849833569 => {
                                c = CHAR_RANGE_d as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17801949008645005255 => {
                                if inclass == 0 && (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    current_block = 8834769789432328951;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            8834769789432328951 => {}
                            _ => {
                                match current_block {
                                    8572234933149657763 => {
                                        if cr.is_null() {
                                            current_block = 11173639622526169465;
                                        } else {
                                            if cr_init_char_range(s, cr, c) != 0 {
                                                return -(1 as i32);
                                            }
                                            c = (c as u32)
                                                .wrapping_add(CLASS_RANGE_BASE as u32)
                                                as uint32_t as uint32_t;
                                            current_block = 8834769789432328951;
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    8834769789432328951 => {}
                                    _ => {
                                        match current_block {
                                            11173639622526169465 => {
                                                p = p.offset(-1);
                                                ret = lre_parse_escape(
                                                    &mut p,
                                                    (*s).is_unicode as i32 * 2 as i32,
                                                );
                                                if ret >= 0 as i32 {
                                                    c = ret as uint32_t;
                                                    current_block = 8834769789432328951;
                                                } else if (*s).is_unicode != 0 {
                                                    current_block = 7227556340696114278;
                                                } else {
                                                    current_block = 16251750946745332477;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            16251750946745332477 => {}
                                            8834769789432328951 => {}
                                            _ => {
                                                return re_parse_error(
            &mut *s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    113 => {
                        current_block = 11152951584060771545;
                        match current_block {
                            11152951584060771545 => {
                                if (*s).unicode_sets != 0 && !cr.is_null() && inclass != 0 {
                                    if parse_class_string_disjunction(s, cr, &mut p) != 0 {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7338405858205321868 => {
                                if (*s).is_unicode != 0 && !cr.is_null() {
                                    if parse_unicode_property(
                                        s,
                                        cr,
                                        &mut p,
                                        (c == 'P' as i32 as uint32_t) as i32,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7105743787279032682 => {
                                c = *p as uint32_t;
                                if c >= 'a' as i32 as uint32_t
                                    && c <= 'z' as i32 as uint32_t
                                    || c >= 'A' as i32 as uint32_t
                                        && c <= 'Z' as i32 as uint32_t
                                    || (c >= '0' as i32 as uint32_t
                                        && c <= '9' as i32 as uint32_t
                                        || c == '_' as i32 as uint32_t) && inclass != 0
                                        && (*s).is_unicode == 0
                                {
                                    c = (c as u32 & 0x1f as u32)
                                        as uint32_t;
                                    p = p.offset(1);
                                    current_block = 8834769789432328951;
                                } else if (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    p = p.offset(-1);
                                    c = '\\' as i32 as uint32_t;
                                    current_block = 8834769789432328951;
                                }
                            }
                            5512075495849833569 => {
                                c = CHAR_RANGE_d as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17801949008645005255 => {
                                if inclass == 0 && (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    current_block = 8834769789432328951;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            8834769789432328951 => {}
                            _ => {
                                match current_block {
                                    8572234933149657763 => {
                                        if cr.is_null() {
                                            current_block = 11173639622526169465;
                                        } else {
                                            if cr_init_char_range(s, cr, c) != 0 {
                                                return -(1 as i32);
                                            }
                                            c = (c as u32)
                                                .wrapping_add(CLASS_RANGE_BASE as u32)
                                                as uint32_t as uint32_t;
                                            current_block = 8834769789432328951;
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    8834769789432328951 => {}
                                    _ => {
                                        match current_block {
                                            11173639622526169465 => {
                                                p = p.offset(-1);
                                                ret = lre_parse_escape(
                                                    &mut p,
                                                    (*s).is_unicode as i32 * 2 as i32,
                                                );
                                                if ret >= 0 as i32 {
                                                    c = ret as uint32_t;
                                                    current_block = 8834769789432328951;
                                                } else if (*s).is_unicode != 0 {
                                                    current_block = 7227556340696114278;
                                                } else {
                                                    current_block = 16251750946745332477;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            16251750946745332477 => {}
                                            8834769789432328951 => {}
                                            _ => {
                                                return re_parse_error(
            &mut *s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        current_block = 11173639622526169465;
                        match current_block {
                            11152951584060771545 => {
                                if (*s).unicode_sets != 0 && !cr.is_null() && inclass != 0 {
                                    if parse_class_string_disjunction(s, cr, &mut p) != 0 {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7338405858205321868 => {
                                if (*s).is_unicode != 0 && !cr.is_null() {
                                    if parse_unicode_property(
                                        s,
                                        cr,
                                        &mut p,
                                        (c == 'P' as i32 as uint32_t) as i32,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as i32);
                                    }
                                    c = CLASS_RANGE_BASE as uint32_t;
                                    current_block = 8834769789432328951;
                                } else {
                                    current_block = 11173639622526169465;
                                }
                            }
                            7105743787279032682 => {
                                c = *p as uint32_t;
                                if c >= 'a' as i32 as uint32_t
                                    && c <= 'z' as i32 as uint32_t
                                    || c >= 'A' as i32 as uint32_t
                                        && c <= 'Z' as i32 as uint32_t
                                    || (c >= '0' as i32 as uint32_t
                                        && c <= '9' as i32 as uint32_t
                                        || c == '_' as i32 as uint32_t) && inclass != 0
                                        && (*s).is_unicode == 0
                                {
                                    c = (c as u32 & 0x1f as u32)
                                        as uint32_t;
                                    p = p.offset(1);
                                    current_block = 8834769789432328951;
                                } else if (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    p = p.offset(-1);
                                    c = '\\' as i32 as uint32_t;
                                    current_block = 8834769789432328951;
                                }
                            }
                            5512075495849833569 => {
                                c = CHAR_RANGE_d as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as i32 as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17801949008645005255 => {
                                if inclass == 0 && (*s).is_unicode != 0 {
                                    current_block = 7227556340696114278;
                                } else {
                                    current_block = 8834769789432328951;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            8834769789432328951 => {}
                            _ => {
                                match current_block {
                                    8572234933149657763 => {
                                        if cr.is_null() {
                                            current_block = 11173639622526169465;
                                        } else {
                                            if cr_init_char_range(s, cr, c) != 0 {
                                                return -(1 as i32);
                                            }
                                            c = (c as u32)
                                                .wrapping_add(CLASS_RANGE_BASE as u32)
                                                as uint32_t as uint32_t;
                                            current_block = 8834769789432328951;
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    8834769789432328951 => {}
                                    _ => {
                                        match current_block {
                                            11173639622526169465 => {
                                                p = p.offset(-1);
                                                ret = lre_parse_escape(
                                                    &mut p,
                                                    (*s).is_unicode as i32 * 2 as i32,
                                                );
                                                if ret >= 0 as i32 {
                                                    c = ret as uint32_t;
                                                    current_block = 8834769789432328951;
                                                } else if (*s).is_unicode != 0 {
                                                    current_block = 7227556340696114278;
                                                } else {
                                                    current_block = 16251750946745332477;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            16251750946745332477 => {}
                                            8834769789432328951 => {}
                                            _ => {
                                                return re_parse_error(
            &mut *s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        0 => {
            if p >= (*s).buf_end {
                current_block = 13671070048700312155;
            } else {
                current_block = 16251750946745332477;
            }
        }
        38 | 33 | 35 | 36 | 37 | 42 | 43 | 44 | 46 | 58 | 59 | 60 | 61 | 62 | 63 | 64
        | 94 | 96 | 126 => {
            if (*s).unicode_sets != 0
                && *p.offset(1 as i32 as isize) as uint32_t == c
            {
                return re_parse_error(
            &mut *s,
                    b"invalid class set operation in regular expression\0" as *const u8
                        as *const i8,
                );
            }
            current_block = 16251750946745332477;
        }
        40 | 41 | 91 | 93 | 123 | 125 | 47 | 45 | 124 => {
            if (*s).unicode_sets != 0 {
                return re_parse_error(
            &mut *s,
                    b"invalid character in class in regular expression\0" as *const u8
                        as *const i8,
                );
            }
            current_block = 16251750946745332477;
        }
        _ => {
            current_block = 16251750946745332477;
        }
    }
    match current_block {
        16251750946745332477 => {
            if c >= 128 as uint32_t {
                // Use safe UTF-8 decoding
                let utf8_slice = std::slice::from_raw_parts(p, UTF8_CHAR_LEN_MAX as usize);
                if let Some((codepoint, bytes)) = unicode_from_utf8(utf8_slice) {
                    c = codepoint;
                    p = p.offset(bytes as isize);
                } else {
                    return re_parse_error(
            &mut *s,
                        b"malformed unicode char\0" as *const u8
                            as *const i8,
                    );
                }
                if c as u32 > 0xffff as u32
                    && (*s).is_unicode == 0
                {
                    return re_parse_error(
            &mut *s,
                        b"malformed unicode char\0" as *const u8
                            as *const i8,
                    );
                }
            } else {
                p = p.offset(1);
            }
        }
        13671070048700312155 => {
            return re_parse_error(
            &mut *s,
                b"unexpected end\0" as *const u8 as *const i8,
            );
        }
        _ => {}
    }
    *pp = p;
    return c as i32;
    } // close unsafe block
}
fn re_emit_range(s: &mut REParseState, cr: &CharRange) -> i32 {
    let len = (cr.len as u32 / 2) as i32;
    if len >= 65535 {
        return re_parse_error(s, b"too many ranges\0" as *const u8 as *const i8);
    }
    if len == 0 {
        re_emit_op_u32(s, REOP_char32 as i32, (-1i32) as u32);
    } else {
        // SAFETY: cr.points is valid for cr.len elements
        unsafe {
            let mut high = *cr.points.offset((cr.len - 1) as isize);
            if high == u32::MAX {
                high = *cr.points.offset((cr.len - 2) as isize);
            }
            if high <= 0xffff {
                re_emit_op_u16(
                    s,
                    if s.ignore_case != 0 { REOP_range_i as i32 } else { REOP_range as i32 },
                    len as u32,
                );
                let mut i = 0;
                while i < cr.len {
                    bb_put_u16(&mut s.byte_code, *cr.points.offset(i as isize) as u16);
                    let mut h = (*cr.points.offset((i + 1) as isize)).wrapping_sub(1);
                    if h == u32::MAX - 1 {
                        h = 0xffff;
                    }
                    bb_put_u16(&mut s.byte_code, h as u16);
                    i += 2;
                }
            } else {
                re_emit_op_u16(
                    s,
                    if s.ignore_case != 0 { REOP_range32_i as i32 } else { REOP_range32 as i32 },
                    len as u32,
                );
                let mut i = 0;
                while i < cr.len {
                    bb_put_u32(&mut s.byte_code, *cr.points.offset(i as isize));
                    bb_put_u32(
                        &mut s.byte_code,
                        (*cr.points.offset((i + 1) as isize)).wrapping_sub(1),
                    );
                    i += 2;
                }
            }
        }
    }
    0
}
fn re_emit_char(s: &mut REParseState, c: i32) {
    if c <= 0xffff {
        re_emit_op_u16(
            s,
            if s.ignore_case != 0 {
                REOP_char_i as i32
            } else {
                REOP_char as i32
            },
            c as u32,
        );
    } else {
        re_emit_op_u32(
            s,
            if s.ignore_case != 0 {
                REOP_char32_i as i32
            } else {
                REOP_char32 as i32
            },
            c as u32,
        );
    }
}
fn re_emit_string_list(
    s: *mut REParseState,
    sl: *const REStringList,
) -> i32 {
    // SAFETY: s and sl are valid pointers from the parser
    unsafe {
    let mut tab: *mut *mut REString = 0 as *mut *mut REString;
    let mut p: *mut REString = 0 as *mut REString;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut split_pos: i32 = 0;
    let mut last_match_pos: i32 = 0;
    let mut n: i32 = 0;
    let mut has_empty_string: BOOL = 0;
    let mut is_last: BOOL = 0;
    if (*sl).n_strings == 0 as uint32_t {
        if re_emit_range(&mut *s, &(*sl).cr) != 0 {
            return -(1 as i32);
        }
    } else {
        tab = lre_realloc(
            (*s).opaque,
            NULL,
            (::core::mem::size_of::<*mut REString>() as size_t)
                .wrapping_mul((*sl).n_strings as size_t),
        ) as *mut *mut REString;
        if tab.is_null() {
            re_parse_out_of_memory(&mut *s);
            return -(1 as i32);
        }
        has_empty_string = FALSE as i32 as BOOL;
        n = 0 as i32;
        i = 0 as i32;
        while (i as uint32_t) < (*sl).hash_size {
            p = *((*sl).hash_table).offset(i as isize);
            while !p.is_null() {
                if (*p).len == 0 as uint32_t {
                    has_empty_string = TRUE as i32 as BOOL;
                } else {
                    let fresh23 = n;
                    n = n + 1;
                    let ref mut fresh24 = *tab.offset(fresh23 as isize);
                    *fresh24 = p;
                }
                p = (*p).next as *mut REString;
            }
            i += 1;
        }
        assert!(n as u32 <= (*sl).n_strings, "n <= sl->n_strings");
        // Sort by length in reverse order (longest first) using Rust's slice sort
        {
            let tab_slice = std::slice::from_raw_parts_mut(tab, n as usize);
            tab_slice.sort_by(|a, b| (**b).len.cmp(&(**a).len));
        }
        last_match_pos = -(1 as i32);
        i = 0 as i32;
        while i < n {
            p = *tab.offset(i as isize);
            is_last = (has_empty_string == 0 && (*sl).cr.len == 0 as i32
                && i == n - 1 as i32) as i32 as BOOL;
            if is_last == 0 {
                split_pos = re_emit_op_u32(
                    &mut *s,
                    REOP_split_next_first as i32,
                    0 as uint32_t,
                );
            } else {
                split_pos = 0 as i32;
            }
            j = 0 as i32;
            while (j as uint32_t) < (*p).len {
                re_emit_char(
                    &mut *s,
                    *((*p).buf).as_mut_ptr().offset(j as isize) as i32,
                );
                j += 1;
            }
            if is_last == 0 {
                last_match_pos = re_emit_op_u32(
                    &mut *s,
                    REOP_goto as i32,
                    last_match_pos as uint32_t,
                );
                (*s).byte_code.set_u32_at(
                    split_pos as usize,
                    ((*s).byte_code.len())
                        .wrapping_sub((split_pos + 4) as usize) as u32,
                );
            }
            i += 1;
        }
        if (*sl).cr.len != 0 as i32 {
            is_last = (has_empty_string == 0) as i32 as BOOL;
            if is_last == 0 {
                split_pos = re_emit_op_u32(
                    &mut *s,
                    REOP_split_next_first as i32,
                    0 as uint32_t,
                );
            } else {
                split_pos = 0 as i32;
            }
            if re_emit_range(&mut *s, &(*sl).cr) != 0 {
                lre_realloc((*s).opaque, tab as *mut std::ffi::c_void, 0 as size_t);
                return -(1 as i32);
            }
            if is_last == 0 {
                (*s).byte_code.set_u32_at(
                    split_pos as usize,
                    ((*s).byte_code.len())
                        .wrapping_sub((split_pos + 4) as usize) as u32,
                );
            }
        }
        while last_match_pos != -(1 as i32) {
            let mut next_pos: i32 = (*s).byte_code.get_u32_at(last_match_pos as usize) as i32;
            (*s).byte_code.set_u32_at(
                last_match_pos as usize,
                ((*s).byte_code.len())
                    .wrapping_sub((last_match_pos + 4) as usize) as u32,
            );
            last_match_pos = next_pos;
        }
        lre_realloc((*s).opaque, tab as *mut std::ffi::c_void, 0 as size_t);
    }
    return 0 as i32;
    } // close unsafe block
}
fn re_parse_class_set_operand(
    s: *mut REParseState,
    cr: *mut REStringList,
    pp: *mut *const uint8_t,
) -> i32 {
    // SAFETY: s, cr, pp are valid pointers from the parser
    unsafe {
        let mut c1: i32 = 0;
        let mut p: *const uint8_t = *pp;
        if *p as i32 == '[' as i32 {
            if re_parse_nested_class(s, cr, pp) != 0 {
                return -(1 as i32);
            }
        } else {
            c1 = get_class_atom(s, cr, pp, TRUE as i32 as BOOL);
            if c1 < 0 as i32 {
                return -(1 as i32);
            }
            if c1 < CLASS_RANGE_BASE {
                re_string_list_init(s, cr);
                if (*s).ignore_case != 0 {
                    c1 = lre_canonicalize(
                        c1 as uint32_t,
                        (*s).is_unicode as i32,
                    );
                }
                if cr_union_interval(&mut (*cr).cr, c1 as uint32_t, c1 as uint32_t) != 0 {
                    re_string_list_free(cr);
                    return -(1 as i32);
                }
            }
        }
        return 0 as i32;
    }
}
fn re_parse_nested_class(
    s: *mut REParseState,
    cr: *mut REStringList,
    pp: *mut *const uint8_t,
) -> i32 {
    // SAFETY: s, cr, pp are valid pointers from the parser
    unsafe {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut c1: uint32_t = 0;
    let mut c2: uint32_t = 0;
    let mut ret: i32 = 0;
    let mut cr1_s: REStringList = REStringList {
        cr: CharRange {
            len: 0,
            size: 0,
            points: 0 as *mut uint32_t,
            mem_opaque: 0 as *mut std::ffi::c_void,
            realloc_func: None,
        },
        n_strings: 0,
        hash_size: 0,
        hash_bits: 0,
        hash_table: 0 as *mut *mut REString,
    };
    let mut cr1: *mut REStringList = &mut cr1_s;
    let mut invert: BOOL = 0;
    let mut is_first: BOOL = 0;
    if lre_check_stack_overflow((*s).opaque, 0 as size_t) != 0 {
        return re_parse_error(
            &mut *s,
            b"stack overflow\0" as *const u8 as *const i8,
        );
    }
    re_string_list_init(s, cr);
    p = *pp;
    p = p.offset(1);
    invert = FALSE as i32 as BOOL;
    if *p as i32 == '^' as i32 {
        p = p.offset(1);
        invert = TRUE as i32 as BOOL;
    }
    is_first = TRUE as i32 as BOOL;
    's_53: loop {
        if *p as i32 == ']' as i32 {
            current_block = 16029476503615101993;
            break;
        }
        if *p as i32 == '[' as i32 && (*s).unicode_sets != 0 {
            if re_parse_nested_class(s, cr1, &mut p) != 0 {
                current_block = 16066372098344664684;
                break;
            }
            current_block = 12477921065514496907;
        } else {
            c1 = get_class_atom(s, cr1, &mut p, TRUE as i32 as BOOL)
                as uint32_t;
            if (c1 as i32) < 0 as i32 {
                current_block = 16066372098344664684;
                break;
            }
            if *p as i32 == '-' as i32
                && *p.offset(1 as i32 as isize) as i32
                    != ']' as i32
            {
                let mut p0: *const uint8_t = p.offset(1 as i32 as isize);
                if *p.offset(1 as i32 as isize) as i32
                    == '-' as i32 && (*s).unicode_sets != 0 && is_first != 0
                {
                    current_block = 5845765319767473285;
                } else {
                    if c1 >= CLASS_RANGE_BASE as uint32_t {
                        if (*s).is_unicode != 0 {
                            re_string_list_free(cr1);
                            current_block = 6364660570595913652;
                        } else {
                            current_block = 5845765319767473285;
                        }
                    } else {
                        c2 = get_class_atom(
                            s,
                            cr1,
                            &mut p0,
                            TRUE as i32 as BOOL,
                        ) as uint32_t;
                        if (c2 as i32) < 0 as i32 {
                            current_block = 16066372098344664684;
                            break;
                        }
                        if c2 >= CLASS_RANGE_BASE as uint32_t {
                            re_string_list_free(cr1);
                            if (*s).is_unicode != 0 {
                                current_block = 6364660570595913652;
                            } else {
                                current_block = 5845765319767473285;
                            }
                        } else {
                            p = p0;
                            if c2 < c1 {
                                current_block = 6364660570595913652;
                            } else {
                                if (*s).ignore_case != 0 {
                                    let mut cr2_s: CharRange = CharRange {
                                        len: 0,
                                        size: 0,
                                        points: 0 as *mut uint32_t,
                                        mem_opaque: 0 as *mut std::ffi::c_void,
                                        realloc_func: None,
                                    };
                                    let mut cr2: *mut CharRange = &mut cr2_s;
                                    cr_init(
                                        cr2,
                                        (*s).opaque,
                                        Some(
                                            lre_realloc
                                                as fn(
                                                    *mut std::ffi::c_void,
                                                    *mut std::ffi::c_void,
                                                    size_t,
                                                ) -> *mut std::ffi::c_void,
                                        ),
                                    );
                                    if cr_add_interval(cr2, c1, c2.wrapping_add(1 as uint32_t))
                                        != 0
                                        || cr_regexp_canonicalize(
                                            cr2,
                                            (*s).is_unicode as i32,
                                        ) != 0
                                        || cr_op1(
                                            &mut (*cr).cr,
                                            (*cr2).points,
                                            (*cr2).len,
                                            CR_OP_UNION as i32,
                                        ) != 0
                                    {
                                        cr_free(cr2);
                                        current_block = 1052705618141951789;
                                        break;
                                    } else {
                                        cr_free(cr2);
                                    }
                                } else if cr_union_interval(&mut (*cr).cr, c1, c2) != 0 {
                                    current_block = 1052705618141951789;
                                    break;
                                }
                                is_first = FALSE as i32 as BOOL;
                                current_block = 1434579379687443766;
                            }
                        }
                    }
                    match current_block {
                        1434579379687443766 => {}
                        5845765319767473285 => {}
                        _ => {
                            re_parse_error(
            &mut *s,
                                b"invalid class range\0" as *const u8
                                    as *const i8,
                            );
                            current_block = 16066372098344664684;
                            break;
                        }
                    }
                }
            } else {
                current_block = 5845765319767473285;
            }
            match current_block {
                1434579379687443766 => {}
                _ => {
                    if c1 >= CLASS_RANGE_BASE as uint32_t {
                        current_block = 12477921065514496907;
                    } else {
                        if (*s).ignore_case != 0 {
                            c1 = lre_canonicalize(
                                c1,
                                (*s).is_unicode as i32,
                            ) as uint32_t;
                        }
                        if cr_union_interval(&mut (*cr).cr, c1, c1) != 0 {
                            current_block = 1052705618141951789;
                            break;
                        }
                        current_block = 1434579379687443766;
                    }
                }
            }
        }
        match current_block {
            12477921065514496907 => {
                ret = re_string_list_op(cr, cr1, CR_OP_UNION as i32);
                re_string_list_free(cr1);
                if ret != 0 {
                    current_block = 1052705618141951789;
                    break;
                }
            }
            _ => {}
        }
        if (*s).unicode_sets != 0 && is_first != 0 {
            if *p as i32 == '&' as i32
                && *p.offset(1 as i32 as isize) as i32
                    == '&' as i32
                && *p.offset(2 as i32 as isize) as i32
                    != '&' as i32
            {
                loop {
                    if *p as i32 == ']' as i32 {
                        current_block = 5684854171168229155;
                        break;
                    }
                    if !(*p as i32 == '&' as i32
                        && *p.offset(1 as i32 as isize) as i32
                            == '&' as i32
                        && *p.offset(2 as i32 as isize) as i32
                            != '&' as i32)
                    {
                        current_block = 10490727108771021976;
                        break;
                    }
                    p = p.offset(2 as i32 as isize);
                    if re_parse_class_set_operand(s, cr1, &mut p) != 0 {
                        current_block = 16066372098344664684;
                        break 's_53;
                    }
                    ret = re_string_list_op(cr, cr1, CR_OP_INTER as i32);
                    re_string_list_free(cr1);
                    if ret != 0 {
                        current_block = 1052705618141951789;
                        break 's_53;
                    }
                }
            } else if *p as i32 == '-' as i32
                && *p.offset(1 as i32 as isize) as i32
                    == '-' as i32
            {
                loop {
                    if *p as i32 == ']' as i32 {
                        current_block = 5684854171168229155;
                        break;
                    }
                    if !(*p as i32 == '-' as i32
                        && *p.offset(1 as i32 as isize) as i32
                            == '-' as i32)
                    {
                        current_block = 10490727108771021976;
                        break;
                    }
                    p = p.offset(2 as i32 as isize);
                    if re_parse_class_set_operand(s, cr1, &mut p) != 0 {
                        current_block = 16066372098344664684;
                        break 's_53;
                    }
                    ret = re_string_list_op(cr, cr1, CR_OP_SUB as i32);
                    re_string_list_free(cr1);
                    if ret != 0 {
                        current_block = 1052705618141951789;
                        break 's_53;
                    }
                }
            } else {
                current_block = 5684854171168229155;
            }
            match current_block {
                5684854171168229155 => {}
                _ => {
                    re_parse_error(
            &mut *s,
                        b"invalid operation in regular expression\0" as *const u8
                            as *const i8,
                    );
                    current_block = 16066372098344664684;
                    break;
                }
            }
        }
        is_first = FALSE as i32 as BOOL;
    }
    match current_block {
        16029476503615101993 => {
            p = p.offset(1);
            *pp = p;
            if invert != 0 {
                if (*cr).n_strings != 0 as uint32_t {
                    re_parse_error(
            &mut *s,
                        b"negated character class with strings in regular expression debugger eval code\0"
                            as *const u8 as *const i8,
                    );
                    current_block = 16066372098344664684;
                } else if cr_invert(&mut (*cr).cr) != 0 {
                    current_block = 1052705618141951789;
                } else {
                    current_block = 1352918242886884122;
                }
            } else {
                current_block = 1352918242886884122;
            }
            match current_block {
                1052705618141951789 => {}
                16066372098344664684 => {}
                _ => return 0 as i32,
            }
        }
        _ => {}
    }
    match current_block {
        1052705618141951789 => {
            re_parse_out_of_memory(&mut *s);
        }
        _ => {}
    }
    re_string_list_free(cr);
    return -(1 as i32);
    } // close unsafe block
}
fn re_parse_char_class(
    s: *mut REParseState,
    pp: *mut *const uint8_t,
) -> i32 {
    let mut cr_s: REStringList = REStringList {
        cr: CharRange {
            len: 0,
            size: 0,
            points: 0 as *mut uint32_t,
            mem_opaque: 0 as *mut std::ffi::c_void,
            realloc_func: None,
        },
        n_strings: 0,
        hash_size: 0,
        hash_bits: 0,
        hash_table: 0 as *mut *mut REString,
    };
    let cr: *mut REStringList = &mut cr_s;
    if re_parse_nested_class(s, cr, pp) != 0 {
        return -(1 as i32);
    }
    if re_emit_string_list(s, cr) != 0 {
        re_string_list_free(cr);
        return -(1 as i32);
    } else {
        re_string_list_free(cr);
        return 0 as i32;
    };
}
/// Checks if bytecode needs advance check and capture init.
/// Returns (need_check_adv, need_capture_init).
fn re_need_check_adv_and_capture_init(bc_buf: &[u8]) -> (bool, bool) {
    let mut pos: usize = 0;
    let mut need_check_adv = true;
    let mut need_capture_init = false;

    while pos < bc_buf.len() {
        let opcode = bc_buf[pos] as i32;
        let mut len = reopcode_info[opcode as usize].size as usize;
        match opcode {
            36 | 37 => {
                // REOP_range, REOP_range_i
                let val = get_u16_safe(&bc_buf[pos + 1..]) as usize;
                len += val * 4;
                need_check_adv = false;
            }
            38 | 39 => {
                // REOP_range32, REOP_range32_i
                let val = get_u16_safe(&bc_buf[pos + 1..]) as usize;
                len += val * 8;
                need_check_adv = false;
            }
            1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                need_check_adv = false;
            }
            9 | 10 | 11 | 12 | 27 | 42 | 28 | 29 | 30 | 31 | 44 | 19 | 20 | 21 => {}
            32 | 33 | 34 | 35 => {
                // Back reference opcodes
                let val = bc_buf[pos + 1] as usize;
                len += val;
                need_capture_init = true;
            }
            _ => {
                need_capture_init = true;
                break;
            }
        }
        pos += len;
    }
    (need_check_adv, need_capture_init)
}
fn re_parse_group_name(
    buf: *mut i8,
    buf_size: i32,
    pp: *mut *const uint8_t,
) -> i32 {
    // SAFETY: buf, pp are valid pointers from the parser
    unsafe {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut p1: *const uint8_t = 0 as *const uint8_t;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut q: *mut i8 = 0 as *mut i8;
    p = *pp;
    q = buf;
    loop {
        c = *p as uint32_t;
        if c == '\\' as i32 as uint32_t {
            p = p.offset(1);
            if *p as i32 != 'u' as i32 {
                return -(1 as i32);
            }
            c = lre_parse_escape(&mut p, 2 as i32) as uint32_t;
        } else {
            if c == '>' as i32 as uint32_t {
                break;
            }
            if c >= 128 as uint32_t {
                // Use safe UTF-8 decoding
                let utf8_slice = std::slice::from_raw_parts(p, UTF8_CHAR_LEN_MAX as usize);
                if let Some((codepoint, bytes)) = unicode_from_utf8(utf8_slice) {
                    c = codepoint;
                    p = p.offset(bytes as isize);
                } else {
                    c = u32::MAX; // Will be caught by c > 0x10ffff check below
                }
                if is_hi_surrogate(c) != 0 {
                    let utf8_slice1 = std::slice::from_raw_parts(p, UTF8_CHAR_LEN_MAX as usize);
                    if let Some((codepoint, bytes)) = unicode_from_utf8(utf8_slice1) {
                        d = codepoint;
                        p1 = p.offset(bytes as isize);
                    } else {
                        d = u32::MAX;
                    }
                    if is_lo_surrogate(d) != 0 {
                        c = from_surrogate(c, d);
                        p = p1;
                    }
                }
            } else {
                p = p.offset(1);
            }
        }
        if c > 0x10ffff as uint32_t {
            return -(1 as i32);
        }
        if q == buf {
            if lre_js_is_ident_first(c) == 0 {
                return -(1 as i32);
            }
        } else if lre_js_is_ident_next(c) == 0 {
            return -(1 as i32)
        }
        if q.offset_from(buf) as i64
            + UTF8_CHAR_LEN_MAX as i64 + 1 as i64
            > buf_size as i64
        {
            return -(1 as i32);
        }
        if c < 128 as uint32_t {
            let fresh41 = q;
            q = q.offset(1);
            *fresh41 = c as i8;
        } else {
            // Use safe UTF-8 encoding
            let utf8_slice = std::slice::from_raw_parts_mut(q as *mut u8, 6);
            let bytes_written = unicode_to_utf8(utf8_slice, c);
            q = q.offset(bytes_written as isize);
        }
    }
    if q == buf {
        return -(1 as i32);
    }
    *q = '\0' as i32 as i8;
    p = p.offset(1);
    *pp = p;
    return 0 as i32;
    } // close unsafe block
}
/// Parses capture groups in the pattern.
fn re_parse_captures(
    s: *mut REParseState,
    phas_named_captures: *mut i32,
    capture_name: *const i8,
    emit_group_index: BOOL,
) -> i32 {
    // SAFETY: all pointers are valid from the parser
    unsafe {
    let mut p: *const uint8_t = (*s).buf_start;
    let mut capture_index: i32 = 1;
    let mut n: i32 = 0;
    let mut name: [i8; 128] = [0; 128];
    *phas_named_captures = 0;

    while p < (*s).buf_end {
        match *p as i32 {
            40 => { // '('
                if *p.offset(1) as i32 == '?' as i32 {
                    if *p.offset(2) as i32 == '<' as i32
                        && *p.offset(3) as i32 != '=' as i32
                        && *p.offset(3) as i32 != '!' as i32
                    {
                        *phas_named_captures = 1;
                        if !capture_name.is_null() {
                            p = p.offset(3);
                            if re_parse_group_name(
                                name.as_mut_ptr(),
                                std::mem::size_of::<[i8; 128]>() as i32,
                                &mut p,
                            ) == 0
                            {
                                if strcmp(name.as_mut_ptr(), capture_name) == 0 {
                                    if emit_group_index != 0 {
                                        bb_putc(&mut (*s).byte_code, capture_index as u8);
                                    }
                                    n += 1;
                                }
                            }
                        }
                        capture_index += 1;
                        if capture_index >= CAPTURE_COUNT_MAX {
                            break;
                        }
                    }
                } else {
                    capture_index += 1;
                    if capture_index >= CAPTURE_COUNT_MAX {
                        break;
                    }
                }
            }
            92 => { // '\\'
                p = p.offset(1);
            }
            91 => { // '['
                p = p.offset(1 + (*p as i32 == ']' as i32) as isize);
                while p < (*s).buf_end && *p as i32 != ']' as i32 {
                    if *p as i32 == '\\' as i32 {
                        p = p.offset(1);
                    }
                    p = p.offset(1);
                }
            }
            _ => {}
        }
        p = p.offset(1);
    }
    if !capture_name.is_null() { n } else { capture_index }
    } // close unsafe block
}
/// Counts total capture groups in the pattern.
/// # Safety
/// s must be a valid REParseState.
fn re_count_captures(s: &mut REParseState) -> i32 {
    if s.total_capture_count < 0 {
        s.total_capture_count = re_parse_captures(
            s as *mut REParseState,
            &mut s.has_named_captures,
            std::ptr::null(),
            FALSE as i32 as BOOL,
        );
    }
    s.total_capture_count
}

/// Checks if the pattern has named capture groups.
fn re_has_named_captures(s: &mut REParseState) -> BOOL {
    if s.has_named_captures < 0 {
        re_count_captures(s);
    }
    s.has_named_captures as BOOL
}
/// Finds group name(s) and optionally emits their indices.
fn find_group_name(s: &mut REParseState, name: *const i8, emit_group_index: BOOL) -> i32 {
    // SAFETY: group_names buffer is valid, name is a valid C string from parser
    unsafe {
        let mut p: *const i8 = s.group_names.as_mut_ptr() as *mut i8;
        if p.is_null() {
            return 0;
        }
        let buf_end = (s.group_names.as_mut_ptr() as *mut i8).offset(s.group_names.len() as isize);
        let name_len = strlen(name);
        let mut capture_index: i32 = 1;
        let mut n: i32 = 0;
        while p < buf_end {
            let len = strlen(p);
            if len == name_len
                && memcmp(name as *const std::ffi::c_void, p as *const std::ffi::c_void, name_len) == 0
            {
                if emit_group_index != 0 {
                    bb_putc(&mut s.byte_code, capture_index as u8);
                }
                n += 1;
            }
            p = p.offset(len.wrapping_add(LRE_GROUP_NAME_TRAILER_LEN as size_t) as isize);
            capture_index += 1;
        }
        n
    }
}
/// Checks if a group name already exists in the same scope.
fn is_duplicate_group_name(s: &mut REParseState, name: *const i8, scope: i32) -> BOOL {
    // SAFETY: group_names buffer is valid, name is a valid C string from parser
    unsafe {
        let mut p: *const i8 = s.group_names.as_mut_ptr() as *mut i8;
        if p.is_null() {
            return FALSE as i32 as BOOL;
        }
        let buf_end = (s.group_names.as_mut_ptr() as *mut i8).offset(s.group_names.len() as isize);
        let name_len = strlen(name);
        while p < buf_end {
            let len = strlen(p);
            if len == name_len
                && memcmp(name as *const std::ffi::c_void, p as *const std::ffi::c_void, name_len) == 0
            {
                let scope1 = *p.offset(len.wrapping_add(1) as isize) as u8 as i32;
                if scope == scope1 {
                    return TRUE as i32 as BOOL;
                }
            }
            p = p.offset(len.wrapping_add(LRE_GROUP_NAME_TRAILER_LEN as size_t) as isize);
        }
        FALSE as i32 as BOOL
    }
}
/// Parses modifier flags (i, m, s).
fn re_parse_modifiers(s: &mut REParseState, pp: *mut *const uint8_t) -> i32 {
    // SAFETY: pp points to a valid pointer that is within the input buffer
    unsafe {
        let mut p: *const uint8_t = *pp;
        let mut mask: i32 = 0;
        loop {
            let val = if *p as i32 == 'i' as i32 {
                LRE_FLAG_IGNORECASE
            } else if *p as i32 == 'm' as i32 {
                LRE_FLAG_MULTILINE
            } else if *p as i32 == 's' as i32 {
                LRE_FLAG_DOTALL
            } else {
                break;
            };
            if mask & val != 0 {
                return re_parse_error(s, b"duplicate modifier\0" as *const u8 as *const i8);
            }
            mask |= val;
            p = p.offset(1);
        }
        *pp = p;
        mask
    }
}
/// Updates a modifier flag based on add/remove masks.
#[inline]
fn update_modifier(val: BOOL, add_mask: i32, remove_mask: i32, mask: i32) -> BOOL {
    if remove_mask & mask != 0 {
        FALSE as BOOL
    } else if add_mask & mask != 0 {
        TRUE as BOOL
    } else {
        val
    }
}
fn re_parse_term(
    s: *mut REParseState,
    is_backward_dir: BOOL,
) -> i32 {
    // SAFETY: s is a valid REParseState pointer from the parser
    unsafe {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut c: i32 = 0;
    let mut last_atom_start: i32 = 0;
    let mut quant_min: i32 = 0;
    let mut quant_max: i32 = 0;
    let mut last_capture_count: i32 = 0;
    let mut greedy: BOOL = 0;
    let mut is_neg: BOOL = 0;
    let mut is_backward_lookahead: BOOL = 0;
    let mut cr_s: REStringList = REStringList {
        cr: CharRange {
            len: 0,
            size: 0,
            points: 0 as *mut uint32_t,
            mem_opaque: 0 as *mut std::ffi::c_void,
            realloc_func: None,
        },
        n_strings: 0,
        hash_size: 0,
        hash_bits: 0,
        hash_table: 0 as *mut *mut REString,
    };
    let mut cr: *mut REStringList = &mut cr_s;
    last_atom_start = -(1 as i32);
    last_capture_count = 0 as i32;
    p = (*s).buf_ptr;
    c = *p as i32;
    match c {
        94 => {
            p = p.offset(1);
            re_emit_op(
                &mut *s,
                if (*s).multi_line != 0 {
                    REOP_line_start_m as i32
                } else {
                    REOP_line_start as i32
                },
            );
            current_block = 1771738965274008886;
        }
        36 => {
            p = p.offset(1);
            re_emit_op(
                &mut *s,
                if (*s).multi_line != 0 {
                    REOP_line_end_m as i32
                } else {
                    REOP_line_end as i32
                },
            );
            current_block = 1771738965274008886;
        }
        46 => {
            p = p.offset(1);
            last_atom_start = (*s).byte_code.len() as i32;
            last_capture_count = (*s).capture_count;
            if is_backward_dir != 0 {
                re_emit_op(&mut *s, REOP_prev as i32);
            }
            re_emit_op(
                &mut *s,
                if (*s).dotall != 0 {
                    REOP_any as i32
                } else {
                    REOP_dot as i32
                },
            );
            if is_backward_dir != 0 {
                re_emit_op(&mut *s, REOP_prev as i32);
            }
            current_block = 1771738965274008886;
        }
        123 => {
            if (*s).is_unicode != 0 {
                return re_parse_error(
            &mut *s,
                    b"syntax error\0" as *const u8 as *const i8,
                )
            } else if is_digit(
                *p.offset(1 as i32 as isize) as i32,
            ) == 0
            {
                current_block = 9143481802853542599;
            } else {
                let mut p1: *const uint8_t = p.offset(1 as i32 as isize);
                parse_digits(&mut p1, TRUE as i32 as BOOL);
                if *p1 as i32 == ',' as i32 {
                    p1 = p1.offset(1);
                    if is_digit(*p1 as i32) != 0 {
                        parse_digits(&mut p1, TRUE as i32 as BOOL);
                    }
                }
                if *p1 as i32 != '}' as i32 {
                    current_block = 9143481802853542599;
                } else {
                    current_block = 18128146392678525522;
                }
            }
        }
        42 | 43 | 63 => {
            current_block = 18128146392678525522;
        }
        40 => {
            let mut pos: i32 = 0;
            let mut capture_index: i32 = 0;
            let mut current_block_118: u64;
            if *p.offset(1 as i32 as isize) as i32
                == '?' as i32
            {
                if *p.offset(2 as i32 as isize) as i32
                    == ':' as i32
                {
                    p = p.offset(3 as i32 as isize);
                    last_atom_start = (*s).byte_code.len() as i32;
                    last_capture_count = (*s).capture_count;
                    (*s).buf_ptr = p;
                    if re_parse_disjunction(s, is_backward_dir) != 0 {
                        return -(1 as i32);
                    }
                    p = (*s).buf_ptr;
                    if re_parse_expect(&mut *s, &mut p, ')' as i32) != 0 {
                        return -(1 as i32);
                    }
                    current_block_118 = 1934991416718554651;
                } else if *p.offset(2 as i32 as isize) as i32
                    == 'i' as i32
                    || *p.offset(2 as i32 as isize) as i32
                        == 'm' as i32
                    || *p.offset(2 as i32 as isize) as i32
                        == 's' as i32
                    || *p.offset(2 as i32 as isize) as i32
                        == '-' as i32
                {
                    let mut saved_ignore_case: BOOL = 0;
                    let mut saved_multi_line: BOOL = 0;
                    let mut saved_dotall: BOOL = 0;
                    let mut add_mask: i32 = 0;
                    let mut remove_mask: i32 = 0;
                    p = p.offset(2 as i32 as isize);
                    remove_mask = 0 as i32;
                    add_mask = re_parse_modifiers(&mut *s, &mut p);
                    if add_mask < 0 as i32 {
                        return -(1 as i32);
                    }
                    if *p as i32 == '-' as i32 {
                        p = p.offset(1);
                        remove_mask = re_parse_modifiers(&mut *s, &mut p);
                        if remove_mask < 0 as i32 {
                            return -(1 as i32);
                        }
                    }
                    if add_mask == 0 as i32
                        && remove_mask == 0 as i32
                        || add_mask & remove_mask != 0 as i32
                    {
                        return re_parse_error(
            &mut *s,
                            b"invalid modifiers\0" as *const u8
                                as *const i8,
                        );
                    }
                    if re_parse_expect(&mut *s, &mut p, ':' as i32) != 0 {
                        return -(1 as i32);
                    }
                    saved_ignore_case = (*s).ignore_case;
                    saved_multi_line = (*s).multi_line;
                    saved_dotall = (*s).dotall;
                    (*s).ignore_case = update_modifier(
                        (*s).ignore_case,
                        add_mask,
                        remove_mask,
                        LRE_FLAG_IGNORECASE,
                    );
                    (*s).multi_line = update_modifier(
                        (*s).multi_line,
                        add_mask,
                        remove_mask,
                        LRE_FLAG_MULTILINE,
                    );
                    (*s).dotall = update_modifier(
                        (*s).dotall,
                        add_mask,
                        remove_mask,
                        LRE_FLAG_DOTALL,
                    );
                    last_atom_start = (*s).byte_code.len() as i32;
                    last_capture_count = (*s).capture_count;
                    (*s).buf_ptr = p;
                    if re_parse_disjunction(s, is_backward_dir) != 0 {
                        return -(1 as i32);
                    }
                    p = (*s).buf_ptr;
                    if re_parse_expect(&mut *s, &mut p, ')' as i32) != 0 {
                        return -(1 as i32);
                    }
                    (*s).ignore_case = saved_ignore_case;
                    (*s).multi_line = saved_multi_line;
                    (*s).dotall = saved_dotall;
                    current_block_118 = 1934991416718554651;
                } else {
                    if *p.offset(2 as i32 as isize) as i32
                        == '=' as i32
                        || *p.offset(2 as i32 as isize) as i32
                            == '!' as i32
                    {
                        is_neg = (*p.offset(2 as i32 as isize)
                            as i32 == '!' as i32) as i32
                            as BOOL;
                        is_backward_lookahead = FALSE as i32 as BOOL;
                        p = p.offset(3 as i32 as isize);
                        current_block_118 = 15996867562260755014;
                    } else if *p.offset(2 as i32 as isize)
                        as i32 == '<' as i32
                        && (*p.offset(3 as i32 as isize) as i32
                            == '=' as i32
                            || *p.offset(3 as i32 as isize)
                                as i32 == '!' as i32)
                    {
                        pos = 0;
                        is_neg = (*p.offset(3 as i32 as isize)
                            as i32 == '!' as i32) as i32
                            as BOOL;
                        is_backward_lookahead = TRUE as i32 as BOOL;
                        p = p.offset(4 as i32 as isize);
                        current_block_118 = 15996867562260755014;
                    } else {
                        if *p.offset(2 as i32 as isize) as i32
                            == '<' as i32
                        {
                            p = p.offset(3 as i32 as isize);
                            if re_parse_group_name(
                                ((*s).u.tmp_buf).as_mut_ptr(),
                                ::core::mem::size_of::<[i8; 128]>()
                                    as i32,
                                &mut p,
                            ) != 0
                            {
                                return re_parse_error(
            &mut *s,
                                    b"invalid group name\0" as *const u8
                                        as *const i8,
                                );
                            }
                            if is_duplicate_group_name(
                                &mut *s,
                                ((*s).u.tmp_buf).as_mut_ptr(),
                                (*s).group_name_scope as i32,
                            ) != 0
                            {
                                return re_parse_error(
            &mut *s,
                                    b"duplicate group name\0" as *const u8
                                        as *const i8,
                                );
                            }
                            {
                                let name_len = strlen(((*s).u.tmp_buf).as_mut_ptr()).wrapping_add(1);
                                let name_slice = std::slice::from_raw_parts(
                                    ((*s).u.tmp_buf).as_ptr() as *const u8,
                                    name_len
                                );
                                bb_put(&mut (*s).group_names, name_slice);
                            }
                            bb_putc(&mut (*s).group_names, (*s).group_name_scope);
                            (*s).has_named_captures = 1 as i32;
                        } else {
                            return re_parse_error(
            &mut *s,
                                b"invalid group\0" as *const u8 as *const i8,
                            )
                        }
                        current_block_118 = 115339115514607209;
                    }
                    match current_block_118 {
                        115339115514607209 => {}
                        _ => {
                            if (*s).is_unicode == 0 && is_backward_lookahead == 0 {
                                last_atom_start = (*s).byte_code.len() as i32;
                                last_capture_count = (*s).capture_count;
                            }
                            pos = re_emit_op_u32(
                                &mut *s,
                                REOP_lookahead as i32
                                    + is_neg as i32,
                                0 as uint32_t,
                            );
                            (*s).buf_ptr = p;
                            if re_parse_disjunction(s, is_backward_lookahead) != 0 {
                                return -(1 as i32);
                            }
                            p = (*s).buf_ptr;
                            if re_parse_expect(&mut *s, &mut p, ')' as i32) != 0 {
                                return -(1 as i32);
                            }
                            re_emit_op(
                                &mut *s,
                                REOP_lookahead_match as i32
                                    + is_neg as i32,
                            );
                            if bb_error(&(*s).byte_code) != 0 {
                                return -(1 as i32);
                            }
                            (*s).byte_code.set_u32_at(
                                pos as usize,
                                ((*s).byte_code.len())
                                    .wrapping_sub((pos + 4) as usize) as u32,
                            );
                            current_block_118 = 1934991416718554651;
                        }
                    }
                }
            } else {
                capture_index = 0;
                p = p.offset(1);
                bb_putc(&mut (*s).group_names, 0 as uint8_t);
                bb_putc(&mut (*s).group_names, 0 as uint8_t);
                current_block_118 = 115339115514607209;
            }
            match current_block_118 {
                115339115514607209 => {
                    if (*s).capture_count >= CAPTURE_COUNT_MAX {
                        return re_parse_error(
            &mut *s,
                            b"too many captures\0" as *const u8
                                as *const i8,
                        );
                    }
                    last_atom_start = (*s).byte_code.len() as i32;
                    last_capture_count = (*s).capture_count;
                    let fresh1 = (*s).capture_count;
                    (*s).capture_count = (*s).capture_count + 1;
                    capture_index = fresh1;
                    re_emit_op_u8(
                        &mut *s,
                        REOP_save_start as i32
                            + is_backward_dir as i32,
                        capture_index as uint32_t,
                    );
                    (*s).buf_ptr = p;
                    if re_parse_disjunction(s, is_backward_dir) != 0 {
                        return -(1 as i32);
                    }
                    p = (*s).buf_ptr;
                    re_emit_op_u8(
                        &mut *s,
                        REOP_save_start as i32 + 1 as i32
                            - is_backward_dir as i32,
                        capture_index as uint32_t,
                    );
                    if re_parse_expect(&mut *s, &mut p, ')' as i32) != 0 {
                        return -(1 as i32);
                    }
                }
                _ => {}
            }
            current_block = 1771738965274008886;
        }
        92 => {
            match *p.offset(1 as i32 as isize) as i32 {
                98 | 66 => {
                    current_block = 8351012430473106477;
                    match current_block {
                        8351012430473106477 => {
                            if *p.offset(1 as i32 as isize)
                                as i32 != 'b' as i32
                            {
                                re_emit_op(
                                    &mut *s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_not_word_boundary_i as i32
                                    } else {
                                        REOP_not_word_boundary as i32
                                    },
                                );
                            } else {
                                re_emit_op(
                                    &mut *s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_word_boundary_i as i32
                                    } else {
                                        REOP_word_boundary as i32
                                    },
                                );
                            }
                            p = p.offset(2 as i32 as isize);
                            current_block = 1771738965274008886;
                        }
                        3824880322238534506 => {
                            p = p.offset(2 as i32 as isize);
                            c = 0 as i32;
                            if (*s).is_unicode != 0 {
                                if is_digit(*p as i32) != 0 {
                                    return re_parse_error(
            &mut *s,
                                        b"invalid decimal escape in regular expression\0"
                                            as *const u8 as *const i8,
                                    );
                                }
                            } else if *p as i32 >= '0' as i32
                                && *p as i32 <= '7' as i32
                            {
                                let fresh2 = p;
                                p = p.offset(1);
                                c = *fresh2 as i32 - '0' as i32;
                                if *p as i32 >= '0' as i32
                                    && *p as i32 <= '7' as i32
                                {
                                    let fresh3 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as i32)
                                        + *fresh3 as i32 - '0' as i32;
                                }
                            }
                            current_block = 4664037234821577316;
                        }
                        5846959088466685742 => {
                            let mut p1_0: *const uint8_t = 0 as *const uint8_t;
                            let mut dummy_res: i32 = 0;
                            let mut n: i32 = 0;
                            let mut is_forward: BOOL = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as i32 as isize)
                                as i32 != '<' as i32
                            {
                                if (*s).is_unicode != 0 || re_has_named_captures(&mut *s) != 0 {
                                    return re_parse_error(
            &mut *s,
                                        b"expecting group name\0" as *const u8
                                            as *const i8,
                                    )
                                } else {
                                    current_block = 9143481802853542599;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as i32 as isize);
                                if re_parse_group_name(
                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                    ::core::mem::size_of::<[i8; 128]>()
                                        as i32,
                                    &mut p1_0,
                                ) != 0
                                {
                                    if (*s).is_unicode != 0 || re_has_named_captures(&mut *s) != 0 {
                                        return re_parse_error(
            &mut *s,
                                            b"invalid group name\0" as *const u8
                                                as *const i8,
                                        )
                                    } else {
                                        current_block = 9143481802853542599;
                                    }
                                } else {
                                    is_forward = FALSE as i32 as BOOL;
                                    n = find_group_name(
                                        &mut *s,
                                        ((*s).u.tmp_buf).as_mut_ptr(),
                                        FALSE as i32 as BOOL,
                                    );
                                    if n == 0 as i32 {
                                        n = re_parse_captures(
                                            &mut *s,
                                            &mut dummy_res,
                                            ((*s).u.tmp_buf).as_mut_ptr(),
                                            FALSE as i32 as BOOL,
                                        );
                                        if n == 0 as i32 {
                                            if (*s).is_unicode != 0 || re_has_named_captures(&mut *s) != 0 {
                                                return re_parse_error(
            &mut *s,
                                                    b"group name not defined\0" as *const u8
                                                        as *const i8,
                                                )
                                            } else {
                                                current_block = 9143481802853542599;
                                            }
                                        } else {
                                            is_forward = TRUE as i32 as BOOL;
                                            current_block = 1069630499025798221;
                                        }
                                    } else {
                                        current_block = 1069630499025798221;
                                    }
                                    match current_block {
                                        9143481802853542599 => {}
                                        _ => {
                                            last_atom_start = (*s).byte_code.len() as i32;
                                            last_capture_count = (*s).capture_count;
                                            re_emit_op_u8(
                                                &mut *s,
                                                REOP_back_reference as i32
                                                    + 2 as i32
                                                        * is_backward_dir as i32
                                                    + (*s).ignore_case as i32,
                                                n as uint32_t,
                                            );
                                            if is_forward != 0 {
                                                re_parse_captures(
                                                    &mut *s,
                                                    &mut dummy_res,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as i32 as BOOL,
                                                );
                                            } else {
                                                find_group_name(
                                        &mut *s,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as i32 as BOOL,
                                                );
                                            }
                                            p = p1_0;
                                            current_block = 1771738965274008886;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            p = p.offset(1);
                            let mut q: *const uint8_t = p;
                            c = parse_digits(&mut p, FALSE as i32 as BOOL);
                            if c < 0 as i32
                                || c >= (*s).capture_count && c >= re_count_captures(&mut *s)
                            {
                                if (*s).is_unicode == 0 {
                                    p = q;
                                    if *p as i32 <= '7' as i32 {
                                        c = 0 as i32;
                                        if *p as i32 <= '3' as i32 {
                                            let fresh4 = p;
                                            p = p.offset(1);
                                            c = *fresh4 as i32 - '0' as i32;
                                        }
                                        if *p as i32 >= '0' as i32
                                            && *p as i32 <= '7' as i32
                                        {
                                            let fresh5 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as i32)
                                                + *fresh5 as i32 - '0' as i32;
                                            if *p as i32 >= '0' as i32
                                                && *p as i32 <= '7' as i32
                                            {
                                                let fresh6 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as i32)
                                                    + *fresh6 as i32 - '0' as i32;
                                            }
                                        }
                                    } else {
                                        let fresh7 = p;
                                        p = p.offset(1);
                                        c = *fresh7 as i32;
                                    }
                                } else {
                                    return re_parse_error(
            &mut *s,
                                        b"back reference out of range in regular expression\0"
                                            as *const u8 as *const i8,
                                    )
                                }
                                current_block = 4664037234821577316;
                            } else {
                                last_atom_start = (*s).byte_code.len() as i32;
                                last_capture_count = (*s).capture_count;
                                re_emit_op_u8(
                                    &mut *s,
                                    REOP_back_reference as i32
                                        + 2 as i32
                                            * is_backward_dir as i32
                                        + (*s).ignore_case as i32,
                                    1 as uint32_t,
                                );
                                bb_putc(&mut (*s).byte_code, c as uint8_t);
                                current_block = 1771738965274008886;
                            }
                        }
                    }
                }
                107 => {
                    current_block = 5846959088466685742;
                    match current_block {
                        8351012430473106477 => {
                            if *p.offset(1 as i32 as isize)
                                as i32 != 'b' as i32
                            {
                                re_emit_op(
                                    &mut *s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_not_word_boundary_i as i32
                                    } else {
                                        REOP_not_word_boundary as i32
                                    },
                                );
                            } else {
                                re_emit_op(
                                    &mut *s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_word_boundary_i as i32
                                    } else {
                                        REOP_word_boundary as i32
                                    },
                                );
                            }
                            p = p.offset(2 as i32 as isize);
                            current_block = 1771738965274008886;
                        }
                        3824880322238534506 => {
                            p = p.offset(2 as i32 as isize);
                            c = 0 as i32;
                            if (*s).is_unicode != 0 {
                                if is_digit(*p as i32) != 0 {
                                    return re_parse_error(
            &mut *s,
                                        b"invalid decimal escape in regular expression\0"
                                            as *const u8 as *const i8,
                                    );
                                }
                            } else if *p as i32 >= '0' as i32
                                && *p as i32 <= '7' as i32
                            {
                                let fresh2 = p;
                                p = p.offset(1);
                                c = *fresh2 as i32 - '0' as i32;
                                if *p as i32 >= '0' as i32
                                    && *p as i32 <= '7' as i32
                                {
                                    let fresh3 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as i32)
                                        + *fresh3 as i32 - '0' as i32;
                                }
                            }
                            current_block = 4664037234821577316;
                        }
                        5846959088466685742 => {
                            let mut p1_0: *const uint8_t = 0 as *const uint8_t;
                            let mut dummy_res: i32 = 0;
                            let mut n: i32 = 0;
                            let mut is_forward: BOOL = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as i32 as isize)
                                as i32 != '<' as i32
                            {
                                if (*s).is_unicode != 0 || re_has_named_captures(&mut *s) != 0 {
                                    return re_parse_error(
            &mut *s,
                                        b"expecting group name\0" as *const u8
                                            as *const i8,
                                    )
                                } else {
                                    current_block = 9143481802853542599;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as i32 as isize);
                                if re_parse_group_name(
                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                    ::core::mem::size_of::<[i8; 128]>()
                                        as i32,
                                    &mut p1_0,
                                ) != 0
                                {
                                    if (*s).is_unicode != 0 || re_has_named_captures(&mut *s) != 0 {
                                        return re_parse_error(
            &mut *s,
                                            b"invalid group name\0" as *const u8
                                                as *const i8,
                                        )
                                    } else {
                                        current_block = 9143481802853542599;
                                    }
                                } else {
                                    is_forward = FALSE as i32 as BOOL;
                                    n = find_group_name(
                                        &mut *s,
                                        ((*s).u.tmp_buf).as_mut_ptr(),
                                        FALSE as i32 as BOOL,
                                    );
                                    if n == 0 as i32 {
                                        n = re_parse_captures(
                                            &mut *s,
                                            &mut dummy_res,
                                            ((*s).u.tmp_buf).as_mut_ptr(),
                                            FALSE as i32 as BOOL,
                                        );
                                        if n == 0 as i32 {
                                            if (*s).is_unicode != 0 || re_has_named_captures(&mut *s) != 0 {
                                                return re_parse_error(
            &mut *s,
                                                    b"group name not defined\0" as *const u8
                                                        as *const i8,
                                                )
                                            } else {
                                                current_block = 9143481802853542599;
                                            }
                                        } else {
                                            is_forward = TRUE as i32 as BOOL;
                                            current_block = 1069630499025798221;
                                        }
                                    } else {
                                        current_block = 1069630499025798221;
                                    }
                                    match current_block {
                                        9143481802853542599 => {}
                                        _ => {
                                            last_atom_start = (*s).byte_code.len() as i32;
                                            last_capture_count = (*s).capture_count;
                                            re_emit_op_u8(
                                                &mut *s,
                                                REOP_back_reference as i32
                                                    + 2 as i32
                                                        * is_backward_dir as i32
                                                    + (*s).ignore_case as i32,
                                                n as uint32_t,
                                            );
                                            if is_forward != 0 {
                                                re_parse_captures(
                                                    &mut *s,
                                                    &mut dummy_res,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as i32 as BOOL,
                                                );
                                            } else {
                                                find_group_name(
                                        &mut *s,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as i32 as BOOL,
                                                );
                                            }
                                            p = p1_0;
                                            current_block = 1771738965274008886;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            p = p.offset(1);
                            let mut q: *const uint8_t = p;
                            c = parse_digits(&mut p, FALSE as i32 as BOOL);
                            if c < 0 as i32
                                || c >= (*s).capture_count && c >= re_count_captures(&mut *s)
                            {
                                if (*s).is_unicode == 0 {
                                    p = q;
                                    if *p as i32 <= '7' as i32 {
                                        c = 0 as i32;
                                        if *p as i32 <= '3' as i32 {
                                            let fresh4 = p;
                                            p = p.offset(1);
                                            c = *fresh4 as i32 - '0' as i32;
                                        }
                                        if *p as i32 >= '0' as i32
                                            && *p as i32 <= '7' as i32
                                        {
                                            let fresh5 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as i32)
                                                + *fresh5 as i32 - '0' as i32;
                                            if *p as i32 >= '0' as i32
                                                && *p as i32 <= '7' as i32
                                            {
                                                let fresh6 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as i32)
                                                    + *fresh6 as i32 - '0' as i32;
                                            }
                                        }
                                    } else {
                                        let fresh7 = p;
                                        p = p.offset(1);
                                        c = *fresh7 as i32;
                                    }
                                } else {
                                    return re_parse_error(
            &mut *s,
                                        b"back reference out of range in regular expression\0"
                                            as *const u8 as *const i8,
                                    )
                                }
                                current_block = 4664037234821577316;
                            } else {
                                last_atom_start = (*s).byte_code.len() as i32;
                                last_capture_count = (*s).capture_count;
                                re_emit_op_u8(
                                    &mut *s,
                                    REOP_back_reference as i32
                                        + 2 as i32
                                            * is_backward_dir as i32
                                        + (*s).ignore_case as i32,
                                    1 as uint32_t,
                                );
                                bb_putc(&mut (*s).byte_code, c as uint8_t);
                                current_block = 1771738965274008886;
                            }
                        }
                    }
                }
                48 => {
                    current_block = 3824880322238534506;
                    match current_block {
                        8351012430473106477 => {
                            if *p.offset(1 as i32 as isize)
                                as i32 != 'b' as i32
                            {
                                re_emit_op(
                                    &mut *s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_not_word_boundary_i as i32
                                    } else {
                                        REOP_not_word_boundary as i32
                                    },
                                );
                            } else {
                                re_emit_op(
                                    &mut *s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_word_boundary_i as i32
                                    } else {
                                        REOP_word_boundary as i32
                                    },
                                );
                            }
                            p = p.offset(2 as i32 as isize);
                            current_block = 1771738965274008886;
                        }
                        3824880322238534506 => {
                            p = p.offset(2 as i32 as isize);
                            c = 0 as i32;
                            if (*s).is_unicode != 0 {
                                if is_digit(*p as i32) != 0 {
                                    return re_parse_error(
            &mut *s,
                                        b"invalid decimal escape in regular expression\0"
                                            as *const u8 as *const i8,
                                    );
                                }
                            } else if *p as i32 >= '0' as i32
                                && *p as i32 <= '7' as i32
                            {
                                let fresh2 = p;
                                p = p.offset(1);
                                c = *fresh2 as i32 - '0' as i32;
                                if *p as i32 >= '0' as i32
                                    && *p as i32 <= '7' as i32
                                {
                                    let fresh3 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as i32)
                                        + *fresh3 as i32 - '0' as i32;
                                }
                            }
                            current_block = 4664037234821577316;
                        }
                        5846959088466685742 => {
                            let mut p1_0: *const uint8_t = 0 as *const uint8_t;
                            let mut dummy_res: i32 = 0;
                            let mut n: i32 = 0;
                            let mut is_forward: BOOL = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as i32 as isize)
                                as i32 != '<' as i32
                            {
                                if (*s).is_unicode != 0 || re_has_named_captures(&mut *s) != 0 {
                                    return re_parse_error(
            &mut *s,
                                        b"expecting group name\0" as *const u8
                                            as *const i8,
                                    )
                                } else {
                                    current_block = 9143481802853542599;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as i32 as isize);
                                if re_parse_group_name(
                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                    ::core::mem::size_of::<[i8; 128]>()
                                        as i32,
                                    &mut p1_0,
                                ) != 0
                                {
                                    if (*s).is_unicode != 0 || re_has_named_captures(&mut *s) != 0 {
                                        return re_parse_error(
            &mut *s,
                                            b"invalid group name\0" as *const u8
                                                as *const i8,
                                        )
                                    } else {
                                        current_block = 9143481802853542599;
                                    }
                                } else {
                                    is_forward = FALSE as i32 as BOOL;
                                    n = find_group_name(
                                        &mut *s,
                                        ((*s).u.tmp_buf).as_mut_ptr(),
                                        FALSE as i32 as BOOL,
                                    );
                                    if n == 0 as i32 {
                                        n = re_parse_captures(
                                            &mut *s,
                                            &mut dummy_res,
                                            ((*s).u.tmp_buf).as_mut_ptr(),
                                            FALSE as i32 as BOOL,
                                        );
                                        if n == 0 as i32 {
                                            if (*s).is_unicode != 0 || re_has_named_captures(&mut *s) != 0 {
                                                return re_parse_error(
            &mut *s,
                                                    b"group name not defined\0" as *const u8
                                                        as *const i8,
                                                )
                                            } else {
                                                current_block = 9143481802853542599;
                                            }
                                        } else {
                                            is_forward = TRUE as i32 as BOOL;
                                            current_block = 1069630499025798221;
                                        }
                                    } else {
                                        current_block = 1069630499025798221;
                                    }
                                    match current_block {
                                        9143481802853542599 => {}
                                        _ => {
                                            last_atom_start = (*s).byte_code.len() as i32;
                                            last_capture_count = (*s).capture_count;
                                            re_emit_op_u8(
                                                &mut *s,
                                                REOP_back_reference as i32
                                                    + 2 as i32
                                                        * is_backward_dir as i32
                                                    + (*s).ignore_case as i32,
                                                n as uint32_t,
                                            );
                                            if is_forward != 0 {
                                                re_parse_captures(
                                                    &mut *s,
                                                    &mut dummy_res,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as i32 as BOOL,
                                                );
                                            } else {
                                                find_group_name(
                                        &mut *s,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as i32 as BOOL,
                                                );
                                            }
                                            p = p1_0;
                                            current_block = 1771738965274008886;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            p = p.offset(1);
                            let mut q: *const uint8_t = p;
                            c = parse_digits(&mut p, FALSE as i32 as BOOL);
                            if c < 0 as i32
                                || c >= (*s).capture_count && c >= re_count_captures(&mut *s)
                            {
                                if (*s).is_unicode == 0 {
                                    p = q;
                                    if *p as i32 <= '7' as i32 {
                                        c = 0 as i32;
                                        if *p as i32 <= '3' as i32 {
                                            let fresh4 = p;
                                            p = p.offset(1);
                                            c = *fresh4 as i32 - '0' as i32;
                                        }
                                        if *p as i32 >= '0' as i32
                                            && *p as i32 <= '7' as i32
                                        {
                                            let fresh5 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as i32)
                                                + *fresh5 as i32 - '0' as i32;
                                            if *p as i32 >= '0' as i32
                                                && *p as i32 <= '7' as i32
                                            {
                                                let fresh6 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as i32)
                                                    + *fresh6 as i32 - '0' as i32;
                                            }
                                        }
                                    } else {
                                        let fresh7 = p;
                                        p = p.offset(1);
                                        c = *fresh7 as i32;
                                    }
                                } else {
                                    return re_parse_error(
            &mut *s,
                                        b"back reference out of range in regular expression\0"
                                            as *const u8 as *const i8,
                                    )
                                }
                                current_block = 4664037234821577316;
                            } else {
                                last_atom_start = (*s).byte_code.len() as i32;
                                last_capture_count = (*s).capture_count;
                                re_emit_op_u8(
                                    &mut *s,
                                    REOP_back_reference as i32
                                        + 2 as i32
                                            * is_backward_dir as i32
                                        + (*s).ignore_case as i32,
                                    1 as uint32_t,
                                );
                                bb_putc(&mut (*s).byte_code, c as uint8_t);
                                current_block = 1771738965274008886;
                            }
                        }
                    }
                }
                49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    current_block = 7315983924538012637;
                    match current_block {
                        8351012430473106477 => {
                            if *p.offset(1 as i32 as isize)
                                as i32 != 'b' as i32
                            {
                                re_emit_op(
                                    &mut *s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_not_word_boundary_i as i32
                                    } else {
                                        REOP_not_word_boundary as i32
                                    },
                                );
                            } else {
                                re_emit_op(
                                    &mut *s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_word_boundary_i as i32
                                    } else {
                                        REOP_word_boundary as i32
                                    },
                                );
                            }
                            p = p.offset(2 as i32 as isize);
                            current_block = 1771738965274008886;
                        }
                        3824880322238534506 => {
                            p = p.offset(2 as i32 as isize);
                            c = 0 as i32;
                            if (*s).is_unicode != 0 {
                                if is_digit(*p as i32) != 0 {
                                    return re_parse_error(
            &mut *s,
                                        b"invalid decimal escape in regular expression\0"
                                            as *const u8 as *const i8,
                                    );
                                }
                            } else if *p as i32 >= '0' as i32
                                && *p as i32 <= '7' as i32
                            {
                                let fresh2 = p;
                                p = p.offset(1);
                                c = *fresh2 as i32 - '0' as i32;
                                if *p as i32 >= '0' as i32
                                    && *p as i32 <= '7' as i32
                                {
                                    let fresh3 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as i32)
                                        + *fresh3 as i32 - '0' as i32;
                                }
                            }
                            current_block = 4664037234821577316;
                        }
                        5846959088466685742 => {
                            let mut p1_0: *const uint8_t = 0 as *const uint8_t;
                            let mut dummy_res: i32 = 0;
                            let mut n: i32 = 0;
                            let mut is_forward: BOOL = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as i32 as isize)
                                as i32 != '<' as i32
                            {
                                if (*s).is_unicode != 0 || re_has_named_captures(&mut *s) != 0 {
                                    return re_parse_error(
            &mut *s,
                                        b"expecting group name\0" as *const u8
                                            as *const i8,
                                    )
                                } else {
                                    current_block = 9143481802853542599;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as i32 as isize);
                                if re_parse_group_name(
                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                    ::core::mem::size_of::<[i8; 128]>()
                                        as i32,
                                    &mut p1_0,
                                ) != 0
                                {
                                    if (*s).is_unicode != 0 || re_has_named_captures(&mut *s) != 0 {
                                        return re_parse_error(
            &mut *s,
                                            b"invalid group name\0" as *const u8
                                                as *const i8,
                                        )
                                    } else {
                                        current_block = 9143481802853542599;
                                    }
                                } else {
                                    is_forward = FALSE as i32 as BOOL;
                                    n = find_group_name(
                                        &mut *s,
                                        ((*s).u.tmp_buf).as_mut_ptr(),
                                        FALSE as i32 as BOOL,
                                    );
                                    if n == 0 as i32 {
                                        n = re_parse_captures(
                                            &mut *s,
                                            &mut dummy_res,
                                            ((*s).u.tmp_buf).as_mut_ptr(),
                                            FALSE as i32 as BOOL,
                                        );
                                        if n == 0 as i32 {
                                            if (*s).is_unicode != 0 || re_has_named_captures(&mut *s) != 0 {
                                                return re_parse_error(
            &mut *s,
                                                    b"group name not defined\0" as *const u8
                                                        as *const i8,
                                                )
                                            } else {
                                                current_block = 9143481802853542599;
                                            }
                                        } else {
                                            is_forward = TRUE as i32 as BOOL;
                                            current_block = 1069630499025798221;
                                        }
                                    } else {
                                        current_block = 1069630499025798221;
                                    }
                                    match current_block {
                                        9143481802853542599 => {}
                                        _ => {
                                            last_atom_start = (*s).byte_code.len() as i32;
                                            last_capture_count = (*s).capture_count;
                                            re_emit_op_u8(
                                                &mut *s,
                                                REOP_back_reference as i32
                                                    + 2 as i32
                                                        * is_backward_dir as i32
                                                    + (*s).ignore_case as i32,
                                                n as uint32_t,
                                            );
                                            if is_forward != 0 {
                                                re_parse_captures(
                                                    &mut *s,
                                                    &mut dummy_res,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as i32 as BOOL,
                                                );
                                            } else {
                                                find_group_name(
                                        &mut *s,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as i32 as BOOL,
                                                );
                                            }
                                            p = p1_0;
                                            current_block = 1771738965274008886;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            p = p.offset(1);
                            let mut q: *const uint8_t = p;
                            c = parse_digits(&mut p, FALSE as i32 as BOOL);
                            if c < 0 as i32
                                || c >= (*s).capture_count && c >= re_count_captures(&mut *s)
                            {
                                if (*s).is_unicode == 0 {
                                    p = q;
                                    if *p as i32 <= '7' as i32 {
                                        c = 0 as i32;
                                        if *p as i32 <= '3' as i32 {
                                            let fresh4 = p;
                                            p = p.offset(1);
                                            c = *fresh4 as i32 - '0' as i32;
                                        }
                                        if *p as i32 >= '0' as i32
                                            && *p as i32 <= '7' as i32
                                        {
                                            let fresh5 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as i32)
                                                + *fresh5 as i32 - '0' as i32;
                                            if *p as i32 >= '0' as i32
                                                && *p as i32 <= '7' as i32
                                            {
                                                let fresh6 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as i32)
                                                    + *fresh6 as i32 - '0' as i32;
                                            }
                                        }
                                    } else {
                                        let fresh7 = p;
                                        p = p.offset(1);
                                        c = *fresh7 as i32;
                                    }
                                } else {
                                    return re_parse_error(
            &mut *s,
                                        b"back reference out of range in regular expression\0"
                                            as *const u8 as *const i8,
                                    )
                                }
                                current_block = 4664037234821577316;
                            } else {
                                last_atom_start = (*s).byte_code.len() as i32;
                                last_capture_count = (*s).capture_count;
                                re_emit_op_u8(
                                    &mut *s,
                                    REOP_back_reference as i32
                                        + 2 as i32
                                            * is_backward_dir as i32
                                        + (*s).ignore_case as i32,
                                    1 as uint32_t,
                                );
                                bb_putc(&mut (*s).byte_code, c as uint8_t);
                                current_block = 1771738965274008886;
                            }
                        }
                    }
                }
                _ => {
                    current_block = 9143481802853542599;
                }
            }
        }
        91 => {
            last_atom_start = (*s).byte_code.len() as i32;
            last_capture_count = (*s).capture_count;
            if is_backward_dir != 0 {
                re_emit_op(&mut *s, REOP_prev as i32);
            }
            if re_parse_char_class(s, &mut p) != 0 {
                return -(1 as i32);
            }
            if is_backward_dir != 0 {
                re_emit_op(&mut *s, REOP_prev as i32);
            }
            current_block = 1771738965274008886;
        }
        93 | 125 => {
            if (*s).is_unicode != 0 {
                return re_parse_error(
            &mut *s,
                    b"syntax error\0" as *const u8 as *const i8,
                );
            }
            current_block = 9143481802853542599;
        }
        _ => {
            current_block = 9143481802853542599;
        }
    }
    match current_block {
        9143481802853542599 => {
            c = get_class_atom(s, cr, &mut p, FALSE as i32 as BOOL);
            if c < 0 as i32 {
                return -(1 as i32);
            }
            current_block = 4664037234821577316;
        }
        18128146392678525522 => {
            return re_parse_error(
            &mut *s,
                b"nothing to repeat\0" as *const u8 as *const i8,
            );
        }
        _ => {}
    }
    match current_block {
        4664037234821577316 => {
            last_atom_start = (*s).byte_code.len() as i32;
            last_capture_count = (*s).capture_count;
            if is_backward_dir != 0 {
                re_emit_op(&mut *s, REOP_prev as i32);
            }
            if c >= CLASS_RANGE_BASE {
                let mut ret: i32 = 0 as i32;
                if c == CLASS_RANGE_BASE + CHAR_RANGE_s as i32 {
                    re_emit_op(&mut *s, REOP_space as i32);
                } else if c == CLASS_RANGE_BASE + CHAR_RANGE_S as i32 {
                    re_emit_op(&mut *s, REOP_not_space as i32);
                } else {
                    ret = re_emit_string_list(s, cr);
                }
                re_string_list_free(cr);
                if ret != 0 {
                    return -(1 as i32);
                }
            } else {
                if (*s).ignore_case != 0 {
                    c = lre_canonicalize(
                        c as uint32_t,
                        (*s).is_unicode as i32,
                    );
                }
                re_emit_char(&mut *s, c);
            }
            if is_backward_dir != 0 {
                re_emit_op(&mut *s, REOP_prev as i32);
            }
        }
        _ => {}
    }
    if last_atom_start >= 0 as i32 {
        c = *p as i32;
        match c {
            42 => {
                current_block = 7069606996288041156;
                match current_block {
                    7069606996288041156 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    18255723851606772592 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = 1 as i32;
                        current_block = 9506328432318339935;
                    }
                    11105283338979551329 => {
                        p = p.offset(1);
                        quant_min = 1 as i32;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    _ => {
                        let mut p1_1: *const uint8_t = p;
                        if is_digit(
                            *p.offset(1 as i32 as isize) as i32,
                        ) == 0
                        {
                            if (*s).is_unicode != 0 {
                                current_block = 6286748319502055763;
                            } else {
                                current_block = 11353886201549099807;
                            }
                        } else {
                            p = p.offset(1);
                            quant_min = parse_digits(
                                &mut p,
                                TRUE as i32 as BOOL,
                            );
                            quant_max = quant_min;
                            if *p as i32 == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as i32) != 0 {
                                    quant_max = parse_digits(
                                        &mut p,
                                        TRUE as i32 as BOOL,
                                    );
                                    if quant_max < quant_min {
                                        current_block = 6286748319502055763;
                                    } else {
                                        current_block = 11165907417739823532;
                                    }
                                } else {
                                    quant_max = INT32_MAX;
                                    current_block = 11165907417739823532;
                                }
                            } else {
                                current_block = 11165907417739823532;
                            }
                            match current_block {
                                6286748319502055763 => {}
                                _ => {
                                    if *p as i32 != '}' as i32
                                        && (*s).is_unicode == 0
                                    {
                                        p = p1_1;
                                        current_block = 11353886201549099807;
                                    } else {
                                        if re_parse_expect(&mut *s, &mut p, '}' as i32) != 0 {
                                            return -(1 as i32);
                                        }
                                        current_block = 9506328432318339935;
                                    }
                                }
                            }
                        }
                        match current_block {
                            9506328432318339935 => {}
                            11353886201549099807 => {}
                            _ => {
                                return re_parse_error(
            &mut *s,
                                    b"invalid repetition count\0" as *const u8
                                        as *const i8,
                                );
                            }
                        }
                    }
                }
                match current_block {
                    11353886201549099807 => {}
                    _ => {
                        greedy = TRUE as i32 as BOOL;
                        if *p as i32 == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as i32 as BOOL;
                        }
                        if last_atom_start < 0 as i32 {
                            return re_parse_error(
            &mut *s,
                                b"nothing to repeat\0" as *const u8
                                    as *const i8,
                            );
                        }
                        let mut need_capture_init: BOOL = 0;
                        let mut add_zero_advance_check: BOOL = 0;
                        let mut len: i32 = 0;
                        let mut pos_0: i32 = 0;
                        let (check_adv, cap_init) = re_need_check_adv_and_capture_init(
                            &(*s).byte_code.as_slice()[last_atom_start as usize..],
                        );
                        add_zero_advance_check = check_adv as BOOL;
                        need_capture_init = cap_init as BOOL;
                        if need_capture_init != 0
                            && last_capture_count != (*s).capture_count
                        {
                            if bb_insert(
                                &mut (*s).byte_code,
                                last_atom_start,
                                3 as i32,
                            ) != 0
                            {
                                current_block = 17551395354807108434;
                            } else {
                                let mut pos_1: i32 = last_atom_start;
                                (&mut (*s).byte_code)[pos_1 as usize] = REOP_save_reset as u8;
                                pos_1 += 1;
                                (&mut (*s).byte_code)[pos_1 as usize] = last_capture_count as u8;
                                pos_1 += 1;
                                (&mut (*s).byte_code)[pos_1 as usize] = ((*s).capture_count - 1) as u8;
                                pos_1 += 1;
                                current_block = 1851490986684842406;
                            }
                        } else {
                            current_block = 1851490986684842406;
                        }
                        match current_block {
                            1851490986684842406 => {
                                len = ((*s).byte_code.len())
                                    .wrapping_sub(last_atom_start as size_t)
                                    as i32;
                                if quant_min == 0 as i32 {
                                    if need_capture_init == 0
                                        && last_capture_count != (*s).capture_count
                                    {
                                        if bb_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            3 as i32,
                                        ) != 0
                                        {
                                            current_block = 17551395354807108434;
                                        } else {
                                            (&mut (*s).byte_code)[last_atom_start as usize] = REOP_save_reset as u8;
                                            last_atom_start += 1;
                                            (&mut (*s).byte_code)[last_atom_start as usize] = last_capture_count as u8;
                                            last_atom_start += 1;
                                            (&mut (*s).byte_code)[last_atom_start as usize] = ((*s).capture_count - 1) as u8;
                                            last_atom_start += 1;
                                            current_block = 8700473759921513224;
                                        }
                                    } else {
                                        current_block = 8700473759921513224;
                                    }
                                    match current_block {
                                        17551395354807108434 => {}
                                        _ => {
                                            if quant_max == 0 as i32 {
                                                (*s).byte_code.truncate(last_atom_start as usize);
                                                current_block = 2588063579017527985;
                                            } else if quant_max == 1 as i32
                                                || quant_max == INT32_MAX
                                            {
                                                let mut has_goto: BOOL = (quant_max == INT32_MAX)
                                                    as i32;
                                                if bb_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    5 as i32
                                                        + add_zero_advance_check as i32
                                                            * 2 as i32,
                                                ) != 0
                                                {
                                                    current_block = 17551395354807108434;
                                                } else {
                                                    (&mut (*s).byte_code)[last_atom_start as usize] = (REOP_split_goto_first
                                                        as i32 as BOOL + greedy) as uint8_t;
                                                    (*s).byte_code.set_u32_at(
                                                        (last_atom_start + 1) as usize,
                                                        (len + 5 * has_goto + add_zero_advance_check * 4) as u32,
                                                    );
                                                    if add_zero_advance_check != 0 {
                                                        (&mut (*s).byte_code)[(last_atom_start + 5) as usize] =
                                                            REOP_set_char_pos as u8;
                                                        (&mut (*s).byte_code)[(last_atom_start + 6) as usize] =
                                                            0 as u8;
                                                        re_emit_op_u8(
                                                            &mut *s,
                                                            REOP_check_advance as i32,
                                                            0 as uint32_t,
                                                        );
                                                    }
                                                    if has_goto != 0 {
                                                        re_emit_goto(
                                                            &mut *s,
                                                            REOP_goto as i32,
                                                            last_atom_start as uint32_t,
                                                        );
                                                    }
                                                    current_block = 2588063579017527985;
                                                }
                                            } else if bb_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                11 as i32
                                                    + add_zero_advance_check as i32
                                                        * 2 as i32,
                                            ) != 0
                                            {
                                                current_block = 17551395354807108434;
                                            } else {
                                                pos_0 = last_atom_start;
                                                (&mut (*s).byte_code)[pos_0 as usize] = (REOP_split_goto_first as i32 + greedy) as u8;
                                                pos_0 += 1;
                                                (*s).byte_code.set_u32_at(
                                                    pos_0 as usize,
                                                    (6 + add_zero_advance_check * 2 + len + 10) as u32,
                                                );
                                                pos_0 += 4;
                                                (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_i32 as u8;
                                                pos_0 += 1;
                                                (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                                pos_0 += 1;
                                                (*s).byte_code.set_u32_at(pos_0 as usize, quant_max as u32);
                                                pos_0 += 4;
                                                last_atom_start = pos_0;
                                                if add_zero_advance_check != 0 {
                                                    (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_char_pos as u8;
                                                    pos_0 += 1;
                                                    (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                                    pos_0 += 1;
                                                }
                                                re_emit_goto_u8_u32(
                                                    &mut *s,
                                                    (if add_zero_advance_check != 0 {
                                                        REOP_loop_check_adv_split_next_first as i32
                                                    } else {
                                                        REOP_loop_split_next_first as i32
                                                    }) - greedy as i32,
                                                    0 as uint32_t,
                                                    quant_max as uint32_t,
                                                    last_atom_start as uint32_t,
                                                );
                                                current_block = 2588063579017527985;
                                            }
                                        }
                                    }
                                } else if quant_min == 1 as i32
                                    && quant_max == INT32_MAX && add_zero_advance_check == 0
                                {
                                    re_emit_goto(
                                        &mut *s,
                                        REOP_split_next_first as i32
                                            - greedy as i32,
                                        last_atom_start as uint32_t,
                                    );
                                    current_block = 2588063579017527985;
                                } else {
                                    if quant_min == quant_max {
                                        add_zero_advance_check = FALSE as i32 as BOOL;
                                    }
                                    if bb_insert(
                                        &mut (*s).byte_code,
                                        last_atom_start,
                                        6 as i32
                                            + add_zero_advance_check as i32
                                                * 2 as i32,
                                    ) != 0
                                    {
                                        current_block = 17551395354807108434;
                                    } else {
                                        pos_0 = last_atom_start;
                                        (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_i32 as u8;
                                        pos_0 += 1;
                                        (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                        pos_0 += 1;
                                        (*s).byte_code.set_u32_at(pos_0 as usize, quant_max as u32);
                                        pos_0 += 4;
                                        last_atom_start = pos_0;
                                        if add_zero_advance_check != 0 {
                                            (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_char_pos as u8;
                                            pos_0 += 1;
                                            (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                            pos_0 += 1;
                                        }
                                        if quant_min == quant_max {
                                            re_emit_goto_u8(
                                                &mut *s,
                                                REOP_loop as i32,
                                                0 as uint32_t,
                                                last_atom_start as uint32_t,
                                            );
                                        } else {
                                            re_emit_goto_u8_u32(
                                                &mut *s,
                                                (if add_zero_advance_check != 0 {
                                                    REOP_loop_check_adv_split_next_first as i32
                                                } else {
                                                    REOP_loop_split_next_first as i32
                                                }) - greedy as i32,
                                                0 as uint32_t,
                                                (quant_max - quant_min) as uint32_t,
                                                last_atom_start as uint32_t,
                                            );
                                        }
                                        current_block = 2588063579017527985;
                                    }
                                }
                                match current_block {
                                    17551395354807108434 => {}
                                    _ => {
                                        last_atom_start = -(1 as i32);
                                        current_block = 11353886201549099807;
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11353886201549099807 => {}
                            _ => return re_parse_out_of_memory(&mut *s),
                        }
                    }
                }
            }
            43 => {
                current_block = 11105283338979551329;
                match current_block {
                    7069606996288041156 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    18255723851606772592 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = 1 as i32;
                        current_block = 9506328432318339935;
                    }
                    11105283338979551329 => {
                        p = p.offset(1);
                        quant_min = 1 as i32;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    _ => {
                        let mut p1_1: *const uint8_t = p;
                        if is_digit(
                            *p.offset(1 as i32 as isize) as i32,
                        ) == 0
                        {
                            if (*s).is_unicode != 0 {
                                current_block = 6286748319502055763;
                            } else {
                                current_block = 11353886201549099807;
                            }
                        } else {
                            p = p.offset(1);
                            quant_min = parse_digits(
                                &mut p,
                                TRUE as i32 as BOOL,
                            );
                            quant_max = quant_min;
                            if *p as i32 == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as i32) != 0 {
                                    quant_max = parse_digits(
                                        &mut p,
                                        TRUE as i32 as BOOL,
                                    );
                                    if quant_max < quant_min {
                                        current_block = 6286748319502055763;
                                    } else {
                                        current_block = 11165907417739823532;
                                    }
                                } else {
                                    quant_max = INT32_MAX;
                                    current_block = 11165907417739823532;
                                }
                            } else {
                                current_block = 11165907417739823532;
                            }
                            match current_block {
                                6286748319502055763 => {}
                                _ => {
                                    if *p as i32 != '}' as i32
                                        && (*s).is_unicode == 0
                                    {
                                        p = p1_1;
                                        current_block = 11353886201549099807;
                                    } else {
                                        if re_parse_expect(&mut *s, &mut p, '}' as i32) != 0 {
                                            return -(1 as i32);
                                        }
                                        current_block = 9506328432318339935;
                                    }
                                }
                            }
                        }
                        match current_block {
                            9506328432318339935 => {}
                            11353886201549099807 => {}
                            _ => {
                                return re_parse_error(
            &mut *s,
                                    b"invalid repetition count\0" as *const u8
                                        as *const i8,
                                );
                            }
                        }
                    }
                }
                match current_block {
                    11353886201549099807 => {}
                    _ => {
                        greedy = TRUE as i32 as BOOL;
                        if *p as i32 == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as i32 as BOOL;
                        }
                        if last_atom_start < 0 as i32 {
                            return re_parse_error(
            &mut *s,
                                b"nothing to repeat\0" as *const u8
                                    as *const i8,
                            );
                        }
                        let mut need_capture_init: BOOL = 0;
                        let mut add_zero_advance_check: BOOL = 0;
                        let mut len: i32 = 0;
                        let mut pos_0: i32 = 0;
                        let (check_adv, cap_init) = re_need_check_adv_and_capture_init(
                            &(*s).byte_code.as_slice()[last_atom_start as usize..],
                        );
                        add_zero_advance_check = check_adv as BOOL;
                        need_capture_init = cap_init as BOOL;
                        if need_capture_init != 0
                            && last_capture_count != (*s).capture_count
                        {
                            if bb_insert(
                                &mut (*s).byte_code,
                                last_atom_start,
                                3 as i32,
                            ) != 0
                            {
                                current_block = 17551395354807108434;
                            } else {
                                let mut pos_1: i32 = last_atom_start;
                                (&mut (*s).byte_code)[pos_1 as usize] = REOP_save_reset as u8;
                                pos_1 += 1;
                                (&mut (*s).byte_code)[pos_1 as usize] = last_capture_count as u8;
                                pos_1 += 1;
                                (&mut (*s).byte_code)[pos_1 as usize] = ((*s).capture_count - 1) as u8;
                                pos_1 += 1;
                                current_block = 1851490986684842406;
                            }
                        } else {
                            current_block = 1851490986684842406;
                        }
                        match current_block {
                            1851490986684842406 => {
                                len = ((*s).byte_code.len())
                                    .wrapping_sub(last_atom_start as size_t)
                                    as i32;
                                if quant_min == 0 as i32 {
                                    if need_capture_init == 0
                                        && last_capture_count != (*s).capture_count
                                    {
                                        if bb_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            3 as i32,
                                        ) != 0
                                        {
                                            current_block = 17551395354807108434;
                                        } else {
                                            (&mut (*s).byte_code)[last_atom_start as usize] = REOP_save_reset as u8;
                                            last_atom_start += 1;
                                            (&mut (*s).byte_code)[last_atom_start as usize] = last_capture_count as u8;
                                            last_atom_start += 1;
                                            (&mut (*s).byte_code)[last_atom_start as usize] = ((*s).capture_count - 1) as u8;
                                            last_atom_start += 1;
                                            current_block = 8700473759921513224;
                                        }
                                    } else {
                                        current_block = 8700473759921513224;
                                    }
                                    match current_block {
                                        17551395354807108434 => {}
                                        _ => {
                                            if quant_max == 0 as i32 {
                                                (*s).byte_code.truncate(last_atom_start as usize);
                                                current_block = 2588063579017527985;
                                            } else if quant_max == 1 as i32
                                                || quant_max == INT32_MAX
                                            {
                                                let mut has_goto: BOOL = (quant_max == INT32_MAX)
                                                    as i32;
                                                if bb_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    5 as i32
                                                        + add_zero_advance_check as i32
                                                            * 2 as i32,
                                                ) != 0
                                                {
                                                    current_block = 17551395354807108434;
                                                } else {
                                                    (&mut (*s).byte_code)[last_atom_start as usize] = (REOP_split_goto_first
                                                        as i32 as BOOL + greedy) as uint8_t;
                                                    (*s).byte_code.set_u32_at(
                                                        (last_atom_start + 1) as usize,
                                                        (len + 5 * has_goto + add_zero_advance_check * 4) as u32,
                                                    );
                                                    if add_zero_advance_check != 0 {
                                                        (&mut (*s).byte_code)[(last_atom_start + 5) as usize] =
                                                            REOP_set_char_pos as u8;
                                                        (&mut (*s).byte_code)[(last_atom_start + 6) as usize] =
                                                            0 as u8;
                                                        re_emit_op_u8(
                                                            &mut *s,
                                                            REOP_check_advance as i32,
                                                            0 as uint32_t,
                                                        );
                                                    }
                                                    if has_goto != 0 {
                                                        re_emit_goto(
                                                            &mut *s,
                                                            REOP_goto as i32,
                                                            last_atom_start as uint32_t,
                                                        );
                                                    }
                                                    current_block = 2588063579017527985;
                                                }
                                            } else if bb_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                11 as i32
                                                    + add_zero_advance_check as i32
                                                        * 2 as i32,
                                            ) != 0
                                            {
                                                current_block = 17551395354807108434;
                                            } else {
                                                pos_0 = last_atom_start;
                                                (&mut (*s).byte_code)[pos_0 as usize] = (REOP_split_goto_first as i32 + greedy) as u8;
                                                pos_0 += 1;
                                                (*s).byte_code.set_u32_at(
                                                    pos_0 as usize,
                                                    (6 + add_zero_advance_check * 2 + len + 10) as u32,
                                                );
                                                pos_0 += 4;
                                                (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_i32 as u8;
                                                pos_0 += 1;
                                                (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                                pos_0 += 1;
                                                (*s).byte_code.set_u32_at(pos_0 as usize, quant_max as u32);
                                                pos_0 += 4;
                                                last_atom_start = pos_0;
                                                if add_zero_advance_check != 0 {
                                                    (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_char_pos as u8;
                                                    pos_0 += 1;
                                                    (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                                    pos_0 += 1;
                                                }
                                                re_emit_goto_u8_u32(
                                                    &mut *s,
                                                    (if add_zero_advance_check != 0 {
                                                        REOP_loop_check_adv_split_next_first as i32
                                                    } else {
                                                        REOP_loop_split_next_first as i32
                                                    }) - greedy as i32,
                                                    0 as uint32_t,
                                                    quant_max as uint32_t,
                                                    last_atom_start as uint32_t,
                                                );
                                                current_block = 2588063579017527985;
                                            }
                                        }
                                    }
                                } else if quant_min == 1 as i32
                                    && quant_max == INT32_MAX && add_zero_advance_check == 0
                                {
                                    re_emit_goto(
                                        &mut *s,
                                        REOP_split_next_first as i32
                                            - greedy as i32,
                                        last_atom_start as uint32_t,
                                    );
                                    current_block = 2588063579017527985;
                                } else {
                                    if quant_min == quant_max {
                                        add_zero_advance_check = FALSE as i32 as BOOL;
                                    }
                                    if bb_insert(
                                        &mut (*s).byte_code,
                                        last_atom_start,
                                        6 as i32
                                            + add_zero_advance_check as i32
                                                * 2 as i32,
                                    ) != 0
                                    {
                                        current_block = 17551395354807108434;
                                    } else {
                                        pos_0 = last_atom_start;
                                        (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_i32 as u8;
                                        pos_0 += 1;
                                        (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                        pos_0 += 1;
                                        (*s).byte_code.set_u32_at(pos_0 as usize, quant_max as u32);
                                        pos_0 += 4;
                                        last_atom_start = pos_0;
                                        if add_zero_advance_check != 0 {
                                            (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_char_pos as u8;
                                            pos_0 += 1;
                                            (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                            pos_0 += 1;
                                        }
                                        if quant_min == quant_max {
                                            re_emit_goto_u8(
                                                &mut *s,
                                                REOP_loop as i32,
                                                0 as uint32_t,
                                                last_atom_start as uint32_t,
                                            );
                                        } else {
                                            re_emit_goto_u8_u32(
                                                &mut *s,
                                                (if add_zero_advance_check != 0 {
                                                    REOP_loop_check_adv_split_next_first as i32
                                                } else {
                                                    REOP_loop_split_next_first as i32
                                                }) - greedy as i32,
                                                0 as uint32_t,
                                                (quant_max - quant_min) as uint32_t,
                                                last_atom_start as uint32_t,
                                            );
                                        }
                                        current_block = 2588063579017527985;
                                    }
                                }
                                match current_block {
                                    17551395354807108434 => {}
                                    _ => {
                                        last_atom_start = -(1 as i32);
                                        current_block = 11353886201549099807;
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11353886201549099807 => {}
                            _ => return re_parse_out_of_memory(&mut *s),
                        }
                    }
                }
            }
            63 => {
                current_block = 18255723851606772592;
                match current_block {
                    7069606996288041156 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    18255723851606772592 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = 1 as i32;
                        current_block = 9506328432318339935;
                    }
                    11105283338979551329 => {
                        p = p.offset(1);
                        quant_min = 1 as i32;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    _ => {
                        let mut p1_1: *const uint8_t = p;
                        if is_digit(
                            *p.offset(1 as i32 as isize) as i32,
                        ) == 0
                        {
                            if (*s).is_unicode != 0 {
                                current_block = 6286748319502055763;
                            } else {
                                current_block = 11353886201549099807;
                            }
                        } else {
                            p = p.offset(1);
                            quant_min = parse_digits(
                                &mut p,
                                TRUE as i32 as BOOL,
                            );
                            quant_max = quant_min;
                            if *p as i32 == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as i32) != 0 {
                                    quant_max = parse_digits(
                                        &mut p,
                                        TRUE as i32 as BOOL,
                                    );
                                    if quant_max < quant_min {
                                        current_block = 6286748319502055763;
                                    } else {
                                        current_block = 11165907417739823532;
                                    }
                                } else {
                                    quant_max = INT32_MAX;
                                    current_block = 11165907417739823532;
                                }
                            } else {
                                current_block = 11165907417739823532;
                            }
                            match current_block {
                                6286748319502055763 => {}
                                _ => {
                                    if *p as i32 != '}' as i32
                                        && (*s).is_unicode == 0
                                    {
                                        p = p1_1;
                                        current_block = 11353886201549099807;
                                    } else {
                                        if re_parse_expect(&mut *s, &mut p, '}' as i32) != 0 {
                                            return -(1 as i32);
                                        }
                                        current_block = 9506328432318339935;
                                    }
                                }
                            }
                        }
                        match current_block {
                            9506328432318339935 => {}
                            11353886201549099807 => {}
                            _ => {
                                return re_parse_error(
            &mut *s,
                                    b"invalid repetition count\0" as *const u8
                                        as *const i8,
                                );
                            }
                        }
                    }
                }
                match current_block {
                    11353886201549099807 => {}
                    _ => {
                        greedy = TRUE as i32 as BOOL;
                        if *p as i32 == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as i32 as BOOL;
                        }
                        if last_atom_start < 0 as i32 {
                            return re_parse_error(
            &mut *s,
                                b"nothing to repeat\0" as *const u8
                                    as *const i8,
                            );
                        }
                        let mut need_capture_init: BOOL = 0;
                        let mut add_zero_advance_check: BOOL = 0;
                        let mut len: i32 = 0;
                        let mut pos_0: i32 = 0;
                        let (check_adv, cap_init) = re_need_check_adv_and_capture_init(
                            &(*s).byte_code.as_slice()[last_atom_start as usize..],
                        );
                        add_zero_advance_check = check_adv as BOOL;
                        need_capture_init = cap_init as BOOL;
                        if need_capture_init != 0
                            && last_capture_count != (*s).capture_count
                        {
                            if bb_insert(
                                &mut (*s).byte_code,
                                last_atom_start,
                                3 as i32,
                            ) != 0
                            {
                                current_block = 17551395354807108434;
                            } else {
                                let mut pos_1: i32 = last_atom_start;
                                (&mut (*s).byte_code)[pos_1 as usize] = REOP_save_reset as u8;
                                pos_1 += 1;
                                (&mut (*s).byte_code)[pos_1 as usize] = last_capture_count as u8;
                                pos_1 += 1;
                                (&mut (*s).byte_code)[pos_1 as usize] = ((*s).capture_count - 1) as u8;
                                pos_1 += 1;
                                current_block = 1851490986684842406;
                            }
                        } else {
                            current_block = 1851490986684842406;
                        }
                        match current_block {
                            1851490986684842406 => {
                                len = ((*s).byte_code.len())
                                    .wrapping_sub(last_atom_start as size_t)
                                    as i32;
                                if quant_min == 0 as i32 {
                                    if need_capture_init == 0
                                        && last_capture_count != (*s).capture_count
                                    {
                                        if bb_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            3 as i32,
                                        ) != 0
                                        {
                                            current_block = 17551395354807108434;
                                        } else {
                                            (&mut (*s).byte_code)[last_atom_start as usize] = REOP_save_reset as u8;
                                            last_atom_start += 1;
                                            (&mut (*s).byte_code)[last_atom_start as usize] = last_capture_count as u8;
                                            last_atom_start += 1;
                                            (&mut (*s).byte_code)[last_atom_start as usize] = ((*s).capture_count - 1) as u8;
                                            last_atom_start += 1;
                                            current_block = 8700473759921513224;
                                        }
                                    } else {
                                        current_block = 8700473759921513224;
                                    }
                                    match current_block {
                                        17551395354807108434 => {}
                                        _ => {
                                            if quant_max == 0 as i32 {
                                                (*s).byte_code.truncate(last_atom_start as usize);
                                                current_block = 2588063579017527985;
                                            } else if quant_max == 1 as i32
                                                || quant_max == INT32_MAX
                                            {
                                                let mut has_goto: BOOL = (quant_max == INT32_MAX)
                                                    as i32;
                                                if bb_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    5 as i32
                                                        + add_zero_advance_check as i32
                                                            * 2 as i32,
                                                ) != 0
                                                {
                                                    current_block = 17551395354807108434;
                                                } else {
                                                    (&mut (*s).byte_code)[last_atom_start as usize] = (REOP_split_goto_first
                                                        as i32 as BOOL + greedy) as uint8_t;
                                                    (*s).byte_code.set_u32_at(
                                                        (last_atom_start + 1) as usize,
                                                        (len + 5 * has_goto + add_zero_advance_check * 4) as u32,
                                                    );
                                                    if add_zero_advance_check != 0 {
                                                        (&mut (*s).byte_code)[(last_atom_start + 5) as usize] =
                                                            REOP_set_char_pos as u8;
                                                        (&mut (*s).byte_code)[(last_atom_start + 6) as usize] =
                                                            0 as u8;
                                                        re_emit_op_u8(
                                                            &mut *s,
                                                            REOP_check_advance as i32,
                                                            0 as uint32_t,
                                                        );
                                                    }
                                                    if has_goto != 0 {
                                                        re_emit_goto(
                                                            &mut *s,
                                                            REOP_goto as i32,
                                                            last_atom_start as uint32_t,
                                                        );
                                                    }
                                                    current_block = 2588063579017527985;
                                                }
                                            } else if bb_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                11 as i32
                                                    + add_zero_advance_check as i32
                                                        * 2 as i32,
                                            ) != 0
                                            {
                                                current_block = 17551395354807108434;
                                            } else {
                                                pos_0 = last_atom_start;
                                                (&mut (*s).byte_code)[pos_0 as usize] = (REOP_split_goto_first as i32 + greedy) as u8;
                                                pos_0 += 1;
                                                (*s).byte_code.set_u32_at(
                                                    pos_0 as usize,
                                                    (6 + add_zero_advance_check * 2 + len + 10) as u32,
                                                );
                                                pos_0 += 4;
                                                (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_i32 as u8;
                                                pos_0 += 1;
                                                (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                                pos_0 += 1;
                                                (*s).byte_code.set_u32_at(pos_0 as usize, quant_max as u32);
                                                pos_0 += 4;
                                                last_atom_start = pos_0;
                                                if add_zero_advance_check != 0 {
                                                    (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_char_pos as u8;
                                                    pos_0 += 1;
                                                    (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                                    pos_0 += 1;
                                                }
                                                re_emit_goto_u8_u32(
                                                    &mut *s,
                                                    (if add_zero_advance_check != 0 {
                                                        REOP_loop_check_adv_split_next_first as i32
                                                    } else {
                                                        REOP_loop_split_next_first as i32
                                                    }) - greedy as i32,
                                                    0 as uint32_t,
                                                    quant_max as uint32_t,
                                                    last_atom_start as uint32_t,
                                                );
                                                current_block = 2588063579017527985;
                                            }
                                        }
                                    }
                                } else if quant_min == 1 as i32
                                    && quant_max == INT32_MAX && add_zero_advance_check == 0
                                {
                                    re_emit_goto(
                                        &mut *s,
                                        REOP_split_next_first as i32
                                            - greedy as i32,
                                        last_atom_start as uint32_t,
                                    );
                                    current_block = 2588063579017527985;
                                } else {
                                    if quant_min == quant_max {
                                        add_zero_advance_check = FALSE as i32 as BOOL;
                                    }
                                    if bb_insert(
                                        &mut (*s).byte_code,
                                        last_atom_start,
                                        6 as i32
                                            + add_zero_advance_check as i32
                                                * 2 as i32,
                                    ) != 0
                                    {
                                        current_block = 17551395354807108434;
                                    } else {
                                        pos_0 = last_atom_start;
                                        (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_i32 as u8;
                                        pos_0 += 1;
                                        (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                        pos_0 += 1;
                                        (*s).byte_code.set_u32_at(pos_0 as usize, quant_max as u32);
                                        pos_0 += 4;
                                        last_atom_start = pos_0;
                                        if add_zero_advance_check != 0 {
                                            (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_char_pos as u8;
                                            pos_0 += 1;
                                            (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                            pos_0 += 1;
                                        }
                                        if quant_min == quant_max {
                                            re_emit_goto_u8(
                                                &mut *s,
                                                REOP_loop as i32,
                                                0 as uint32_t,
                                                last_atom_start as uint32_t,
                                            );
                                        } else {
                                            re_emit_goto_u8_u32(
                                                &mut *s,
                                                (if add_zero_advance_check != 0 {
                                                    REOP_loop_check_adv_split_next_first as i32
                                                } else {
                                                    REOP_loop_split_next_first as i32
                                                }) - greedy as i32,
                                                0 as uint32_t,
                                                (quant_max - quant_min) as uint32_t,
                                                last_atom_start as uint32_t,
                                            );
                                        }
                                        current_block = 2588063579017527985;
                                    }
                                }
                                match current_block {
                                    17551395354807108434 => {}
                                    _ => {
                                        last_atom_start = -(1 as i32);
                                        current_block = 11353886201549099807;
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11353886201549099807 => {}
                            _ => return re_parse_out_of_memory(&mut *s),
                        }
                    }
                }
            }
            123 => {
                current_block = 16167632229894708628;
                match current_block {
                    7069606996288041156 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    18255723851606772592 => {
                        p = p.offset(1);
                        quant_min = 0 as i32;
                        quant_max = 1 as i32;
                        current_block = 9506328432318339935;
                    }
                    11105283338979551329 => {
                        p = p.offset(1);
                        quant_min = 1 as i32;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    _ => {
                        let mut p1_1: *const uint8_t = p;
                        if is_digit(
                            *p.offset(1 as i32 as isize) as i32,
                        ) == 0
                        {
                            if (*s).is_unicode != 0 {
                                current_block = 6286748319502055763;
                            } else {
                                current_block = 11353886201549099807;
                            }
                        } else {
                            p = p.offset(1);
                            quant_min = parse_digits(
                                &mut p,
                                TRUE as i32 as BOOL,
                            );
                            quant_max = quant_min;
                            if *p as i32 == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as i32) != 0 {
                                    quant_max = parse_digits(
                                        &mut p,
                                        TRUE as i32 as BOOL,
                                    );
                                    if quant_max < quant_min {
                                        current_block = 6286748319502055763;
                                    } else {
                                        current_block = 11165907417739823532;
                                    }
                                } else {
                                    quant_max = INT32_MAX;
                                    current_block = 11165907417739823532;
                                }
                            } else {
                                current_block = 11165907417739823532;
                            }
                            match current_block {
                                6286748319502055763 => {}
                                _ => {
                                    if *p as i32 != '}' as i32
                                        && (*s).is_unicode == 0
                                    {
                                        p = p1_1;
                                        current_block = 11353886201549099807;
                                    } else {
                                        if re_parse_expect(&mut *s, &mut p, '}' as i32) != 0 {
                                            return -(1 as i32);
                                        }
                                        current_block = 9506328432318339935;
                                    }
                                }
                            }
                        }
                        match current_block {
                            9506328432318339935 => {}
                            11353886201549099807 => {}
                            _ => {
                                return re_parse_error(
            &mut *s,
                                    b"invalid repetition count\0" as *const u8
                                        as *const i8,
                                );
                            }
                        }
                    }
                }
                match current_block {
                    11353886201549099807 => {}
                    _ => {
                        greedy = TRUE as i32 as BOOL;
                        if *p as i32 == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as i32 as BOOL;
                        }
                        if last_atom_start < 0 as i32 {
                            return re_parse_error(
            &mut *s,
                                b"nothing to repeat\0" as *const u8
                                    as *const i8,
                            );
                        }
                        let mut need_capture_init: BOOL = 0;
                        let mut add_zero_advance_check: BOOL = 0;
                        let mut len: i32 = 0;
                        let mut pos_0: i32 = 0;
                        let (check_adv, cap_init) = re_need_check_adv_and_capture_init(
                            &(*s).byte_code.as_slice()[last_atom_start as usize..],
                        );
                        add_zero_advance_check = check_adv as BOOL;
                        need_capture_init = cap_init as BOOL;
                        if need_capture_init != 0
                            && last_capture_count != (*s).capture_count
                        {
                            if bb_insert(
                                &mut (*s).byte_code,
                                last_atom_start,
                                3 as i32,
                            ) != 0
                            {
                                current_block = 17551395354807108434;
                            } else {
                                let mut pos_1: i32 = last_atom_start;
                                (&mut (*s).byte_code)[pos_1 as usize] = REOP_save_reset as u8;
                                pos_1 += 1;
                                (&mut (*s).byte_code)[pos_1 as usize] = last_capture_count as u8;
                                pos_1 += 1;
                                (&mut (*s).byte_code)[pos_1 as usize] = ((*s).capture_count - 1) as u8;
                                pos_1 += 1;
                                current_block = 1851490986684842406;
                            }
                        } else {
                            current_block = 1851490986684842406;
                        }
                        match current_block {
                            1851490986684842406 => {
                                len = ((*s).byte_code.len())
                                    .wrapping_sub(last_atom_start as size_t)
                                    as i32;
                                if quant_min == 0 as i32 {
                                    if need_capture_init == 0
                                        && last_capture_count != (*s).capture_count
                                    {
                                        if bb_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            3 as i32,
                                        ) != 0
                                        {
                                            current_block = 17551395354807108434;
                                        } else {
                                            (&mut (*s).byte_code)[last_atom_start as usize] = REOP_save_reset as u8;
                                            last_atom_start += 1;
                                            (&mut (*s).byte_code)[last_atom_start as usize] = last_capture_count as u8;
                                            last_atom_start += 1;
                                            (&mut (*s).byte_code)[last_atom_start as usize] = ((*s).capture_count - 1) as u8;
                                            last_atom_start += 1;
                                            current_block = 8700473759921513224;
                                        }
                                    } else {
                                        current_block = 8700473759921513224;
                                    }
                                    match current_block {
                                        17551395354807108434 => {}
                                        _ => {
                                            if quant_max == 0 as i32 {
                                                (*s).byte_code.truncate(last_atom_start as usize);
                                                current_block = 2588063579017527985;
                                            } else if quant_max == 1 as i32
                                                || quant_max == INT32_MAX
                                            {
                                                let mut has_goto: BOOL = (quant_max == INT32_MAX)
                                                    as i32;
                                                if bb_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    5 as i32
                                                        + add_zero_advance_check as i32
                                                            * 2 as i32,
                                                ) != 0
                                                {
                                                    current_block = 17551395354807108434;
                                                } else {
                                                    (&mut (*s).byte_code)[last_atom_start as usize] = (REOP_split_goto_first
                                                        as i32 as BOOL + greedy) as uint8_t;
                                                    (*s).byte_code.set_u32_at(
                                                        (last_atom_start + 1) as usize,
                                                        (len + 5 * has_goto + add_zero_advance_check * 4) as u32,
                                                    );
                                                    if add_zero_advance_check != 0 {
                                                        (&mut (*s).byte_code)[(last_atom_start + 5) as usize] =
                                                            REOP_set_char_pos as u8;
                                                        (&mut (*s).byte_code)[(last_atom_start + 6) as usize] =
                                                            0 as u8;
                                                        re_emit_op_u8(
                                                            &mut *s,
                                                            REOP_check_advance as i32,
                                                            0 as uint32_t,
                                                        );
                                                    }
                                                    if has_goto != 0 {
                                                        re_emit_goto(
                                                            &mut *s,
                                                            REOP_goto as i32,
                                                            last_atom_start as uint32_t,
                                                        );
                                                    }
                                                    current_block = 2588063579017527985;
                                                }
                                            } else if bb_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                11 as i32
                                                    + add_zero_advance_check as i32
                                                        * 2 as i32,
                                            ) != 0
                                            {
                                                current_block = 17551395354807108434;
                                            } else {
                                                pos_0 = last_atom_start;
                                                (&mut (*s).byte_code)[pos_0 as usize] = (REOP_split_goto_first as i32 + greedy) as u8;
                                                pos_0 += 1;
                                                (*s).byte_code.set_u32_at(
                                                    pos_0 as usize,
                                                    (6 + add_zero_advance_check * 2 + len + 10) as u32,
                                                );
                                                pos_0 += 4;
                                                (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_i32 as u8;
                                                pos_0 += 1;
                                                (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                                pos_0 += 1;
                                                (*s).byte_code.set_u32_at(pos_0 as usize, quant_max as u32);
                                                pos_0 += 4;
                                                last_atom_start = pos_0;
                                                if add_zero_advance_check != 0 {
                                                    (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_char_pos as u8;
                                                    pos_0 += 1;
                                                    (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                                    pos_0 += 1;
                                                }
                                                re_emit_goto_u8_u32(
                                                    &mut *s,
                                                    (if add_zero_advance_check != 0 {
                                                        REOP_loop_check_adv_split_next_first as i32
                                                    } else {
                                                        REOP_loop_split_next_first as i32
                                                    }) - greedy as i32,
                                                    0 as uint32_t,
                                                    quant_max as uint32_t,
                                                    last_atom_start as uint32_t,
                                                );
                                                current_block = 2588063579017527985;
                                            }
                                        }
                                    }
                                } else if quant_min == 1 as i32
                                    && quant_max == INT32_MAX && add_zero_advance_check == 0
                                {
                                    re_emit_goto(
                                        &mut *s,
                                        REOP_split_next_first as i32
                                            - greedy as i32,
                                        last_atom_start as uint32_t,
                                    );
                                    current_block = 2588063579017527985;
                                } else {
                                    if quant_min == quant_max {
                                        add_zero_advance_check = FALSE as i32 as BOOL;
                                    }
                                    if bb_insert(
                                        &mut (*s).byte_code,
                                        last_atom_start,
                                        6 as i32
                                            + add_zero_advance_check as i32
                                                * 2 as i32,
                                    ) != 0
                                    {
                                        current_block = 17551395354807108434;
                                    } else {
                                        pos_0 = last_atom_start;
                                        (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_i32 as u8;
                                        pos_0 += 1;
                                        (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                        pos_0 += 1;
                                        (*s).byte_code.set_u32_at(pos_0 as usize, quant_max as u32);
                                        pos_0 += 4;
                                        last_atom_start = pos_0;
                                        if add_zero_advance_check != 0 {
                                            (&mut (*s).byte_code)[pos_0 as usize] = REOP_set_char_pos as u8;
                                            pos_0 += 1;
                                            (&mut (*s).byte_code)[pos_0 as usize] = 0;
                                            pos_0 += 1;
                                        }
                                        if quant_min == quant_max {
                                            re_emit_goto_u8(
                                                &mut *s,
                                                REOP_loop as i32,
                                                0 as uint32_t,
                                                last_atom_start as uint32_t,
                                            );
                                        } else {
                                            re_emit_goto_u8_u32(
                                                &mut *s,
                                                (if add_zero_advance_check != 0 {
                                                    REOP_loop_check_adv_split_next_first as i32
                                                } else {
                                                    REOP_loop_split_next_first as i32
                                                }) - greedy as i32,
                                                0 as uint32_t,
                                                (quant_max - quant_min) as uint32_t,
                                                last_atom_start as uint32_t,
                                            );
                                        }
                                        current_block = 2588063579017527985;
                                    }
                                }
                                match current_block {
                                    17551395354807108434 => {}
                                    _ => {
                                        last_atom_start = -(1 as i32);
                                        current_block = 11353886201549099807;
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11353886201549099807 => {}
                            _ => return re_parse_out_of_memory(&mut *s),
                        }
                    }
                }
            }
            _ => {}
        }
    }
    (*s).buf_ptr = p;
    return 0 as i32;
    } // close unsafe block
}
fn re_parse_alternative(
    s: *mut REParseState,
    is_backward_dir: BOOL,
) -> i32 {
    // SAFETY: s is a valid REParseState pointer from the parser
    unsafe {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut ret: i32 = 0;
    let mut start: size_t = 0;
    let mut term_start: size_t = 0;
    let mut end: size_t = 0;
    let mut term_size: size_t = 0;
    start = (*s).byte_code.len();
    loop {
        p = (*s).buf_ptr;
        if p >= (*s).buf_end {
            break;
        }
        if *p as i32 == '|' as i32 || *p as i32 == ')' as i32 {
            break;
        }
        term_start = (*s).byte_code.len();
        ret = re_parse_term(s, is_backward_dir);
        if ret != 0 {
            return ret;
        }
        if is_backward_dir != 0 {
            end = (*s).byte_code.len();
            term_size = end.wrapping_sub(term_start);
            // Resize buffer to accommodate the additional bytes
            (*s).byte_code.resize(end.wrapping_add(term_size));
            // Shift [start..end] up by term_size to make room at front
            (*s).byte_code.as_mut_slice().copy_within(start..end, start + term_size);
            // Copy the new term from [end..end+term_size] to [start..start+term_size]
            (*s).byte_code.as_mut_slice().copy_within(end..end + term_size, start);
        }
    }
    return 0 as i32;
    } // close unsafe block
}
fn re_parse_disjunction(
    s: *mut REParseState,
    is_backward_dir: BOOL,
) -> i32 {
    // SAFETY: s is a valid REParseState pointer from the parser
    unsafe {
    let mut start: i32 = 0;
    let mut len: i32 = 0;
    let mut pos: i32 = 0;
    if lre_check_stack_overflow((*s).opaque, 0 as size_t) != 0 {
        return re_parse_error(
            &mut *s,
            b"stack overflow\0" as *const u8 as *const i8,
        );
    }
    start = (*s).byte_code.len() as i32;
    if re_parse_alternative(s, is_backward_dir) != 0 {
        return -(1 as i32);
    }
    while *(*s).buf_ptr as i32 == '|' as i32 {
        (*s).buf_ptr = ((*s).buf_ptr).offset(1);
        len = ((*s).byte_code.len()).wrapping_sub(start as size_t) as i32;
        if bb_insert(&mut (*s).byte_code, start, 5 as i32) != 0 {
            return re_parse_out_of_memory(&mut *s);
        }
        (&mut (*s).byte_code)[start as usize] = REOP_split_next_first as u8;
        (*s).byte_code.set_u32_at((start + 1) as usize, (len + 5) as u32);
        pos = re_emit_op_u32(&mut *s, REOP_goto as i32, 0 as uint32_t);
        (*s).group_name_scope = ((*s).group_name_scope).wrapping_add(1);
        if re_parse_alternative(s, is_backward_dir) != 0 {
            return -(1 as i32);
        }
        len = ((*s).byte_code.len()).wrapping_sub((pos + 4 as i32) as size_t)
            as i32;
        (*s).byte_code.set_u32_at(pos as usize, len as u32);
    }
    return 0 as i32;
    } // close unsafe block
}
/// Computes register count and modifies bytecode in place.
/// Returns the maximum stack size, or -1 on error.
fn compute_register_count(bc_buf: &mut [u8]) -> i32 {
    let mut stack_size: i32 = 0;
    let mut stack_size_max: i32 = 0;
    let mut pos: usize = RE_HEADER_LEN as usize;
    let bc_len = bc_buf.len();

    while pos < bc_len {
        let opcode = bc_buf[pos] as i32;
        let mut len = reopcode_info[opcode as usize].size as usize;

        debug_assert!(opcode < REOP_COUNT as i32);
        debug_assert!(pos + len <= bc_len);

        match opcode {
            27 | 42 => {
                // REOP_set_i32, REOP_set_char_pos
                bc_buf[pos + 1] = stack_size as u8;
                stack_size += 1;
                if stack_size > stack_size_max {
                    if stack_size > REGISTER_COUNT_MAX {
                        return -1;
                    }
                    stack_size_max = stack_size;
                }
            }
            43 | 22 | 23 | 24 => {
                // REOP_check_advance, REOP_loop, REOP_loop_split_goto_first, REOP_loop_split_next_first
                debug_assert!(stack_size > 0);
                stack_size -= 1;
                bc_buf[pos + 1] = stack_size as u8;
            }
            25 | 26 => {
                // REOP_loop_check_adv_split_goto_first, REOP_loop_check_adv_split_next_first
                debug_assert!(stack_size >= 2);
                stack_size -= 2;
                bc_buf[pos + 1] = stack_size as u8;
            }
            36 | 37 => {
                // REOP_range, REOP_range_i
                let val = get_u16_safe(&bc_buf[pos + 1..]) as usize;
                len += val * 4;
            }
            38 | 39 => {
                // REOP_range32, REOP_range32_i
                let val = get_u16_safe(&bc_buf[pos + 1..]) as usize;
                len += val * 8;
            }
            32 | 33 | 34 | 35 => {
                // Back reference opcodes
                let val = bc_buf[pos + 1] as usize;
                len += val;
            }
            _ => {}
        }
        pos += len;
    }
    stack_size_max
}

fn lre_bytecode_realloc(
    opaque: *mut std::ffi::c_void,
    ptr: *mut std::ffi::c_void,
    size: size_t,
) -> *mut std::ffi::c_void {
    if size > (INT32_MAX / 2 as i32) as size_t {
        return NULL
    } else {
        return lre_realloc(opaque, ptr, size)
    };
}
#[no_mangle]
pub fn lre_compile(
    mut plen: *mut i32,
    mut error_msg: *mut i8,
    mut error_msg_size: i32,
    mut buf: *const i8,
    mut buf_len: size_t,
    mut re_flags: i32,
    mut opaque: *mut std::ffi::c_void,
) -> *mut uint8_t {
    // SAFETY: All pointers are valid from the caller (mod.rs public API)
    unsafe {
    let mut s_s: REParseState = REParseState {
        byte_code: ByteBuffer::new(),
        buf_ptr: std::ptr::null(),
        buf_end: std::ptr::null(),
        buf_start: std::ptr::null(),
        re_flags: 0,
        is_unicode: 0,
        unicode_sets: 0,
        ignore_case: 0,
        multi_line: 0,
        dotall: 0,
        group_name_scope: 0,
        capture_count: 0,
        total_capture_count: 0,
        has_named_captures: 0,
        opaque: std::ptr::null_mut(),
        group_names: ByteBuffer::new(),
        u: C2RustUnnamed_0 {
            error_msg: [0; 128],
        },
    };
    let mut s: *mut REParseState = &mut s_s;
    let mut register_count: i32 = 0;
    let mut is_sticky: BOOL = 0;
    (*s).opaque = opaque;
    (*s).buf_ptr = buf as *const uint8_t;
    (*s).buf_end = ((*s).buf_ptr).offset(buf_len as isize);
    (*s).buf_start = (*s).buf_ptr;
    (*s).re_flags = re_flags;
    (*s).is_unicode = (re_flags & (LRE_FLAG_UNICODE | LRE_FLAG_UNICODE_SETS)
        != 0 as i32) as i32 as BOOL;
    is_sticky = (re_flags & LRE_FLAG_STICKY != 0 as i32) as i32
        as BOOL;
    (*s).ignore_case = (re_flags & LRE_FLAG_IGNORECASE != 0 as i32)
        as i32 as BOOL;
    (*s).multi_line = (re_flags & LRE_FLAG_MULTILINE != 0 as i32)
        as i32 as BOOL;
    (*s).dotall = (re_flags & LRE_FLAG_DOTALL != 0 as i32)
        as i32 as BOOL;
    (*s).unicode_sets = (re_flags & LRE_FLAG_UNICODE_SETS != 0 as i32)
        as i32 as BOOL;
    (*s).capture_count = 1 as i32;
    (*s).total_capture_count = -(1 as i32);
    (*s).has_named_captures = -(1 as i32);
    // ByteBuffer is already initialized in struct initialization
    bb_put_u16(&mut (*s).byte_code, re_flags as uint16_t);
    bb_putc(&mut (*s).byte_code, 0 as uint8_t);
    bb_putc(&mut (*s).byte_code, 0 as uint8_t);
    bb_put_u32(&mut (*s).byte_code, 0 as uint32_t);
    if is_sticky == 0 {
        re_emit_op_u32(
            &mut *s,
            REOP_split_goto_first as i32,
            (1 as i32 + 5 as i32) as uint32_t,
        );
        re_emit_op(&mut *s, REOP_any as i32);
        re_emit_op_u32(
            &mut *s,
            REOP_goto as i32,
            -(5 as i32 + 1 as i32 + 5 as i32)
                as uint32_t,
        );
    }
    re_emit_op_u8(&mut *s, REOP_save_start as i32, 0 as uint32_t);
    if !(re_parse_disjunction(s, FALSE as i32 as BOOL) != 0) {
        re_emit_op_u8(&mut *s, REOP_save_end as i32, 0 as uint32_t);
        re_emit_op(&mut *s, REOP_match as i32);
        if *(*s).buf_ptr as i32 != '\0' as i32 {
            re_parse_error(
            &mut *s,
                b"extraneous characters at the end\0" as *const u8
                    as *const i8,
            );
        } else if bb_error(&(*s).byte_code) != 0 {
            re_parse_out_of_memory(&mut *s);
        } else {
            register_count = compute_register_count(
                (*s).byte_code.as_mut_slice(),
            );
            if register_count < 0 as i32 {
                re_parse_error(
            &mut *s,
                    b"too many imbricated quantifiers\0" as *const u8
                        as *const i8,
                );
            } else {
                (&mut (*s).byte_code)[RE_HEADER_CAPTURE_COUNT as usize] = (*s).capture_count as u8;
                (&mut (*s).byte_code)[RE_HEADER_REGISTER_COUNT as usize] = register_count as u8;
                (*s).byte_code.set_u32_at(
                    RE_HEADER_BYTECODE_LEN as usize,
                    ((*s).byte_code.len() - RE_HEADER_LEN as usize) as u32,
                );
                if (*s).group_names.len()
                    > (((*s).capture_count - 1 as i32)
                        * LRE_GROUP_NAME_TRAILER_LEN) as usize
                {
                    // Extend byte_code with group_names content
                    bb_put(&mut (*s).byte_code, (*s).group_names.as_slice());
                    let bc_slice = &(*s).byte_code.as_slice()[..RE_HEADER_LEN as usize];
                    (*s).byte_code.set_u16_at(
                        RE_HEADER_FLAGS as usize,
                        (lre_get_flags(bc_slice) | LRE_FLAG_NAMED_GROUPS) as u16,
                    );
                }
                // group_names drops automatically
                *error_msg.offset(0 as i32 as isize) = '\0' as i32
                    as i8;
                *plen = (*s).byte_code.len() as i32;
                // Transfer ownership: convert ByteBuffer to raw pointer
                // The caller is responsible for freeing this with libc::free
                let result = std::mem::take(&mut (*s).byte_code).into_raw();
                return result;
            }
        }
    }
    // ByteBuffer fields drop automatically when s_s goes out of scope
    pstrcpy(error_msg, error_msg_size, ((*s).u.error_msg).as_mut_ptr());
    *plen = 0 as i32;
    return 0 as *mut uint8_t;
    } // close unsafe block
}
/// Returns true if the character is a line terminator (LF, CR, LS, PS).
fn is_line_terminator(c: uint32_t) -> BOOL {
    (c == '\n' as i32 as uint32_t || c == '\r' as i32 as uint32_t
        || c == CP_LS as uint32_t || c == CP_PS as uint32_t) as i32
}
fn lre_poll_timeout(s: *mut REExecContext) -> i32 {
    // SAFETY: s is a valid REExecContext pointer from the executor
    unsafe {
        (*s).interrupt_counter -= 1;
        if ((*s).interrupt_counter <= 0 as i32) as i32
            as i64 != 0
        {
            (*s).interrupt_counter = INTERRUPT_COUNTER_INIT;
            if lre_check_timeout((*s).opaque) != 0 {
                return LRE_RET_TIMEOUT;
            }
        }
        return 0 as i32;
    }
}
#[inline(never)]
fn stack_realloc(
    s: *mut REExecContext,
    n: size_t,
) -> i32 {
    // SAFETY: s is a valid REExecContext pointer from the executor
    unsafe {
        let mut new_stack: *mut StackElem = 0 as *mut StackElem;
        let mut new_size: size_t = 0;
        new_size = ((*s).stack_size).wrapping_mul(3 as size_t).wrapping_div(2 as size_t);
        if new_size < n {
            new_size = n;
        }
        if (*s).stack_buf == ((*s).static_stack_buf).as_mut_ptr() {
            new_stack = lre_realloc(
                (*s).opaque,
                NULL,
                new_size.wrapping_mul(::core::mem::size_of::<StackElem>() as size_t),
            ) as *mut StackElem;
            if new_stack.is_null() {
                return -(1 as i32);
            }
            memcpy(
                new_stack as *mut std::ffi::c_void,
                (*s).stack_buf as *const std::ffi::c_void,
                ((*s).stack_size).wrapping_mul(::core::mem::size_of::<StackElem>() as size_t),
            );
        } else {
            new_stack = lre_realloc(
                (*s).opaque,
                (*s).stack_buf as *mut std::ffi::c_void,
                new_size.wrapping_mul(::core::mem::size_of::<StackElem>() as size_t),
            ) as *mut StackElem;
            if new_stack.is_null() {
                return -(1 as i32);
            }
        }
        (*s).stack_size = new_size;
        (*s).stack_buf = new_stack;
        return 0 as i32;
    }
}

/// Main bytecode interpreter loop for regex execution.
///
/// NOTE: This function intentionally retains c2rust-generated loop constructs.
/// The complex control flow with goto-like patterns (current_block, loop/break/continue)
/// is performance-critical and matches the original C implementation's behavior exactly.
/// Converting to idiomatic Rust iterators/loops would risk introducing subtle bugs in
/// this carefully tuned backtracking algorithm.
///
/// All pointer parameters must be valid. The pc pointer must point to valid bytecode.
fn lre_exec_backtrack(
    s: *mut REExecContext,
    capture: *mut *mut uint8_t,
    mut pc: *const uint8_t,
    mut cptr: *const uint8_t,
) -> intptr_t {
    // SAFETY: All pointers are valid from the execution context
    unsafe {
    let mut current_block: u64;
    let mut opcode: i32 = 0;
    let mut cbuf_type: i32 = 0;
    let mut val: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut idx: uint32_t = 0;
    let mut cbuf_end: *const uint8_t = 0 as *const uint8_t;
    let mut sp: *mut StackElem = 0 as *mut StackElem;
    let mut bp: *mut StackElem = 0 as *mut StackElem;
    let mut stack_end: *mut StackElem = 0 as *mut StackElem;
    cbuf_type = (*s).cbuf_type;
    cbuf_end = (*s).cbuf_end;
    sp = (*s).stack_buf;
    bp = (*s).stack_buf;
    stack_end = ((*s).stack_buf).offset((*s).stack_size as isize);
    's_31: loop {
        let fresh43 = pc;
        pc = pc.offset(1);
        opcode = *fresh43 as i32;
        match opcode {
            16 => return 1 as intptr_t,
            17 => {
                let mut sp1: *mut StackElem = 0 as *mut StackElem;
                let mut sp_top: *mut StackElem = 0 as *mut StackElem;
                let mut next_sp: *mut StackElem = 0 as *mut StackElem;
                let mut type_1: REExecStateEnum = RE_EXEC_STATE_SPLIT;
                sp_top = sp;
                loop {
                    sp1 = sp;
                    sp = bp;
                    pc = (*sp.offset(-(3 as i32) as isize)).ptr;
                    cptr = (*sp.offset(-(2 as i32) as isize)).ptr;
                    type_1 = ((*sp.offset(-(1 as i32) as isize)).bp)
                        .type_0() as REExecStateEnum;
                    bp = ((*s).stack_buf)
                        .offset(
                            ((*sp.offset(-(1 as i32) as isize)).bp).val()
                                as isize,
                        );
                    let ref mut fresh45 = (*sp.offset(-(1 as i32) as isize))
                        .ptr;
                    *fresh45 = sp1 as *mut std::ffi::c_void as *mut uint8_t;
                    sp = sp.offset(-(3 as i32 as isize));
                    if type_1 as u32
                        == RE_EXEC_STATE_LOOKAHEAD as i32
                            as u32
                    {
                        break;
                    }
                }
                if sp != (*s).stack_buf {
                    sp1 = sp;
                    while sp1 < sp_top {
                        next_sp = (*sp1.offset(2 as i32 as isize)).ptr
                            as *mut std::ffi::c_void as *mut StackElem;
                        sp1 = sp1.offset(3 as i32 as isize);
                        while sp1 < next_sp {
                            let fresh46 = sp1;
                            sp1 = sp1.offset(1);
                            let fresh47 = sp;
                            sp = sp.offset(1);
                            *fresh47 = *fresh46;
                        }
                    }
                }
                continue;
            }
            18 => {
                loop {
                    let mut type_2: REExecStateEnum = RE_EXEC_STATE_SPLIT;
                    type_2 = ((*bp.offset(-(1 as i32) as isize)).bp)
                        .type_0() as REExecStateEnum;
                    while sp > bp {
                        let ref mut fresh48 = *capture
                            .offset(
                                (*sp.offset(-(2 as i32) as isize)).val as isize,
                            );
                        *fresh48 = (*sp.offset(-(1 as i32) as isize)).ptr;
                        sp = sp.offset(-(2 as i32 as isize));
                    }
                    pc = (*sp.offset(-(3 as i32) as isize)).ptr;
                    cptr = (*sp.offset(-(2 as i32) as isize)).ptr;
                    type_2 = ((*sp.offset(-(1 as i32) as isize)).bp)
                        .type_0() as REExecStateEnum;
                    bp = ((*s).stack_buf)
                        .offset(
                            ((*sp.offset(-(1 as i32) as isize)).bp).val()
                                as isize,
                        );
                    sp = sp.offset(-(3 as i32 as isize));
                    if type_2 as u32
                        == RE_EXEC_STATE_NEGATIVE_LOOKAHEAD as i32
                            as u32
                    {
                        break;
                    }
                }
                current_block = 1885734024781174349;
            }
            3 | 4 => {
                val = get_u32(pc);
                pc = pc.offset(4 as i32 as isize);
                current_block = 13538245850655155981;
            }
            1 | 2 => {
                val = get_u16(pc);
                pc = pc.offset(2 as i32 as isize);
                current_block = 13538245850655155981;
            }
            14 | 15 => {
                let mut pc1: *const uint8_t = 0 as *const uint8_t;
                val = get_u32(pc);
                pc = pc.offset(4 as i32 as isize);
                if opcode == REOP_split_next_first as i32 {
                    pc1 = pc.offset(val as i32 as isize);
                } else {
                    pc1 = pc;
                    pc = pc.offset(val as i32 as isize);
                }
                if ((stack_end.offset_from(sp) as i64)
                    < 3 as i64) as i32 as i64
                    != 0
                {
                    let mut saved_sp: size_t = sp.offset_from((*s).stack_buf)
                        as i64 as size_t;
                    let mut saved_bp: size_t = bp.offset_from((*s).stack_buf)
                        as i64 as size_t;
                    if stack_realloc(
                        s,
                        (sp.offset_from((*s).stack_buf) as i64
                            + 3 as i64) as size_t,
                    ) != 0
                    {
                        return LRE_RET_MEMORY_ERROR as intptr_t;
                    }
                    stack_end = ((*s).stack_buf).offset((*s).stack_size as isize);
                    sp = ((*s).stack_buf).offset(saved_sp as isize);
                    bp = ((*s).stack_buf).offset(saved_bp as isize);
                }
                let ref mut fresh52 = (*sp.offset(0 as i32 as isize)).ptr;
                *fresh52 = pc1 as *mut uint8_t;
                let ref mut fresh53 = (*sp.offset(1 as i32 as isize)).ptr;
                *fresh53 = cptr as *mut uint8_t;
                let ref mut fresh54 = (*sp.offset(2 as i32 as isize)).bp;
                (*fresh54)
                    .set_val(
                        bp.offset_from((*s).stack_buf) as i64 as u64,
                    );
                let ref mut fresh55 = (*sp.offset(2 as i32 as isize)).bp;
                (*fresh55)
                    .set_type_0(
                        RE_EXEC_STATE_SPLIT as i32 as u64,
                    );
                sp = sp.offset(3 as i32 as isize);
                bp = sp;
                continue;
            }
            40 | 41 => {
                val = get_u32(pc);
                pc = pc.offset(4 as i32 as isize);
                if ((stack_end.offset_from(sp) as i64)
                    < 3 as i64) as i32 as i64
                    != 0
                {
                    let mut saved_sp_0: size_t = sp.offset_from((*s).stack_buf)
                        as i64 as size_t;
                    let mut saved_bp_0: size_t = bp.offset_from((*s).stack_buf)
                        as i64 as size_t;
                    if stack_realloc(
                        s,
                        (sp.offset_from((*s).stack_buf) as i64
                            + 3 as i64) as size_t,
                    ) != 0
                    {
                        return LRE_RET_MEMORY_ERROR as intptr_t;
                    }
                    stack_end = ((*s).stack_buf).offset((*s).stack_size as isize);
                    sp = ((*s).stack_buf).offset(saved_sp_0 as isize);
                    bp = ((*s).stack_buf).offset(saved_bp_0 as isize);
                }
                let ref mut fresh56 = (*sp.offset(0 as i32 as isize)).ptr;
                *fresh56 = pc.offset(val as i32 as isize) as *mut uint8_t;
                let ref mut fresh57 = (*sp.offset(1 as i32 as isize)).ptr;
                *fresh57 = cptr as *mut uint8_t;
                let ref mut fresh58 = (*sp.offset(2 as i32 as isize)).bp;
                (*fresh58)
                    .set_val(
                        bp.offset_from((*s).stack_buf) as i64 as u64,
                    );
                let ref mut fresh59 = (*sp.offset(2 as i32 as isize)).bp;
                (*fresh59)
                    .set_type_0(
                        (RE_EXEC_STATE_LOOKAHEAD as i32 + opcode
                            - REOP_lookahead as i32) as u64,
                    );
                sp = sp.offset(3 as i32 as isize);
                bp = sp;
                continue;
            }
            13 => {
                val = get_u32(pc);
                pc = pc
                    .offset((4 as i32 + val as i32) as isize);
                if lre_poll_timeout(s) != 0 {
                    return LRE_RET_TIMEOUT as intptr_t;
                }
                continue;
            }
            9 | 10 => {
                if cptr == (*s).cbuf {
                    continue;
                }
                if opcode == REOP_line_start as i32 {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as i32 {
                        c = *cptr.offset(-(1 as i32) as isize) as uint32_t;
                    } else {
                        let mut _p_0: *const uint16_t = (cptr as *const uint16_t)
                            .offset(-(1 as i32 as isize));
                        let mut _start: *const uint16_t = (*s).cbuf as *const uint16_t;
                        c = *_p_0 as uint32_t;
                        if is_lo_surrogate(c) != 0 && cbuf_type == 2 as i32
                        {
                            if _p_0 > _start
                                && is_hi_surrogate(
                                    *_p_0.offset(-(1 as i32) as isize) as uint32_t,
                                ) != 0
                            {
                                _p_0 = _p_0.offset(-1);
                                c = from_surrogate(*_p_0 as uint32_t, c);
                            }
                        }
                    }
                    if !(is_line_terminator(c) == 0) {
                        continue;
                    }
                    current_block = 1885734024781174349;
                }
            }
            11 | 12 => {
                if cptr == cbuf_end {
                    continue;
                }
                if opcode == REOP_line_end as i32 {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as i32 {
                        c = *cptr.offset(0 as i32 as isize) as uint32_t;
                    } else {
                        let mut _p_1: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_0: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh60 = _p_1;
                        _p_1 = _p_1.offset(1);
                        c = *fresh60 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as i32
                        {
                            if _p_1 < _end_0 && is_lo_surrogate(*_p_1 as uint32_t) != 0 {
                                c = from_surrogate(c, *_p_1 as uint32_t);
                            }
                        }
                    }
                    if !(is_line_terminator(c) == 0) {
                        continue;
                    }
                    current_block = 1885734024781174349;
                }
            }
            5 => {
                if cptr == cbuf_end {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as i32 {
                        let fresh61 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh61 as uint32_t;
                    } else {
                        let mut _p_2: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_1: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh62 = _p_2;
                        _p_2 = _p_2.offset(1);
                        c = *fresh62 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as i32
                        {
                            if _p_2 < _end_1 && is_lo_surrogate(*_p_2 as uint32_t) != 0 {
                                let fresh63 = _p_2;
                                _p_2 = _p_2.offset(1);
                                c = from_surrogate(c, *fresh63 as uint32_t);
                            }
                        }
                        cptr = _p_2 as *const std::ffi::c_void as *const uint8_t;
                    }
                    if !(is_line_terminator(c) != 0) {
                        continue;
                    }
                    current_block = 1885734024781174349;
                }
            }
            6 => {
                if cptr == cbuf_end {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as i32 {
                        let fresh64 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh64 as uint32_t;
                    } else {
                        let mut _p_3: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_2: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh65 = _p_3;
                        _p_3 = _p_3.offset(1);
                        c = *fresh65 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as i32
                        {
                            if _p_3 < _end_2 && is_lo_surrogate(*_p_3 as uint32_t) != 0 {
                                let fresh66 = _p_3;
                                _p_3 = _p_3.offset(1);
                                c = from_surrogate(c, *fresh66 as uint32_t);
                            }
                        }
                        cptr = _p_3 as *const std::ffi::c_void as *const uint8_t;
                    }
                    continue;
                }
            }
            7 => {
                if cptr == cbuf_end {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as i32 {
                        let fresh67 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh67 as uint32_t;
                    } else {
                        let mut _p_4: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_3: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh68 = _p_4;
                        _p_4 = _p_4.offset(1);
                        c = *fresh68 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as i32
                        {
                            if _p_4 < _end_3 && is_lo_surrogate(*_p_4 as uint32_t) != 0 {
                                let fresh69 = _p_4;
                                _p_4 = _p_4.offset(1);
                                c = from_surrogate(c, *fresh69 as uint32_t);
                            }
                        }
                        cptr = _p_4 as *const std::ffi::c_void as *const uint8_t;
                    }
                    if !(lre_is_space(c) == 0) {
                        continue;
                    }
                    current_block = 1885734024781174349;
                }
            }
            8 => {
                if cptr == cbuf_end {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as i32 {
                        let fresh70 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh70 as uint32_t;
                    } else {
                        let mut _p_5: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_4: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh71 = _p_5;
                        _p_5 = _p_5.offset(1);
                        c = *fresh71 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as i32
                        {
                            if _p_5 < _end_4 && is_lo_surrogate(*_p_5 as uint32_t) != 0 {
                                let fresh72 = _p_5;
                                _p_5 = _p_5.offset(1);
                                c = from_surrogate(c, *fresh72 as uint32_t);
                            }
                        }
                        cptr = _p_5 as *const std::ffi::c_void as *const uint8_t;
                    }
                    if !(lre_is_space(c) != 0) {
                        continue;
                    }
                    current_block = 1885734024781174349;
                }
            }
            19 | 20 => {
                let fresh73 = pc;
                pc = pc.offset(1);
                val = *fresh73 as uint32_t;
                assert!(val < (*s).capture_count as u32, "val < s->capture_count");
                idx = (2 as uint32_t)
                    .wrapping_mul(val)
                    .wrapping_add(opcode as uint32_t)
                    .wrapping_sub(REOP_save_start as i32 as uint32_t);
                if ((stack_end.offset_from(sp) as i64)
                    < 2 as i64) as i32 as i64
                    != 0
                {
                    let mut saved_sp_1: size_t = sp.offset_from((*s).stack_buf)
                        as i64 as size_t;
                    let mut saved_bp_1: size_t = bp.offset_from((*s).stack_buf)
                        as i64 as size_t;
                    if stack_realloc(
                        s,
                        (sp.offset_from((*s).stack_buf) as i64
                            + 2 as i64) as size_t,
                    ) != 0
                    {
                        return LRE_RET_MEMORY_ERROR as intptr_t;
                    }
                    stack_end = ((*s).stack_buf).offset((*s).stack_size as isize);
                    sp = ((*s).stack_buf).offset(saved_sp_1 as isize);
                    bp = ((*s).stack_buf).offset(saved_bp_1 as isize);
                }
                (*sp.offset(0 as i32 as isize)).val = idx as intptr_t;
                let ref mut fresh74 = (*sp.offset(1 as i32 as isize)).ptr;
                *fresh74 = *capture.offset(idx as isize);
                sp = sp.offset(2 as i32 as isize);
                let ref mut fresh75 = *capture.offset(idx as isize);
                *fresh75 = cptr as *mut uint8_t;
                continue;
            }
            21 => {
                let mut val2: uint32_t = 0;
                val = *pc.offset(0 as i32 as isize) as uint32_t;
                val2 = *pc.offset(1 as i32 as isize) as uint32_t;
                pc = pc.offset(2 as i32 as isize);
                assert!(val2 < (*s).capture_count as u32, "val2 < s->capture_count");
                if ((stack_end.offset_from(sp) as i64)
                    < (2 as uint32_t)
                        .wrapping_mul(val2.wrapping_sub(val).wrapping_add(1 as uint32_t))
                        as i64) as i32 as i64
                    != 0
                {
                    let mut saved_sp_2: size_t = sp.offset_from((*s).stack_buf)
                        as i64 as size_t;
                    let mut saved_bp_2: size_t = bp.offset_from((*s).stack_buf)
                        as i64 as size_t;
                    if stack_realloc(
                        s,
                        (sp.offset_from((*s).stack_buf) as i64
                            + (2 as uint32_t)
                                .wrapping_mul(
                                    val2.wrapping_sub(val).wrapping_add(1 as uint32_t),
                                ) as i64) as size_t,
                    ) != 0
                    {
                        return LRE_RET_MEMORY_ERROR as intptr_t;
                    }
                    stack_end = ((*s).stack_buf).offset((*s).stack_size as isize);
                    sp = ((*s).stack_buf).offset(saved_sp_2 as isize);
                    bp = ((*s).stack_buf).offset(saved_bp_2 as isize);
                }
                while val <= val2 {
                    idx = (2 as uint32_t).wrapping_mul(val);
                    if ((stack_end.offset_from(sp) as i64)
                        < 2 as i64) as i32
                        as i64 != 0
                    {
                        let mut saved_sp_3: size_t = sp.offset_from((*s).stack_buf)
                            as i64 as size_t;
                        let mut saved_bp_3: size_t = bp.offset_from((*s).stack_buf)
                            as i64 as size_t;
                        if stack_realloc(
                            s,
                            (sp.offset_from((*s).stack_buf) as i64
                                + 2 as i64) as size_t,
                        ) != 0
                        {
                            return LRE_RET_MEMORY_ERROR as intptr_t;
                        }
                        stack_end = ((*s).stack_buf).offset((*s).stack_size as isize);
                        sp = ((*s).stack_buf).offset(saved_sp_3 as isize);
                        bp = ((*s).stack_buf).offset(saved_bp_3 as isize);
                    }
                    (*sp.offset(0 as i32 as isize)).val = idx as intptr_t;
                    let ref mut fresh76 = (*sp.offset(1 as i32 as isize))
                        .ptr;
                    *fresh76 = *capture.offset(idx as isize);
                    sp = sp.offset(2 as i32 as isize);
                    let ref mut fresh77 = *capture.offset(idx as isize);
                    *fresh77 = 0 as *mut uint8_t;
                    idx = (2 as uint32_t).wrapping_mul(val).wrapping_add(1 as uint32_t);
                    if ((stack_end.offset_from(sp) as i64)
                        < 2 as i64) as i32
                        as i64 != 0
                    {
                        let mut saved_sp_4: size_t = sp.offset_from((*s).stack_buf)
                            as i64 as size_t;
                        let mut saved_bp_4: size_t = bp.offset_from((*s).stack_buf)
                            as i64 as size_t;
                        if stack_realloc(
                            s,
                            (sp.offset_from((*s).stack_buf) as i64
                                + 2 as i64) as size_t,
                        ) != 0
                        {
                            return LRE_RET_MEMORY_ERROR as intptr_t;
                        }
                        stack_end = ((*s).stack_buf).offset((*s).stack_size as isize);
                        sp = ((*s).stack_buf).offset(saved_sp_4 as isize);
                        bp = ((*s).stack_buf).offset(saved_bp_4 as isize);
                    }
                    (*sp.offset(0 as i32 as isize)).val = idx as intptr_t;
                    let ref mut fresh78 = (*sp.offset(1 as i32 as isize))
                        .ptr;
                    *fresh78 = *capture.offset(idx as isize);
                    sp = sp.offset(2 as i32 as isize);
                    let ref mut fresh79 = *capture.offset(idx as isize);
                    *fresh79 = 0 as *mut uint8_t;
                    val = val.wrapping_add(1);
                }
                continue;
            }
            27 => {
                idx = (2 as i32 * (*s).capture_count
                    + *pc.offset(0 as i32 as isize) as i32)
                    as uint32_t;
                val = get_u32(pc.offset(1 as i32 as isize));
                pc = pc.offset(5 as i32 as isize);
                let mut sp1_0: *mut StackElem = 0 as *mut StackElem;
                sp1_0 = sp;
                loop {
                    if sp1_0 > bp {
                        if (*sp1_0.offset(-(2 as i32) as isize)).val
                            == idx as intptr_t
                        {
                            break;
                        }
                        sp1_0 = sp1_0.offset(-(2 as i32 as isize));
                    } else {
                        if ((stack_end.offset_from(sp) as i64)
                            < 2 as i64) as i32
                            as i64 != 0
                        {
                            let mut saved_sp_5: size_t = sp.offset_from((*s).stack_buf)
                                as i64 as size_t;
                            let mut saved_bp_5: size_t = bp.offset_from((*s).stack_buf)
                                as i64 as size_t;
                            if stack_realloc(
                                s,
                                (sp.offset_from((*s).stack_buf) as i64
                                    + 2 as i64) as size_t,
                            ) != 0
                            {
                                return LRE_RET_MEMORY_ERROR as intptr_t;
                            }
                            stack_end = ((*s).stack_buf)
                                .offset((*s).stack_size as isize);
                            sp = ((*s).stack_buf).offset(saved_sp_5 as isize);
                            bp = ((*s).stack_buf).offset(saved_bp_5 as isize);
                        }
                        (*sp.offset(0 as i32 as isize)).val = idx
                            as intptr_t;
                        let ref mut fresh80 = (*sp
                            .offset(1 as i32 as isize))
                            .ptr;
                        *fresh80 = *capture.offset(idx as isize);
                        sp = sp.offset(2 as i32 as isize);
                        break;
                    }
                }
                let ref mut fresh81 = *capture.offset(idx as isize);
                *fresh81 = val as uintptr_t as *mut std::ffi::c_void as *mut uint8_t;
                continue;
            }
            22 => {
                let mut val2_0: uint32_t = 0;
                idx = (2 as i32 * (*s).capture_count
                    + *pc.offset(0 as i32 as isize) as i32)
                    as uint32_t;
                val = get_u32(pc.offset(1 as i32 as isize));
                pc = pc.offset(5 as i32 as isize);
                val2_0 = (*capture.offset(idx as isize) as uintptr_t)
                    .wrapping_sub(1 as uintptr_t) as uint32_t;
                let mut sp1_1: *mut StackElem = 0 as *mut StackElem;
                sp1_1 = sp;
                loop {
                    if sp1_1 > bp {
                        if (*sp1_1.offset(-(2 as i32) as isize)).val
                            == idx as intptr_t
                        {
                            break;
                        }
                        sp1_1 = sp1_1.offset(-(2 as i32 as isize));
                    } else {
                        if ((stack_end.offset_from(sp) as i64)
                            < 2 as i64) as i32
                            as i64 != 0
                        {
                            let mut saved_sp_6: size_t = sp.offset_from((*s).stack_buf)
                                as i64 as size_t;
                            let mut saved_bp_6: size_t = bp.offset_from((*s).stack_buf)
                                as i64 as size_t;
                            if stack_realloc(
                                s,
                                (sp.offset_from((*s).stack_buf) as i64
                                    + 2 as i64) as size_t,
                            ) != 0
                            {
                                return LRE_RET_MEMORY_ERROR as intptr_t;
                            }
                            stack_end = ((*s).stack_buf)
                                .offset((*s).stack_size as isize);
                            sp = ((*s).stack_buf).offset(saved_sp_6 as isize);
                            bp = ((*s).stack_buf).offset(saved_bp_6 as isize);
                        }
                        (*sp.offset(0 as i32 as isize)).val = idx
                            as intptr_t;
                        let ref mut fresh82 = (*sp
                            .offset(1 as i32 as isize))
                            .ptr;
                        *fresh82 = *capture.offset(idx as isize);
                        sp = sp.offset(2 as i32 as isize);
                        break;
                    }
                }
                let ref mut fresh83 = *capture.offset(idx as isize);
                *fresh83 = val2_0 as uintptr_t as *mut std::ffi::c_void as *mut uint8_t;
                if val2_0 != 0 as uint32_t {
                    pc = pc.offset(val as i32 as isize);
                    if lre_poll_timeout(s) != 0 {
                        return LRE_RET_TIMEOUT as intptr_t;
                    }
                }
                continue;
            }
            23 | 24 | 25 | 26 => {
                let mut pc1_0: *const uint8_t = 0 as *const uint8_t;
                let mut val2_1: uint32_t = 0;
                let mut limit: uint32_t = 0;
                idx = (2 as i32 * (*s).capture_count
                    + *pc.offset(0 as i32 as isize) as i32)
                    as uint32_t;
                limit = get_u32(pc.offset(1 as i32 as isize));
                val = get_u32(pc.offset(5 as i32 as isize));
                pc = pc.offset(9 as i32 as isize);
                val2_1 = (*capture.offset(idx as isize) as uintptr_t)
                    .wrapping_sub(1 as uintptr_t) as uint32_t;
                let mut sp1_2: *mut StackElem = 0 as *mut StackElem;
                sp1_2 = sp;
                loop {
                    if sp1_2 > bp {
                        if (*sp1_2.offset(-(2 as i32) as isize)).val
                            == idx as intptr_t
                        {
                            break;
                        }
                        sp1_2 = sp1_2.offset(-(2 as i32 as isize));
                    } else {
                        if ((stack_end.offset_from(sp) as i64)
                            < 2 as i64) as i32
                            as i64 != 0
                        {
                            let mut saved_sp_7: size_t = sp.offset_from((*s).stack_buf)
                                as i64 as size_t;
                            let mut saved_bp_7: size_t = bp.offset_from((*s).stack_buf)
                                as i64 as size_t;
                            if stack_realloc(
                                s,
                                (sp.offset_from((*s).stack_buf) as i64
                                    + 2 as i64) as size_t,
                            ) != 0
                            {
                                return LRE_RET_MEMORY_ERROR as intptr_t;
                            }
                            stack_end = ((*s).stack_buf)
                                .offset((*s).stack_size as isize);
                            sp = ((*s).stack_buf).offset(saved_sp_7 as isize);
                            bp = ((*s).stack_buf).offset(saved_bp_7 as isize);
                        }
                        (*sp.offset(0 as i32 as isize)).val = idx
                            as intptr_t;
                        let ref mut fresh84 = (*sp
                            .offset(1 as i32 as isize))
                            .ptr;
                        *fresh84 = *capture.offset(idx as isize);
                        sp = sp.offset(2 as i32 as isize);
                        break;
                    }
                }
                let ref mut fresh85 = *capture.offset(idx as isize);
                *fresh85 = val2_1 as uintptr_t as *mut std::ffi::c_void as *mut uint8_t;
                if val2_1 > limit {
                    pc = pc.offset(val as i32 as isize);
                    if lre_poll_timeout(s) != 0 {
                        return LRE_RET_TIMEOUT as intptr_t;
                    }
                    continue;
                } else if !((opcode
                    == REOP_loop_check_adv_split_goto_first as i32
                    || opcode
                        == REOP_loop_check_adv_split_next_first as i32)
                    && *capture.offset(idx.wrapping_add(1 as uint32_t) as isize)
                        == cptr as *mut uint8_t && val2_1 != limit)
                {
                    if val2_1 != 0 as uint32_t {
                        if opcode == REOP_loop_split_next_first as i32
                            || opcode
                                == REOP_loop_check_adv_split_next_first as i32
                        {
                            pc1_0 = pc.offset(val as i32 as isize);
                        } else {
                            pc1_0 = pc;
                            pc = pc.offset(val as i32 as isize);
                        }
                        if ((stack_end.offset_from(sp) as i64)
                            < 3 as i64) as i32
                            as i64 != 0
                        {
                            let mut saved_sp_8: size_t = sp.offset_from((*s).stack_buf)
                                as i64 as size_t;
                            let mut saved_bp_8: size_t = bp.offset_from((*s).stack_buf)
                                as i64 as size_t;
                            if stack_realloc(
                                s,
                                (sp.offset_from((*s).stack_buf) as i64
                                    + 3 as i64) as size_t,
                            ) != 0
                            {
                                return LRE_RET_MEMORY_ERROR as intptr_t;
                            }
                            stack_end = ((*s).stack_buf)
                                .offset((*s).stack_size as isize);
                            sp = ((*s).stack_buf).offset(saved_sp_8 as isize);
                            bp = ((*s).stack_buf).offset(saved_bp_8 as isize);
                        }
                        let ref mut fresh86 = (*sp
                            .offset(0 as i32 as isize))
                            .ptr;
                        *fresh86 = pc1_0 as *mut uint8_t;
                        let ref mut fresh87 = (*sp
                            .offset(1 as i32 as isize))
                            .ptr;
                        *fresh87 = cptr as *mut uint8_t;
                        let ref mut fresh88 = (*sp
                            .offset(2 as i32 as isize))
                            .bp;
                        (*fresh88)
                            .set_val(
                                bp.offset_from((*s).stack_buf) as i64 as u64,
                            );
                        let ref mut fresh89 = (*sp
                            .offset(2 as i32 as isize))
                            .bp;
                        (*fresh89)
                            .set_type_0(
                                RE_EXEC_STATE_SPLIT as i32 as u64,
                            );
                        sp = sp.offset(3 as i32 as isize);
                        bp = sp;
                    }
                    continue;
                }
                current_block = 1885734024781174349;
            }
            42 => {
                idx = (2 as i32 * (*s).capture_count
                    + *pc.offset(0 as i32 as isize) as i32)
                    as uint32_t;
                pc = pc.offset(1);
                let mut sp1_3: *mut StackElem = 0 as *mut StackElem;
                sp1_3 = sp;
                loop {
                    if sp1_3 > bp {
                        if (*sp1_3.offset(-(2 as i32) as isize)).val
                            == idx as intptr_t
                        {
                            break;
                        }
                        sp1_3 = sp1_3.offset(-(2 as i32 as isize));
                    } else {
                        if ((stack_end.offset_from(sp) as i64)
                            < 2 as i64) as i32
                            as i64 != 0
                        {
                            let mut saved_sp_9: size_t = sp.offset_from((*s).stack_buf)
                                as i64 as size_t;
                            let mut saved_bp_9: size_t = bp.offset_from((*s).stack_buf)
                                as i64 as size_t;
                            if stack_realloc(
                                s,
                                (sp.offset_from((*s).stack_buf) as i64
                                    + 2 as i64) as size_t,
                            ) != 0
                            {
                                return LRE_RET_MEMORY_ERROR as intptr_t;
                            }
                            stack_end = ((*s).stack_buf)
                                .offset((*s).stack_size as isize);
                            sp = ((*s).stack_buf).offset(saved_sp_9 as isize);
                            bp = ((*s).stack_buf).offset(saved_bp_9 as isize);
                        }
                        (*sp.offset(0 as i32 as isize)).val = idx
                            as intptr_t;
                        let ref mut fresh90 = (*sp
                            .offset(1 as i32 as isize))
                            .ptr;
                        *fresh90 = *capture.offset(idx as isize);
                        sp = sp.offset(2 as i32 as isize);
                        break;
                    }
                }
                let ref mut fresh91 = *capture.offset(idx as isize);
                *fresh91 = cptr as *mut uint8_t;
                continue;
            }
            43 => {
                idx = (2 as i32 * (*s).capture_count
                    + *pc.offset(0 as i32 as isize) as i32)
                    as uint32_t;
                pc = pc.offset(1);
                if !(*capture.offset(idx as isize) == cptr as *mut uint8_t) {
                    continue;
                }
                current_block = 1885734024781174349;
            }
            28 | 29 | 30 | 31 => {
                let mut v1: BOOL = 0;
                let mut v2: BOOL = 0;
                let mut ignore_case: i32 = (opcode
                    == REOP_word_boundary_i as i32
                    || opcode == REOP_not_word_boundary_i as i32)
                    as i32;
                let mut is_boundary: BOOL = (opcode
                    == REOP_word_boundary as i32
                    || opcode == REOP_word_boundary_i as i32)
                    as i32;
                if cptr == (*s).cbuf {
                    v1 = FALSE as i32 as BOOL;
                } else {
                    if cbuf_type == 0 as i32 {
                        c = *cptr.offset(-(1 as i32) as isize) as uint32_t;
                    } else {
                        let mut _p_6: *const uint16_t = (cptr as *const uint16_t)
                            .offset(-(1 as i32 as isize));
                        let mut _start_0: *const uint16_t = (*s).cbuf as *const uint16_t;
                        c = *_p_6 as uint32_t;
                        if is_lo_surrogate(c) != 0 && cbuf_type == 2 as i32
                        {
                            if _p_6 > _start_0
                                && is_hi_surrogate(
                                    *_p_6.offset(-(1 as i32) as isize) as uint32_t,
                                ) != 0
                            {
                                _p_6 = _p_6.offset(-1);
                                c = from_surrogate(*_p_6 as uint32_t, c);
                            }
                        }
                    }
                    if c < 256 as uint32_t {
                        v1 = (lre_is_word_byte(c as uint8_t) != 0 as i32)
                            as i32 as BOOL;
                    } else {
                        v1 = (ignore_case != 0
                            && (c == 0x17f as uint32_t || c == 0x212a as uint32_t))
                            as i32 as BOOL;
                    }
                }
                if cptr >= cbuf_end {
                    v2 = FALSE as i32 as BOOL;
                } else {
                    if cbuf_type == 0 as i32 {
                        c = *cptr.offset(0 as i32 as isize) as uint32_t;
                    } else {
                        let mut _p_7: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_5: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh92 = _p_7;
                        _p_7 = _p_7.offset(1);
                        c = *fresh92 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as i32
                        {
                            if _p_7 < _end_5 && is_lo_surrogate(*_p_7 as uint32_t) != 0 {
                                c = from_surrogate(c, *_p_7 as uint32_t);
                            }
                        }
                    }
                    if c < 256 as uint32_t {
                        v2 = (lre_is_word_byte(c as uint8_t) != 0 as i32)
                            as i32 as BOOL;
                    } else {
                        v2 = (ignore_case != 0
                            && (c == 0x17f as uint32_t || c == 0x212a as uint32_t))
                            as i32 as BOOL;
                    }
                }
                if !(v1 ^ v2 ^ is_boundary != 0) {
                    continue;
                }
                current_block = 1885734024781174349;
            }
            32 | 33 | 34 | 35 => {
                let mut cptr1: *const uint8_t = 0 as *const uint8_t;
                let mut cptr1_end: *const uint8_t = 0 as *const uint8_t;
                let mut cptr1_start: *const uint8_t = 0 as *const uint8_t;
                let mut pc1_1: *const uint8_t = 0 as *const uint8_t;
                let mut c1: uint32_t = 0;
                let mut c2: uint32_t = 0;
                let mut i: i32 = 0;
                let mut n: i32 = 0;
                let fresh93 = pc;
                pc = pc.offset(1);
                n = *fresh93 as i32;
                pc1_1 = pc;
                pc = pc.offset(n as isize);
                i = 0 as i32;
                's_2002: loop {
                    if !(i < n) {
                        continue 's_31;
                    }
                    val = *pc1_1.offset(i as isize) as uint32_t;
                    if val >= (*s).capture_count as uint32_t {
                        break;
                    }
                    cptr1_start = *capture
                        .offset((2 as uint32_t).wrapping_mul(val) as isize);
                    cptr1_end = *capture
                        .offset(
                            (2 as uint32_t).wrapping_mul(val).wrapping_add(1 as uint32_t)
                                as isize,
                        );
                    if !cptr1_start.is_null() && !cptr1_end.is_null() {
                        if opcode == REOP_back_reference as i32
                            || opcode == REOP_back_reference_i as i32
                        {
                            cptr1 = cptr1_start;
                            loop {
                                if !(cptr1 < cptr1_end) {
                                    continue 's_31;
                                }
                                if cptr >= cbuf_end {
                                    break 's_2002;
                                }
                                if cbuf_type == 0 as i32 {
                                    let fresh94 = cptr1;
                                    cptr1 = cptr1.offset(1);
                                    c1 = *fresh94 as uint32_t;
                                } else {
                                    let mut _p_8: *const uint16_t = cptr1 as *const uint16_t;
                                    let mut _end_6: *const uint16_t = cptr1_end
                                        as *const uint16_t;
                                    let fresh95 = _p_8;
                                    _p_8 = _p_8.offset(1);
                                    c1 = *fresh95 as uint32_t;
                                    if is_hi_surrogate(c1) != 0
                                        && cbuf_type == 2 as i32
                                    {
                                        if _p_8 < _end_6 && is_lo_surrogate(*_p_8 as uint32_t) != 0
                                        {
                                            let fresh96 = _p_8;
                                            _p_8 = _p_8.offset(1);
                                            c1 = from_surrogate(c1, *fresh96 as uint32_t);
                                        }
                                    }
                                    cptr1 = _p_8 as *const std::ffi::c_void as *const uint8_t;
                                }
                                if cbuf_type == 0 as i32 {
                                    let fresh97 = cptr;
                                    cptr = cptr.offset(1);
                                    c2 = *fresh97 as uint32_t;
                                } else {
                                    let mut _p_9: *const uint16_t = cptr as *const uint16_t;
                                    let mut _end_7: *const uint16_t = cbuf_end
                                        as *const uint16_t;
                                    let fresh98 = _p_9;
                                    _p_9 = _p_9.offset(1);
                                    c2 = *fresh98 as uint32_t;
                                    if is_hi_surrogate(c2) != 0
                                        && cbuf_type == 2 as i32
                                    {
                                        if _p_9 < _end_7 && is_lo_surrogate(*_p_9 as uint32_t) != 0
                                        {
                                            let fresh99 = _p_9;
                                            _p_9 = _p_9.offset(1);
                                            c2 = from_surrogate(c2, *fresh99 as uint32_t);
                                        }
                                    }
                                    cptr = _p_9 as *const std::ffi::c_void as *const uint8_t;
                                }
                                if opcode == REOP_back_reference_i as i32 {
                                    c1 = lre_canonicalize(
                                        c1,
                                        (*s).is_unicode as i32,
                                    ) as uint32_t;
                                    c2 = lre_canonicalize(
                                        c2,
                                        (*s).is_unicode as i32,
                                    ) as uint32_t;
                                }
                                if c1 != c2 {
                                    break 's_2002;
                                }
                            }
                        } else {
                            cptr1 = cptr1_end;
                            loop {
                                if !(cptr1 > cptr1_start) {
                                    continue 's_31;
                                }
                                if cptr == (*s).cbuf {
                                    break 's_2002;
                                }
                                if cbuf_type == 0 as i32 {
                                    cptr1 = cptr1.offset(-1);
                                    c1 = *cptr1.offset(0 as i32 as isize)
                                        as uint32_t;
                                } else {
                                    let mut _p_10: *const uint16_t = (cptr1 as *const uint16_t)
                                        .offset(-(1 as i32 as isize));
                                    let mut _start_1: *const uint16_t = cptr1_start
                                        as *const uint16_t;
                                    c1 = *_p_10 as uint32_t;
                                    if is_lo_surrogate(c1) != 0
                                        && cbuf_type == 2 as i32
                                    {
                                        if _p_10 > _start_1
                                            && is_hi_surrogate(
                                                *_p_10.offset(-(1 as i32) as isize) as uint32_t,
                                            ) != 0
                                        {
                                            _p_10 = _p_10.offset(-1);
                                            c1 = from_surrogate(*_p_10 as uint32_t, c1);
                                        }
                                    }
                                    cptr1 = _p_10 as *const std::ffi::c_void as *const uint8_t;
                                }
                                if cbuf_type == 0 as i32 {
                                    cptr = cptr.offset(-1);
                                    c2 = *cptr.offset(0 as i32 as isize)
                                        as uint32_t;
                                } else {
                                    let mut _p_11: *const uint16_t = (cptr as *const uint16_t)
                                        .offset(-(1 as i32 as isize));
                                    let mut _start_2: *const uint16_t = (*s).cbuf
                                        as *const uint16_t;
                                    c2 = *_p_11 as uint32_t;
                                    if is_lo_surrogate(c2) != 0
                                        && cbuf_type == 2 as i32
                                    {
                                        if _p_11 > _start_2
                                            && is_hi_surrogate(
                                                *_p_11.offset(-(1 as i32) as isize) as uint32_t,
                                            ) != 0
                                        {
                                            _p_11 = _p_11.offset(-1);
                                            c2 = from_surrogate(*_p_11 as uint32_t, c2);
                                        }
                                    }
                                    cptr = _p_11 as *const std::ffi::c_void as *const uint8_t;
                                }
                                if opcode
                                    == REOP_backward_back_reference_i as i32
                                {
                                    c1 = lre_canonicalize(
                                        c1,
                                        (*s).is_unicode as i32,
                                    ) as uint32_t;
                                    c2 = lre_canonicalize(
                                        c2,
                                        (*s).is_unicode as i32,
                                    ) as uint32_t;
                                }
                                if c1 != c2 {
                                    break 's_2002;
                                }
                            }
                        }
                    } else {
                        i += 1;
                    }
                }
                current_block = 1885734024781174349;
            }
            36 | 37 => {
                let mut n_0: i32 = 0;
                let mut low: uint32_t = 0;
                let mut high: uint32_t = 0;
                let mut idx_min: uint32_t = 0;
                let mut idx_max: uint32_t = 0;
                let mut idx_0: uint32_t = 0;
                n_0 = get_u16(pc) as i32;
                pc = pc.offset(2 as i32 as isize);
                if cptr >= cbuf_end {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as i32 {
                        let fresh100 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh100 as uint32_t;
                    } else {
                        let mut _p_12: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_8: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh101 = _p_12;
                        _p_12 = _p_12.offset(1);
                        c = *fresh101 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as i32
                        {
                            if _p_12 < _end_8 && is_lo_surrogate(*_p_12 as uint32_t) != 0
                            {
                                let fresh102 = _p_12;
                                _p_12 = _p_12.offset(1);
                                c = from_surrogate(c, *fresh102 as uint32_t);
                            }
                        }
                        cptr = _p_12 as *const std::ffi::c_void as *const uint8_t;
                    }
                    if opcode == REOP_range_i as i32 {
                        c = lre_canonicalize(c, (*s).is_unicode as i32)
                            as uint32_t;
                    }
                    idx_min = 0 as uint32_t;
                    low = get_u16(
                        pc
                            .offset(
                                (0 as i32 * 4 as i32) as isize,
                            ),
                    );
                    if c < low {
                        current_block = 1885734024781174349;
                    } else {
                        idx_max = (n_0 - 1 as i32) as uint32_t;
                        high = get_u16(
                            pc
                                .offset(idx_max.wrapping_mul(4 as uint32_t) as isize)
                                .offset(2 as i32 as isize),
                        );
                        if (c >= 0xffff as uint32_t) as i32
                            as i64 != 0 && high == 0xffff as uint32_t
                        {
                            current_block = 12610801258230029348;
                        } else if c > high {
                            current_block = 1885734024781174349;
                        } else {
                            loop {
                                if !(idx_min <= idx_max) {
                                    current_block = 1885734024781174349;
                                    break;
                                }
                                idx_0 = idx_min
                                    .wrapping_add(idx_max)
                                    .wrapping_div(2 as uint32_t);
                                low = get_u16(
                                    pc.offset(idx_0.wrapping_mul(4 as uint32_t) as isize),
                                );
                                high = get_u16(
                                    pc
                                        .offset(idx_0.wrapping_mul(4 as uint32_t) as isize)
                                        .offset(2 as i32 as isize),
                                );
                                if c < low {
                                    idx_max = idx_0.wrapping_sub(1 as uint32_t);
                                } else {
                                    if !(c > high) {
                                        current_block = 12610801258230029348;
                                        break;
                                    }
                                    idx_min = idx_0.wrapping_add(1 as uint32_t);
                                }
                            }
                        }
                        match current_block {
                            1885734024781174349 => {}
                            _ => {
                                pc = pc.offset((4 as i32 * n_0) as isize);
                                continue;
                            }
                        }
                    }
                }
            }
            38 | 39 => {
                let mut n_1: i32 = 0;
                let mut low_0: uint32_t = 0;
                let mut high_0: uint32_t = 0;
                let mut idx_min_0: uint32_t = 0;
                let mut idx_max_0: uint32_t = 0;
                let mut idx_1: uint32_t = 0;
                n_1 = get_u16(pc) as i32;
                pc = pc.offset(2 as i32 as isize);
                if cptr >= cbuf_end {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as i32 {
                        let fresh103 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh103 as uint32_t;
                    } else {
                        let mut _p_13: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_9: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh104 = _p_13;
                        _p_13 = _p_13.offset(1);
                        c = *fresh104 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as i32
                        {
                            if _p_13 < _end_9 && is_lo_surrogate(*_p_13 as uint32_t) != 0
                            {
                                let fresh105 = _p_13;
                                _p_13 = _p_13.offset(1);
                                c = from_surrogate(c, *fresh105 as uint32_t);
                            }
                        }
                        cptr = _p_13 as *const std::ffi::c_void as *const uint8_t;
                    }
                    if opcode == REOP_range32_i as i32 {
                        c = lre_canonicalize(c, (*s).is_unicode as i32)
                            as uint32_t;
                    }
                    idx_min_0 = 0 as uint32_t;
                    low_0 = get_u32(
                        pc
                            .offset(
                                (0 as i32 * 8 as i32) as isize,
                            ),
                    );
                    if c < low_0 {
                        current_block = 1885734024781174349;
                    } else {
                        idx_max_0 = (n_1 - 1 as i32) as uint32_t;
                        high_0 = get_u32(
                            pc
                                .offset(idx_max_0.wrapping_mul(8 as uint32_t) as isize)
                                .offset(4 as i32 as isize),
                        );
                        if c > high_0 {
                            current_block = 1885734024781174349;
                        } else {
                            loop {
                                if !(idx_min_0 <= idx_max_0) {
                                    current_block = 1885734024781174349;
                                    break;
                                }
                                idx_1 = idx_min_0
                                    .wrapping_add(idx_max_0)
                                    .wrapping_div(2 as uint32_t);
                                low_0 = get_u32(
                                    pc.offset(idx_1.wrapping_mul(8 as uint32_t) as isize),
                                );
                                high_0 = get_u32(
                                    pc
                                        .offset(idx_1.wrapping_mul(8 as uint32_t) as isize)
                                        .offset(4 as i32 as isize),
                                );
                                if c < low_0 {
                                    idx_max_0 = idx_1.wrapping_sub(1 as uint32_t);
                                } else {
                                    if !(c > high_0) {
                                        current_block = 11336263301252774654;
                                        break;
                                    }
                                    idx_min_0 = idx_1.wrapping_add(1 as uint32_t);
                                }
                            }
                            match current_block {
                                1885734024781174349 => {}
                                _ => {
                                    pc = pc.offset((8 as i32 * n_1) as isize);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            44 => {
                if cptr == (*s).cbuf {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as i32 {
                        cptr = cptr.offset(-1);
                    } else {
                        let mut _p_14: *const uint16_t = (cptr as *const uint16_t)
                            .offset(-(1 as i32 as isize));
                        let mut _start_3: *const uint16_t = (*s).cbuf as *const uint16_t;
                        if is_lo_surrogate(*_p_14 as uint32_t) != 0
                            && cbuf_type == 2 as i32
                        {
                            if _p_14 > _start_3
                                && is_hi_surrogate(
                                    *_p_14.offset(-(1 as i32) as isize) as uint32_t,
                                ) != 0
                            {
                                _p_14 = _p_14.offset(-1);
                            }
                        }
                        cptr = _p_14 as *const std::ffi::c_void as *const uint8_t;
                    }
                    continue;
                }
            }
            _ => {
                std::process::abort();
            }
        }
        match current_block {
            13538245850655155981 => {
                if !(cptr >= cbuf_end) {
                    if cbuf_type == 0 as i32 {
                        let fresh49 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh49 as uint32_t;
                    } else {
                        let mut _p: *const uint16_t = cptr as *const uint16_t;
                        let mut _end: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh50 = _p;
                        _p = _p.offset(1);
                        c = *fresh50 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as i32
                        {
                            if _p < _end && is_lo_surrogate(*_p as uint32_t) != 0 {
                                let fresh51 = _p;
                                _p = _p.offset(1);
                                c = from_surrogate(c, *fresh51 as uint32_t);
                            }
                        }
                        cptr = _p as *const std::ffi::c_void as *const uint8_t;
                    }
                    if opcode == REOP_char_i as i32
                        || opcode == REOP_char32_i as i32
                    {
                        c = lre_canonicalize(c, (*s).is_unicode as i32)
                            as uint32_t;
                    }
                    if !(val != c) {
                        continue;
                    }
                }
            }
            _ => {}
        }
        loop {
            let mut type_0: REExecStateEnum = RE_EXEC_STATE_SPLIT;
            if bp == (*s).stack_buf {
                return 0 as intptr_t;
            }
            while sp > bp {
                let ref mut fresh44 = *capture
                    .offset(
                        (*sp.offset(-(2 as i32) as isize)).val as isize,
                    );
                *fresh44 = (*sp.offset(-(1 as i32) as isize)).ptr;
                sp = sp.offset(-(2 as i32 as isize));
            }
            pc = (*sp.offset(-(3 as i32) as isize)).ptr;
            cptr = (*sp.offset(-(2 as i32) as isize)).ptr;
            type_0 = ((*sp.offset(-(1 as i32) as isize)).bp).type_0()
                as REExecStateEnum;
            bp = ((*s).stack_buf)
                .offset(
                    ((*sp.offset(-(1 as i32) as isize)).bp).val() as isize,
                );
            sp = sp.offset(-(3 as i32 as isize));
            if type_0 as u32
                != RE_EXEC_STATE_LOOKAHEAD as i32 as u32
            {
                break;
            }
        }
        if lre_poll_timeout(s) != 0 {
            return LRE_RET_TIMEOUT as intptr_t;
        }
    };
    } // close unsafe block
}
#[no_mangle]
pub fn lre_exec(
    mut capture: *mut *mut uint8_t,
    mut bc_buf: *const uint8_t,
    mut cbuf: *const uint8_t,
    mut cindex: i32,
    mut clen: i32,
    mut cbuf_type: i32,
    mut opaque: *mut std::ffi::c_void,
) -> i32 {
    // SAFETY: All pointers are valid from the caller (mod.rs public API)
    unsafe {
    let mut s_s: REExecContext = REExecContext {
        cbuf: 0 as *const uint8_t,
        cbuf_end: 0 as *const uint8_t,
        cbuf_type: 0,
        capture_count: 0,
        is_unicode: 0,
        interrupt_counter: 0,
        opaque: 0 as *mut std::ffi::c_void,
        stack_buf: 0 as *mut StackElem,
        stack_size: 0,
        static_stack_buf: [StackElem {
            ptr: 0 as *mut uint8_t,
        }; 32],
    };
    let mut s: *mut REExecContext = &mut s_s;
    let mut re_flags: i32 = 0;
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    let mut cptr: *const uint8_t = 0 as *const uint8_t;
    let bc_header = std::slice::from_raw_parts(bc_buf, RE_HEADER_LEN as usize);
    re_flags = lre_get_flags(bc_header);
    (*s).is_unicode = (re_flags & (LRE_FLAG_UNICODE | LRE_FLAG_UNICODE_SETS)
        != 0 as i32) as i32 as BOOL;
    (*s).capture_count = *bc_buf.offset(RE_HEADER_CAPTURE_COUNT as isize)
        as i32;
    (*s).cbuf = cbuf;
    (*s).cbuf_end = cbuf.offset((clen << cbuf_type) as isize);
    (*s).cbuf_type = cbuf_type;
    if (*s).cbuf_type == 1 as i32 && (*s).is_unicode != 0 {
        (*s).cbuf_type = 2 as i32;
    }
    (*s).interrupt_counter = INTERRUPT_COUNTER_INIT;
    (*s).opaque = opaque;
    (*s).stack_buf = ((*s).static_stack_buf).as_mut_ptr();
    (*s).stack_size = (::core::mem::size_of::<[StackElem; 32]>() as usize)
        .wrapping_div(::core::mem::size_of::<StackElem>() as usize) as size_t;
    i = 0 as i32;
    while i < (*s).capture_count * 2 as i32 {
        let ref mut fresh42 = *capture.offset(i as isize);
        *fresh42 = 0 as *mut uint8_t;
        i += 1;
    }
    cptr = cbuf.offset((cindex << cbuf_type) as isize);
    if (0 as i32) < cindex && cindex < clen
        && (*s).cbuf_type == 2 as i32
    {
        let mut p: *const uint16_t = cptr as *const uint16_t;
        if is_lo_surrogate(*p as uint32_t) != 0
            && is_hi_surrogate(*p.offset(-(1 as i32) as isize) as uint32_t)
                != 0
        {
            cptr = p.offset(-(1 as i32 as isize)) as *const uint8_t;
        }
    }
    ret = lre_exec_backtrack(s, capture, bc_buf.offset(RE_HEADER_LEN as isize), cptr)
        as i32;
    if (*s).stack_buf != ((*s).static_stack_buf).as_mut_ptr() {
        lre_realloc((*s).opaque, (*s).stack_buf as *mut std::ffi::c_void, 0 as size_t);
    }
    return ret;
    } // close unsafe block
}
/// Gets the allocation count from bytecode header.
pub(crate) fn lre_get_alloc_count(bc_buf: &[u8]) -> i32 {
    bc_buf[RE_HEADER_CAPTURE_COUNT as usize] as i32 * 2
        + bc_buf[RE_HEADER_REGISTER_COUNT as usize] as i32
}

/// Gets the capture count from bytecode header.
pub(crate) fn lre_get_capture_count(bc_buf: &[u8]) -> i32 {
    bc_buf[RE_HEADER_CAPTURE_COUNT as usize] as i32
}

/// Gets the flags from bytecode header.
pub(crate) fn lre_get_flags(bc_buf: &[u8]) -> i32 {
    get_u16_safe(&bc_buf[RE_HEADER_FLAGS as usize..]) as i32
}

/// Gets the bytecode length from header.
pub(crate) fn lre_get_bytecode_len(bc_buf: &[u8]) -> u32 {
    get_u32_safe(&bc_buf[RE_HEADER_BYTECODE_LEN as usize..])
}
pub fn lre_get_groupnames(bc_buf: *const uint8_t) -> *const i8 {
    // SAFETY: bc_buf is a valid bytecode buffer from compilation
    unsafe {
        let bc_header = std::slice::from_raw_parts(bc_buf, RE_HEADER_LEN as usize);
        if lre_get_flags(bc_header) & LRE_FLAG_NAMED_GROUPS == 0 as i32 {
            return 0 as *const i8;
        }
        let re_bytecode_len = get_u32(bc_buf.offset(RE_HEADER_BYTECODE_LEN as isize));
        return bc_buf.offset(RE_HEADER_LEN as isize).offset(re_bytecode_len as isize)
            as *const i8;
    }
}
#[inline]
fn cr_add_point(
    cr: *mut CharRange,
    v: uint32_t,
) -> i32 {
    // SAFETY: cr is a valid CharRange pointer managed by the parser
    unsafe {
        if (*cr).len >= (*cr).size {
            if cr_realloc(cr, (*cr).len + 1 as i32) != 0 {
                return -(1 as i32);
            }
        }
        let fresh38 = (*cr).len;
        (*cr).len = (*cr).len + 1;
        *((*cr).points).offset(fresh38 as isize) = v;
        return 0 as i32;
    }
}
#[inline]
fn cr_add_interval(
    cr: *mut CharRange,
    c1: uint32_t,
    c2: uint32_t,
) -> i32 {
    // SAFETY: cr is a valid CharRange pointer managed by the parser
    unsafe {
        if (*cr).len + 2 as i32 > (*cr).size {
            if cr_realloc(cr, (*cr).len + 2 as i32) != 0 {
                return -(1 as i32);
            }
        }
        let fresh39 = (*cr).len;
        (*cr).len = (*cr).len + 1;
        *((*cr).points).offset(fresh39 as isize) = c1;
        let fresh40 = (*cr).len;
        (*cr).len = (*cr).len + 1;
        *((*cr).points).offset(fresh40 as isize) = c2;
        return 0 as i32;
    }
}
#[inline]
fn cr_union_interval(
    cr: *mut CharRange,
    c1: uint32_t,
    c2: uint32_t,
) -> i32 {
    let mut b_pt: [uint32_t; 2] = [0; 2];
    b_pt[0 as i32 as usize] = c1;
    b_pt[1 as i32 as usize] = c2.wrapping_add(1 as uint32_t);
    return cr_op1(
        cr,
        b_pt.as_mut_ptr(),
        2 as i32,
        CR_OP_UNION as i32,
    );
}
/// Checks if a byte is a whitespace character.
#[inline]
fn lre_is_space_byte(c: uint8_t) -> i32 {
    lre_ctype_bits[c as usize] as i32 & UNICODE_C_SPACE as i32
}
/// Checks if a byte is valid as the start of an identifier.
#[inline]
fn lre_is_id_start_byte(c: uint8_t) -> i32 {
    lre_ctype_bits[c as usize] as i32
        & (UNICODE_C_UPPER as i32 | UNICODE_C_LOWER as i32
            | UNICODE_C_UNDER as i32
            | UNICODE_C_DOLLAR as i32)
}
/// Checks if a byte is valid for continuing an identifier.
#[inline]
fn lre_is_id_continue_byte(c: uint8_t) -> i32 {
    lre_ctype_bits[c as usize] as i32
        & (UNICODE_C_UPPER as i32 | UNICODE_C_LOWER as i32
            | UNICODE_C_UNDER as i32 | UNICODE_C_DOLLAR as i32
            | UNICODE_C_DIGIT as i32)
}
/// Checks if a byte is a word character (alphanumeric or underscore).
#[inline]
fn lre_is_word_byte(c: uint8_t) -> i32 {
    lre_ctype_bits[c as usize] as i32
        & (UNICODE_C_UPPER as i32 | UNICODE_C_LOWER as i32
            | UNICODE_C_UNDER as i32 | UNICODE_C_DIGIT as i32)
}
/// Checks if a Unicode codepoint is a whitespace character.
#[inline]
fn lre_is_space(c: uint32_t) -> i32 {
    if c < 256 as uint32_t {
        lre_is_space_byte(c as uint8_t)
    } else {
        lre_is_space_non_ascii(c)
    }
}
/// Checks if a character is a valid JavaScript identifier start character.
#[inline]
fn lre_js_is_ident_first(c: u32) -> i32 {
    if c < 128 {
        lre_is_id_start_byte(c as u8)
    } else {
        lre_is_id_start(c)
    }
}

/// Checks if a character is a valid JavaScript identifier continuation character.
#[inline]
fn lre_js_is_ident_next(c: u32) -> i32 {
    if c < 128 {
        lre_is_id_continue_byte(c as u8)
    } else {
        if c >= 0x200c && c <= 0x200d {
            return TRUE as i32;
        }
        lre_is_id_continue(c)
    }
}
pub const LRE_FLAG_DOTALL: i32 = (1 as i32)
    << 3 as i32;
pub const LRE_FLAG_UNICODE: i32 = (1 as i32)
    << 4 as i32;
pub const LRE_FLAG_STICKY: i32 = (1 as i32)
    << 5 as i32;
pub const LRE_FLAG_NAMED_GROUPS: i32 = (1 as i32)
    << 7 as i32;
pub const LRE_FLAG_UNICODE_SETS: i32 = (1 as i32)
    << 8 as i32;
pub const LRE_RET_MEMORY_ERROR: i32 = -(1 as i32);
pub const LRE_RET_TIMEOUT: i32 = -(2 as i32);
pub const LRE_GROUP_NAME_TRAILER_LEN: i32 = 2 as i32;

use ::c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;

// Use abort and __assert_fail from cutils
use super::cutils::abort;
use super::cutils::__assert_fail;

extern "C" {
    fn vsnprintf(
        __s: *mut core::ffi::c_char,
        __maxlen: size_t,
        __format: *const core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> core::ffi::c_int;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memmove(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memcmp(
        __s1: *const core::ffi::c_void,
        __s2: *const core::ffi::c_void,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn strcmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
    fn pstrcpy(
        buf: *mut core::ffi::c_char,
        buf_size: core::ffi::c_int,
        str: *const core::ffi::c_char,
    );
    fn dbuf_init2(
        s: *mut DynBuf,
        opaque: *mut core::ffi::c_void,
        realloc_func: Option<DynBufReallocFunc>,
    );
    fn dbuf_claim(s: *mut DynBuf, len: size_t) -> core::ffi::c_int;
    fn dbuf_put(s: *mut DynBuf, data: *const uint8_t, len: size_t) -> core::ffi::c_int;
    fn __dbuf_putc(s: *mut DynBuf, c: uint8_t) -> core::ffi::c_int;
    fn __dbuf_put_u16(s: *mut DynBuf, val: uint16_t) -> core::ffi::c_int;
    fn __dbuf_put_u32(s: *mut DynBuf, val: uint32_t) -> core::ffi::c_int;
    fn dbuf_free(s: *mut DynBuf);
    fn unicode_to_utf8(buf: *mut uint8_t, c: core::ffi::c_uint) -> core::ffi::c_int;
    fn unicode_from_utf8(
        p: *const uint8_t,
        max_len: core::ffi::c_int,
        pp: *mut *const uint8_t,
    ) -> core::ffi::c_int;
    fn rqsort(
        base: *mut core::ffi::c_void,
        nmemb: size_t,
        size: size_t,
        cmp: Option<
            unsafe extern "C" fn(
                *const core::ffi::c_void,
                *const core::ffi::c_void,
                *mut core::ffi::c_void,
            ) -> core::ffi::c_int,
        >,
        arg: *mut core::ffi::c_void,
    );
    fn cr_init(
        cr: *mut CharRange,
        mem_opaque: *mut core::ffi::c_void,
        realloc_func: Option<
            unsafe extern "C" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                size_t,
            ) -> *mut core::ffi::c_void,
        >,
    );
    fn cr_free(cr: *mut CharRange);
    fn cr_realloc(cr: *mut CharRange, size: core::ffi::c_int) -> core::ffi::c_int;
    fn cr_op1(
        cr: *mut CharRange,
        b_pt: *const uint32_t,
        b_len: core::ffi::c_int,
        op: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn cr_invert(cr: *mut CharRange) -> core::ffi::c_int;
    fn cr_regexp_canonicalize(
        cr: *mut CharRange,
        is_unicode: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn unicode_script(
        cr: *mut CharRange,
        script_name: *const core::ffi::c_char,
        is_ext: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn unicode_general_category(
        cr: *mut CharRange,
        gc_name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn unicode_prop(
        cr: *mut CharRange,
        prop_name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn unicode_sequence_prop(
        prop_name: *const core::ffi::c_char,
        cb: Option<UnicodeSequencePropCB>,
        opaque: *mut core::ffi::c_void,
        cr: *mut CharRange,
    ) -> core::ffi::c_int;
    fn lre_canonicalize(c: uint32_t, is_unicode: core::ffi::c_int) -> core::ffi::c_int;
    static lre_ctype_bits: [uint8_t; 256];
    fn lre_is_id_start(c: uint32_t) -> core::ffi::c_int;
    fn lre_is_id_continue(c: uint32_t) -> core::ffi::c_int;
    fn lre_is_space_non_ascii(c: uint32_t) -> core::ffi::c_int;
    fn lre_check_stack_overflow(
        opaque: *mut core::ffi::c_void,
        alloca_size: size_t,
    ) -> core::ffi::c_int;
    fn lre_check_timeout(opaque: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn lre_realloc(
        opaque: *mut core::ffi::c_void,
        ptr: *mut core::ffi::c_void,
        size: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: core::ffi::c_uint,
    pub fp_offset: core::ffi::c_uint,
    pub overflow_arg_area: *mut core::ffi::c_void,
    pub reg_save_area: *mut core::ffi::c_void,
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type va_list = __builtin_va_list;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type BOOL = core::ffi::c_int;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct packed_u32 {
    pub v: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct packed_u16 {
    pub v: uint16_t,
}
pub type DynBufReallocFunc = unsafe extern "C" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    size_t,
) -> *mut core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynBuf {
    pub buf: *mut uint8_t,
    pub size: size_t,
    pub allocated_size: size_t,
    pub error: BOOL,
    pub realloc_func: Option<DynBufReallocFunc>,
    pub opaque: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REParseState {
    pub byte_code: DynBuf,
    pub buf_ptr: *const uint8_t,
    pub buf_end: *const uint8_t,
    pub buf_start: *const uint8_t,
    pub re_flags: core::ffi::c_int,
    pub is_unicode: BOOL,
    pub unicode_sets: BOOL,
    pub ignore_case: BOOL,
    pub multi_line: BOOL,
    pub dotall: BOOL,
    pub group_name_scope: uint8_t,
    pub capture_count: core::ffi::c_int,
    pub total_capture_count: core::ffi::c_int,
    pub has_named_captures: core::ffi::c_int,
    pub opaque: *mut core::ffi::c_void,
    pub group_names: DynBuf,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub error_msg: [core::ffi::c_char; 128],
    pub tmp_buf: [core::ffi::c_char; 128],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REStringList {
    pub cr: CharRange,
    pub n_strings: uint32_t,
    pub hash_size: uint32_t,
    pub hash_bits: core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharRange {
    pub len: core::ffi::c_int,
    pub size: core::ffi::c_int,
    pub points: *mut uint32_t,
    pub mem_opaque: *mut core::ffi::c_void,
    pub realloc_func: Option<
        unsafe extern "C" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            size_t,
        ) -> *mut core::ffi::c_void,
    >,
}
pub const CHAR_RANGE_S: C2RustUnnamed_5 = 3;
pub const CHAR_RANGE_s: C2RustUnnamed_5 = 2;
pub const CR_OP_UNION: C2RustUnnamed_2 = 0;
pub type UnicodeSequencePropCB = unsafe extern "C" fn(
    *mut core::ffi::c_void,
    *const uint32_t,
    core::ffi::c_int,
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
    pub cbuf_type: core::ffi::c_int,
    pub capture_count: core::ffi::c_int,
    pub is_unicode: BOOL,
    pub interrupt_counter: core::ffi::c_int,
    pub opaque: *mut core::ffi::c_void,
    pub stack_buf: *mut StackElem,
    pub stack_size: size_t,
    pub static_stack_buf: [StackElem; 32],
}
pub const RE_EXEC_STATE_LOOKAHEAD: REExecStateEnum = 1;
pub type REExecStateEnum = core::ffi::c_uint;
pub const RE_EXEC_STATE_NEGATIVE_LOOKAHEAD: REExecStateEnum = 2;
pub const RE_EXEC_STATE_SPLIT: REExecStateEnum = 0;
pub const UNICODE_C_SPACE: C2RustUnnamed_3 = 1;
pub const REOP_negative_lookahead: C2RustUnnamed_4 = 41;
pub const REOP_negative_lookahead_match: C2RustUnnamed_4 = 18;
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const CR_OP_XOR: C2RustUnnamed_2 = 2;
pub type C2RustUnnamed_3 = core::ffi::c_uint;
pub const UNICODE_C_XDIGIT: C2RustUnnamed_3 = 64;
pub type C2RustUnnamed_4 = core::ffi::c_uint;
pub const REOP_invalid: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = core::ffi::c_uint;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INT32_MAX: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const LRE_FLAG_IGNORECASE: core::ffi::c_int = (1 as core::ffi::c_int)
    << 1 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn max_int(
    mut a: core::ffi::c_int,
    mut b: core::ffi::c_int,
) -> core::ffi::c_int {
    if a > b { return a } else { return b };
}
#[inline]
unsafe extern "C" fn get_u32(mut tab: *const uint8_t) -> uint32_t {
    return (*(tab as *const packed_u32)).v;
}
#[inline]
unsafe extern "C" fn put_u32(mut tab: *mut uint8_t, mut val: uint32_t) {
    (*(tab as *mut packed_u32)).v = val;
}
#[inline]
unsafe extern "C" fn get_u16(mut tab: *const uint8_t) -> uint32_t {
    return (*(tab as *const packed_u16)).v as uint32_t;
}
#[inline]
unsafe extern "C" fn put_u16(mut tab: *mut uint8_t, mut val: uint16_t) {
    (*(tab as *mut packed_u16)).v = val;
}
#[inline]
unsafe extern "C" fn dbuf_putc(
    mut s: *mut DynBuf,
    mut val: uint8_t,
) -> core::ffi::c_int {
    if (((*s).allocated_size).wrapping_sub((*s).size) < 1 as size_t) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        return __dbuf_putc(s, val)
    } else {
        let fresh0 = (*s).size;
        (*s).size = ((*s).size).wrapping_add(1);
        *((*s).buf).offset(fresh0 as isize) = val;
        return 0 as core::ffi::c_int;
    };
}
#[inline]
unsafe extern "C" fn dbuf_put_u16(
    mut s: *mut DynBuf,
    mut val: uint16_t,
) -> core::ffi::c_int {
    if (((*s).allocated_size).wrapping_sub((*s).size) < 2 as size_t) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        return __dbuf_put_u16(s, val)
    } else {
        put_u16(((*s).buf).offset((*s).size as isize), val);
        (*s).size = ((*s).size as core::ffi::c_ulong)
            .wrapping_add(2 as core::ffi::c_ulong) as size_t as size_t;
        return 0 as core::ffi::c_int;
    };
}
#[inline]
unsafe extern "C" fn dbuf_put_u32(
    mut s: *mut DynBuf,
    mut val: uint32_t,
) -> core::ffi::c_int {
    if (((*s).allocated_size).wrapping_sub((*s).size) < 4 as size_t) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        return __dbuf_put_u32(s, val)
    } else {
        put_u32(((*s).buf).offset((*s).size as isize), val);
        (*s).size = ((*s).size as core::ffi::c_ulong)
            .wrapping_add(4 as core::ffi::c_ulong) as size_t as size_t;
        return 0 as core::ffi::c_int;
    };
}
#[inline]
unsafe extern "C" fn dbuf_error(mut s: *mut DynBuf) -> BOOL {
    return (*s).error;
}
pub const UTF8_CHAR_LEN_MAX: core::ffi::c_int = 6 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn is_hi_surrogate(mut c: uint32_t) -> BOOL {
    return (c >> 10 as core::ffi::c_int
        == (0xd800 as core::ffi::c_int >> 10 as core::ffi::c_int) as uint32_t)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn is_lo_surrogate(mut c: uint32_t) -> BOOL {
    return (c >> 10 as core::ffi::c_int
        == (0xdc00 as core::ffi::c_int >> 10 as core::ffi::c_int) as uint32_t)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn from_surrogate(mut hi: uint32_t, mut lo: uint32_t) -> uint32_t {
    return (0x10000 as uint32_t)
        .wrapping_add(
            (0x400 as uint32_t).wrapping_mul(hi.wrapping_sub(0xd800 as uint32_t)),
        )
        .wrapping_add(lo.wrapping_sub(0xdc00 as uint32_t));
}
#[inline]
unsafe extern "C" fn from_hex(mut c: core::ffi::c_int) -> core::ffi::c_int {
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32
    } else if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 10 as core::ffi::c_int
    } else if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 10 as core::ffi::c_int
    } else {
        return -(1 as core::ffi::c_int)
    };
}
pub const LRE_FLAG_MULTILINE: core::ffi::c_int = (1 as core::ffi::c_int)
    << 2 as core::ffi::c_int;
pub const CAPTURE_COUNT_MAX: core::ffi::c_int = 255 as core::ffi::c_int;
pub const REGISTER_COUNT_MAX: core::ffi::c_int = 255 as core::ffi::c_int;
pub const INTERRUPT_COUNTER_INIT: core::ffi::c_int = 10000 as core::ffi::c_int;
pub const CP_LS: core::ffi::c_int = 0x2028 as core::ffi::c_int;
pub const CP_PS: core::ffi::c_int = 0x2029 as core::ffi::c_int;
static mut reopcode_info: [REOpCode; 45] = [
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
pub const RE_HEADER_FLAGS: core::ffi::c_int = 0 as core::ffi::c_int;
pub const RE_HEADER_CAPTURE_COUNT: core::ffi::c_int = 2 as core::ffi::c_int;
pub const RE_HEADER_REGISTER_COUNT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const RE_HEADER_BYTECODE_LEN: core::ffi::c_int = 4 as core::ffi::c_int;
pub const RE_HEADER_LEN: core::ffi::c_int = 8 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn is_digit(mut c: core::ffi::c_int) -> core::ffi::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as core::ffi::c_int;
}
unsafe extern "C" fn dbuf_insert(
    mut s: *mut DynBuf,
    mut pos: core::ffi::c_int,
    mut len: core::ffi::c_int,
) -> core::ffi::c_int {
    if dbuf_claim(s, len as size_t) != 0 {
        return -(1 as core::ffi::c_int);
    }
    memmove(
        ((*s).buf).offset(pos as isize).offset(len as isize) as *mut core::ffi::c_void,
        ((*s).buf).offset(pos as isize) as *const core::ffi::c_void,
        ((*s).size).wrapping_sub(pos as size_t),
    );
    (*s).size = ((*s).size as core::ffi::c_ulong).wrapping_add(len as core::ffi::c_ulong)
        as size_t as size_t;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn re_string_hash(
    mut len: core::ffi::c_int,
    mut buf: *const uint32_t,
) -> uint32_t {
    let mut i: core::ffi::c_int = 0;
    let mut h: uint32_t = 0;
    h = 1 as uint32_t;
    i = 0 as core::ffi::c_int;
    while i < len {
        h = h.wrapping_mul(263 as uint32_t).wrapping_add(*buf.offset(i as isize));
        i += 1;
    }
    return h.wrapping_mul(0x61c88647 as uint32_t);
}
unsafe extern "C" fn re_string_list_init(
    mut s1: *mut REParseState,
    mut s: *mut REStringList,
) {
    cr_init(
        &mut (*s).cr,
        (*s1).opaque,
        Some(
            lre_realloc
                as unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    size_t,
                ) -> *mut core::ffi::c_void,
        ),
    );
    (*s).n_strings = 0 as uint32_t;
    (*s).hash_size = 0 as uint32_t;
    (*s).hash_bits = 0 as core::ffi::c_int;
    (*s).hash_table = 0 as *mut *mut REString;
}
unsafe extern "C" fn re_string_list_free(mut s: *mut REStringList) {
    let mut p: *mut REString = 0 as *mut REString;
    let mut p_next: *mut REString = 0 as *mut REString;
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while (i as uint32_t) < (*s).hash_size {
        p = *((*s).hash_table).offset(i as isize);
        while !p.is_null() {
            p_next = (*p).next as *mut REString;
            lre_realloc((*s).cr.mem_opaque, p as *mut core::ffi::c_void, 0 as size_t);
            p = p_next;
        }
        i += 1;
    }
    lre_realloc(
        (*s).cr.mem_opaque,
        (*s).hash_table as *mut core::ffi::c_void,
        0 as size_t,
    );
    cr_free(&mut (*s).cr);
}
unsafe extern "C" fn re_string_find2(
    mut s: *mut REStringList,
    mut len: core::ffi::c_int,
    mut buf: *const uint32_t,
    mut h0: uint32_t,
    mut add_flag: BOOL,
) -> core::ffi::c_int {
    let mut h: uint32_t = 0 as uint32_t;
    let mut p: *mut REString = 0 as *mut REString;
    if (*s).n_strings != 0 as uint32_t {
        h = h0 >> 32 as core::ffi::c_int - (*s).hash_bits;
        p = *((*s).hash_table).offset(h as isize);
        while !p.is_null() {
            if (*p).hash == h0 && (*p).len == len as uint32_t
                && memcmp(
                    ((*p).buf).as_mut_ptr() as *const core::ffi::c_void,
                    buf as *const core::ffi::c_void,
                    (len as size_t)
                        .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
                ) == 0
            {
                return 1 as core::ffi::c_int;
            }
            p = (*p).next as *mut REString;
        }
    }
    if add_flag == 0 {
        return 0 as core::ffi::c_int;
    }
    if (((*s).n_strings).wrapping_add(1 as uint32_t) > (*s).hash_size)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        let mut new_hash_table: *mut *mut REString = 0 as *mut *mut REString;
        let mut p_next: *mut REString = 0 as *mut REString;
        let mut new_hash_bits: core::ffi::c_int = 0;
        let mut i: core::ffi::c_int = 0;
        let mut new_hash_size: uint32_t = 0;
        new_hash_bits = max_int(
            (*s).hash_bits + 1 as core::ffi::c_int,
            4 as core::ffi::c_int,
        );
        new_hash_size = ((1 as core::ffi::c_int) << new_hash_bits) as uint32_t;
        new_hash_table = lre_realloc(
            (*s).cr.mem_opaque,
            NULL,
            (::core::mem::size_of::<*mut REString>() as size_t)
                .wrapping_mul(new_hash_size as size_t),
        ) as *mut *mut REString;
        if new_hash_table.is_null() {
            return -(1 as core::ffi::c_int);
        }
        memset(
            new_hash_table as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<*mut REString>() as size_t)
                .wrapping_mul(new_hash_size as size_t),
        );
        i = 0 as core::ffi::c_int;
        while (i as uint32_t) < (*s).hash_size {
            p = *((*s).hash_table).offset(i as isize);
            while !p.is_null() {
                p_next = (*p).next as *mut REString;
                h = (*p).hash >> 32 as core::ffi::c_int - new_hash_bits;
                (*p).next = *new_hash_table.offset(h as isize) as *mut REString;
                let ref mut fresh31 = *new_hash_table.offset(h as isize);
                *fresh31 = p;
                p = p_next;
            }
            i += 1;
        }
        lre_realloc(
            (*s).cr.mem_opaque,
            (*s).hash_table as *mut core::ffi::c_void,
            0 as size_t,
        );
        (*s).hash_bits = new_hash_bits;
        (*s).hash_size = new_hash_size;
        (*s).hash_table = new_hash_table;
        h = h0 >> 32 as core::ffi::c_int - (*s).hash_bits;
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
        return -(1 as core::ffi::c_int);
    }
    (*p).next = *((*s).hash_table).offset(h as isize) as *mut REString;
    let ref mut fresh32 = *((*s).hash_table).offset(h as isize);
    *fresh32 = p;
    (*s).n_strings = ((*s).n_strings).wrapping_add(1);
    (*p).hash = h0;
    (*p).len = len as uint32_t;
    memcpy(
        ((*p).buf).as_mut_ptr() as *mut core::ffi::c_void,
        buf as *const core::ffi::c_void,
        (::core::mem::size_of::<uint32_t>() as size_t).wrapping_mul(len as size_t),
    );
    return 1 as core::ffi::c_int;
}
unsafe extern "C" fn re_string_find(
    mut s: *mut REStringList,
    mut len: core::ffi::c_int,
    mut buf: *const uint32_t,
    mut add_flag: BOOL,
) -> core::ffi::c_int {
    let mut h0: uint32_t = 0;
    h0 = re_string_hash(len, buf);
    return re_string_find2(s, len, buf, h0, add_flag);
}
unsafe extern "C" fn re_string_add(
    mut s: *mut REStringList,
    mut len: core::ffi::c_int,
    mut buf: *const uint32_t,
) -> core::ffi::c_int {
    if len == 1 as core::ffi::c_int {
        return cr_union_interval(
            &mut (*s).cr,
            *buf.offset(0 as core::ffi::c_int as isize),
            *buf.offset(0 as core::ffi::c_int as isize),
        );
    }
    if re_string_find(s, len, buf, TRUE as core::ffi::c_int as BOOL)
        < 0 as core::ffi::c_int
    {
        return -(1 as core::ffi::c_int);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn re_string_list_op(
    mut a: *mut REStringList,
    mut b: *mut REStringList,
    mut op: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let mut ret: core::ffi::c_int = 0;
    let mut p: *mut REString = 0 as *mut REString;
    let mut pp: *mut *mut REString = 0 as *mut *mut REString;
    if cr_op1(&mut (*a).cr, (*b).cr.points, (*b).cr.len, op) != 0 {
        return -(1 as core::ffi::c_int);
    }
    match op {
        0 => {
            if (*b).n_strings != 0 as uint32_t {
                i = 0 as core::ffi::c_int;
                while (i as uint32_t) < (*b).hash_size {
                    p = *((*b).hash_table).offset(i as isize);
                    while !p.is_null() {
                        if re_string_find2(
                            a,
                            (*p).len as core::ffi::c_int,
                            ((*p).buf).as_mut_ptr(),
                            (*p).hash,
                            TRUE as core::ffi::c_int as BOOL,
                        ) < 0 as core::ffi::c_int
                        {
                            return -(1 as core::ffi::c_int);
                        }
                        p = (*p).next as *mut REString;
                    }
                    i += 1;
                }
            }
        }
        1 | 3 => {
            i = 0 as core::ffi::c_int;
            while (i as uint32_t) < (*a).hash_size {
                pp = &mut *((*a).hash_table).offset(i as isize) as *mut *mut REString;
                loop {
                    p = *pp;
                    if p.is_null() {
                        break;
                    }
                    ret = re_string_find2(
                        b,
                        (*p).len as core::ffi::c_int,
                        ((*p).buf).as_mut_ptr(),
                        (*p).hash,
                        FALSE as core::ffi::c_int as BOOL,
                    );
                    if op == CR_OP_SUB as core::ffi::c_int {
                        ret = (ret == 0) as core::ffi::c_int;
                    }
                    if ret == 0 {
                        *pp = (*p).next as *mut REString;
                        (*a).n_strings = ((*a).n_strings).wrapping_sub(1);
                        lre_realloc(
                            (*a).cr.mem_opaque,
                            p as *mut core::ffi::c_void,
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
            abort();
        }
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn re_string_list_canonicalize(
    mut s1: *mut REParseState,
    mut s: *mut REStringList,
    mut is_unicode: BOOL,
) -> core::ffi::c_int {
    if cr_regexp_canonicalize(&mut (*s).cr, is_unicode as core::ffi::c_int) != 0 {
        return -(1 as core::ffi::c_int);
    }
    if (*s).n_strings != 0 as uint32_t {
        let mut a_s: REStringList = REStringList {
            cr: CharRange {
                len: 0,
                size: 0,
                points: 0 as *mut uint32_t,
                mem_opaque: 0 as *mut core::ffi::c_void,
                realloc_func: None,
            },
            n_strings: 0,
            hash_size: 0,
            hash_bits: 0,
            hash_table: 0 as *mut *mut REString,
        };
        let mut a: *mut REStringList = &mut a_s;
        let mut i: core::ffi::c_int = 0;
        let mut j: core::ffi::c_int = 0;
        let mut p: *mut REString = 0 as *mut REString;
        re_string_list_init(s1, a);
        (*a).n_strings = (*s).n_strings;
        (*a).hash_size = (*s).hash_size;
        (*a).hash_bits = (*s).hash_bits;
        (*a).hash_table = (*s).hash_table;
        (*s).n_strings = 0 as uint32_t;
        (*s).hash_size = 0 as uint32_t;
        (*s).hash_bits = 0 as core::ffi::c_int;
        (*s).hash_table = 0 as *mut *mut REString;
        i = 0 as core::ffi::c_int;
        while (i as uint32_t) < (*a).hash_size {
            p = *((*a).hash_table).offset(i as isize);
            while !p.is_null() {
                j = 0 as core::ffi::c_int;
                while (j as uint32_t) < (*p).len {
                    *((*p).buf).as_mut_ptr().offset(j as isize) = lre_canonicalize(
                        *((*p).buf).as_mut_ptr().offset(j as isize),
                        is_unicode as core::ffi::c_int,
                    ) as uint32_t;
                    j += 1;
                }
                if re_string_add(
                    s,
                    (*p).len as core::ffi::c_int,
                    ((*p).buf).as_mut_ptr(),
                ) != 0
                {
                    re_string_list_free(a);
                    return -(1 as core::ffi::c_int);
                }
                p = (*p).next as *mut REString;
            }
            i += 1;
        }
        re_string_list_free(a);
    }
    return 0 as core::ffi::c_int;
}
static char_range_d: [uint16_t; 3] = [
    1 as core::ffi::c_int as uint16_t,
    0x30 as core::ffi::c_int as uint16_t,
    (0x39 as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
];
static char_range_s: [uint16_t; 21] = [
    10 as core::ffi::c_int as uint16_t,
    0x9 as core::ffi::c_int as uint16_t,
    (0xd as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
    0x20 as core::ffi::c_int as uint16_t,
    (0x20 as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
    0xa0 as core::ffi::c_int as uint16_t,
    (0xa0 as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
    0x1680 as core::ffi::c_int as uint16_t,
    (0x1680 as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
    0x2000 as core::ffi::c_int as uint16_t,
    (0x200a as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
    0x2028 as core::ffi::c_int as uint16_t,
    (0x2029 as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
    0x202f as core::ffi::c_int as uint16_t,
    (0x202f as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
    0x205f as core::ffi::c_int as uint16_t,
    (0x205f as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
    0x3000 as core::ffi::c_int as uint16_t,
    (0x3000 as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
    0xfeff as core::ffi::c_int as uint16_t,
    (0xfeff as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
];
static char_range_w: [uint16_t; 9] = [
    4 as core::ffi::c_int as uint16_t,
    0x30 as core::ffi::c_int as uint16_t,
    (0x39 as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
    0x41 as core::ffi::c_int as uint16_t,
    (0x5a as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
    0x5f as core::ffi::c_int as uint16_t,
    (0x5f as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
    0x61 as core::ffi::c_int as uint16_t,
    (0x7a as core::ffi::c_int + 1 as core::ffi::c_int) as uint16_t,
];
pub const CLASS_RANGE_BASE: core::ffi::c_int = 0x40000000 as core::ffi::c_int;

// Helper function to get char_range_table entry at runtime
// This avoids the static initialization issues with pointers
#[inline]
unsafe fn get_char_range_table(idx: usize) -> *const uint16_t {
    match idx {
        0 => char_range_d.as_ptr(),
        1 => char_range_s.as_ptr(),
        2 => char_range_w.as_ptr(),
        _ => core::ptr::null(),
    }
}
unsafe extern "C" fn cr_init_char_range(
    mut s: *mut REParseState,
    mut cr: *mut REStringList,
    mut c: uint32_t,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let mut invert: BOOL = 0;
    let mut c_pt: *const uint16_t = 0 as *const uint16_t;
    let mut len: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    invert = (c & 1 as uint32_t) as BOOL;
    c_pt = get_char_range_table((c >> 1 as core::ffi::c_int) as usize);
    let fresh37 = c_pt;
    c_pt = c_pt.offset(1);
    len = *fresh37 as core::ffi::c_int;
    re_string_list_init(s, cr);
    i = 0 as core::ffi::c_int;
    loop {
        if !(i < len * 2 as core::ffi::c_int) {
            current_block = 10879442775620481940;
            break;
        }
        if cr_add_point(&mut (*cr).cr, *c_pt.offset(i as isize) as uint32_t) != 0 {
            current_block = 7915227757654860830;
            break;
        }
        i += 1;
    }
    match current_block {
        10879442775620481940 => {
            if invert != 0 {
                if cr_invert(&mut (*cr).cr) != 0 {
                    current_block = 7915227757654860830;
                } else {
                    current_block = 13183875560443969876;
                }
            } else {
                current_block = 13183875560443969876;
            }
            match current_block {
                7915227757654860830 => {}
                _ => return 0 as core::ffi::c_int,
            }
        }
        _ => {}
    }
    re_string_list_free(cr);
    return -(1 as core::ffi::c_int);
}
unsafe extern "C" fn re_emit_op(mut s: *mut REParseState, mut op: core::ffi::c_int) {
    dbuf_putc(&mut (*s).byte_code, op as uint8_t);
}
unsafe extern "C" fn re_emit_op_u32(
    mut s: *mut REParseState,
    mut op: core::ffi::c_int,
    mut val: uint32_t,
) -> core::ffi::c_int {
    let mut pos: core::ffi::c_int = 0;
    dbuf_putc(&mut (*s).byte_code, op as uint8_t);
    pos = (*s).byte_code.size as core::ffi::c_int;
    dbuf_put_u32(&mut (*s).byte_code, val);
    return pos;
}
unsafe extern "C" fn re_emit_goto(
    mut s: *mut REParseState,
    mut op: core::ffi::c_int,
    mut val: uint32_t,
) -> core::ffi::c_int {
    let mut pos: core::ffi::c_int = 0;
    dbuf_putc(&mut (*s).byte_code, op as uint8_t);
    pos = (*s).byte_code.size as core::ffi::c_int;
    dbuf_put_u32(
        &mut (*s).byte_code,
        val.wrapping_sub((pos + 4 as core::ffi::c_int) as uint32_t),
    );
    return pos;
}
unsafe extern "C" fn re_emit_goto_u8(
    mut s: *mut REParseState,
    mut op: core::ffi::c_int,
    mut arg: uint32_t,
    mut val: uint32_t,
) -> core::ffi::c_int {
    let mut pos: core::ffi::c_int = 0;
    dbuf_putc(&mut (*s).byte_code, op as uint8_t);
    dbuf_putc(&mut (*s).byte_code, arg as uint8_t);
    pos = (*s).byte_code.size as core::ffi::c_int;
    dbuf_put_u32(
        &mut (*s).byte_code,
        val.wrapping_sub((pos + 4 as core::ffi::c_int) as uint32_t),
    );
    return pos;
}
unsafe extern "C" fn re_emit_goto_u8_u32(
    mut s: *mut REParseState,
    mut op: core::ffi::c_int,
    mut arg0: uint32_t,
    mut arg1: uint32_t,
    mut val: uint32_t,
) -> core::ffi::c_int {
    let mut pos: core::ffi::c_int = 0;
    dbuf_putc(&mut (*s).byte_code, op as uint8_t);
    dbuf_putc(&mut (*s).byte_code, arg0 as uint8_t);
    dbuf_put_u32(&mut (*s).byte_code, arg1);
    pos = (*s).byte_code.size as core::ffi::c_int;
    dbuf_put_u32(
        &mut (*s).byte_code,
        val.wrapping_sub((pos + 4 as core::ffi::c_int) as uint32_t),
    );
    return pos;
}
unsafe extern "C" fn re_emit_op_u8(
    mut s: *mut REParseState,
    mut op: core::ffi::c_int,
    mut val: uint32_t,
) {
    dbuf_putc(&mut (*s).byte_code, op as uint8_t);
    dbuf_putc(&mut (*s).byte_code, val as uint8_t);
}
unsafe extern "C" fn re_emit_op_u16(
    mut s: *mut REParseState,
    mut op: core::ffi::c_int,
    mut val: uint32_t,
) {
    dbuf_putc(&mut (*s).byte_code, op as uint8_t);
    dbuf_put_u16(&mut (*s).byte_code, val as uint16_t);
}
// STUB: re_parse_error uses C variadic arguments
// The function sets an error message and returns -1
unsafe extern "C" fn re_parse_error(
    mut s: *mut REParseState,
    mut fmt: *const core::ffi::c_char,
    mut _args: ...
) -> core::ffi::c_int {
    // Set a generic error message instead of formatted one
    let msg = b"regex parse error\0";
    let dst = ((*s).u.error_msg).as_mut_ptr();
    for (i, &byte) in msg.iter().enumerate() {
        if i >= 127 { break; }
        *dst.add(i) = byte as core::ffi::c_char;
    }
    return -(1 as core::ffi::c_int);
}
unsafe extern "C" fn re_parse_out_of_memory(
    mut s: *mut REParseState,
) -> core::ffi::c_int {
    return re_parse_error(
        s,
        b"out of memory\0" as *const u8 as *const core::ffi::c_char,
    );
}
unsafe extern "C" fn parse_digits(
    mut pp: *mut *const uint8_t,
    mut allow_overflow: BOOL,
) -> core::ffi::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut v: uint64_t = 0;
    let mut c: core::ffi::c_int = 0;
    p = *pp;
    v = 0 as uint64_t;
    loop {
        c = *p as core::ffi::c_int;
        if c < '0' as i32 || c > '9' as i32 {
            break;
        }
        v = v
            .wrapping_mul(10 as uint64_t)
            .wrapping_add(c as uint64_t)
            .wrapping_sub('0' as i32 as uint64_t);
        if v >= INT32_MAX as uint64_t {
            if allow_overflow != 0 {
                v = INT32_MAX as uint64_t;
            } else {
                return -(1 as core::ffi::c_int)
            }
        }
        p = p.offset(1);
    }
    *pp = p;
    return v as core::ffi::c_int;
}
unsafe extern "C" fn re_parse_expect(
    mut s: *mut REParseState,
    mut pp: *mut *const uint8_t,
    mut c: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    p = *pp;
    if *p as core::ffi::c_int != c {
        return re_parse_error(
            s,
            b"expecting '%c'\0" as *const u8 as *const core::ffi::c_char,
            c,
        );
    }
    p = p.offset(1);
    *pp = p;
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lre_parse_escape(
    mut pp: *mut *const uint8_t,
    mut allow_utf16: core::ffi::c_int,
) -> core::ffi::c_int {
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
            let mut h0: core::ffi::c_int = 0;
            let mut h1: core::ffi::c_int = 0;
            let fresh27 = p;
            p = p.offset(1);
            h0 = from_hex(*fresh27 as core::ffi::c_int);
            if h0 < 0 as core::ffi::c_int {
                return -(1 as core::ffi::c_int);
            }
            let fresh28 = p;
            p = p.offset(1);
            h1 = from_hex(*fresh28 as core::ffi::c_int);
            if h1 < 0 as core::ffi::c_int {
                return -(1 as core::ffi::c_int);
            }
            c = (h0 << 4 as core::ffi::c_int | h1) as uint32_t;
        }
        117 => {
            let mut h: core::ffi::c_int = 0;
            let mut i: core::ffi::c_int = 0;
            let mut c1: uint32_t = 0;
            if *p as core::ffi::c_int == '{' as i32 && allow_utf16 != 0 {
                p = p.offset(1);
                c = 0 as uint32_t;
                loop {
                    let fresh29 = p;
                    p = p.offset(1);
                    h = from_hex(*fresh29 as core::ffi::c_int);
                    if h < 0 as core::ffi::c_int {
                        return -(1 as core::ffi::c_int);
                    }
                    c = c << 4 as core::ffi::c_int | h as uint32_t;
                    if c > 0x10ffff as uint32_t {
                        return -(1 as core::ffi::c_int);
                    }
                    if *p as core::ffi::c_int == '}' as i32 {
                        break;
                    }
                }
                p = p.offset(1);
            } else {
                c = 0 as uint32_t;
                i = 0 as core::ffi::c_int;
                while i < 4 as core::ffi::c_int {
                    let fresh30 = p;
                    p = p.offset(1);
                    h = from_hex(*fresh30 as core::ffi::c_int);
                    if h < 0 as core::ffi::c_int {
                        return -(1 as core::ffi::c_int);
                    }
                    c = c << 4 as core::ffi::c_int | h as uint32_t;
                    i += 1;
                }
                if is_hi_surrogate(c) != 0 && allow_utf16 == 2 as core::ffi::c_int
                    && *p.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        == '\\' as i32
                    && *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                        == 'u' as i32
                {
                    c1 = 0 as uint32_t;
                    i = 0 as core::ffi::c_int;
                    while i < 4 as core::ffi::c_int {
                        h = from_hex(
                            *p.offset((2 as core::ffi::c_int + i) as isize)
                                as core::ffi::c_int,
                        );
                        if h < 0 as core::ffi::c_int {
                            break;
                        }
                        c1 = c1 << 4 as core::ffi::c_int | h as uint32_t;
                        i += 1;
                    }
                    if i == 4 as core::ffi::c_int && is_lo_surrogate(c1) != 0 {
                        p = p.offset(6 as core::ffi::c_int as isize);
                        c = from_surrogate(c, c1);
                    }
                }
            }
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
            c = (c as core::ffi::c_uint).wrapping_sub('0' as i32 as core::ffi::c_uint)
                as uint32_t as uint32_t;
            if allow_utf16 == 2 as core::ffi::c_int {
                if c != 0 as uint32_t || is_digit(*p as core::ffi::c_int) != 0 {
                    return -(1 as core::ffi::c_int);
                }
            } else {
                let mut v: uint32_t = 0;
                v = (*p as core::ffi::c_int - '0' as i32) as uint32_t;
                if !(v > 7 as uint32_t) {
                    c = c << 3 as core::ffi::c_int | v;
                    p = p.offset(1);
                    if !(c >= 32 as uint32_t) {
                        v = (*p as core::ffi::c_int - '0' as i32) as uint32_t;
                        if !(v > 7 as uint32_t) {
                            c = c << 3 as core::ffi::c_int | v;
                            p = p.offset(1);
                        }
                    }
                }
            }
        }
        _ => return -(2 as core::ffi::c_int),
    }
    *pp = p;
    return c as core::ffi::c_int;
}
unsafe extern "C" fn is_unicode_char(mut c: core::ffi::c_int) -> BOOL {
    return (c >= '0' as i32 && c <= '9' as i32 || c >= 'A' as i32 && c <= 'Z' as i32
        || c >= 'a' as i32 && c <= 'z' as i32 || c == '_' as i32) as core::ffi::c_int;
}
unsafe extern "C" fn seq_prop_cb(
    mut opaque: *mut core::ffi::c_void,
    mut seq: *const uint32_t,
    mut seq_len: core::ffi::c_int,
) {
    let mut sl: *mut REStringList = opaque as *mut REStringList;
    re_string_add(sl, seq_len, seq);
}
unsafe extern "C" fn parse_unicode_property(
    mut s: *mut REParseState,
    mut cr: *mut REStringList,
    mut pp: *mut *const uint8_t,
    mut is_inv: BOOL,
    mut allow_sequence_prop: BOOL,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut name: [core::ffi::c_char; 64] = [0; 64];
    let mut value: [core::ffi::c_char; 64] = [0; 64];
    let mut q: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut script_ext: BOOL = 0;
    let mut ret: core::ffi::c_int = 0;
    p = *pp;
    if *p as core::ffi::c_int != '{' as i32 {
        return re_parse_error(
            s,
            b"expecting '{' after \\p\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    p = p.offset(1);
    q = name.as_mut_ptr();
    loop {
        if !(is_unicode_char(*p as core::ffi::c_int) != 0) {
            current_block = 14523784380283086299;
            break;
        }
        if q.offset_from(name.as_mut_ptr()) as core::ffi::c_long as usize
            >= (::core::mem::size_of::<[core::ffi::c_char; 64]>() as usize)
                .wrapping_sub(1 as usize)
        {
            current_block = 15312645817252085012;
            break;
        }
        let fresh33 = p;
        p = p.offset(1);
        let fresh34 = q;
        q = q.offset(1);
        *fresh34 = *fresh33 as core::ffi::c_char;
    }
    match current_block {
        14523784380283086299 => {
            *q = '\0' as i32 as core::ffi::c_char;
            q = value.as_mut_ptr();
            if *p as core::ffi::c_int == '=' as i32 {
                p = p.offset(1);
                while is_unicode_char(*p as core::ffi::c_int) != 0 {
                    if q.offset_from(value.as_mut_ptr()) as core::ffi::c_long as usize
                        >= (::core::mem::size_of::<[core::ffi::c_char; 64]>() as usize)
                            .wrapping_sub(1 as usize)
                    {
                        return re_parse_error(
                            s,
                            b"unknown unicode property value\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    let fresh35 = p;
                    p = p.offset(1);
                    let fresh36 = q;
                    q = q.offset(1);
                    *fresh36 = *fresh35 as core::ffi::c_char;
                }
            }
            *q = '\0' as i32 as core::ffi::c_char;
            if *p as core::ffi::c_int != '}' as i32 {
                return re_parse_error(
                    s,
                    b"expecting '}'\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            p = p.offset(1);
            if strcmp(
                name.as_mut_ptr(),
                b"Script\0" as *const u8 as *const core::ffi::c_char,
            ) == 0
                || strcmp(
                    name.as_mut_ptr(),
                    b"sc\0" as *const u8 as *const core::ffi::c_char,
                ) == 0
            {
                script_ext = FALSE as core::ffi::c_int as BOOL;
                current_block = 12375508261711692781;
            } else if strcmp(
                name.as_mut_ptr(),
                b"Script_Extensions\0" as *const u8 as *const core::ffi::c_char,
            ) == 0
                || strcmp(
                    name.as_mut_ptr(),
                    b"scx\0" as *const u8 as *const core::ffi::c_char,
                ) == 0
            {
                script_ext = TRUE as core::ffi::c_int as BOOL;
                current_block = 12375508261711692781;
            } else if strcmp(
                name.as_mut_ptr(),
                b"General_Category\0" as *const u8 as *const core::ffi::c_char,
            ) == 0
                || strcmp(
                    name.as_mut_ptr(),
                    b"gc\0" as *const u8 as *const core::ffi::c_char,
                ) == 0
            {
                re_string_list_init(s, cr);
                ret = unicode_general_category(&mut (*cr).cr, value.as_mut_ptr());
                if ret != 0 {
                    re_string_list_free(cr);
                    if ret == -(2 as core::ffi::c_int) {
                        return re_parse_error(
                            s,
                            b"unknown unicode general category\0" as *const u8
                                as *const core::ffi::c_char,
                        )
                    } else {
                        current_block = 11105379909962262667;
                    }
                } else {
                    current_block = 11793792312832361944;
                }
            } else if value[0 as core::ffi::c_int as usize] as core::ffi::c_int
                == '\0' as i32
            {
                re_string_list_init(s, cr);
                ret = unicode_general_category(&mut (*cr).cr, name.as_mut_ptr());
                if ret == -(1 as core::ffi::c_int) {
                    re_string_list_free(cr);
                    current_block = 11105379909962262667;
                } else {
                    if ret < 0 as core::ffi::c_int {
                        ret = unicode_prop(&mut (*cr).cr, name.as_mut_ptr());
                        if ret == -(1 as core::ffi::c_int) {
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
                            if ret < 0 as core::ffi::c_int && is_inv == 0
                                && allow_sequence_prop != 0
                            {
                                let mut cr_tmp: CharRange = CharRange {
                                    len: 0,
                                    size: 0,
                                    points: 0 as *mut uint32_t,
                                    mem_opaque: 0 as *mut core::ffi::c_void,
                                    realloc_func: None,
                                };
                                cr_init(
                                    &mut cr_tmp,
                                    (*s).opaque,
                                    Some(
                                        lre_realloc
                                            as unsafe extern "C" fn(
                                                *mut core::ffi::c_void,
                                                *mut core::ffi::c_void,
                                                size_t,
                                            ) -> *mut core::ffi::c_void,
                                    ),
                                );
                                ret = unicode_sequence_prop(
                                    name.as_mut_ptr(),
                                    Some(
                                        seq_prop_cb
                                            as unsafe extern "C" fn(
                                                *mut core::ffi::c_void,
                                                *const uint32_t,
                                                core::ffi::c_int,
                                            ) -> (),
                                    ),
                                    cr as *mut core::ffi::c_void,
                                    &mut cr_tmp,
                                );
                                cr_free(&mut cr_tmp);
                                if ret == -(1 as core::ffi::c_int) {
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
                                    if ret < 0 as core::ffi::c_int {
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
                                script_ext as core::ffi::c_int,
                            );
                            if ret != 0 {
                                re_string_list_free(cr);
                                if ret == -(2 as core::ffi::c_int) {
                                    return re_parse_error(
                                        s,
                                        b"unknown unicode script\0" as *const u8
                                            as *const core::ffi::c_char,
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
                                                    return 0 as core::ffi::c_int;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    return re_parse_out_of_memory(s);
                }
            }
        }
        _ => {}
    }
    return re_parse_error(
        s,
        b"unknown unicode property name\0" as *const u8 as *const core::ffi::c_char,
    );
}
unsafe extern "C" fn parse_class_string_disjunction(
    mut s: *mut REParseState,
    mut cr: *mut REStringList,
    mut pp: *mut *const uint8_t,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut str: DynBuf = DynBuf {
        buf: 0 as *mut uint8_t,
        size: 0,
        allocated_size: 0,
        error: 0,
        realloc_func: None,
        opaque: 0 as *mut core::ffi::c_void,
    };
    let mut c: core::ffi::c_int = 0;
    p = *pp;
    if *p as core::ffi::c_int != '{' as i32 {
        return re_parse_error(
            s,
            b"expecting '{' after \\q\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    dbuf_init2(
        &mut str,
        (*s).opaque,
        Some(
            lre_realloc
                as unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    size_t,
                ) -> *mut core::ffi::c_void,
        ),
    );
    re_string_list_init(s, cr);
    p = p.offset(1);
    's_31: loop {
        str.size = 0 as size_t;
        while *p as core::ffi::c_int != '}' as i32
            && *p as core::ffi::c_int != '|' as i32
        {
            c = get_class_atom(
                s,
                0 as *mut REStringList,
                &mut p,
                FALSE as core::ffi::c_int as BOOL,
            );
            if c < 0 as core::ffi::c_int {
                current_block = 4849670670732935458;
                break 's_31;
            }
            if !(dbuf_put_u32(&mut str, c as uint32_t) != 0) {
                continue;
            }
            re_parse_out_of_memory(s);
            current_block = 4849670670732935458;
            break 's_31;
        }
        if re_string_add(
            cr,
            (str.size).wrapping_div(4 as size_t) as core::ffi::c_int,
            str.buf as *mut uint32_t,
        ) != 0
        {
            re_parse_out_of_memory(s);
            current_block = 4849670670732935458;
            break;
        } else {
            if *p as core::ffi::c_int == '}' as i32 {
                current_block = 8831408221741692167;
                break;
            }
            p = p.offset(1);
        }
    }
    match current_block {
        8831408221741692167 => {
            if (*s).ignore_case != 0 {
                if re_string_list_canonicalize(s, cr, TRUE as core::ffi::c_int as BOOL)
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
                    dbuf_free(&mut str);
                    *pp = p;
                    return 0 as core::ffi::c_int;
                }
            }
        }
        _ => {}
    }
    dbuf_free(&mut str);
    re_string_list_free(cr);
    return -(1 as core::ffi::c_int);
}
unsafe extern "C" fn get_class_atom(
    mut s: *mut REParseState,
    mut cr: *mut REStringList,
    mut pp: *mut *const uint8_t,
    mut inclass: BOOL,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut c: uint32_t = 0;
    let mut ret: core::ffi::c_int = 0;
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
                                        return -(1 as core::ffi::c_int);
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
                                        (c == 'P' as i32 as uint32_t) as core::ffi::c_int,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as core::ffi::c_int);
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
                                    c = (c as core::ffi::c_uint & 0x1f as core::ffi::c_uint)
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
                                c = CHAR_RANGE_d as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as core::ffi::c_int as uint32_t;
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
                                                return -(1 as core::ffi::c_int);
                                            }
                                            c = (c as core::ffi::c_uint)
                                                .wrapping_add(CLASS_RANGE_BASE as core::ffi::c_uint)
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
                                                    (*s).is_unicode as core::ffi::c_int * 2 as core::ffi::c_int,
                                                );
                                                if ret >= 0 as core::ffi::c_int {
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
                                                    s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const core::ffi::c_char,
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
                                        return -(1 as core::ffi::c_int);
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
                                        (c == 'P' as i32 as uint32_t) as core::ffi::c_int,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as core::ffi::c_int);
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
                                    c = (c as core::ffi::c_uint & 0x1f as core::ffi::c_uint)
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
                                c = CHAR_RANGE_d as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as core::ffi::c_int as uint32_t;
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
                                                return -(1 as core::ffi::c_int);
                                            }
                                            c = (c as core::ffi::c_uint)
                                                .wrapping_add(CLASS_RANGE_BASE as core::ffi::c_uint)
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
                                                    (*s).is_unicode as core::ffi::c_int * 2 as core::ffi::c_int,
                                                );
                                                if ret >= 0 as core::ffi::c_int {
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
                                                    s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const core::ffi::c_char,
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
                                        return -(1 as core::ffi::c_int);
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
                                        (c == 'P' as i32 as uint32_t) as core::ffi::c_int,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as core::ffi::c_int);
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
                                    c = (c as core::ffi::c_uint & 0x1f as core::ffi::c_uint)
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
                                c = CHAR_RANGE_d as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as core::ffi::c_int as uint32_t;
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
                                                return -(1 as core::ffi::c_int);
                                            }
                                            c = (c as core::ffi::c_uint)
                                                .wrapping_add(CLASS_RANGE_BASE as core::ffi::c_uint)
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
                                                    (*s).is_unicode as core::ffi::c_int * 2 as core::ffi::c_int,
                                                );
                                                if ret >= 0 as core::ffi::c_int {
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
                                                    s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const core::ffi::c_char,
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
                                        return -(1 as core::ffi::c_int);
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
                                        (c == 'P' as i32 as uint32_t) as core::ffi::c_int,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as core::ffi::c_int);
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
                                    c = (c as core::ffi::c_uint & 0x1f as core::ffi::c_uint)
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
                                c = CHAR_RANGE_d as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as core::ffi::c_int as uint32_t;
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
                                                return -(1 as core::ffi::c_int);
                                            }
                                            c = (c as core::ffi::c_uint)
                                                .wrapping_add(CLASS_RANGE_BASE as core::ffi::c_uint)
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
                                                    (*s).is_unicode as core::ffi::c_int * 2 as core::ffi::c_int,
                                                );
                                                if ret >= 0 as core::ffi::c_int {
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
                                                    s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const core::ffi::c_char,
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
                                        return -(1 as core::ffi::c_int);
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
                                        (c == 'P' as i32 as uint32_t) as core::ffi::c_int,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as core::ffi::c_int);
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
                                    c = (c as core::ffi::c_uint & 0x1f as core::ffi::c_uint)
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
                                c = CHAR_RANGE_d as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as core::ffi::c_int as uint32_t;
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
                                                return -(1 as core::ffi::c_int);
                                            }
                                            c = (c as core::ffi::c_uint)
                                                .wrapping_add(CLASS_RANGE_BASE as core::ffi::c_uint)
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
                                                    (*s).is_unicode as core::ffi::c_int * 2 as core::ffi::c_int,
                                                );
                                                if ret >= 0 as core::ffi::c_int {
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
                                                    s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const core::ffi::c_char,
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
                                        return -(1 as core::ffi::c_int);
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
                                        (c == 'P' as i32 as uint32_t) as core::ffi::c_int,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as core::ffi::c_int);
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
                                    c = (c as core::ffi::c_uint & 0x1f as core::ffi::c_uint)
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
                                c = CHAR_RANGE_d as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as core::ffi::c_int as uint32_t;
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
                                                return -(1 as core::ffi::c_int);
                                            }
                                            c = (c as core::ffi::c_uint)
                                                .wrapping_add(CLASS_RANGE_BASE as core::ffi::c_uint)
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
                                                    (*s).is_unicode as core::ffi::c_int * 2 as core::ffi::c_int,
                                                );
                                                if ret >= 0 as core::ffi::c_int {
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
                                                    s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const core::ffi::c_char,
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
                                        return -(1 as core::ffi::c_int);
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
                                        (c == 'P' as i32 as uint32_t) as core::ffi::c_int,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as core::ffi::c_int);
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
                                    c = (c as core::ffi::c_uint & 0x1f as core::ffi::c_uint)
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
                                c = CHAR_RANGE_d as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as core::ffi::c_int as uint32_t;
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
                                                return -(1 as core::ffi::c_int);
                                            }
                                            c = (c as core::ffi::c_uint)
                                                .wrapping_add(CLASS_RANGE_BASE as core::ffi::c_uint)
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
                                                    (*s).is_unicode as core::ffi::c_int * 2 as core::ffi::c_int,
                                                );
                                                if ret >= 0 as core::ffi::c_int {
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
                                                    s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const core::ffi::c_char,
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
                                        return -(1 as core::ffi::c_int);
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
                                        (c == 'P' as i32 as uint32_t) as core::ffi::c_int,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as core::ffi::c_int);
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
                                    c = (c as core::ffi::c_uint & 0x1f as core::ffi::c_uint)
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
                                c = CHAR_RANGE_d as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as core::ffi::c_int as uint32_t;
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
                                                return -(1 as core::ffi::c_int);
                                            }
                                            c = (c as core::ffi::c_uint)
                                                .wrapping_add(CLASS_RANGE_BASE as core::ffi::c_uint)
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
                                                    (*s).is_unicode as core::ffi::c_int * 2 as core::ffi::c_int,
                                                );
                                                if ret >= 0 as core::ffi::c_int {
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
                                                    s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const core::ffi::c_char,
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
                                        return -(1 as core::ffi::c_int);
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
                                        (c == 'P' as i32 as uint32_t) as core::ffi::c_int,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as core::ffi::c_int);
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
                                    c = (c as core::ffi::c_uint & 0x1f as core::ffi::c_uint)
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
                                c = CHAR_RANGE_d as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as core::ffi::c_int as uint32_t;
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
                                                return -(1 as core::ffi::c_int);
                                            }
                                            c = (c as core::ffi::c_uint)
                                                .wrapping_add(CLASS_RANGE_BASE as core::ffi::c_uint)
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
                                                    (*s).is_unicode as core::ffi::c_int * 2 as core::ffi::c_int,
                                                );
                                                if ret >= 0 as core::ffi::c_int {
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
                                                    s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const core::ffi::c_char,
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
                                        return -(1 as core::ffi::c_int);
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
                                        (c == 'P' as i32 as uint32_t) as core::ffi::c_int,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as core::ffi::c_int);
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
                                    c = (c as core::ffi::c_uint & 0x1f as core::ffi::c_uint)
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
                                c = CHAR_RANGE_d as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as core::ffi::c_int as uint32_t;
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
                                                return -(1 as core::ffi::c_int);
                                            }
                                            c = (c as core::ffi::c_uint)
                                                .wrapping_add(CLASS_RANGE_BASE as core::ffi::c_uint)
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
                                                    (*s).is_unicode as core::ffi::c_int * 2 as core::ffi::c_int,
                                                );
                                                if ret >= 0 as core::ffi::c_int {
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
                                                    s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const core::ffi::c_char,
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
                                        return -(1 as core::ffi::c_int);
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
                                        (c == 'P' as i32 as uint32_t) as core::ffi::c_int,
                                        (*s).unicode_sets,
                                    ) != 0
                                    {
                                        return -(1 as core::ffi::c_int);
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
                                    c = (c as core::ffi::c_uint & 0x1f as core::ffi::c_uint)
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
                                c = CHAR_RANGE_d as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            17770042771771916326 => {
                                c = CHAR_RANGE_D as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            802061165721266012 => {
                                c = CHAR_RANGE_s as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10822250284037535193 => {
                                c = CHAR_RANGE_S as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            1814055369555096573 => {
                                c = CHAR_RANGE_w as core::ffi::c_int as uint32_t;
                                current_block = 8572234933149657763;
                            }
                            10254712216801151959 => {
                                c = CHAR_RANGE_W as core::ffi::c_int as uint32_t;
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
                                                return -(1 as core::ffi::c_int);
                                            }
                                            c = (c as core::ffi::c_uint)
                                                .wrapping_add(CLASS_RANGE_BASE as core::ffi::c_uint)
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
                                                    (*s).is_unicode as core::ffi::c_int * 2 as core::ffi::c_int,
                                                );
                                                if ret >= 0 as core::ffi::c_int {
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
                                                    s,
                                                    b"invalid escape sequence in regular expression\0"
                                                        as *const u8 as *const core::ffi::c_char,
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
                && *p.offset(1 as core::ffi::c_int as isize) as uint32_t == c
            {
                return re_parse_error(
                    s,
                    b"invalid class set operation in regular expression\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            current_block = 16251750946745332477;
        }
        40 | 41 | 91 | 93 | 123 | 125 | 47 | 45 | 124 => {
            if (*s).unicode_sets != 0 {
                return re_parse_error(
                    s,
                    b"invalid character in class in regular expression\0" as *const u8
                        as *const core::ffi::c_char,
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
                c = unicode_from_utf8(p, UTF8_CHAR_LEN_MAX, &mut p) as uint32_t;
                if c as core::ffi::c_uint > 0xffff as core::ffi::c_uint
                    && (*s).is_unicode == 0
                {
                    return re_parse_error(
                        s,
                        b"malformed unicode char\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
            } else {
                p = p.offset(1);
            }
        }
        13671070048700312155 => {
            return re_parse_error(
                s,
                b"unexpected end\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        _ => {}
    }
    *pp = p;
    return c as core::ffi::c_int;
}
unsafe extern "C" fn re_emit_range(
    mut s: *mut REParseState,
    mut cr: *const CharRange,
) -> core::ffi::c_int {
    let mut len: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    let mut high: uint32_t = 0;
    len = ((*cr).len as core::ffi::c_uint).wrapping_div(2 as core::ffi::c_uint)
        as core::ffi::c_int;
    if len >= 65535 as core::ffi::c_int {
        return re_parse_error(
            s,
            b"too many ranges\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if len == 0 as core::ffi::c_int {
        re_emit_op_u32(
            s,
            REOP_char32 as core::ffi::c_int,
            -(1 as core::ffi::c_int) as uint32_t,
        );
    } else {
        high = *((*cr).points).offset(((*cr).len - 1 as core::ffi::c_int) as isize);
        if high == UINT32_MAX as uint32_t {
            high = *((*cr).points).offset(((*cr).len - 2 as core::ffi::c_int) as isize);
        }
        if high <= 0xffff as uint32_t {
            re_emit_op_u16(
                s,
                if (*s).ignore_case != 0 {
                    REOP_range_i as core::ffi::c_int
                } else {
                    REOP_range as core::ffi::c_int
                },
                len as uint32_t,
            );
            i = 0 as core::ffi::c_int;
            while i < (*cr).len {
                dbuf_put_u16(
                    &mut (*s).byte_code,
                    *((*cr).points).offset(i as isize) as uint16_t,
                );
                high = (*((*cr).points).offset((i + 1 as core::ffi::c_int) as isize))
                    .wrapping_sub(1 as uint32_t);
                if high == (UINT32_MAX as uint32_t).wrapping_sub(1 as uint32_t) {
                    high = 0xffff as uint32_t;
                }
                dbuf_put_u16(&mut (*s).byte_code, high as uint16_t);
                i += 2 as core::ffi::c_int;
            }
        } else {
            re_emit_op_u16(
                s,
                if (*s).ignore_case != 0 {
                    REOP_range32_i as core::ffi::c_int
                } else {
                    REOP_range32 as core::ffi::c_int
                },
                len as uint32_t,
            );
            i = 0 as core::ffi::c_int;
            while i < (*cr).len {
                dbuf_put_u32(&mut (*s).byte_code, *((*cr).points).offset(i as isize));
                dbuf_put_u32(
                    &mut (*s).byte_code,
                    (*((*cr).points).offset((i + 1 as core::ffi::c_int) as isize))
                        .wrapping_sub(1 as uint32_t),
                );
                i += 2 as core::ffi::c_int;
            }
        }
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn re_string_cmp_len(
    mut a: *const core::ffi::c_void,
    mut b: *const core::ffi::c_void,
    mut arg: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let mut p1: *mut REString = *(a as *mut *mut REString);
    let mut p2: *mut REString = *(b as *mut *mut REString);
    return ((*p1).len < (*p2).len) as core::ffi::c_int
        - ((*p1).len > (*p2).len) as core::ffi::c_int;
}
unsafe extern "C" fn re_emit_char(mut s: *mut REParseState, mut c: core::ffi::c_int) {
    if c <= 0xffff as core::ffi::c_int {
        re_emit_op_u16(
            s,
            if (*s).ignore_case != 0 {
                REOP_char_i as core::ffi::c_int
            } else {
                REOP_char as core::ffi::c_int
            },
            c as uint32_t,
        );
    } else {
        re_emit_op_u32(
            s,
            if (*s).ignore_case != 0 {
                REOP_char32_i as core::ffi::c_int
            } else {
                REOP_char32 as core::ffi::c_int
            },
            c as uint32_t,
        );
    };
}
unsafe extern "C" fn re_emit_string_list(
    mut s: *mut REParseState,
    mut sl: *const REStringList,
) -> core::ffi::c_int {
    let mut tab: *mut *mut REString = 0 as *mut *mut REString;
    let mut p: *mut REString = 0 as *mut REString;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut split_pos: core::ffi::c_int = 0;
    let mut last_match_pos: core::ffi::c_int = 0;
    let mut n: core::ffi::c_int = 0;
    let mut has_empty_string: BOOL = 0;
    let mut is_last: BOOL = 0;
    if (*sl).n_strings == 0 as uint32_t {
        if re_emit_range(s, &(*sl).cr) != 0 {
            return -(1 as core::ffi::c_int);
        }
    } else {
        tab = lre_realloc(
            (*s).opaque,
            NULL,
            (::core::mem::size_of::<*mut REString>() as size_t)
                .wrapping_mul((*sl).n_strings as size_t),
        ) as *mut *mut REString;
        if tab.is_null() {
            re_parse_out_of_memory(s);
            return -(1 as core::ffi::c_int);
        }
        has_empty_string = FALSE as core::ffi::c_int as BOOL;
        n = 0 as core::ffi::c_int;
        i = 0 as core::ffi::c_int;
        while (i as uint32_t) < (*sl).hash_size {
            p = *((*sl).hash_table).offset(i as isize);
            while !p.is_null() {
                if (*p).len == 0 as uint32_t {
                    has_empty_string = TRUE as core::ffi::c_int as BOOL;
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
        if n as uint32_t <= (*sl).n_strings {} else {
            __assert_fail(
                b"n <= sl->n_strings\0" as *const u8 as *const core::ffi::c_char,
                b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                1306 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 62],
                    [core::ffi::c_char; 62],
                >(*b"int re_emit_string_list(REParseState *, const REStringList *)\0"))
                    .as_ptr(),
            );
        }
        'c_7863: {
            if n as uint32_t <= (*sl).n_strings {} else {
                __assert_fail(
                    b"n <= sl->n_strings\0" as *const u8 as *const core::ffi::c_char,
                    b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                    1306 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 62],
                        [core::ffi::c_char; 62],
                    >(
                        *b"int re_emit_string_list(REParseState *, const REStringList *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        rqsort(
            tab as *mut core::ffi::c_void,
            n as size_t,
            ::core::mem::size_of::<*mut REString>() as size_t,
            Some(
                re_string_cmp_len
                    as unsafe extern "C" fn(
                        *const core::ffi::c_void,
                        *const core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> core::ffi::c_int,
            ),
            NULL,
        );
        last_match_pos = -(1 as core::ffi::c_int);
        i = 0 as core::ffi::c_int;
        while i < n {
            p = *tab.offset(i as isize);
            is_last = (has_empty_string == 0 && (*sl).cr.len == 0 as core::ffi::c_int
                && i == n - 1 as core::ffi::c_int) as core::ffi::c_int as BOOL;
            if is_last == 0 {
                split_pos = re_emit_op_u32(
                    s,
                    REOP_split_next_first as core::ffi::c_int,
                    0 as uint32_t,
                );
            } else {
                split_pos = 0 as core::ffi::c_int;
            }
            j = 0 as core::ffi::c_int;
            while (j as uint32_t) < (*p).len {
                re_emit_char(
                    s,
                    *((*p).buf).as_mut_ptr().offset(j as isize) as core::ffi::c_int,
                );
                j += 1;
            }
            if is_last == 0 {
                last_match_pos = re_emit_op_u32(
                    s,
                    REOP_goto as core::ffi::c_int,
                    last_match_pos as uint32_t,
                );
                put_u32(
                    ((*s).byte_code.buf).offset(split_pos as isize),
                    ((*s).byte_code.size)
                        .wrapping_sub((split_pos + 4 as core::ffi::c_int) as size_t)
                        as uint32_t,
                );
            }
            i += 1;
        }
        if (*sl).cr.len != 0 as core::ffi::c_int {
            is_last = (has_empty_string == 0) as core::ffi::c_int as BOOL;
            if is_last == 0 {
                split_pos = re_emit_op_u32(
                    s,
                    REOP_split_next_first as core::ffi::c_int,
                    0 as uint32_t,
                );
            } else {
                split_pos = 0 as core::ffi::c_int;
            }
            if re_emit_range(s, &(*sl).cr) != 0 {
                lre_realloc((*s).opaque, tab as *mut core::ffi::c_void, 0 as size_t);
                return -(1 as core::ffi::c_int);
            }
            if is_last == 0 {
                put_u32(
                    ((*s).byte_code.buf).offset(split_pos as isize),
                    ((*s).byte_code.size)
                        .wrapping_sub((split_pos + 4 as core::ffi::c_int) as size_t)
                        as uint32_t,
                );
            }
        }
        while last_match_pos != -(1 as core::ffi::c_int) {
            let mut next_pos: core::ffi::c_int = get_u32(
                ((*s).byte_code.buf).offset(last_match_pos as isize),
            ) as core::ffi::c_int;
            put_u32(
                ((*s).byte_code.buf).offset(last_match_pos as isize),
                ((*s).byte_code.size)
                    .wrapping_sub((last_match_pos + 4 as core::ffi::c_int) as size_t)
                    as uint32_t,
            );
            last_match_pos = next_pos;
        }
        lre_realloc((*s).opaque, tab as *mut core::ffi::c_void, 0 as size_t);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn re_parse_class_set_operand(
    mut s: *mut REParseState,
    mut cr: *mut REStringList,
    mut pp: *mut *const uint8_t,
) -> core::ffi::c_int {
    let mut c1: core::ffi::c_int = 0;
    let mut p: *const uint8_t = *pp;
    if *p as core::ffi::c_int == '[' as i32 {
        if re_parse_nested_class(s, cr, pp) != 0 {
            return -(1 as core::ffi::c_int);
        }
    } else {
        c1 = get_class_atom(s, cr, pp, TRUE as core::ffi::c_int as BOOL);
        if c1 < 0 as core::ffi::c_int {
            return -(1 as core::ffi::c_int);
        }
        if c1 < CLASS_RANGE_BASE {
            re_string_list_init(s, cr);
            if (*s).ignore_case != 0 {
                c1 = lre_canonicalize(
                    c1 as uint32_t,
                    (*s).is_unicode as core::ffi::c_int,
                );
            }
            if cr_union_interval(&mut (*cr).cr, c1 as uint32_t, c1 as uint32_t) != 0 {
                re_string_list_free(cr);
                return -(1 as core::ffi::c_int);
            }
        }
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn re_parse_nested_class(
    mut s: *mut REParseState,
    mut cr: *mut REStringList,
    mut pp: *mut *const uint8_t,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut c1: uint32_t = 0;
    let mut c2: uint32_t = 0;
    let mut ret: core::ffi::c_int = 0;
    let mut cr1_s: REStringList = REStringList {
        cr: CharRange {
            len: 0,
            size: 0,
            points: 0 as *mut uint32_t,
            mem_opaque: 0 as *mut core::ffi::c_void,
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
            s,
            b"stack overflow\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    re_string_list_init(s, cr);
    p = *pp;
    p = p.offset(1);
    invert = FALSE as core::ffi::c_int as BOOL;
    if *p as core::ffi::c_int == '^' as i32 {
        p = p.offset(1);
        invert = TRUE as core::ffi::c_int as BOOL;
    }
    is_first = TRUE as core::ffi::c_int as BOOL;
    's_53: loop {
        if *p as core::ffi::c_int == ']' as i32 {
            current_block = 16029476503615101993;
            break;
        }
        if *p as core::ffi::c_int == '[' as i32 && (*s).unicode_sets != 0 {
            if re_parse_nested_class(s, cr1, &mut p) != 0 {
                current_block = 16066372098344664684;
                break;
            }
            current_block = 12477921065514496907;
        } else {
            c1 = get_class_atom(s, cr1, &mut p, TRUE as core::ffi::c_int as BOOL)
                as uint32_t;
            if (c1 as core::ffi::c_int) < 0 as core::ffi::c_int {
                current_block = 16066372098344664684;
                break;
            }
            if *p as core::ffi::c_int == '-' as i32
                && *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    != ']' as i32
            {
                let mut p0: *const uint8_t = p.offset(1 as core::ffi::c_int as isize);
                if *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
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
                            TRUE as core::ffi::c_int as BOOL,
                        ) as uint32_t;
                        if (c2 as core::ffi::c_int) < 0 as core::ffi::c_int {
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
                                        mem_opaque: 0 as *mut core::ffi::c_void,
                                        realloc_func: None,
                                    };
                                    let mut cr2: *mut CharRange = &mut cr2_s;
                                    cr_init(
                                        cr2,
                                        (*s).opaque,
                                        Some(
                                            lre_realloc
                                                as unsafe extern "C" fn(
                                                    *mut core::ffi::c_void,
                                                    *mut core::ffi::c_void,
                                                    size_t,
                                                ) -> *mut core::ffi::c_void,
                                        ),
                                    );
                                    if cr_add_interval(cr2, c1, c2.wrapping_add(1 as uint32_t))
                                        != 0
                                        || cr_regexp_canonicalize(
                                            cr2,
                                            (*s).is_unicode as core::ffi::c_int,
                                        ) != 0
                                        || cr_op1(
                                            &mut (*cr).cr,
                                            (*cr2).points,
                                            (*cr2).len,
                                            CR_OP_UNION as core::ffi::c_int,
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
                                is_first = FALSE as core::ffi::c_int as BOOL;
                                current_block = 1434579379687443766;
                            }
                        }
                    }
                    match current_block {
                        1434579379687443766 => {}
                        5845765319767473285 => {}
                        _ => {
                            re_parse_error(
                                s,
                                b"invalid class range\0" as *const u8
                                    as *const core::ffi::c_char,
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
                                (*s).is_unicode as core::ffi::c_int,
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
                ret = re_string_list_op(cr, cr1, CR_OP_UNION as core::ffi::c_int);
                re_string_list_free(cr1);
                if ret != 0 {
                    current_block = 1052705618141951789;
                    break;
                }
            }
            _ => {}
        }
        if (*s).unicode_sets != 0 && is_first != 0 {
            if *p as core::ffi::c_int == '&' as i32
                && *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    == '&' as i32
                && *p.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                    != '&' as i32
            {
                loop {
                    if *p as core::ffi::c_int == ']' as i32 {
                        current_block = 5684854171168229155;
                        break;
                    }
                    if !(*p as core::ffi::c_int == '&' as i32
                        && *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                            == '&' as i32
                        && *p.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                            != '&' as i32)
                    {
                        current_block = 10490727108771021976;
                        break;
                    }
                    p = p.offset(2 as core::ffi::c_int as isize);
                    if re_parse_class_set_operand(s, cr1, &mut p) != 0 {
                        current_block = 16066372098344664684;
                        break 's_53;
                    }
                    ret = re_string_list_op(cr, cr1, CR_OP_INTER as core::ffi::c_int);
                    re_string_list_free(cr1);
                    if ret != 0 {
                        current_block = 1052705618141951789;
                        break 's_53;
                    }
                }
            } else if *p as core::ffi::c_int == '-' as i32
                && *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    == '-' as i32
            {
                loop {
                    if *p as core::ffi::c_int == ']' as i32 {
                        current_block = 5684854171168229155;
                        break;
                    }
                    if !(*p as core::ffi::c_int == '-' as i32
                        && *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                            == '-' as i32)
                    {
                        current_block = 10490727108771021976;
                        break;
                    }
                    p = p.offset(2 as core::ffi::c_int as isize);
                    if re_parse_class_set_operand(s, cr1, &mut p) != 0 {
                        current_block = 16066372098344664684;
                        break 's_53;
                    }
                    ret = re_string_list_op(cr, cr1, CR_OP_SUB as core::ffi::c_int);
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
                        s,
                        b"invalid operation in regular expression\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    current_block = 16066372098344664684;
                    break;
                }
            }
        }
        is_first = FALSE as core::ffi::c_int as BOOL;
    }
    match current_block {
        16029476503615101993 => {
            p = p.offset(1);
            *pp = p;
            if invert != 0 {
                if (*cr).n_strings != 0 as uint32_t {
                    re_parse_error(
                        s,
                        b"negated character class with strings in regular expression debugger eval code\0"
                            as *const u8 as *const core::ffi::c_char,
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
                _ => return 0 as core::ffi::c_int,
            }
        }
        _ => {}
    }
    match current_block {
        1052705618141951789 => {
            re_parse_out_of_memory(s);
        }
        _ => {}
    }
    re_string_list_free(cr);
    return -(1 as core::ffi::c_int);
}
unsafe extern "C" fn re_parse_char_class(
    mut s: *mut REParseState,
    mut pp: *mut *const uint8_t,
) -> core::ffi::c_int {
    let mut cr_s: REStringList = REStringList {
        cr: CharRange {
            len: 0,
            size: 0,
            points: 0 as *mut uint32_t,
            mem_opaque: 0 as *mut core::ffi::c_void,
            realloc_func: None,
        },
        n_strings: 0,
        hash_size: 0,
        hash_bits: 0,
        hash_table: 0 as *mut *mut REString,
    };
    let mut cr: *mut REStringList = &mut cr_s;
    if re_parse_nested_class(s, cr, pp) != 0 {
        return -(1 as core::ffi::c_int);
    }
    if re_emit_string_list(s, cr) != 0 {
        re_string_list_free(cr);
        return -(1 as core::ffi::c_int);
    } else {
        re_string_list_free(cr);
        return 0 as core::ffi::c_int;
    };
}
unsafe extern "C" fn re_need_check_adv_and_capture_init(
    mut pneed_capture_init: *mut BOOL,
    mut bc_buf: *const uint8_t,
    mut bc_buf_len: core::ffi::c_int,
) -> BOOL {
    let mut pos: core::ffi::c_int = 0;
    let mut opcode: core::ffi::c_int = 0;
    let mut len: core::ffi::c_int = 0;
    let mut val: uint32_t = 0;
    let mut need_check_adv: BOOL = 0;
    let mut need_capture_init: BOOL = 0;
    need_check_adv = TRUE as core::ffi::c_int as BOOL;
    need_capture_init = FALSE as core::ffi::c_int as BOOL;
    pos = 0 as core::ffi::c_int;
    while pos < bc_buf_len {
        opcode = *bc_buf.offset(pos as isize) as core::ffi::c_int;
        len = reopcode_info[opcode as usize].size as core::ffi::c_int;
        match opcode {
            36 | 37 => {
                val = get_u16(
                    bc_buf.offset(pos as isize).offset(1 as core::ffi::c_int as isize),
                );
                len = (len as core::ffi::c_uint)
                    .wrapping_add(val.wrapping_mul(4 as uint32_t) as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_int;
                need_check_adv = FALSE as core::ffi::c_int as BOOL;
            }
            38 | 39 => {
                val = get_u16(
                    bc_buf.offset(pos as isize).offset(1 as core::ffi::c_int as isize),
                );
                len = (len as core::ffi::c_uint)
                    .wrapping_add(val.wrapping_mul(8 as uint32_t) as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_int;
                need_check_adv = FALSE as core::ffi::c_int as BOOL;
            }
            1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                need_check_adv = FALSE as core::ffi::c_int as BOOL;
            }
            9 | 10 | 11 | 12 | 27 | 42 | 28 | 29 | 30 | 31 | 44 | 19 | 20 | 21 => {}
            32 | 33 | 34 | 35 => {
                val = *bc_buf.offset((pos + 1 as core::ffi::c_int) as isize) as uint32_t;
                len = (len as core::ffi::c_uint).wrapping_add(val as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_int;
                need_capture_init = TRUE as core::ffi::c_int as BOOL;
            }
            _ => {
                need_capture_init = TRUE as core::ffi::c_int as BOOL;
                break;
            }
        }
        pos += len;
    }
    *pneed_capture_init = need_capture_init;
    return need_check_adv;
}
unsafe extern "C" fn re_parse_group_name(
    mut buf: *mut core::ffi::c_char,
    mut buf_size: core::ffi::c_int,
    mut pp: *mut *const uint8_t,
) -> core::ffi::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut p1: *const uint8_t = 0 as *const uint8_t;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut q: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    p = *pp;
    q = buf;
    loop {
        c = *p as uint32_t;
        if c == '\\' as i32 as uint32_t {
            p = p.offset(1);
            if *p as core::ffi::c_int != 'u' as i32 {
                return -(1 as core::ffi::c_int);
            }
            c = lre_parse_escape(&mut p, 2 as core::ffi::c_int) as uint32_t;
        } else {
            if c == '>' as i32 as uint32_t {
                break;
            }
            if c >= 128 as uint32_t {
                c = unicode_from_utf8(p, UTF8_CHAR_LEN_MAX, &mut p) as uint32_t;
                if is_hi_surrogate(c) != 0 {
                    d = unicode_from_utf8(p, UTF8_CHAR_LEN_MAX, &mut p1) as uint32_t;
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
            return -(1 as core::ffi::c_int);
        }
        if q == buf {
            if lre_js_is_ident_first(c) == 0 {
                return -(1 as core::ffi::c_int);
            }
        } else if lre_js_is_ident_next(c) == 0 {
            return -(1 as core::ffi::c_int)
        }
        if q.offset_from(buf) as core::ffi::c_long
            + UTF8_CHAR_LEN_MAX as core::ffi::c_long + 1 as core::ffi::c_long
            > buf_size as core::ffi::c_long
        {
            return -(1 as core::ffi::c_int);
        }
        if c < 128 as uint32_t {
            let fresh41 = q;
            q = q.offset(1);
            *fresh41 = c as core::ffi::c_char;
        } else {
            q = q
                .offset(
                    unicode_to_utf8(q as *mut uint8_t, c as core::ffi::c_uint) as isize,
                );
        }
    }
    if q == buf {
        return -(1 as core::ffi::c_int);
    }
    *q = '\0' as i32 as core::ffi::c_char;
    p = p.offset(1);
    *pp = p;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn re_parse_captures(
    mut s: *mut REParseState,
    mut phas_named_captures: *mut core::ffi::c_int,
    mut capture_name: *const core::ffi::c_char,
    mut emit_group_index: BOOL,
) -> core::ffi::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut capture_index: core::ffi::c_int = 0;
    let mut n: core::ffi::c_int = 0;
    let mut name: [core::ffi::c_char; 128] = [0; 128];
    capture_index = 1 as core::ffi::c_int;
    n = 0 as core::ffi::c_int;
    *phas_named_captures = 0 as core::ffi::c_int;
    p = (*s).buf_start;
    while p < (*s).buf_end {
        match *p as core::ffi::c_int {
            40 => {
                if *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    == '?' as i32
                {
                    if *p.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                        == '<' as i32
                        && *p.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                            != '=' as i32
                        && *p.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                            != '!' as i32
                    {
                        *phas_named_captures = 1 as core::ffi::c_int;
                        if !capture_name.is_null() {
                            p = p.offset(3 as core::ffi::c_int as isize);
                            if re_parse_group_name(
                                name.as_mut_ptr(),
                                ::core::mem::size_of::<[core::ffi::c_char; 128]>()
                                    as core::ffi::c_int,
                                &mut p,
                            ) == 0 as core::ffi::c_int
                            {
                                if strcmp(name.as_mut_ptr(), capture_name) == 0 {
                                    if emit_group_index != 0 {
                                        dbuf_putc(&mut (*s).byte_code, capture_index as uint8_t);
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
            92 => {
                p = p.offset(1);
            }
            91 => {
                p = p
                    .offset(
                        (1 as core::ffi::c_int
                            + (*p as core::ffi::c_int == ']' as i32) as core::ffi::c_int)
                            as isize,
                    );
                while p < (*s).buf_end && *p as core::ffi::c_int != ']' as i32 {
                    if *p as core::ffi::c_int == '\\' as i32 {
                        p = p.offset(1);
                    }
                    p = p.offset(1);
                }
            }
            _ => {}
        }
        p = p.offset(1);
    }
    if !capture_name.is_null() { return n } else { return capture_index };
}
unsafe extern "C" fn re_count_captures(mut s: *mut REParseState) -> core::ffi::c_int {
    if (*s).total_capture_count < 0 as core::ffi::c_int {
        (*s).total_capture_count = re_parse_captures(
            s,
            &mut (*s).has_named_captures,
            0 as *const core::ffi::c_char,
            FALSE as core::ffi::c_int as BOOL,
        );
    }
    return (*s).total_capture_count;
}
unsafe extern "C" fn re_has_named_captures(mut s: *mut REParseState) -> BOOL {
    if (*s).has_named_captures < 0 as core::ffi::c_int {
        re_count_captures(s);
    }
    return (*s).has_named_captures as BOOL;
}
unsafe extern "C" fn find_group_name(
    mut s: *mut REParseState,
    mut name: *const core::ffi::c_char,
    mut emit_group_index: BOOL,
) -> core::ffi::c_int {
    let mut p: *const core::ffi::c_char = 0 as *const core::ffi::c_char;
    let mut buf_end: *const core::ffi::c_char = 0 as *const core::ffi::c_char;
    let mut len: size_t = 0;
    let mut name_len: size_t = 0;
    let mut capture_index: core::ffi::c_int = 0;
    let mut n: core::ffi::c_int = 0;
    p = (*s).group_names.buf as *mut core::ffi::c_char;
    if p.is_null() {
        return 0 as core::ffi::c_int;
    }
    buf_end = ((*s).group_names.buf as *mut core::ffi::c_char)
        .offset((*s).group_names.size as isize);
    name_len = strlen(name);
    capture_index = 1 as core::ffi::c_int;
    n = 0 as core::ffi::c_int;
    while p < buf_end {
        len = strlen(p);
        if len == name_len
            && memcmp(
                name as *const core::ffi::c_void,
                p as *const core::ffi::c_void,
                name_len,
            ) == 0 as core::ffi::c_int
        {
            if emit_group_index != 0 {
                dbuf_putc(&mut (*s).byte_code, capture_index as uint8_t);
            }
            n += 1;
        }
        p = p.offset(len.wrapping_add(LRE_GROUP_NAME_TRAILER_LEN as size_t) as isize);
        capture_index += 1;
    }
    return n;
}
unsafe extern "C" fn is_duplicate_group_name(
    mut s: *mut REParseState,
    mut name: *const core::ffi::c_char,
    mut scope: core::ffi::c_int,
) -> BOOL {
    let mut p: *const core::ffi::c_char = 0 as *const core::ffi::c_char;
    let mut buf_end: *const core::ffi::c_char = 0 as *const core::ffi::c_char;
    let mut len: size_t = 0;
    let mut name_len: size_t = 0;
    let mut scope1: core::ffi::c_int = 0;
    p = (*s).group_names.buf as *mut core::ffi::c_char;
    if p.is_null() {
        return 0 as BOOL;
    }
    buf_end = ((*s).group_names.buf as *mut core::ffi::c_char)
        .offset((*s).group_names.size as isize);
    name_len = strlen(name);
    while p < buf_end {
        len = strlen(p);
        if len == name_len
            && memcmp(
                name as *const core::ffi::c_void,
                p as *const core::ffi::c_void,
                name_len,
            ) == 0 as core::ffi::c_int
        {
            scope1 = *p.offset(len.wrapping_add(1 as size_t) as isize) as uint8_t
                as core::ffi::c_int;
            if scope == scope1 {
                return TRUE as core::ffi::c_int as BOOL;
            }
        }
        p = p.offset(len.wrapping_add(LRE_GROUP_NAME_TRAILER_LEN as size_t) as isize);
    }
    return FALSE as core::ffi::c_int as BOOL;
}
unsafe extern "C" fn re_parse_modifiers(
    mut s: *mut REParseState,
    mut pp: *mut *const uint8_t,
) -> core::ffi::c_int {
    let mut p: *const uint8_t = *pp;
    let mut mask: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut val: core::ffi::c_int = 0;
    loop {
        if *p as core::ffi::c_int == 'i' as i32 {
            val = LRE_FLAG_IGNORECASE;
        } else if *p as core::ffi::c_int == 'm' as i32 {
            val = LRE_FLAG_MULTILINE;
        } else {
            if !(*p as core::ffi::c_int == 's' as i32) {
                break;
            }
            val = LRE_FLAG_DOTALL;
        }
        if mask & val != 0 {
            return re_parse_error(
                s,
                b"duplicate modifier: '%c'\0" as *const u8 as *const core::ffi::c_char,
                *p as core::ffi::c_int,
            );
        }
        mask |= val;
        p = p.offset(1);
    }
    *pp = p;
    return mask;
}
unsafe extern "C" fn update_modifier(
    mut val: BOOL,
    mut add_mask: core::ffi::c_int,
    mut remove_mask: core::ffi::c_int,
    mut mask: core::ffi::c_int,
) -> BOOL {
    if add_mask & mask != 0 {
        val = TRUE as core::ffi::c_int as BOOL;
    }
    if remove_mask & mask != 0 {
        val = FALSE as core::ffi::c_int as BOOL;
    }
    return val;
}
unsafe extern "C" fn re_parse_term(
    mut s: *mut REParseState,
    mut is_backward_dir: BOOL,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut c: core::ffi::c_int = 0;
    let mut last_atom_start: core::ffi::c_int = 0;
    let mut quant_min: core::ffi::c_int = 0;
    let mut quant_max: core::ffi::c_int = 0;
    let mut last_capture_count: core::ffi::c_int = 0;
    let mut greedy: BOOL = 0;
    let mut is_neg: BOOL = 0;
    let mut is_backward_lookahead: BOOL = 0;
    let mut cr_s: REStringList = REStringList {
        cr: CharRange {
            len: 0,
            size: 0,
            points: 0 as *mut uint32_t,
            mem_opaque: 0 as *mut core::ffi::c_void,
            realloc_func: None,
        },
        n_strings: 0,
        hash_size: 0,
        hash_bits: 0,
        hash_table: 0 as *mut *mut REString,
    };
    let mut cr: *mut REStringList = &mut cr_s;
    last_atom_start = -(1 as core::ffi::c_int);
    last_capture_count = 0 as core::ffi::c_int;
    p = (*s).buf_ptr;
    c = *p as core::ffi::c_int;
    match c {
        94 => {
            p = p.offset(1);
            re_emit_op(
                s,
                if (*s).multi_line != 0 {
                    REOP_line_start_m as core::ffi::c_int
                } else {
                    REOP_line_start as core::ffi::c_int
                },
            );
            current_block = 1771738965274008886;
        }
        36 => {
            p = p.offset(1);
            re_emit_op(
                s,
                if (*s).multi_line != 0 {
                    REOP_line_end_m as core::ffi::c_int
                } else {
                    REOP_line_end as core::ffi::c_int
                },
            );
            current_block = 1771738965274008886;
        }
        46 => {
            p = p.offset(1);
            last_atom_start = (*s).byte_code.size as core::ffi::c_int;
            last_capture_count = (*s).capture_count;
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as core::ffi::c_int);
            }
            re_emit_op(
                s,
                if (*s).dotall != 0 {
                    REOP_any as core::ffi::c_int
                } else {
                    REOP_dot as core::ffi::c_int
                },
            );
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as core::ffi::c_int);
            }
            current_block = 1771738965274008886;
        }
        123 => {
            if (*s).is_unicode != 0 {
                return re_parse_error(
                    s,
                    b"syntax error\0" as *const u8 as *const core::ffi::c_char,
                )
            } else if is_digit(
                *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int,
            ) == 0
            {
                current_block = 9143481802853542599;
            } else {
                let mut p1: *const uint8_t = p.offset(1 as core::ffi::c_int as isize);
                parse_digits(&mut p1, TRUE as core::ffi::c_int as BOOL);
                if *p1 as core::ffi::c_int == ',' as i32 {
                    p1 = p1.offset(1);
                    if is_digit(*p1 as core::ffi::c_int) != 0 {
                        parse_digits(&mut p1, TRUE as core::ffi::c_int as BOOL);
                    }
                }
                if *p1 as core::ffi::c_int != '}' as i32 {
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
            let mut pos: core::ffi::c_int = 0;
            let mut capture_index: core::ffi::c_int = 0;
            let mut current_block_118: u64;
            if *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                == '?' as i32
            {
                if *p.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                    == ':' as i32
                {
                    p = p.offset(3 as core::ffi::c_int as isize);
                    last_atom_start = (*s).byte_code.size as core::ffi::c_int;
                    last_capture_count = (*s).capture_count;
                    (*s).buf_ptr = p;
                    if re_parse_disjunction(s, is_backward_dir) != 0 {
                        return -(1 as core::ffi::c_int);
                    }
                    p = (*s).buf_ptr;
                    if re_parse_expect(s, &mut p, ')' as i32) != 0 {
                        return -(1 as core::ffi::c_int);
                    }
                    current_block_118 = 1934991416718554651;
                } else if *p.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                    == 'i' as i32
                    || *p.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                        == 'm' as i32
                    || *p.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                        == 's' as i32
                    || *p.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                        == '-' as i32
                {
                    let mut saved_ignore_case: BOOL = 0;
                    let mut saved_multi_line: BOOL = 0;
                    let mut saved_dotall: BOOL = 0;
                    let mut add_mask: core::ffi::c_int = 0;
                    let mut remove_mask: core::ffi::c_int = 0;
                    p = p.offset(2 as core::ffi::c_int as isize);
                    remove_mask = 0 as core::ffi::c_int;
                    add_mask = re_parse_modifiers(s, &mut p);
                    if add_mask < 0 as core::ffi::c_int {
                        return -(1 as core::ffi::c_int);
                    }
                    if *p as core::ffi::c_int == '-' as i32 {
                        p = p.offset(1);
                        remove_mask = re_parse_modifiers(s, &mut p);
                        if remove_mask < 0 as core::ffi::c_int {
                            return -(1 as core::ffi::c_int);
                        }
                    }
                    if add_mask == 0 as core::ffi::c_int
                        && remove_mask == 0 as core::ffi::c_int
                        || add_mask & remove_mask != 0 as core::ffi::c_int
                    {
                        return re_parse_error(
                            s,
                            b"invalid modifiers\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    if re_parse_expect(s, &mut p, ':' as i32) != 0 {
                        return -(1 as core::ffi::c_int);
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
                    last_atom_start = (*s).byte_code.size as core::ffi::c_int;
                    last_capture_count = (*s).capture_count;
                    (*s).buf_ptr = p;
                    if re_parse_disjunction(s, is_backward_dir) != 0 {
                        return -(1 as core::ffi::c_int);
                    }
                    p = (*s).buf_ptr;
                    if re_parse_expect(s, &mut p, ')' as i32) != 0 {
                        return -(1 as core::ffi::c_int);
                    }
                    (*s).ignore_case = saved_ignore_case;
                    (*s).multi_line = saved_multi_line;
                    (*s).dotall = saved_dotall;
                    current_block_118 = 1934991416718554651;
                } else {
                    if *p.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                        == '=' as i32
                        || *p.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                            == '!' as i32
                    {
                        is_neg = (*p.offset(2 as core::ffi::c_int as isize)
                            as core::ffi::c_int == '!' as i32) as core::ffi::c_int
                            as BOOL;
                        is_backward_lookahead = FALSE as core::ffi::c_int as BOOL;
                        p = p.offset(3 as core::ffi::c_int as isize);
                        current_block_118 = 15996867562260755014;
                    } else if *p.offset(2 as core::ffi::c_int as isize)
                        as core::ffi::c_int == '<' as i32
                        && (*p.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                            == '=' as i32
                            || *p.offset(3 as core::ffi::c_int as isize)
                                as core::ffi::c_int == '!' as i32)
                    {
                        pos = 0;
                        is_neg = (*p.offset(3 as core::ffi::c_int as isize)
                            as core::ffi::c_int == '!' as i32) as core::ffi::c_int
                            as BOOL;
                        is_backward_lookahead = TRUE as core::ffi::c_int as BOOL;
                        p = p.offset(4 as core::ffi::c_int as isize);
                        current_block_118 = 15996867562260755014;
                    } else {
                        if *p.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                            == '<' as i32
                        {
                            p = p.offset(3 as core::ffi::c_int as isize);
                            if re_parse_group_name(
                                ((*s).u.tmp_buf).as_mut_ptr(),
                                ::core::mem::size_of::<[core::ffi::c_char; 128]>()
                                    as core::ffi::c_int,
                                &mut p,
                            ) != 0
                            {
                                return re_parse_error(
                                    s,
                                    b"invalid group name\0" as *const u8
                                        as *const core::ffi::c_char,
                                );
                            }
                            if is_duplicate_group_name(
                                s,
                                ((*s).u.tmp_buf).as_mut_ptr(),
                                (*s).group_name_scope as core::ffi::c_int,
                            ) != 0
                            {
                                return re_parse_error(
                                    s,
                                    b"duplicate group name\0" as *const u8
                                        as *const core::ffi::c_char,
                                );
                            }
                            dbuf_put(
                                &mut (*s).group_names,
                                ((*s).u.tmp_buf).as_mut_ptr() as *mut uint8_t,
                                (strlen(((*s).u.tmp_buf).as_mut_ptr()))
                                    .wrapping_add(1 as size_t),
                            );
                            dbuf_putc(&mut (*s).group_names, (*s).group_name_scope);
                            (*s).has_named_captures = 1 as core::ffi::c_int;
                        } else {
                            return re_parse_error(
                                s,
                                b"invalid group\0" as *const u8 as *const core::ffi::c_char,
                            )
                        }
                        current_block_118 = 115339115514607209;
                    }
                    match current_block_118 {
                        115339115514607209 => {}
                        _ => {
                            if (*s).is_unicode == 0 && is_backward_lookahead == 0 {
                                last_atom_start = (*s).byte_code.size as core::ffi::c_int;
                                last_capture_count = (*s).capture_count;
                            }
                            pos = re_emit_op_u32(
                                s,
                                REOP_lookahead as core::ffi::c_int
                                    + is_neg as core::ffi::c_int,
                                0 as uint32_t,
                            );
                            (*s).buf_ptr = p;
                            if re_parse_disjunction(s, is_backward_lookahead) != 0 {
                                return -(1 as core::ffi::c_int);
                            }
                            p = (*s).buf_ptr;
                            if re_parse_expect(s, &mut p, ')' as i32) != 0 {
                                return -(1 as core::ffi::c_int);
                            }
                            re_emit_op(
                                s,
                                REOP_lookahead_match as core::ffi::c_int
                                    + is_neg as core::ffi::c_int,
                            );
                            if dbuf_error(&mut (*s).byte_code) != 0 {
                                return -(1 as core::ffi::c_int);
                            }
                            put_u32(
                                ((*s).byte_code.buf).offset(pos as isize),
                                ((*s).byte_code.size)
                                    .wrapping_sub((pos + 4 as core::ffi::c_int) as size_t)
                                    as uint32_t,
                            );
                            current_block_118 = 1934991416718554651;
                        }
                    }
                }
            } else {
                capture_index = 0;
                p = p.offset(1);
                dbuf_putc(&mut (*s).group_names, 0 as uint8_t);
                dbuf_putc(&mut (*s).group_names, 0 as uint8_t);
                current_block_118 = 115339115514607209;
            }
            match current_block_118 {
                115339115514607209 => {
                    if (*s).capture_count >= CAPTURE_COUNT_MAX {
                        return re_parse_error(
                            s,
                            b"too many captures\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    last_atom_start = (*s).byte_code.size as core::ffi::c_int;
                    last_capture_count = (*s).capture_count;
                    let fresh1 = (*s).capture_count;
                    (*s).capture_count = (*s).capture_count + 1;
                    capture_index = fresh1;
                    re_emit_op_u8(
                        s,
                        REOP_save_start as core::ffi::c_int
                            + is_backward_dir as core::ffi::c_int,
                        capture_index as uint32_t,
                    );
                    (*s).buf_ptr = p;
                    if re_parse_disjunction(s, is_backward_dir) != 0 {
                        return -(1 as core::ffi::c_int);
                    }
                    p = (*s).buf_ptr;
                    re_emit_op_u8(
                        s,
                        REOP_save_start as core::ffi::c_int + 1 as core::ffi::c_int
                            - is_backward_dir as core::ffi::c_int,
                        capture_index as uint32_t,
                    );
                    if re_parse_expect(s, &mut p, ')' as i32) != 0 {
                        return -(1 as core::ffi::c_int);
                    }
                }
                _ => {}
            }
            current_block = 1771738965274008886;
        }
        92 => {
            match *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int {
                98 | 66 => {
                    current_block = 8351012430473106477;
                    match current_block {
                        8351012430473106477 => {
                            if *p.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int != 'b' as i32
                            {
                                re_emit_op(
                                    s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_not_word_boundary_i as core::ffi::c_int
                                    } else {
                                        REOP_not_word_boundary as core::ffi::c_int
                                    },
                                );
                            } else {
                                re_emit_op(
                                    s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_word_boundary_i as core::ffi::c_int
                                    } else {
                                        REOP_word_boundary as core::ffi::c_int
                                    },
                                );
                            }
                            p = p.offset(2 as core::ffi::c_int as isize);
                            current_block = 1771738965274008886;
                        }
                        3824880322238534506 => {
                            p = p.offset(2 as core::ffi::c_int as isize);
                            c = 0 as core::ffi::c_int;
                            if (*s).is_unicode != 0 {
                                if is_digit(*p as core::ffi::c_int) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"invalid decimal escape in regular expression\0"
                                            as *const u8 as *const core::ffi::c_char,
                                    );
                                }
                            } else if *p as core::ffi::c_int >= '0' as i32
                                && *p as core::ffi::c_int <= '7' as i32
                            {
                                let fresh2 = p;
                                p = p.offset(1);
                                c = *fresh2 as core::ffi::c_int - '0' as i32;
                                if *p as core::ffi::c_int >= '0' as i32
                                    && *p as core::ffi::c_int <= '7' as i32
                                {
                                    let fresh3 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as core::ffi::c_int)
                                        + *fresh3 as core::ffi::c_int - '0' as i32;
                                }
                            }
                            current_block = 4664037234821577316;
                        }
                        5846959088466685742 => {
                            let mut p1_0: *const uint8_t = 0 as *const uint8_t;
                            let mut dummy_res: core::ffi::c_int = 0;
                            let mut n: core::ffi::c_int = 0;
                            let mut is_forward: BOOL = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as core::ffi::c_int as isize)
                                as core::ffi::c_int != '<' as i32
                            {
                                if (*s).is_unicode != 0 || re_has_named_captures(s) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"expecting group name\0" as *const u8
                                            as *const core::ffi::c_char,
                                    )
                                } else {
                                    current_block = 9143481802853542599;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as core::ffi::c_int as isize);
                                if re_parse_group_name(
                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                    ::core::mem::size_of::<[core::ffi::c_char; 128]>()
                                        as core::ffi::c_int,
                                    &mut p1_0,
                                ) != 0
                                {
                                    if (*s).is_unicode != 0 || re_has_named_captures(s) != 0 {
                                        return re_parse_error(
                                            s,
                                            b"invalid group name\0" as *const u8
                                                as *const core::ffi::c_char,
                                        )
                                    } else {
                                        current_block = 9143481802853542599;
                                    }
                                } else {
                                    is_forward = FALSE as core::ffi::c_int as BOOL;
                                    n = find_group_name(
                                        s,
                                        ((*s).u.tmp_buf).as_mut_ptr(),
                                        FALSE as core::ffi::c_int as BOOL,
                                    );
                                    if n == 0 as core::ffi::c_int {
                                        n = re_parse_captures(
                                            s,
                                            &mut dummy_res,
                                            ((*s).u.tmp_buf).as_mut_ptr(),
                                            FALSE as core::ffi::c_int as BOOL,
                                        );
                                        if n == 0 as core::ffi::c_int {
                                            if (*s).is_unicode != 0 || re_has_named_captures(s) != 0 {
                                                return re_parse_error(
                                                    s,
                                                    b"group name not defined\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                )
                                            } else {
                                                current_block = 9143481802853542599;
                                            }
                                        } else {
                                            is_forward = TRUE as core::ffi::c_int as BOOL;
                                            current_block = 1069630499025798221;
                                        }
                                    } else {
                                        current_block = 1069630499025798221;
                                    }
                                    match current_block {
                                        9143481802853542599 => {}
                                        _ => {
                                            last_atom_start = (*s).byte_code.size as core::ffi::c_int;
                                            last_capture_count = (*s).capture_count;
                                            re_emit_op_u8(
                                                s,
                                                REOP_back_reference as core::ffi::c_int
                                                    + 2 as core::ffi::c_int
                                                        * is_backward_dir as core::ffi::c_int
                                                    + (*s).ignore_case as core::ffi::c_int,
                                                n as uint32_t,
                                            );
                                            if is_forward != 0 {
                                                re_parse_captures(
                                                    s,
                                                    &mut dummy_res,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as core::ffi::c_int as BOOL,
                                                );
                                            } else {
                                                find_group_name(
                                                    s,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as core::ffi::c_int as BOOL,
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
                            c = parse_digits(&mut p, FALSE as core::ffi::c_int as BOOL);
                            if c < 0 as core::ffi::c_int
                                || c >= (*s).capture_count && c >= re_count_captures(s)
                            {
                                if (*s).is_unicode == 0 {
                                    p = q;
                                    if *p as core::ffi::c_int <= '7' as i32 {
                                        c = 0 as core::ffi::c_int;
                                        if *p as core::ffi::c_int <= '3' as i32 {
                                            let fresh4 = p;
                                            p = p.offset(1);
                                            c = *fresh4 as core::ffi::c_int - '0' as i32;
                                        }
                                        if *p as core::ffi::c_int >= '0' as i32
                                            && *p as core::ffi::c_int <= '7' as i32
                                        {
                                            let fresh5 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as core::ffi::c_int)
                                                + *fresh5 as core::ffi::c_int - '0' as i32;
                                            if *p as core::ffi::c_int >= '0' as i32
                                                && *p as core::ffi::c_int <= '7' as i32
                                            {
                                                let fresh6 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as core::ffi::c_int)
                                                    + *fresh6 as core::ffi::c_int - '0' as i32;
                                            }
                                        }
                                    } else {
                                        let fresh7 = p;
                                        p = p.offset(1);
                                        c = *fresh7 as core::ffi::c_int;
                                    }
                                } else {
                                    return re_parse_error(
                                        s,
                                        b"back reference out of range in regular expression\0"
                                            as *const u8 as *const core::ffi::c_char,
                                    )
                                }
                                current_block = 4664037234821577316;
                            } else {
                                last_atom_start = (*s).byte_code.size as core::ffi::c_int;
                                last_capture_count = (*s).capture_count;
                                re_emit_op_u8(
                                    s,
                                    REOP_back_reference as core::ffi::c_int
                                        + 2 as core::ffi::c_int
                                            * is_backward_dir as core::ffi::c_int
                                        + (*s).ignore_case as core::ffi::c_int,
                                    1 as uint32_t,
                                );
                                dbuf_putc(&mut (*s).byte_code, c as uint8_t);
                                current_block = 1771738965274008886;
                            }
                        }
                    }
                }
                107 => {
                    current_block = 5846959088466685742;
                    match current_block {
                        8351012430473106477 => {
                            if *p.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int != 'b' as i32
                            {
                                re_emit_op(
                                    s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_not_word_boundary_i as core::ffi::c_int
                                    } else {
                                        REOP_not_word_boundary as core::ffi::c_int
                                    },
                                );
                            } else {
                                re_emit_op(
                                    s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_word_boundary_i as core::ffi::c_int
                                    } else {
                                        REOP_word_boundary as core::ffi::c_int
                                    },
                                );
                            }
                            p = p.offset(2 as core::ffi::c_int as isize);
                            current_block = 1771738965274008886;
                        }
                        3824880322238534506 => {
                            p = p.offset(2 as core::ffi::c_int as isize);
                            c = 0 as core::ffi::c_int;
                            if (*s).is_unicode != 0 {
                                if is_digit(*p as core::ffi::c_int) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"invalid decimal escape in regular expression\0"
                                            as *const u8 as *const core::ffi::c_char,
                                    );
                                }
                            } else if *p as core::ffi::c_int >= '0' as i32
                                && *p as core::ffi::c_int <= '7' as i32
                            {
                                let fresh2 = p;
                                p = p.offset(1);
                                c = *fresh2 as core::ffi::c_int - '0' as i32;
                                if *p as core::ffi::c_int >= '0' as i32
                                    && *p as core::ffi::c_int <= '7' as i32
                                {
                                    let fresh3 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as core::ffi::c_int)
                                        + *fresh3 as core::ffi::c_int - '0' as i32;
                                }
                            }
                            current_block = 4664037234821577316;
                        }
                        5846959088466685742 => {
                            let mut p1_0: *const uint8_t = 0 as *const uint8_t;
                            let mut dummy_res: core::ffi::c_int = 0;
                            let mut n: core::ffi::c_int = 0;
                            let mut is_forward: BOOL = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as core::ffi::c_int as isize)
                                as core::ffi::c_int != '<' as i32
                            {
                                if (*s).is_unicode != 0 || re_has_named_captures(s) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"expecting group name\0" as *const u8
                                            as *const core::ffi::c_char,
                                    )
                                } else {
                                    current_block = 9143481802853542599;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as core::ffi::c_int as isize);
                                if re_parse_group_name(
                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                    ::core::mem::size_of::<[core::ffi::c_char; 128]>()
                                        as core::ffi::c_int,
                                    &mut p1_0,
                                ) != 0
                                {
                                    if (*s).is_unicode != 0 || re_has_named_captures(s) != 0 {
                                        return re_parse_error(
                                            s,
                                            b"invalid group name\0" as *const u8
                                                as *const core::ffi::c_char,
                                        )
                                    } else {
                                        current_block = 9143481802853542599;
                                    }
                                } else {
                                    is_forward = FALSE as core::ffi::c_int as BOOL;
                                    n = find_group_name(
                                        s,
                                        ((*s).u.tmp_buf).as_mut_ptr(),
                                        FALSE as core::ffi::c_int as BOOL,
                                    );
                                    if n == 0 as core::ffi::c_int {
                                        n = re_parse_captures(
                                            s,
                                            &mut dummy_res,
                                            ((*s).u.tmp_buf).as_mut_ptr(),
                                            FALSE as core::ffi::c_int as BOOL,
                                        );
                                        if n == 0 as core::ffi::c_int {
                                            if (*s).is_unicode != 0 || re_has_named_captures(s) != 0 {
                                                return re_parse_error(
                                                    s,
                                                    b"group name not defined\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                )
                                            } else {
                                                current_block = 9143481802853542599;
                                            }
                                        } else {
                                            is_forward = TRUE as core::ffi::c_int as BOOL;
                                            current_block = 1069630499025798221;
                                        }
                                    } else {
                                        current_block = 1069630499025798221;
                                    }
                                    match current_block {
                                        9143481802853542599 => {}
                                        _ => {
                                            last_atom_start = (*s).byte_code.size as core::ffi::c_int;
                                            last_capture_count = (*s).capture_count;
                                            re_emit_op_u8(
                                                s,
                                                REOP_back_reference as core::ffi::c_int
                                                    + 2 as core::ffi::c_int
                                                        * is_backward_dir as core::ffi::c_int
                                                    + (*s).ignore_case as core::ffi::c_int,
                                                n as uint32_t,
                                            );
                                            if is_forward != 0 {
                                                re_parse_captures(
                                                    s,
                                                    &mut dummy_res,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as core::ffi::c_int as BOOL,
                                                );
                                            } else {
                                                find_group_name(
                                                    s,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as core::ffi::c_int as BOOL,
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
                            c = parse_digits(&mut p, FALSE as core::ffi::c_int as BOOL);
                            if c < 0 as core::ffi::c_int
                                || c >= (*s).capture_count && c >= re_count_captures(s)
                            {
                                if (*s).is_unicode == 0 {
                                    p = q;
                                    if *p as core::ffi::c_int <= '7' as i32 {
                                        c = 0 as core::ffi::c_int;
                                        if *p as core::ffi::c_int <= '3' as i32 {
                                            let fresh4 = p;
                                            p = p.offset(1);
                                            c = *fresh4 as core::ffi::c_int - '0' as i32;
                                        }
                                        if *p as core::ffi::c_int >= '0' as i32
                                            && *p as core::ffi::c_int <= '7' as i32
                                        {
                                            let fresh5 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as core::ffi::c_int)
                                                + *fresh5 as core::ffi::c_int - '0' as i32;
                                            if *p as core::ffi::c_int >= '0' as i32
                                                && *p as core::ffi::c_int <= '7' as i32
                                            {
                                                let fresh6 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as core::ffi::c_int)
                                                    + *fresh6 as core::ffi::c_int - '0' as i32;
                                            }
                                        }
                                    } else {
                                        let fresh7 = p;
                                        p = p.offset(1);
                                        c = *fresh7 as core::ffi::c_int;
                                    }
                                } else {
                                    return re_parse_error(
                                        s,
                                        b"back reference out of range in regular expression\0"
                                            as *const u8 as *const core::ffi::c_char,
                                    )
                                }
                                current_block = 4664037234821577316;
                            } else {
                                last_atom_start = (*s).byte_code.size as core::ffi::c_int;
                                last_capture_count = (*s).capture_count;
                                re_emit_op_u8(
                                    s,
                                    REOP_back_reference as core::ffi::c_int
                                        + 2 as core::ffi::c_int
                                            * is_backward_dir as core::ffi::c_int
                                        + (*s).ignore_case as core::ffi::c_int,
                                    1 as uint32_t,
                                );
                                dbuf_putc(&mut (*s).byte_code, c as uint8_t);
                                current_block = 1771738965274008886;
                            }
                        }
                    }
                }
                48 => {
                    current_block = 3824880322238534506;
                    match current_block {
                        8351012430473106477 => {
                            if *p.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int != 'b' as i32
                            {
                                re_emit_op(
                                    s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_not_word_boundary_i as core::ffi::c_int
                                    } else {
                                        REOP_not_word_boundary as core::ffi::c_int
                                    },
                                );
                            } else {
                                re_emit_op(
                                    s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_word_boundary_i as core::ffi::c_int
                                    } else {
                                        REOP_word_boundary as core::ffi::c_int
                                    },
                                );
                            }
                            p = p.offset(2 as core::ffi::c_int as isize);
                            current_block = 1771738965274008886;
                        }
                        3824880322238534506 => {
                            p = p.offset(2 as core::ffi::c_int as isize);
                            c = 0 as core::ffi::c_int;
                            if (*s).is_unicode != 0 {
                                if is_digit(*p as core::ffi::c_int) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"invalid decimal escape in regular expression\0"
                                            as *const u8 as *const core::ffi::c_char,
                                    );
                                }
                            } else if *p as core::ffi::c_int >= '0' as i32
                                && *p as core::ffi::c_int <= '7' as i32
                            {
                                let fresh2 = p;
                                p = p.offset(1);
                                c = *fresh2 as core::ffi::c_int - '0' as i32;
                                if *p as core::ffi::c_int >= '0' as i32
                                    && *p as core::ffi::c_int <= '7' as i32
                                {
                                    let fresh3 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as core::ffi::c_int)
                                        + *fresh3 as core::ffi::c_int - '0' as i32;
                                }
                            }
                            current_block = 4664037234821577316;
                        }
                        5846959088466685742 => {
                            let mut p1_0: *const uint8_t = 0 as *const uint8_t;
                            let mut dummy_res: core::ffi::c_int = 0;
                            let mut n: core::ffi::c_int = 0;
                            let mut is_forward: BOOL = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as core::ffi::c_int as isize)
                                as core::ffi::c_int != '<' as i32
                            {
                                if (*s).is_unicode != 0 || re_has_named_captures(s) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"expecting group name\0" as *const u8
                                            as *const core::ffi::c_char,
                                    )
                                } else {
                                    current_block = 9143481802853542599;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as core::ffi::c_int as isize);
                                if re_parse_group_name(
                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                    ::core::mem::size_of::<[core::ffi::c_char; 128]>()
                                        as core::ffi::c_int,
                                    &mut p1_0,
                                ) != 0
                                {
                                    if (*s).is_unicode != 0 || re_has_named_captures(s) != 0 {
                                        return re_parse_error(
                                            s,
                                            b"invalid group name\0" as *const u8
                                                as *const core::ffi::c_char,
                                        )
                                    } else {
                                        current_block = 9143481802853542599;
                                    }
                                } else {
                                    is_forward = FALSE as core::ffi::c_int as BOOL;
                                    n = find_group_name(
                                        s,
                                        ((*s).u.tmp_buf).as_mut_ptr(),
                                        FALSE as core::ffi::c_int as BOOL,
                                    );
                                    if n == 0 as core::ffi::c_int {
                                        n = re_parse_captures(
                                            s,
                                            &mut dummy_res,
                                            ((*s).u.tmp_buf).as_mut_ptr(),
                                            FALSE as core::ffi::c_int as BOOL,
                                        );
                                        if n == 0 as core::ffi::c_int {
                                            if (*s).is_unicode != 0 || re_has_named_captures(s) != 0 {
                                                return re_parse_error(
                                                    s,
                                                    b"group name not defined\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                )
                                            } else {
                                                current_block = 9143481802853542599;
                                            }
                                        } else {
                                            is_forward = TRUE as core::ffi::c_int as BOOL;
                                            current_block = 1069630499025798221;
                                        }
                                    } else {
                                        current_block = 1069630499025798221;
                                    }
                                    match current_block {
                                        9143481802853542599 => {}
                                        _ => {
                                            last_atom_start = (*s).byte_code.size as core::ffi::c_int;
                                            last_capture_count = (*s).capture_count;
                                            re_emit_op_u8(
                                                s,
                                                REOP_back_reference as core::ffi::c_int
                                                    + 2 as core::ffi::c_int
                                                        * is_backward_dir as core::ffi::c_int
                                                    + (*s).ignore_case as core::ffi::c_int,
                                                n as uint32_t,
                                            );
                                            if is_forward != 0 {
                                                re_parse_captures(
                                                    s,
                                                    &mut dummy_res,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as core::ffi::c_int as BOOL,
                                                );
                                            } else {
                                                find_group_name(
                                                    s,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as core::ffi::c_int as BOOL,
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
                            c = parse_digits(&mut p, FALSE as core::ffi::c_int as BOOL);
                            if c < 0 as core::ffi::c_int
                                || c >= (*s).capture_count && c >= re_count_captures(s)
                            {
                                if (*s).is_unicode == 0 {
                                    p = q;
                                    if *p as core::ffi::c_int <= '7' as i32 {
                                        c = 0 as core::ffi::c_int;
                                        if *p as core::ffi::c_int <= '3' as i32 {
                                            let fresh4 = p;
                                            p = p.offset(1);
                                            c = *fresh4 as core::ffi::c_int - '0' as i32;
                                        }
                                        if *p as core::ffi::c_int >= '0' as i32
                                            && *p as core::ffi::c_int <= '7' as i32
                                        {
                                            let fresh5 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as core::ffi::c_int)
                                                + *fresh5 as core::ffi::c_int - '0' as i32;
                                            if *p as core::ffi::c_int >= '0' as i32
                                                && *p as core::ffi::c_int <= '7' as i32
                                            {
                                                let fresh6 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as core::ffi::c_int)
                                                    + *fresh6 as core::ffi::c_int - '0' as i32;
                                            }
                                        }
                                    } else {
                                        let fresh7 = p;
                                        p = p.offset(1);
                                        c = *fresh7 as core::ffi::c_int;
                                    }
                                } else {
                                    return re_parse_error(
                                        s,
                                        b"back reference out of range in regular expression\0"
                                            as *const u8 as *const core::ffi::c_char,
                                    )
                                }
                                current_block = 4664037234821577316;
                            } else {
                                last_atom_start = (*s).byte_code.size as core::ffi::c_int;
                                last_capture_count = (*s).capture_count;
                                re_emit_op_u8(
                                    s,
                                    REOP_back_reference as core::ffi::c_int
                                        + 2 as core::ffi::c_int
                                            * is_backward_dir as core::ffi::c_int
                                        + (*s).ignore_case as core::ffi::c_int,
                                    1 as uint32_t,
                                );
                                dbuf_putc(&mut (*s).byte_code, c as uint8_t);
                                current_block = 1771738965274008886;
                            }
                        }
                    }
                }
                49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    current_block = 7315983924538012637;
                    match current_block {
                        8351012430473106477 => {
                            if *p.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int != 'b' as i32
                            {
                                re_emit_op(
                                    s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_not_word_boundary_i as core::ffi::c_int
                                    } else {
                                        REOP_not_word_boundary as core::ffi::c_int
                                    },
                                );
                            } else {
                                re_emit_op(
                                    s,
                                    if (*s).ignore_case != 0 && (*s).is_unicode != 0 {
                                        REOP_word_boundary_i as core::ffi::c_int
                                    } else {
                                        REOP_word_boundary as core::ffi::c_int
                                    },
                                );
                            }
                            p = p.offset(2 as core::ffi::c_int as isize);
                            current_block = 1771738965274008886;
                        }
                        3824880322238534506 => {
                            p = p.offset(2 as core::ffi::c_int as isize);
                            c = 0 as core::ffi::c_int;
                            if (*s).is_unicode != 0 {
                                if is_digit(*p as core::ffi::c_int) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"invalid decimal escape in regular expression\0"
                                            as *const u8 as *const core::ffi::c_char,
                                    );
                                }
                            } else if *p as core::ffi::c_int >= '0' as i32
                                && *p as core::ffi::c_int <= '7' as i32
                            {
                                let fresh2 = p;
                                p = p.offset(1);
                                c = *fresh2 as core::ffi::c_int - '0' as i32;
                                if *p as core::ffi::c_int >= '0' as i32
                                    && *p as core::ffi::c_int <= '7' as i32
                                {
                                    let fresh3 = p;
                                    p = p.offset(1);
                                    c = (c << 3 as core::ffi::c_int)
                                        + *fresh3 as core::ffi::c_int - '0' as i32;
                                }
                            }
                            current_block = 4664037234821577316;
                        }
                        5846959088466685742 => {
                            let mut p1_0: *const uint8_t = 0 as *const uint8_t;
                            let mut dummy_res: core::ffi::c_int = 0;
                            let mut n: core::ffi::c_int = 0;
                            let mut is_forward: BOOL = 0;
                            p1_0 = p;
                            if *p1_0.offset(2 as core::ffi::c_int as isize)
                                as core::ffi::c_int != '<' as i32
                            {
                                if (*s).is_unicode != 0 || re_has_named_captures(s) != 0 {
                                    return re_parse_error(
                                        s,
                                        b"expecting group name\0" as *const u8
                                            as *const core::ffi::c_char,
                                    )
                                } else {
                                    current_block = 9143481802853542599;
                                }
                            } else {
                                p1_0 = p1_0.offset(3 as core::ffi::c_int as isize);
                                if re_parse_group_name(
                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                    ::core::mem::size_of::<[core::ffi::c_char; 128]>()
                                        as core::ffi::c_int,
                                    &mut p1_0,
                                ) != 0
                                {
                                    if (*s).is_unicode != 0 || re_has_named_captures(s) != 0 {
                                        return re_parse_error(
                                            s,
                                            b"invalid group name\0" as *const u8
                                                as *const core::ffi::c_char,
                                        )
                                    } else {
                                        current_block = 9143481802853542599;
                                    }
                                } else {
                                    is_forward = FALSE as core::ffi::c_int as BOOL;
                                    n = find_group_name(
                                        s,
                                        ((*s).u.tmp_buf).as_mut_ptr(),
                                        FALSE as core::ffi::c_int as BOOL,
                                    );
                                    if n == 0 as core::ffi::c_int {
                                        n = re_parse_captures(
                                            s,
                                            &mut dummy_res,
                                            ((*s).u.tmp_buf).as_mut_ptr(),
                                            FALSE as core::ffi::c_int as BOOL,
                                        );
                                        if n == 0 as core::ffi::c_int {
                                            if (*s).is_unicode != 0 || re_has_named_captures(s) != 0 {
                                                return re_parse_error(
                                                    s,
                                                    b"group name not defined\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                )
                                            } else {
                                                current_block = 9143481802853542599;
                                            }
                                        } else {
                                            is_forward = TRUE as core::ffi::c_int as BOOL;
                                            current_block = 1069630499025798221;
                                        }
                                    } else {
                                        current_block = 1069630499025798221;
                                    }
                                    match current_block {
                                        9143481802853542599 => {}
                                        _ => {
                                            last_atom_start = (*s).byte_code.size as core::ffi::c_int;
                                            last_capture_count = (*s).capture_count;
                                            re_emit_op_u8(
                                                s,
                                                REOP_back_reference as core::ffi::c_int
                                                    + 2 as core::ffi::c_int
                                                        * is_backward_dir as core::ffi::c_int
                                                    + (*s).ignore_case as core::ffi::c_int,
                                                n as uint32_t,
                                            );
                                            if is_forward != 0 {
                                                re_parse_captures(
                                                    s,
                                                    &mut dummy_res,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as core::ffi::c_int as BOOL,
                                                );
                                            } else {
                                                find_group_name(
                                                    s,
                                                    ((*s).u.tmp_buf).as_mut_ptr(),
                                                    TRUE as core::ffi::c_int as BOOL,
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
                            c = parse_digits(&mut p, FALSE as core::ffi::c_int as BOOL);
                            if c < 0 as core::ffi::c_int
                                || c >= (*s).capture_count && c >= re_count_captures(s)
                            {
                                if (*s).is_unicode == 0 {
                                    p = q;
                                    if *p as core::ffi::c_int <= '7' as i32 {
                                        c = 0 as core::ffi::c_int;
                                        if *p as core::ffi::c_int <= '3' as i32 {
                                            let fresh4 = p;
                                            p = p.offset(1);
                                            c = *fresh4 as core::ffi::c_int - '0' as i32;
                                        }
                                        if *p as core::ffi::c_int >= '0' as i32
                                            && *p as core::ffi::c_int <= '7' as i32
                                        {
                                            let fresh5 = p;
                                            p = p.offset(1);
                                            c = (c << 3 as core::ffi::c_int)
                                                + *fresh5 as core::ffi::c_int - '0' as i32;
                                            if *p as core::ffi::c_int >= '0' as i32
                                                && *p as core::ffi::c_int <= '7' as i32
                                            {
                                                let fresh6 = p;
                                                p = p.offset(1);
                                                c = (c << 3 as core::ffi::c_int)
                                                    + *fresh6 as core::ffi::c_int - '0' as i32;
                                            }
                                        }
                                    } else {
                                        let fresh7 = p;
                                        p = p.offset(1);
                                        c = *fresh7 as core::ffi::c_int;
                                    }
                                } else {
                                    return re_parse_error(
                                        s,
                                        b"back reference out of range in regular expression\0"
                                            as *const u8 as *const core::ffi::c_char,
                                    )
                                }
                                current_block = 4664037234821577316;
                            } else {
                                last_atom_start = (*s).byte_code.size as core::ffi::c_int;
                                last_capture_count = (*s).capture_count;
                                re_emit_op_u8(
                                    s,
                                    REOP_back_reference as core::ffi::c_int
                                        + 2 as core::ffi::c_int
                                            * is_backward_dir as core::ffi::c_int
                                        + (*s).ignore_case as core::ffi::c_int,
                                    1 as uint32_t,
                                );
                                dbuf_putc(&mut (*s).byte_code, c as uint8_t);
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
            last_atom_start = (*s).byte_code.size as core::ffi::c_int;
            last_capture_count = (*s).capture_count;
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as core::ffi::c_int);
            }
            if re_parse_char_class(s, &mut p) != 0 {
                return -(1 as core::ffi::c_int);
            }
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as core::ffi::c_int);
            }
            current_block = 1771738965274008886;
        }
        93 | 125 => {
            if (*s).is_unicode != 0 {
                return re_parse_error(
                    s,
                    b"syntax error\0" as *const u8 as *const core::ffi::c_char,
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
            c = get_class_atom(s, cr, &mut p, FALSE as core::ffi::c_int as BOOL);
            if c < 0 as core::ffi::c_int {
                return -(1 as core::ffi::c_int);
            }
            current_block = 4664037234821577316;
        }
        18128146392678525522 => {
            return re_parse_error(
                s,
                b"nothing to repeat\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        _ => {}
    }
    match current_block {
        4664037234821577316 => {
            last_atom_start = (*s).byte_code.size as core::ffi::c_int;
            last_capture_count = (*s).capture_count;
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as core::ffi::c_int);
            }
            if c >= CLASS_RANGE_BASE {
                let mut ret: core::ffi::c_int = 0 as core::ffi::c_int;
                if c == CLASS_RANGE_BASE + CHAR_RANGE_s as core::ffi::c_int {
                    re_emit_op(s, REOP_space as core::ffi::c_int);
                } else if c == CLASS_RANGE_BASE + CHAR_RANGE_S as core::ffi::c_int {
                    re_emit_op(s, REOP_not_space as core::ffi::c_int);
                } else {
                    ret = re_emit_string_list(s, cr);
                }
                re_string_list_free(cr);
                if ret != 0 {
                    return -(1 as core::ffi::c_int);
                }
            } else {
                if (*s).ignore_case != 0 {
                    c = lre_canonicalize(
                        c as uint32_t,
                        (*s).is_unicode as core::ffi::c_int,
                    );
                }
                re_emit_char(s, c);
            }
            if is_backward_dir != 0 {
                re_emit_op(s, REOP_prev as core::ffi::c_int);
            }
        }
        _ => {}
    }
    if last_atom_start >= 0 as core::ffi::c_int {
        c = *p as core::ffi::c_int;
        match c {
            42 => {
                current_block = 7069606996288041156;
                match current_block {
                    7069606996288041156 => {
                        p = p.offset(1);
                        quant_min = 0 as core::ffi::c_int;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    18255723851606772592 => {
                        p = p.offset(1);
                        quant_min = 0 as core::ffi::c_int;
                        quant_max = 1 as core::ffi::c_int;
                        current_block = 9506328432318339935;
                    }
                    11105283338979551329 => {
                        p = p.offset(1);
                        quant_min = 1 as core::ffi::c_int;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    _ => {
                        let mut p1_1: *const uint8_t = p;
                        if is_digit(
                            *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int,
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
                                TRUE as core::ffi::c_int as BOOL,
                            );
                            quant_max = quant_min;
                            if *p as core::ffi::c_int == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as core::ffi::c_int) != 0 {
                                    quant_max = parse_digits(
                                        &mut p,
                                        TRUE as core::ffi::c_int as BOOL,
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
                                    if *p as core::ffi::c_int != '}' as i32
                                        && (*s).is_unicode == 0
                                    {
                                        p = p1_1;
                                        current_block = 11353886201549099807;
                                    } else {
                                        if re_parse_expect(s, &mut p, '}' as i32) != 0 {
                                            return -(1 as core::ffi::c_int);
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
                                    s,
                                    b"invalid repetition count\0" as *const u8
                                        as *const core::ffi::c_char,
                                );
                            }
                        }
                    }
                }
                match current_block {
                    11353886201549099807 => {}
                    _ => {
                        greedy = TRUE as core::ffi::c_int as BOOL;
                        if *p as core::ffi::c_int == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as core::ffi::c_int as BOOL;
                        }
                        if last_atom_start < 0 as core::ffi::c_int {
                            return re_parse_error(
                                s,
                                b"nothing to repeat\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                        let mut need_capture_init: BOOL = 0;
                        let mut add_zero_advance_check: BOOL = 0;
                        let mut len: core::ffi::c_int = 0;
                        let mut pos_0: core::ffi::c_int = 0;
                        add_zero_advance_check = re_need_check_adv_and_capture_init(
                            &mut need_capture_init,
                            ((*s).byte_code.buf).offset(last_atom_start as isize),
                            ((*s).byte_code.size).wrapping_sub(last_atom_start as size_t)
                                as core::ffi::c_int,
                        );
                        if need_capture_init != 0
                            && last_capture_count != (*s).capture_count
                        {
                            if dbuf_insert(
                                &mut (*s).byte_code,
                                last_atom_start,
                                3 as core::ffi::c_int,
                            ) != 0
                            {
                                current_block = 17551395354807108434;
                            } else {
                                let mut pos_1: core::ffi::c_int = last_atom_start;
                                let fresh8 = pos_1;
                                pos_1 = pos_1 + 1;
                                *((*s).byte_code.buf).offset(fresh8 as isize) = REOP_save_reset
                                    as core::ffi::c_int as uint8_t;
                                let fresh9 = pos_1;
                                pos_1 = pos_1 + 1;
                                *((*s).byte_code.buf).offset(fresh9 as isize) = last_capture_count
                                    as uint8_t;
                                let fresh10 = pos_1;
                                pos_1 = pos_1 + 1;
                                *((*s).byte_code.buf).offset(fresh10 as isize) = ((*s)
                                    .capture_count - 1 as core::ffi::c_int) as uint8_t;
                                current_block = 1851490986684842406;
                            }
                        } else {
                            current_block = 1851490986684842406;
                        }
                        match current_block {
                            1851490986684842406 => {
                                len = ((*s).byte_code.size)
                                    .wrapping_sub(last_atom_start as size_t)
                                    as core::ffi::c_int;
                                if quant_min == 0 as core::ffi::c_int {
                                    if need_capture_init == 0
                                        && last_capture_count != (*s).capture_count
                                    {
                                        if dbuf_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            3 as core::ffi::c_int,
                                        ) != 0
                                        {
                                            current_block = 17551395354807108434;
                                        } else {
                                            let fresh11 = last_atom_start;
                                            last_atom_start = last_atom_start + 1;
                                            *((*s).byte_code.buf).offset(fresh11 as isize) = REOP_save_reset
                                                as core::ffi::c_int as uint8_t;
                                            let fresh12 = last_atom_start;
                                            last_atom_start = last_atom_start + 1;
                                            *((*s).byte_code.buf).offset(fresh12 as isize) = last_capture_count
                                                as uint8_t;
                                            let fresh13 = last_atom_start;
                                            last_atom_start = last_atom_start + 1;
                                            *((*s).byte_code.buf).offset(fresh13 as isize) = ((*s)
                                                .capture_count - 1 as core::ffi::c_int) as uint8_t;
                                            current_block = 8700473759921513224;
                                        }
                                    } else {
                                        current_block = 8700473759921513224;
                                    }
                                    match current_block {
                                        17551395354807108434 => {}
                                        _ => {
                                            if quant_max == 0 as core::ffi::c_int {
                                                (*s).byte_code.size = last_atom_start as size_t;
                                                current_block = 2588063579017527985;
                                            } else if quant_max == 1 as core::ffi::c_int
                                                || quant_max == INT32_MAX
                                            {
                                                let mut has_goto: BOOL = (quant_max == INT32_MAX)
                                                    as core::ffi::c_int;
                                                if dbuf_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    5 as core::ffi::c_int
                                                        + add_zero_advance_check as core::ffi::c_int
                                                            * 2 as core::ffi::c_int,
                                                ) != 0
                                                {
                                                    current_block = 17551395354807108434;
                                                } else {
                                                    *((*s).byte_code.buf).offset(last_atom_start as isize) = (REOP_split_goto_first
                                                        as core::ffi::c_int as BOOL + greedy) as uint8_t;
                                                    put_u32(
                                                        ((*s).byte_code.buf)
                                                            .offset(last_atom_start as isize)
                                                            .offset(1 as core::ffi::c_int as isize),
                                                        (len + 5 as core::ffi::c_int * has_goto as core::ffi::c_int
                                                            + add_zero_advance_check as core::ffi::c_int
                                                                * 2 as core::ffi::c_int * 2 as core::ffi::c_int) as uint32_t,
                                                    );
                                                    if add_zero_advance_check != 0 {
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                (last_atom_start + 1 as core::ffi::c_int
                                                                    + 4 as core::ffi::c_int) as isize,
                                                            ) = REOP_set_char_pos as core::ffi::c_int as uint8_t;
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                (last_atom_start + 1 as core::ffi::c_int
                                                                    + 4 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                                                            ) = 0 as uint8_t;
                                                        re_emit_op_u8(
                                                            s,
                                                            REOP_check_advance as core::ffi::c_int,
                                                            0 as uint32_t,
                                                        );
                                                    }
                                                    if has_goto != 0 {
                                                        re_emit_goto(
                                                            s,
                                                            REOP_goto as core::ffi::c_int,
                                                            last_atom_start as uint32_t,
                                                        );
                                                    }
                                                    current_block = 2588063579017527985;
                                                }
                                            } else if dbuf_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                11 as core::ffi::c_int
                                                    + add_zero_advance_check as core::ffi::c_int
                                                        * 2 as core::ffi::c_int,
                                            ) != 0
                                            {
                                                current_block = 17551395354807108434;
                                            } else {
                                                pos_0 = last_atom_start;
                                                let fresh14 = pos_0;
                                                pos_0 = pos_0 + 1;
                                                *((*s).byte_code.buf).offset(fresh14 as isize) = (REOP_split_goto_first
                                                    as core::ffi::c_int as BOOL + greedy) as uint8_t;
                                                put_u32(
                                                    ((*s).byte_code.buf).offset(pos_0 as isize),
                                                    (6 as core::ffi::c_int
                                                        + add_zero_advance_check as core::ffi::c_int
                                                            * 2 as core::ffi::c_int + len + 10 as core::ffi::c_int)
                                                        as uint32_t,
                                                );
                                                pos_0 += 4 as core::ffi::c_int;
                                                let fresh15 = pos_0;
                                                pos_0 = pos_0 + 1;
                                                *((*s).byte_code.buf).offset(fresh15 as isize) = REOP_set_i32
                                                    as core::ffi::c_int as uint8_t;
                                                let fresh16 = pos_0;
                                                pos_0 = pos_0 + 1;
                                                *((*s).byte_code.buf).offset(fresh16 as isize) = 0
                                                    as uint8_t;
                                                put_u32(
                                                    ((*s).byte_code.buf).offset(pos_0 as isize),
                                                    quant_max as uint32_t,
                                                );
                                                pos_0 += 4 as core::ffi::c_int;
                                                last_atom_start = pos_0;
                                                if add_zero_advance_check != 0 {
                                                    let fresh17 = pos_0;
                                                    pos_0 = pos_0 + 1;
                                                    *((*s).byte_code.buf).offset(fresh17 as isize) = REOP_set_char_pos
                                                        as core::ffi::c_int as uint8_t;
                                                    let fresh18 = pos_0;
                                                    pos_0 = pos_0 + 1;
                                                    *((*s).byte_code.buf).offset(fresh18 as isize) = 0
                                                        as uint8_t;
                                                }
                                                re_emit_goto_u8_u32(
                                                    s,
                                                    (if add_zero_advance_check != 0 {
                                                        REOP_loop_check_adv_split_next_first as core::ffi::c_int
                                                    } else {
                                                        REOP_loop_split_next_first as core::ffi::c_int
                                                    }) - greedy as core::ffi::c_int,
                                                    0 as uint32_t,
                                                    quant_max as uint32_t,
                                                    last_atom_start as uint32_t,
                                                );
                                                current_block = 2588063579017527985;
                                            }
                                        }
                                    }
                                } else if quant_min == 1 as core::ffi::c_int
                                    && quant_max == INT32_MAX && add_zero_advance_check == 0
                                {
                                    re_emit_goto(
                                        s,
                                        REOP_split_next_first as core::ffi::c_int
                                            - greedy as core::ffi::c_int,
                                        last_atom_start as uint32_t,
                                    );
                                    current_block = 2588063579017527985;
                                } else {
                                    if quant_min == quant_max {
                                        add_zero_advance_check = FALSE as core::ffi::c_int as BOOL;
                                    }
                                    if dbuf_insert(
                                        &mut (*s).byte_code,
                                        last_atom_start,
                                        6 as core::ffi::c_int
                                            + add_zero_advance_check as core::ffi::c_int
                                                * 2 as core::ffi::c_int,
                                    ) != 0
                                    {
                                        current_block = 17551395354807108434;
                                    } else {
                                        pos_0 = last_atom_start;
                                        let fresh19 = pos_0;
                                        pos_0 = pos_0 + 1;
                                        *((*s).byte_code.buf).offset(fresh19 as isize) = REOP_set_i32
                                            as core::ffi::c_int as uint8_t;
                                        let fresh20 = pos_0;
                                        pos_0 = pos_0 + 1;
                                        *((*s).byte_code.buf).offset(fresh20 as isize) = 0
                                            as uint8_t;
                                        put_u32(
                                            ((*s).byte_code.buf).offset(pos_0 as isize),
                                            quant_max as uint32_t,
                                        );
                                        pos_0 += 4 as core::ffi::c_int;
                                        last_atom_start = pos_0;
                                        if add_zero_advance_check != 0 {
                                            let fresh21 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *((*s).byte_code.buf).offset(fresh21 as isize) = REOP_set_char_pos
                                                as core::ffi::c_int as uint8_t;
                                            let fresh22 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *((*s).byte_code.buf).offset(fresh22 as isize) = 0
                                                as uint8_t;
                                        }
                                        if quant_min == quant_max {
                                            re_emit_goto_u8(
                                                s,
                                                REOP_loop as core::ffi::c_int,
                                                0 as uint32_t,
                                                last_atom_start as uint32_t,
                                            );
                                        } else {
                                            re_emit_goto_u8_u32(
                                                s,
                                                (if add_zero_advance_check != 0 {
                                                    REOP_loop_check_adv_split_next_first as core::ffi::c_int
                                                } else {
                                                    REOP_loop_split_next_first as core::ffi::c_int
                                                }) - greedy as core::ffi::c_int,
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
                                        last_atom_start = -(1 as core::ffi::c_int);
                                        current_block = 11353886201549099807;
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11353886201549099807 => {}
                            _ => return re_parse_out_of_memory(s),
                        }
                    }
                }
            }
            43 => {
                current_block = 11105283338979551329;
                match current_block {
                    7069606996288041156 => {
                        p = p.offset(1);
                        quant_min = 0 as core::ffi::c_int;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    18255723851606772592 => {
                        p = p.offset(1);
                        quant_min = 0 as core::ffi::c_int;
                        quant_max = 1 as core::ffi::c_int;
                        current_block = 9506328432318339935;
                    }
                    11105283338979551329 => {
                        p = p.offset(1);
                        quant_min = 1 as core::ffi::c_int;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    _ => {
                        let mut p1_1: *const uint8_t = p;
                        if is_digit(
                            *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int,
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
                                TRUE as core::ffi::c_int as BOOL,
                            );
                            quant_max = quant_min;
                            if *p as core::ffi::c_int == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as core::ffi::c_int) != 0 {
                                    quant_max = parse_digits(
                                        &mut p,
                                        TRUE as core::ffi::c_int as BOOL,
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
                                    if *p as core::ffi::c_int != '}' as i32
                                        && (*s).is_unicode == 0
                                    {
                                        p = p1_1;
                                        current_block = 11353886201549099807;
                                    } else {
                                        if re_parse_expect(s, &mut p, '}' as i32) != 0 {
                                            return -(1 as core::ffi::c_int);
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
                                    s,
                                    b"invalid repetition count\0" as *const u8
                                        as *const core::ffi::c_char,
                                );
                            }
                        }
                    }
                }
                match current_block {
                    11353886201549099807 => {}
                    _ => {
                        greedy = TRUE as core::ffi::c_int as BOOL;
                        if *p as core::ffi::c_int == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as core::ffi::c_int as BOOL;
                        }
                        if last_atom_start < 0 as core::ffi::c_int {
                            return re_parse_error(
                                s,
                                b"nothing to repeat\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                        let mut need_capture_init: BOOL = 0;
                        let mut add_zero_advance_check: BOOL = 0;
                        let mut len: core::ffi::c_int = 0;
                        let mut pos_0: core::ffi::c_int = 0;
                        add_zero_advance_check = re_need_check_adv_and_capture_init(
                            &mut need_capture_init,
                            ((*s).byte_code.buf).offset(last_atom_start as isize),
                            ((*s).byte_code.size).wrapping_sub(last_atom_start as size_t)
                                as core::ffi::c_int,
                        );
                        if need_capture_init != 0
                            && last_capture_count != (*s).capture_count
                        {
                            if dbuf_insert(
                                &mut (*s).byte_code,
                                last_atom_start,
                                3 as core::ffi::c_int,
                            ) != 0
                            {
                                current_block = 17551395354807108434;
                            } else {
                                let mut pos_1: core::ffi::c_int = last_atom_start;
                                let fresh8 = pos_1;
                                pos_1 = pos_1 + 1;
                                *((*s).byte_code.buf).offset(fresh8 as isize) = REOP_save_reset
                                    as core::ffi::c_int as uint8_t;
                                let fresh9 = pos_1;
                                pos_1 = pos_1 + 1;
                                *((*s).byte_code.buf).offset(fresh9 as isize) = last_capture_count
                                    as uint8_t;
                                let fresh10 = pos_1;
                                pos_1 = pos_1 + 1;
                                *((*s).byte_code.buf).offset(fresh10 as isize) = ((*s)
                                    .capture_count - 1 as core::ffi::c_int) as uint8_t;
                                current_block = 1851490986684842406;
                            }
                        } else {
                            current_block = 1851490986684842406;
                        }
                        match current_block {
                            1851490986684842406 => {
                                len = ((*s).byte_code.size)
                                    .wrapping_sub(last_atom_start as size_t)
                                    as core::ffi::c_int;
                                if quant_min == 0 as core::ffi::c_int {
                                    if need_capture_init == 0
                                        && last_capture_count != (*s).capture_count
                                    {
                                        if dbuf_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            3 as core::ffi::c_int,
                                        ) != 0
                                        {
                                            current_block = 17551395354807108434;
                                        } else {
                                            let fresh11 = last_atom_start;
                                            last_atom_start = last_atom_start + 1;
                                            *((*s).byte_code.buf).offset(fresh11 as isize) = REOP_save_reset
                                                as core::ffi::c_int as uint8_t;
                                            let fresh12 = last_atom_start;
                                            last_atom_start = last_atom_start + 1;
                                            *((*s).byte_code.buf).offset(fresh12 as isize) = last_capture_count
                                                as uint8_t;
                                            let fresh13 = last_atom_start;
                                            last_atom_start = last_atom_start + 1;
                                            *((*s).byte_code.buf).offset(fresh13 as isize) = ((*s)
                                                .capture_count - 1 as core::ffi::c_int) as uint8_t;
                                            current_block = 8700473759921513224;
                                        }
                                    } else {
                                        current_block = 8700473759921513224;
                                    }
                                    match current_block {
                                        17551395354807108434 => {}
                                        _ => {
                                            if quant_max == 0 as core::ffi::c_int {
                                                (*s).byte_code.size = last_atom_start as size_t;
                                                current_block = 2588063579017527985;
                                            } else if quant_max == 1 as core::ffi::c_int
                                                || quant_max == INT32_MAX
                                            {
                                                let mut has_goto: BOOL = (quant_max == INT32_MAX)
                                                    as core::ffi::c_int;
                                                if dbuf_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    5 as core::ffi::c_int
                                                        + add_zero_advance_check as core::ffi::c_int
                                                            * 2 as core::ffi::c_int,
                                                ) != 0
                                                {
                                                    current_block = 17551395354807108434;
                                                } else {
                                                    *((*s).byte_code.buf).offset(last_atom_start as isize) = (REOP_split_goto_first
                                                        as core::ffi::c_int as BOOL + greedy) as uint8_t;
                                                    put_u32(
                                                        ((*s).byte_code.buf)
                                                            .offset(last_atom_start as isize)
                                                            .offset(1 as core::ffi::c_int as isize),
                                                        (len + 5 as core::ffi::c_int * has_goto as core::ffi::c_int
                                                            + add_zero_advance_check as core::ffi::c_int
                                                                * 2 as core::ffi::c_int * 2 as core::ffi::c_int) as uint32_t,
                                                    );
                                                    if add_zero_advance_check != 0 {
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                (last_atom_start + 1 as core::ffi::c_int
                                                                    + 4 as core::ffi::c_int) as isize,
                                                            ) = REOP_set_char_pos as core::ffi::c_int as uint8_t;
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                (last_atom_start + 1 as core::ffi::c_int
                                                                    + 4 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                                                            ) = 0 as uint8_t;
                                                        re_emit_op_u8(
                                                            s,
                                                            REOP_check_advance as core::ffi::c_int,
                                                            0 as uint32_t,
                                                        );
                                                    }
                                                    if has_goto != 0 {
                                                        re_emit_goto(
                                                            s,
                                                            REOP_goto as core::ffi::c_int,
                                                            last_atom_start as uint32_t,
                                                        );
                                                    }
                                                    current_block = 2588063579017527985;
                                                }
                                            } else if dbuf_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                11 as core::ffi::c_int
                                                    + add_zero_advance_check as core::ffi::c_int
                                                        * 2 as core::ffi::c_int,
                                            ) != 0
                                            {
                                                current_block = 17551395354807108434;
                                            } else {
                                                pos_0 = last_atom_start;
                                                let fresh14 = pos_0;
                                                pos_0 = pos_0 + 1;
                                                *((*s).byte_code.buf).offset(fresh14 as isize) = (REOP_split_goto_first
                                                    as core::ffi::c_int as BOOL + greedy) as uint8_t;
                                                put_u32(
                                                    ((*s).byte_code.buf).offset(pos_0 as isize),
                                                    (6 as core::ffi::c_int
                                                        + add_zero_advance_check as core::ffi::c_int
                                                            * 2 as core::ffi::c_int + len + 10 as core::ffi::c_int)
                                                        as uint32_t,
                                                );
                                                pos_0 += 4 as core::ffi::c_int;
                                                let fresh15 = pos_0;
                                                pos_0 = pos_0 + 1;
                                                *((*s).byte_code.buf).offset(fresh15 as isize) = REOP_set_i32
                                                    as core::ffi::c_int as uint8_t;
                                                let fresh16 = pos_0;
                                                pos_0 = pos_0 + 1;
                                                *((*s).byte_code.buf).offset(fresh16 as isize) = 0
                                                    as uint8_t;
                                                put_u32(
                                                    ((*s).byte_code.buf).offset(pos_0 as isize),
                                                    quant_max as uint32_t,
                                                );
                                                pos_0 += 4 as core::ffi::c_int;
                                                last_atom_start = pos_0;
                                                if add_zero_advance_check != 0 {
                                                    let fresh17 = pos_0;
                                                    pos_0 = pos_0 + 1;
                                                    *((*s).byte_code.buf).offset(fresh17 as isize) = REOP_set_char_pos
                                                        as core::ffi::c_int as uint8_t;
                                                    let fresh18 = pos_0;
                                                    pos_0 = pos_0 + 1;
                                                    *((*s).byte_code.buf).offset(fresh18 as isize) = 0
                                                        as uint8_t;
                                                }
                                                re_emit_goto_u8_u32(
                                                    s,
                                                    (if add_zero_advance_check != 0 {
                                                        REOP_loop_check_adv_split_next_first as core::ffi::c_int
                                                    } else {
                                                        REOP_loop_split_next_first as core::ffi::c_int
                                                    }) - greedy as core::ffi::c_int,
                                                    0 as uint32_t,
                                                    quant_max as uint32_t,
                                                    last_atom_start as uint32_t,
                                                );
                                                current_block = 2588063579017527985;
                                            }
                                        }
                                    }
                                } else if quant_min == 1 as core::ffi::c_int
                                    && quant_max == INT32_MAX && add_zero_advance_check == 0
                                {
                                    re_emit_goto(
                                        s,
                                        REOP_split_next_first as core::ffi::c_int
                                            - greedy as core::ffi::c_int,
                                        last_atom_start as uint32_t,
                                    );
                                    current_block = 2588063579017527985;
                                } else {
                                    if quant_min == quant_max {
                                        add_zero_advance_check = FALSE as core::ffi::c_int as BOOL;
                                    }
                                    if dbuf_insert(
                                        &mut (*s).byte_code,
                                        last_atom_start,
                                        6 as core::ffi::c_int
                                            + add_zero_advance_check as core::ffi::c_int
                                                * 2 as core::ffi::c_int,
                                    ) != 0
                                    {
                                        current_block = 17551395354807108434;
                                    } else {
                                        pos_0 = last_atom_start;
                                        let fresh19 = pos_0;
                                        pos_0 = pos_0 + 1;
                                        *((*s).byte_code.buf).offset(fresh19 as isize) = REOP_set_i32
                                            as core::ffi::c_int as uint8_t;
                                        let fresh20 = pos_0;
                                        pos_0 = pos_0 + 1;
                                        *((*s).byte_code.buf).offset(fresh20 as isize) = 0
                                            as uint8_t;
                                        put_u32(
                                            ((*s).byte_code.buf).offset(pos_0 as isize),
                                            quant_max as uint32_t,
                                        );
                                        pos_0 += 4 as core::ffi::c_int;
                                        last_atom_start = pos_0;
                                        if add_zero_advance_check != 0 {
                                            let fresh21 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *((*s).byte_code.buf).offset(fresh21 as isize) = REOP_set_char_pos
                                                as core::ffi::c_int as uint8_t;
                                            let fresh22 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *((*s).byte_code.buf).offset(fresh22 as isize) = 0
                                                as uint8_t;
                                        }
                                        if quant_min == quant_max {
                                            re_emit_goto_u8(
                                                s,
                                                REOP_loop as core::ffi::c_int,
                                                0 as uint32_t,
                                                last_atom_start as uint32_t,
                                            );
                                        } else {
                                            re_emit_goto_u8_u32(
                                                s,
                                                (if add_zero_advance_check != 0 {
                                                    REOP_loop_check_adv_split_next_first as core::ffi::c_int
                                                } else {
                                                    REOP_loop_split_next_first as core::ffi::c_int
                                                }) - greedy as core::ffi::c_int,
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
                                        last_atom_start = -(1 as core::ffi::c_int);
                                        current_block = 11353886201549099807;
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11353886201549099807 => {}
                            _ => return re_parse_out_of_memory(s),
                        }
                    }
                }
            }
            63 => {
                current_block = 18255723851606772592;
                match current_block {
                    7069606996288041156 => {
                        p = p.offset(1);
                        quant_min = 0 as core::ffi::c_int;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    18255723851606772592 => {
                        p = p.offset(1);
                        quant_min = 0 as core::ffi::c_int;
                        quant_max = 1 as core::ffi::c_int;
                        current_block = 9506328432318339935;
                    }
                    11105283338979551329 => {
                        p = p.offset(1);
                        quant_min = 1 as core::ffi::c_int;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    _ => {
                        let mut p1_1: *const uint8_t = p;
                        if is_digit(
                            *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int,
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
                                TRUE as core::ffi::c_int as BOOL,
                            );
                            quant_max = quant_min;
                            if *p as core::ffi::c_int == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as core::ffi::c_int) != 0 {
                                    quant_max = parse_digits(
                                        &mut p,
                                        TRUE as core::ffi::c_int as BOOL,
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
                                    if *p as core::ffi::c_int != '}' as i32
                                        && (*s).is_unicode == 0
                                    {
                                        p = p1_1;
                                        current_block = 11353886201549099807;
                                    } else {
                                        if re_parse_expect(s, &mut p, '}' as i32) != 0 {
                                            return -(1 as core::ffi::c_int);
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
                                    s,
                                    b"invalid repetition count\0" as *const u8
                                        as *const core::ffi::c_char,
                                );
                            }
                        }
                    }
                }
                match current_block {
                    11353886201549099807 => {}
                    _ => {
                        greedy = TRUE as core::ffi::c_int as BOOL;
                        if *p as core::ffi::c_int == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as core::ffi::c_int as BOOL;
                        }
                        if last_atom_start < 0 as core::ffi::c_int {
                            return re_parse_error(
                                s,
                                b"nothing to repeat\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                        let mut need_capture_init: BOOL = 0;
                        let mut add_zero_advance_check: BOOL = 0;
                        let mut len: core::ffi::c_int = 0;
                        let mut pos_0: core::ffi::c_int = 0;
                        add_zero_advance_check = re_need_check_adv_and_capture_init(
                            &mut need_capture_init,
                            ((*s).byte_code.buf).offset(last_atom_start as isize),
                            ((*s).byte_code.size).wrapping_sub(last_atom_start as size_t)
                                as core::ffi::c_int,
                        );
                        if need_capture_init != 0
                            && last_capture_count != (*s).capture_count
                        {
                            if dbuf_insert(
                                &mut (*s).byte_code,
                                last_atom_start,
                                3 as core::ffi::c_int,
                            ) != 0
                            {
                                current_block = 17551395354807108434;
                            } else {
                                let mut pos_1: core::ffi::c_int = last_atom_start;
                                let fresh8 = pos_1;
                                pos_1 = pos_1 + 1;
                                *((*s).byte_code.buf).offset(fresh8 as isize) = REOP_save_reset
                                    as core::ffi::c_int as uint8_t;
                                let fresh9 = pos_1;
                                pos_1 = pos_1 + 1;
                                *((*s).byte_code.buf).offset(fresh9 as isize) = last_capture_count
                                    as uint8_t;
                                let fresh10 = pos_1;
                                pos_1 = pos_1 + 1;
                                *((*s).byte_code.buf).offset(fresh10 as isize) = ((*s)
                                    .capture_count - 1 as core::ffi::c_int) as uint8_t;
                                current_block = 1851490986684842406;
                            }
                        } else {
                            current_block = 1851490986684842406;
                        }
                        match current_block {
                            1851490986684842406 => {
                                len = ((*s).byte_code.size)
                                    .wrapping_sub(last_atom_start as size_t)
                                    as core::ffi::c_int;
                                if quant_min == 0 as core::ffi::c_int {
                                    if need_capture_init == 0
                                        && last_capture_count != (*s).capture_count
                                    {
                                        if dbuf_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            3 as core::ffi::c_int,
                                        ) != 0
                                        {
                                            current_block = 17551395354807108434;
                                        } else {
                                            let fresh11 = last_atom_start;
                                            last_atom_start = last_atom_start + 1;
                                            *((*s).byte_code.buf).offset(fresh11 as isize) = REOP_save_reset
                                                as core::ffi::c_int as uint8_t;
                                            let fresh12 = last_atom_start;
                                            last_atom_start = last_atom_start + 1;
                                            *((*s).byte_code.buf).offset(fresh12 as isize) = last_capture_count
                                                as uint8_t;
                                            let fresh13 = last_atom_start;
                                            last_atom_start = last_atom_start + 1;
                                            *((*s).byte_code.buf).offset(fresh13 as isize) = ((*s)
                                                .capture_count - 1 as core::ffi::c_int) as uint8_t;
                                            current_block = 8700473759921513224;
                                        }
                                    } else {
                                        current_block = 8700473759921513224;
                                    }
                                    match current_block {
                                        17551395354807108434 => {}
                                        _ => {
                                            if quant_max == 0 as core::ffi::c_int {
                                                (*s).byte_code.size = last_atom_start as size_t;
                                                current_block = 2588063579017527985;
                                            } else if quant_max == 1 as core::ffi::c_int
                                                || quant_max == INT32_MAX
                                            {
                                                let mut has_goto: BOOL = (quant_max == INT32_MAX)
                                                    as core::ffi::c_int;
                                                if dbuf_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    5 as core::ffi::c_int
                                                        + add_zero_advance_check as core::ffi::c_int
                                                            * 2 as core::ffi::c_int,
                                                ) != 0
                                                {
                                                    current_block = 17551395354807108434;
                                                } else {
                                                    *((*s).byte_code.buf).offset(last_atom_start as isize) = (REOP_split_goto_first
                                                        as core::ffi::c_int as BOOL + greedy) as uint8_t;
                                                    put_u32(
                                                        ((*s).byte_code.buf)
                                                            .offset(last_atom_start as isize)
                                                            .offset(1 as core::ffi::c_int as isize),
                                                        (len + 5 as core::ffi::c_int * has_goto as core::ffi::c_int
                                                            + add_zero_advance_check as core::ffi::c_int
                                                                * 2 as core::ffi::c_int * 2 as core::ffi::c_int) as uint32_t,
                                                    );
                                                    if add_zero_advance_check != 0 {
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                (last_atom_start + 1 as core::ffi::c_int
                                                                    + 4 as core::ffi::c_int) as isize,
                                                            ) = REOP_set_char_pos as core::ffi::c_int as uint8_t;
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                (last_atom_start + 1 as core::ffi::c_int
                                                                    + 4 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                                                            ) = 0 as uint8_t;
                                                        re_emit_op_u8(
                                                            s,
                                                            REOP_check_advance as core::ffi::c_int,
                                                            0 as uint32_t,
                                                        );
                                                    }
                                                    if has_goto != 0 {
                                                        re_emit_goto(
                                                            s,
                                                            REOP_goto as core::ffi::c_int,
                                                            last_atom_start as uint32_t,
                                                        );
                                                    }
                                                    current_block = 2588063579017527985;
                                                }
                                            } else if dbuf_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                11 as core::ffi::c_int
                                                    + add_zero_advance_check as core::ffi::c_int
                                                        * 2 as core::ffi::c_int,
                                            ) != 0
                                            {
                                                current_block = 17551395354807108434;
                                            } else {
                                                pos_0 = last_atom_start;
                                                let fresh14 = pos_0;
                                                pos_0 = pos_0 + 1;
                                                *((*s).byte_code.buf).offset(fresh14 as isize) = (REOP_split_goto_first
                                                    as core::ffi::c_int as BOOL + greedy) as uint8_t;
                                                put_u32(
                                                    ((*s).byte_code.buf).offset(pos_0 as isize),
                                                    (6 as core::ffi::c_int
                                                        + add_zero_advance_check as core::ffi::c_int
                                                            * 2 as core::ffi::c_int + len + 10 as core::ffi::c_int)
                                                        as uint32_t,
                                                );
                                                pos_0 += 4 as core::ffi::c_int;
                                                let fresh15 = pos_0;
                                                pos_0 = pos_0 + 1;
                                                *((*s).byte_code.buf).offset(fresh15 as isize) = REOP_set_i32
                                                    as core::ffi::c_int as uint8_t;
                                                let fresh16 = pos_0;
                                                pos_0 = pos_0 + 1;
                                                *((*s).byte_code.buf).offset(fresh16 as isize) = 0
                                                    as uint8_t;
                                                put_u32(
                                                    ((*s).byte_code.buf).offset(pos_0 as isize),
                                                    quant_max as uint32_t,
                                                );
                                                pos_0 += 4 as core::ffi::c_int;
                                                last_atom_start = pos_0;
                                                if add_zero_advance_check != 0 {
                                                    let fresh17 = pos_0;
                                                    pos_0 = pos_0 + 1;
                                                    *((*s).byte_code.buf).offset(fresh17 as isize) = REOP_set_char_pos
                                                        as core::ffi::c_int as uint8_t;
                                                    let fresh18 = pos_0;
                                                    pos_0 = pos_0 + 1;
                                                    *((*s).byte_code.buf).offset(fresh18 as isize) = 0
                                                        as uint8_t;
                                                }
                                                re_emit_goto_u8_u32(
                                                    s,
                                                    (if add_zero_advance_check != 0 {
                                                        REOP_loop_check_adv_split_next_first as core::ffi::c_int
                                                    } else {
                                                        REOP_loop_split_next_first as core::ffi::c_int
                                                    }) - greedy as core::ffi::c_int,
                                                    0 as uint32_t,
                                                    quant_max as uint32_t,
                                                    last_atom_start as uint32_t,
                                                );
                                                current_block = 2588063579017527985;
                                            }
                                        }
                                    }
                                } else if quant_min == 1 as core::ffi::c_int
                                    && quant_max == INT32_MAX && add_zero_advance_check == 0
                                {
                                    re_emit_goto(
                                        s,
                                        REOP_split_next_first as core::ffi::c_int
                                            - greedy as core::ffi::c_int,
                                        last_atom_start as uint32_t,
                                    );
                                    current_block = 2588063579017527985;
                                } else {
                                    if quant_min == quant_max {
                                        add_zero_advance_check = FALSE as core::ffi::c_int as BOOL;
                                    }
                                    if dbuf_insert(
                                        &mut (*s).byte_code,
                                        last_atom_start,
                                        6 as core::ffi::c_int
                                            + add_zero_advance_check as core::ffi::c_int
                                                * 2 as core::ffi::c_int,
                                    ) != 0
                                    {
                                        current_block = 17551395354807108434;
                                    } else {
                                        pos_0 = last_atom_start;
                                        let fresh19 = pos_0;
                                        pos_0 = pos_0 + 1;
                                        *((*s).byte_code.buf).offset(fresh19 as isize) = REOP_set_i32
                                            as core::ffi::c_int as uint8_t;
                                        let fresh20 = pos_0;
                                        pos_0 = pos_0 + 1;
                                        *((*s).byte_code.buf).offset(fresh20 as isize) = 0
                                            as uint8_t;
                                        put_u32(
                                            ((*s).byte_code.buf).offset(pos_0 as isize),
                                            quant_max as uint32_t,
                                        );
                                        pos_0 += 4 as core::ffi::c_int;
                                        last_atom_start = pos_0;
                                        if add_zero_advance_check != 0 {
                                            let fresh21 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *((*s).byte_code.buf).offset(fresh21 as isize) = REOP_set_char_pos
                                                as core::ffi::c_int as uint8_t;
                                            let fresh22 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *((*s).byte_code.buf).offset(fresh22 as isize) = 0
                                                as uint8_t;
                                        }
                                        if quant_min == quant_max {
                                            re_emit_goto_u8(
                                                s,
                                                REOP_loop as core::ffi::c_int,
                                                0 as uint32_t,
                                                last_atom_start as uint32_t,
                                            );
                                        } else {
                                            re_emit_goto_u8_u32(
                                                s,
                                                (if add_zero_advance_check != 0 {
                                                    REOP_loop_check_adv_split_next_first as core::ffi::c_int
                                                } else {
                                                    REOP_loop_split_next_first as core::ffi::c_int
                                                }) - greedy as core::ffi::c_int,
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
                                        last_atom_start = -(1 as core::ffi::c_int);
                                        current_block = 11353886201549099807;
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11353886201549099807 => {}
                            _ => return re_parse_out_of_memory(s),
                        }
                    }
                }
            }
            123 => {
                current_block = 16167632229894708628;
                match current_block {
                    7069606996288041156 => {
                        p = p.offset(1);
                        quant_min = 0 as core::ffi::c_int;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    18255723851606772592 => {
                        p = p.offset(1);
                        quant_min = 0 as core::ffi::c_int;
                        quant_max = 1 as core::ffi::c_int;
                        current_block = 9506328432318339935;
                    }
                    11105283338979551329 => {
                        p = p.offset(1);
                        quant_min = 1 as core::ffi::c_int;
                        quant_max = INT32_MAX;
                        current_block = 9506328432318339935;
                    }
                    _ => {
                        let mut p1_1: *const uint8_t = p;
                        if is_digit(
                            *p.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int,
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
                                TRUE as core::ffi::c_int as BOOL,
                            );
                            quant_max = quant_min;
                            if *p as core::ffi::c_int == ',' as i32 {
                                p = p.offset(1);
                                if is_digit(*p as core::ffi::c_int) != 0 {
                                    quant_max = parse_digits(
                                        &mut p,
                                        TRUE as core::ffi::c_int as BOOL,
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
                                    if *p as core::ffi::c_int != '}' as i32
                                        && (*s).is_unicode == 0
                                    {
                                        p = p1_1;
                                        current_block = 11353886201549099807;
                                    } else {
                                        if re_parse_expect(s, &mut p, '}' as i32) != 0 {
                                            return -(1 as core::ffi::c_int);
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
                                    s,
                                    b"invalid repetition count\0" as *const u8
                                        as *const core::ffi::c_char,
                                );
                            }
                        }
                    }
                }
                match current_block {
                    11353886201549099807 => {}
                    _ => {
                        greedy = TRUE as core::ffi::c_int as BOOL;
                        if *p as core::ffi::c_int == '?' as i32 {
                            p = p.offset(1);
                            greedy = FALSE as core::ffi::c_int as BOOL;
                        }
                        if last_atom_start < 0 as core::ffi::c_int {
                            return re_parse_error(
                                s,
                                b"nothing to repeat\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                        let mut need_capture_init: BOOL = 0;
                        let mut add_zero_advance_check: BOOL = 0;
                        let mut len: core::ffi::c_int = 0;
                        let mut pos_0: core::ffi::c_int = 0;
                        add_zero_advance_check = re_need_check_adv_and_capture_init(
                            &mut need_capture_init,
                            ((*s).byte_code.buf).offset(last_atom_start as isize),
                            ((*s).byte_code.size).wrapping_sub(last_atom_start as size_t)
                                as core::ffi::c_int,
                        );
                        if need_capture_init != 0
                            && last_capture_count != (*s).capture_count
                        {
                            if dbuf_insert(
                                &mut (*s).byte_code,
                                last_atom_start,
                                3 as core::ffi::c_int,
                            ) != 0
                            {
                                current_block = 17551395354807108434;
                            } else {
                                let mut pos_1: core::ffi::c_int = last_atom_start;
                                let fresh8 = pos_1;
                                pos_1 = pos_1 + 1;
                                *((*s).byte_code.buf).offset(fresh8 as isize) = REOP_save_reset
                                    as core::ffi::c_int as uint8_t;
                                let fresh9 = pos_1;
                                pos_1 = pos_1 + 1;
                                *((*s).byte_code.buf).offset(fresh9 as isize) = last_capture_count
                                    as uint8_t;
                                let fresh10 = pos_1;
                                pos_1 = pos_1 + 1;
                                *((*s).byte_code.buf).offset(fresh10 as isize) = ((*s)
                                    .capture_count - 1 as core::ffi::c_int) as uint8_t;
                                current_block = 1851490986684842406;
                            }
                        } else {
                            current_block = 1851490986684842406;
                        }
                        match current_block {
                            1851490986684842406 => {
                                len = ((*s).byte_code.size)
                                    .wrapping_sub(last_atom_start as size_t)
                                    as core::ffi::c_int;
                                if quant_min == 0 as core::ffi::c_int {
                                    if need_capture_init == 0
                                        && last_capture_count != (*s).capture_count
                                    {
                                        if dbuf_insert(
                                            &mut (*s).byte_code,
                                            last_atom_start,
                                            3 as core::ffi::c_int,
                                        ) != 0
                                        {
                                            current_block = 17551395354807108434;
                                        } else {
                                            let fresh11 = last_atom_start;
                                            last_atom_start = last_atom_start + 1;
                                            *((*s).byte_code.buf).offset(fresh11 as isize) = REOP_save_reset
                                                as core::ffi::c_int as uint8_t;
                                            let fresh12 = last_atom_start;
                                            last_atom_start = last_atom_start + 1;
                                            *((*s).byte_code.buf).offset(fresh12 as isize) = last_capture_count
                                                as uint8_t;
                                            let fresh13 = last_atom_start;
                                            last_atom_start = last_atom_start + 1;
                                            *((*s).byte_code.buf).offset(fresh13 as isize) = ((*s)
                                                .capture_count - 1 as core::ffi::c_int) as uint8_t;
                                            current_block = 8700473759921513224;
                                        }
                                    } else {
                                        current_block = 8700473759921513224;
                                    }
                                    match current_block {
                                        17551395354807108434 => {}
                                        _ => {
                                            if quant_max == 0 as core::ffi::c_int {
                                                (*s).byte_code.size = last_atom_start as size_t;
                                                current_block = 2588063579017527985;
                                            } else if quant_max == 1 as core::ffi::c_int
                                                || quant_max == INT32_MAX
                                            {
                                                let mut has_goto: BOOL = (quant_max == INT32_MAX)
                                                    as core::ffi::c_int;
                                                if dbuf_insert(
                                                    &mut (*s).byte_code,
                                                    last_atom_start,
                                                    5 as core::ffi::c_int
                                                        + add_zero_advance_check as core::ffi::c_int
                                                            * 2 as core::ffi::c_int,
                                                ) != 0
                                                {
                                                    current_block = 17551395354807108434;
                                                } else {
                                                    *((*s).byte_code.buf).offset(last_atom_start as isize) = (REOP_split_goto_first
                                                        as core::ffi::c_int as BOOL + greedy) as uint8_t;
                                                    put_u32(
                                                        ((*s).byte_code.buf)
                                                            .offset(last_atom_start as isize)
                                                            .offset(1 as core::ffi::c_int as isize),
                                                        (len + 5 as core::ffi::c_int * has_goto as core::ffi::c_int
                                                            + add_zero_advance_check as core::ffi::c_int
                                                                * 2 as core::ffi::c_int * 2 as core::ffi::c_int) as uint32_t,
                                                    );
                                                    if add_zero_advance_check != 0 {
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                (last_atom_start + 1 as core::ffi::c_int
                                                                    + 4 as core::ffi::c_int) as isize,
                                                            ) = REOP_set_char_pos as core::ffi::c_int as uint8_t;
                                                        *((*s).byte_code.buf)
                                                            .offset(
                                                                (last_atom_start + 1 as core::ffi::c_int
                                                                    + 4 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                                                            ) = 0 as uint8_t;
                                                        re_emit_op_u8(
                                                            s,
                                                            REOP_check_advance as core::ffi::c_int,
                                                            0 as uint32_t,
                                                        );
                                                    }
                                                    if has_goto != 0 {
                                                        re_emit_goto(
                                                            s,
                                                            REOP_goto as core::ffi::c_int,
                                                            last_atom_start as uint32_t,
                                                        );
                                                    }
                                                    current_block = 2588063579017527985;
                                                }
                                            } else if dbuf_insert(
                                                &mut (*s).byte_code,
                                                last_atom_start,
                                                11 as core::ffi::c_int
                                                    + add_zero_advance_check as core::ffi::c_int
                                                        * 2 as core::ffi::c_int,
                                            ) != 0
                                            {
                                                current_block = 17551395354807108434;
                                            } else {
                                                pos_0 = last_atom_start;
                                                let fresh14 = pos_0;
                                                pos_0 = pos_0 + 1;
                                                *((*s).byte_code.buf).offset(fresh14 as isize) = (REOP_split_goto_first
                                                    as core::ffi::c_int as BOOL + greedy) as uint8_t;
                                                put_u32(
                                                    ((*s).byte_code.buf).offset(pos_0 as isize),
                                                    (6 as core::ffi::c_int
                                                        + add_zero_advance_check as core::ffi::c_int
                                                            * 2 as core::ffi::c_int + len + 10 as core::ffi::c_int)
                                                        as uint32_t,
                                                );
                                                pos_0 += 4 as core::ffi::c_int;
                                                let fresh15 = pos_0;
                                                pos_0 = pos_0 + 1;
                                                *((*s).byte_code.buf).offset(fresh15 as isize) = REOP_set_i32
                                                    as core::ffi::c_int as uint8_t;
                                                let fresh16 = pos_0;
                                                pos_0 = pos_0 + 1;
                                                *((*s).byte_code.buf).offset(fresh16 as isize) = 0
                                                    as uint8_t;
                                                put_u32(
                                                    ((*s).byte_code.buf).offset(pos_0 as isize),
                                                    quant_max as uint32_t,
                                                );
                                                pos_0 += 4 as core::ffi::c_int;
                                                last_atom_start = pos_0;
                                                if add_zero_advance_check != 0 {
                                                    let fresh17 = pos_0;
                                                    pos_0 = pos_0 + 1;
                                                    *((*s).byte_code.buf).offset(fresh17 as isize) = REOP_set_char_pos
                                                        as core::ffi::c_int as uint8_t;
                                                    let fresh18 = pos_0;
                                                    pos_0 = pos_0 + 1;
                                                    *((*s).byte_code.buf).offset(fresh18 as isize) = 0
                                                        as uint8_t;
                                                }
                                                re_emit_goto_u8_u32(
                                                    s,
                                                    (if add_zero_advance_check != 0 {
                                                        REOP_loop_check_adv_split_next_first as core::ffi::c_int
                                                    } else {
                                                        REOP_loop_split_next_first as core::ffi::c_int
                                                    }) - greedy as core::ffi::c_int,
                                                    0 as uint32_t,
                                                    quant_max as uint32_t,
                                                    last_atom_start as uint32_t,
                                                );
                                                current_block = 2588063579017527985;
                                            }
                                        }
                                    }
                                } else if quant_min == 1 as core::ffi::c_int
                                    && quant_max == INT32_MAX && add_zero_advance_check == 0
                                {
                                    re_emit_goto(
                                        s,
                                        REOP_split_next_first as core::ffi::c_int
                                            - greedy as core::ffi::c_int,
                                        last_atom_start as uint32_t,
                                    );
                                    current_block = 2588063579017527985;
                                } else {
                                    if quant_min == quant_max {
                                        add_zero_advance_check = FALSE as core::ffi::c_int as BOOL;
                                    }
                                    if dbuf_insert(
                                        &mut (*s).byte_code,
                                        last_atom_start,
                                        6 as core::ffi::c_int
                                            + add_zero_advance_check as core::ffi::c_int
                                                * 2 as core::ffi::c_int,
                                    ) != 0
                                    {
                                        current_block = 17551395354807108434;
                                    } else {
                                        pos_0 = last_atom_start;
                                        let fresh19 = pos_0;
                                        pos_0 = pos_0 + 1;
                                        *((*s).byte_code.buf).offset(fresh19 as isize) = REOP_set_i32
                                            as core::ffi::c_int as uint8_t;
                                        let fresh20 = pos_0;
                                        pos_0 = pos_0 + 1;
                                        *((*s).byte_code.buf).offset(fresh20 as isize) = 0
                                            as uint8_t;
                                        put_u32(
                                            ((*s).byte_code.buf).offset(pos_0 as isize),
                                            quant_max as uint32_t,
                                        );
                                        pos_0 += 4 as core::ffi::c_int;
                                        last_atom_start = pos_0;
                                        if add_zero_advance_check != 0 {
                                            let fresh21 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *((*s).byte_code.buf).offset(fresh21 as isize) = REOP_set_char_pos
                                                as core::ffi::c_int as uint8_t;
                                            let fresh22 = pos_0;
                                            pos_0 = pos_0 + 1;
                                            *((*s).byte_code.buf).offset(fresh22 as isize) = 0
                                                as uint8_t;
                                        }
                                        if quant_min == quant_max {
                                            re_emit_goto_u8(
                                                s,
                                                REOP_loop as core::ffi::c_int,
                                                0 as uint32_t,
                                                last_atom_start as uint32_t,
                                            );
                                        } else {
                                            re_emit_goto_u8_u32(
                                                s,
                                                (if add_zero_advance_check != 0 {
                                                    REOP_loop_check_adv_split_next_first as core::ffi::c_int
                                                } else {
                                                    REOP_loop_split_next_first as core::ffi::c_int
                                                }) - greedy as core::ffi::c_int,
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
                                        last_atom_start = -(1 as core::ffi::c_int);
                                        current_block = 11353886201549099807;
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11353886201549099807 => {}
                            _ => return re_parse_out_of_memory(s),
                        }
                    }
                }
            }
            _ => {}
        }
    }
    (*s).buf_ptr = p;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn re_parse_alternative(
    mut s: *mut REParseState,
    mut is_backward_dir: BOOL,
) -> core::ffi::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut ret: core::ffi::c_int = 0;
    let mut start: size_t = 0;
    let mut term_start: size_t = 0;
    let mut end: size_t = 0;
    let mut term_size: size_t = 0;
    start = (*s).byte_code.size;
    loop {
        p = (*s).buf_ptr;
        if p >= (*s).buf_end {
            break;
        }
        if *p as core::ffi::c_int == '|' as i32 || *p as core::ffi::c_int == ')' as i32 {
            break;
        }
        term_start = (*s).byte_code.size;
        ret = re_parse_term(s, is_backward_dir);
        if ret != 0 {
            return ret;
        }
        if is_backward_dir != 0 {
            end = (*s).byte_code.size;
            term_size = end.wrapping_sub(term_start);
            if dbuf_claim(&mut (*s).byte_code, term_size) != 0 {
                return -(1 as core::ffi::c_int);
            }
            memmove(
                ((*s).byte_code.buf).offset(start as isize).offset(term_size as isize)
                    as *mut core::ffi::c_void,
                ((*s).byte_code.buf).offset(start as isize) as *const core::ffi::c_void,
                end.wrapping_sub(start),
            );
            memcpy(
                ((*s).byte_code.buf).offset(start as isize) as *mut core::ffi::c_void,
                ((*s).byte_code.buf).offset(end as isize) as *const core::ffi::c_void,
                term_size,
            );
        }
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn re_parse_disjunction(
    mut s: *mut REParseState,
    mut is_backward_dir: BOOL,
) -> core::ffi::c_int {
    let mut start: core::ffi::c_int = 0;
    let mut len: core::ffi::c_int = 0;
    let mut pos: core::ffi::c_int = 0;
    if lre_check_stack_overflow((*s).opaque, 0 as size_t) != 0 {
        return re_parse_error(
            s,
            b"stack overflow\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    start = (*s).byte_code.size as core::ffi::c_int;
    if re_parse_alternative(s, is_backward_dir) != 0 {
        return -(1 as core::ffi::c_int);
    }
    while *(*s).buf_ptr as core::ffi::c_int == '|' as i32 {
        (*s).buf_ptr = ((*s).buf_ptr).offset(1);
        len = ((*s).byte_code.size).wrapping_sub(start as size_t) as core::ffi::c_int;
        if dbuf_insert(&mut (*s).byte_code, start, 5 as core::ffi::c_int) != 0 {
            return re_parse_out_of_memory(s);
        }
        *((*s).byte_code.buf).offset(start as isize) = REOP_split_next_first
            as core::ffi::c_int as uint8_t;
        put_u32(
            ((*s).byte_code.buf)
                .offset(start as isize)
                .offset(1 as core::ffi::c_int as isize),
            (len + 5 as core::ffi::c_int) as uint32_t,
        );
        pos = re_emit_op_u32(s, REOP_goto as core::ffi::c_int, 0 as uint32_t);
        (*s).group_name_scope = ((*s).group_name_scope).wrapping_add(1);
        if re_parse_alternative(s, is_backward_dir) != 0 {
            return -(1 as core::ffi::c_int);
        }
        len = ((*s).byte_code.size).wrapping_sub((pos + 4 as core::ffi::c_int) as size_t)
            as core::ffi::c_int;
        put_u32(((*s).byte_code.buf).offset(pos as isize), len as uint32_t);
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn compute_register_count(
    mut bc_buf: *mut uint8_t,
    mut bc_buf_len: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut stack_size: core::ffi::c_int = 0;
    let mut stack_size_max: core::ffi::c_int = 0;
    let mut pos: core::ffi::c_int = 0;
    let mut opcode: core::ffi::c_int = 0;
    let mut len: core::ffi::c_int = 0;
    let mut val: uint32_t = 0;
    stack_size = 0 as core::ffi::c_int;
    stack_size_max = 0 as core::ffi::c_int;
    bc_buf = bc_buf.offset(RE_HEADER_LEN as isize);
    bc_buf_len -= RE_HEADER_LEN;
    pos = 0 as core::ffi::c_int;
    while pos < bc_buf_len {
        opcode = *bc_buf.offset(pos as isize) as core::ffi::c_int;
        len = reopcode_info[opcode as usize].size as core::ffi::c_int;
        if opcode < REOP_COUNT as core::ffi::c_int {} else {
            __assert_fail(
                b"opcode < REOP_COUNT\0" as *const u8 as *const core::ffi::c_char,
                b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                2457 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 43],
                    [core::ffi::c_char; 43],
                >(*b"int compute_register_count(uint8_t *, int)\0"))
                    .as_ptr(),
            );
        }
        'c_4570: {
            if opcode < REOP_COUNT as core::ffi::c_int {} else {
                __assert_fail(
                    b"opcode < REOP_COUNT\0" as *const u8 as *const core::ffi::c_char,
                    b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                    2457 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 43],
                        [core::ffi::c_char; 43],
                    >(*b"int compute_register_count(uint8_t *, int)\0"))
                        .as_ptr(),
                );
            }
        };
        if pos + len <= bc_buf_len {} else {
            __assert_fail(
                b"(pos + len) <= bc_buf_len\0" as *const u8 as *const core::ffi::c_char,
                b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                2458 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 43],
                    [core::ffi::c_char; 43],
                >(*b"int compute_register_count(uint8_t *, int)\0"))
                    .as_ptr(),
            );
        }
        'c_4523: {
            if pos + len <= bc_buf_len {} else {
                __assert_fail(
                    b"(pos + len) <= bc_buf_len\0" as *const u8
                        as *const core::ffi::c_char,
                    b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                    2458 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 43],
                        [core::ffi::c_char; 43],
                    >(*b"int compute_register_count(uint8_t *, int)\0"))
                        .as_ptr(),
                );
            }
        };
        match opcode {
            27 | 42 => {
                *bc_buf.offset((pos + 1 as core::ffi::c_int) as isize) = stack_size
                    as uint8_t;
                stack_size += 1;
                if stack_size > stack_size_max {
                    if stack_size > REGISTER_COUNT_MAX {
                        return -(1 as core::ffi::c_int);
                    }
                    stack_size_max = stack_size;
                }
            }
            43 | 22 | 23 | 24 => {
                if stack_size > 0 as core::ffi::c_int {} else {
                    __assert_fail(
                        b"stack_size > 0\0" as *const u8 as *const core::ffi::c_char,
                        b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                        2474 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 43],
                            [core::ffi::c_char; 43],
                        >(*b"int compute_register_count(uint8_t *, int)\0"))
                            .as_ptr(),
                    );
                }
                'c_4435: {
                    if stack_size > 0 as core::ffi::c_int {} else {
                        __assert_fail(
                            b"stack_size > 0\0" as *const u8 as *const core::ffi::c_char,
                            b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                            2474 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 43],
                                [core::ffi::c_char; 43],
                            >(*b"int compute_register_count(uint8_t *, int)\0"))
                                .as_ptr(),
                        );
                    }
                };
                stack_size -= 1;
                *bc_buf.offset((pos + 1 as core::ffi::c_int) as isize) = stack_size
                    as uint8_t;
            }
            25 | 26 => {
                if stack_size >= 2 as core::ffi::c_int {} else {
                    __assert_fail(
                        b"stack_size >= 2\0" as *const u8 as *const core::ffi::c_char,
                        b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                        2480 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 43],
                            [core::ffi::c_char; 43],
                        >(*b"int compute_register_count(uint8_t *, int)\0"))
                            .as_ptr(),
                    );
                }
                'c_4369: {
                    if stack_size >= 2 as core::ffi::c_int {} else {
                        __assert_fail(
                            b"stack_size >= 2\0" as *const u8
                                as *const core::ffi::c_char,
                            b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                            2480 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 43],
                                [core::ffi::c_char; 43],
                            >(*b"int compute_register_count(uint8_t *, int)\0"))
                                .as_ptr(),
                        );
                    }
                };
                stack_size -= 2 as core::ffi::c_int;
                *bc_buf.offset((pos + 1 as core::ffi::c_int) as isize) = stack_size
                    as uint8_t;
            }
            36 | 37 => {
                val = get_u16(
                    bc_buf.offset(pos as isize).offset(1 as core::ffi::c_int as isize),
                );
                len = (len as core::ffi::c_uint)
                    .wrapping_add(val.wrapping_mul(4 as uint32_t) as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_int;
            }
            38 | 39 => {
                val = get_u16(
                    bc_buf.offset(pos as isize).offset(1 as core::ffi::c_int as isize),
                );
                len = (len as core::ffi::c_uint)
                    .wrapping_add(val.wrapping_mul(8 as uint32_t) as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_int;
            }
            32 | 33 | 34 | 35 => {
                val = *bc_buf.offset((pos + 1 as core::ffi::c_int) as isize) as uint32_t;
                len = (len as core::ffi::c_uint).wrapping_add(val as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_int;
            }
            _ => {}
        }
        pos += len;
    }
    return stack_size_max;
}
unsafe extern "C" fn lre_bytecode_realloc(
    mut opaque: *mut core::ffi::c_void,
    mut ptr: *mut core::ffi::c_void,
    mut size: size_t,
) -> *mut core::ffi::c_void {
    if size > (INT32_MAX / 2 as core::ffi::c_int) as size_t {
        return NULL
    } else {
        return lre_realloc(opaque, ptr, size)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lre_compile(
    mut plen: *mut core::ffi::c_int,
    mut error_msg: *mut core::ffi::c_char,
    mut error_msg_size: core::ffi::c_int,
    mut buf: *const core::ffi::c_char,
    mut buf_len: size_t,
    mut re_flags: core::ffi::c_int,
    mut opaque: *mut core::ffi::c_void,
) -> *mut uint8_t {
    let mut s_s: REParseState = REParseState {
        byte_code: DynBuf {
            buf: 0 as *mut uint8_t,
            size: 0,
            allocated_size: 0,
            error: 0,
            realloc_func: None,
            opaque: 0 as *mut core::ffi::c_void,
        },
        buf_ptr: 0 as *const uint8_t,
        buf_end: 0 as *const uint8_t,
        buf_start: 0 as *const uint8_t,
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
        opaque: 0 as *mut core::ffi::c_void,
        group_names: DynBuf {
            buf: 0 as *mut uint8_t,
            size: 0,
            allocated_size: 0,
            error: 0,
            realloc_func: None,
            opaque: 0 as *mut core::ffi::c_void,
        },
        u: C2RustUnnamed_0 {
            error_msg: [0; 128],
        },
    };
    let mut s: *mut REParseState = &mut s_s;
    let mut register_count: core::ffi::c_int = 0;
    let mut is_sticky: BOOL = 0;
    memset(
        s as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<REParseState>() as size_t,
    );
    (*s).opaque = opaque;
    (*s).buf_ptr = buf as *const uint8_t;
    (*s).buf_end = ((*s).buf_ptr).offset(buf_len as isize);
    (*s).buf_start = (*s).buf_ptr;
    (*s).re_flags = re_flags;
    (*s).is_unicode = (re_flags & (LRE_FLAG_UNICODE | LRE_FLAG_UNICODE_SETS)
        != 0 as core::ffi::c_int) as core::ffi::c_int as BOOL;
    is_sticky = (re_flags & LRE_FLAG_STICKY != 0 as core::ffi::c_int) as core::ffi::c_int
        as BOOL;
    (*s).ignore_case = (re_flags & LRE_FLAG_IGNORECASE != 0 as core::ffi::c_int)
        as core::ffi::c_int as BOOL;
    (*s).multi_line = (re_flags & LRE_FLAG_MULTILINE != 0 as core::ffi::c_int)
        as core::ffi::c_int as BOOL;
    (*s).dotall = (re_flags & LRE_FLAG_DOTALL != 0 as core::ffi::c_int)
        as core::ffi::c_int as BOOL;
    (*s).unicode_sets = (re_flags & LRE_FLAG_UNICODE_SETS != 0 as core::ffi::c_int)
        as core::ffi::c_int as BOOL;
    (*s).capture_count = 1 as core::ffi::c_int;
    (*s).total_capture_count = -(1 as core::ffi::c_int);
    (*s).has_named_captures = -(1 as core::ffi::c_int);
    dbuf_init2(
        &mut (*s).byte_code,
        opaque,
        Some(
            lre_bytecode_realloc
                as unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    size_t,
                ) -> *mut core::ffi::c_void,
        ),
    );
    dbuf_init2(
        &mut (*s).group_names,
        opaque,
        Some(
            lre_realloc
                as unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    size_t,
                ) -> *mut core::ffi::c_void,
        ),
    );
    dbuf_put_u16(&mut (*s).byte_code, re_flags as uint16_t);
    dbuf_putc(&mut (*s).byte_code, 0 as uint8_t);
    dbuf_putc(&mut (*s).byte_code, 0 as uint8_t);
    dbuf_put_u32(&mut (*s).byte_code, 0 as uint32_t);
    if is_sticky == 0 {
        re_emit_op_u32(
            s,
            REOP_split_goto_first as core::ffi::c_int,
            (1 as core::ffi::c_int + 5 as core::ffi::c_int) as uint32_t,
        );
        re_emit_op(s, REOP_any as core::ffi::c_int);
        re_emit_op_u32(
            s,
            REOP_goto as core::ffi::c_int,
            -(5 as core::ffi::c_int + 1 as core::ffi::c_int + 5 as core::ffi::c_int)
                as uint32_t,
        );
    }
    re_emit_op_u8(s, REOP_save_start as core::ffi::c_int, 0 as uint32_t);
    if !(re_parse_disjunction(s, FALSE as core::ffi::c_int as BOOL) != 0) {
        re_emit_op_u8(s, REOP_save_end as core::ffi::c_int, 0 as uint32_t);
        re_emit_op(s, REOP_match as core::ffi::c_int);
        if *(*s).buf_ptr as core::ffi::c_int != '\0' as i32 {
            re_parse_error(
                s,
                b"extraneous characters at the end\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else if dbuf_error(&mut (*s).byte_code) != 0 {
            re_parse_out_of_memory(s);
        } else {
            register_count = compute_register_count(
                (*s).byte_code.buf,
                (*s).byte_code.size as core::ffi::c_int,
            );
            if register_count < 0 as core::ffi::c_int {
                re_parse_error(
                    s,
                    b"too many imbricated quantifiers\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {
                *((*s).byte_code.buf).offset(RE_HEADER_CAPTURE_COUNT as isize) = (*s)
                    .capture_count as uint8_t;
                *((*s).byte_code.buf).offset(RE_HEADER_REGISTER_COUNT as isize) = register_count
                    as uint8_t;
                put_u32(
                    ((*s).byte_code.buf).offset(RE_HEADER_BYTECODE_LEN as isize),
                    ((*s).byte_code.size).wrapping_sub(RE_HEADER_LEN as size_t)
                        as uint32_t,
                );
                if (*s).group_names.size
                    > (((*s).capture_count - 1 as core::ffi::c_int)
                        * LRE_GROUP_NAME_TRAILER_LEN) as size_t
                {
                    dbuf_put(
                        &mut (*s).byte_code,
                        (*s).group_names.buf,
                        (*s).group_names.size,
                    );
                    put_u16(
                        ((*s).byte_code.buf).offset(RE_HEADER_FLAGS as isize),
                        (lre_get_flags((*s).byte_code.buf) | LRE_FLAG_NAMED_GROUPS)
                            as uint16_t,
                    );
                }
                dbuf_free(&mut (*s).group_names);
                *error_msg.offset(0 as core::ffi::c_int as isize) = '\0' as i32
                    as core::ffi::c_char;
                *plen = (*s).byte_code.size as core::ffi::c_int;
                return (*s).byte_code.buf;
            }
        }
    }
    dbuf_free(&mut (*s).byte_code);
    dbuf_free(&mut (*s).group_names);
    pstrcpy(error_msg, error_msg_size, ((*s).u.error_msg).as_mut_ptr());
    *plen = 0 as core::ffi::c_int;
    return 0 as *mut uint8_t;
}
unsafe extern "C" fn is_line_terminator(mut c: uint32_t) -> BOOL {
    return (c == '\n' as i32 as uint32_t || c == '\r' as i32 as uint32_t
        || c == CP_LS as uint32_t || c == CP_PS as uint32_t) as core::ffi::c_int;
}
unsafe extern "C" fn lre_poll_timeout(mut s: *mut REExecContext) -> core::ffi::c_int {
    (*s).interrupt_counter -= 1;
    if ((*s).interrupt_counter <= 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        (*s).interrupt_counter = INTERRUPT_COUNTER_INIT;
        if lre_check_timeout((*s).opaque) != 0 {
            return LRE_RET_TIMEOUT;
        }
    }
    return 0 as core::ffi::c_int;
}
#[inline(never)]
unsafe extern "C" fn stack_realloc(
    mut s: *mut REExecContext,
    mut n: size_t,
) -> core::ffi::c_int {
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
            return -(1 as core::ffi::c_int);
        }
        memcpy(
            new_stack as *mut core::ffi::c_void,
            (*s).stack_buf as *const core::ffi::c_void,
            ((*s).stack_size).wrapping_mul(::core::mem::size_of::<StackElem>() as size_t),
        );
    } else {
        new_stack = lre_realloc(
            (*s).opaque,
            (*s).stack_buf as *mut core::ffi::c_void,
            new_size.wrapping_mul(::core::mem::size_of::<StackElem>() as size_t),
        ) as *mut StackElem;
        if new_stack.is_null() {
            return -(1 as core::ffi::c_int);
        }
    }
    (*s).stack_size = new_size;
    (*s).stack_buf = new_stack;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn lre_exec_backtrack(
    mut s: *mut REExecContext,
    mut capture: *mut *mut uint8_t,
    mut pc: *const uint8_t,
    mut cptr: *const uint8_t,
) -> intptr_t {
    let mut current_block: u64;
    let mut opcode: core::ffi::c_int = 0;
    let mut cbuf_type: core::ffi::c_int = 0;
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
        opcode = *fresh43 as core::ffi::c_int;
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
                    pc = (*sp.offset(-(3 as core::ffi::c_int) as isize)).ptr;
                    cptr = (*sp.offset(-(2 as core::ffi::c_int) as isize)).ptr;
                    type_1 = ((*sp.offset(-(1 as core::ffi::c_int) as isize)).bp)
                        .type_0() as REExecStateEnum;
                    bp = ((*s).stack_buf)
                        .offset(
                            ((*sp.offset(-(1 as core::ffi::c_int) as isize)).bp).val()
                                as isize,
                        );
                    let ref mut fresh45 = (*sp.offset(-(1 as core::ffi::c_int) as isize))
                        .ptr;
                    *fresh45 = sp1 as *mut core::ffi::c_void as *mut uint8_t;
                    sp = sp.offset(-(3 as core::ffi::c_int as isize));
                    if type_1 as core::ffi::c_uint
                        == RE_EXEC_STATE_LOOKAHEAD as core::ffi::c_int
                            as core::ffi::c_uint
                    {
                        break;
                    }
                }
                if sp != (*s).stack_buf {
                    sp1 = sp;
                    while sp1 < sp_top {
                        next_sp = (*sp1.offset(2 as core::ffi::c_int as isize)).ptr
                            as *mut core::ffi::c_void as *mut StackElem;
                        sp1 = sp1.offset(3 as core::ffi::c_int as isize);
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
                    type_2 = ((*bp.offset(-(1 as core::ffi::c_int) as isize)).bp)
                        .type_0() as REExecStateEnum;
                    while sp > bp {
                        let ref mut fresh48 = *capture
                            .offset(
                                (*sp.offset(-(2 as core::ffi::c_int) as isize)).val as isize,
                            );
                        *fresh48 = (*sp.offset(-(1 as core::ffi::c_int) as isize)).ptr;
                        sp = sp.offset(-(2 as core::ffi::c_int as isize));
                    }
                    pc = (*sp.offset(-(3 as core::ffi::c_int) as isize)).ptr;
                    cptr = (*sp.offset(-(2 as core::ffi::c_int) as isize)).ptr;
                    type_2 = ((*sp.offset(-(1 as core::ffi::c_int) as isize)).bp)
                        .type_0() as REExecStateEnum;
                    bp = ((*s).stack_buf)
                        .offset(
                            ((*sp.offset(-(1 as core::ffi::c_int) as isize)).bp).val()
                                as isize,
                        );
                    sp = sp.offset(-(3 as core::ffi::c_int as isize));
                    if type_2 as core::ffi::c_uint
                        == RE_EXEC_STATE_NEGATIVE_LOOKAHEAD as core::ffi::c_int
                            as core::ffi::c_uint
                    {
                        break;
                    }
                }
                current_block = 1885734024781174349;
            }
            3 | 4 => {
                val = get_u32(pc);
                pc = pc.offset(4 as core::ffi::c_int as isize);
                current_block = 13538245850655155981;
            }
            1 | 2 => {
                val = get_u16(pc);
                pc = pc.offset(2 as core::ffi::c_int as isize);
                current_block = 13538245850655155981;
            }
            14 | 15 => {
                let mut pc1: *const uint8_t = 0 as *const uint8_t;
                val = get_u32(pc);
                pc = pc.offset(4 as core::ffi::c_int as isize);
                if opcode == REOP_split_next_first as core::ffi::c_int {
                    pc1 = pc.offset(val as core::ffi::c_int as isize);
                } else {
                    pc1 = pc;
                    pc = pc.offset(val as core::ffi::c_int as isize);
                }
                if ((stack_end.offset_from(sp) as core::ffi::c_long)
                    < 3 as core::ffi::c_long) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    let mut saved_sp: size_t = sp.offset_from((*s).stack_buf)
                        as core::ffi::c_long as size_t;
                    let mut saved_bp: size_t = bp.offset_from((*s).stack_buf)
                        as core::ffi::c_long as size_t;
                    if stack_realloc(
                        s,
                        (sp.offset_from((*s).stack_buf) as core::ffi::c_long
                            + 3 as core::ffi::c_long) as size_t,
                    ) != 0
                    {
                        return LRE_RET_MEMORY_ERROR as intptr_t;
                    }
                    stack_end = ((*s).stack_buf).offset((*s).stack_size as isize);
                    sp = ((*s).stack_buf).offset(saved_sp as isize);
                    bp = ((*s).stack_buf).offset(saved_bp as isize);
                }
                let ref mut fresh52 = (*sp.offset(0 as core::ffi::c_int as isize)).ptr;
                *fresh52 = pc1 as *mut uint8_t;
                let ref mut fresh53 = (*sp.offset(1 as core::ffi::c_int as isize)).ptr;
                *fresh53 = cptr as *mut uint8_t;
                let ref mut fresh54 = (*sp.offset(2 as core::ffi::c_int as isize)).bp;
                (*fresh54)
                    .set_val(
                        bp.offset_from((*s).stack_buf) as core::ffi::c_long as u64,
                    );
                let ref mut fresh55 = (*sp.offset(2 as core::ffi::c_int as isize)).bp;
                (*fresh55)
                    .set_type_0(
                        RE_EXEC_STATE_SPLIT as core::ffi::c_int as u64,
                    );
                sp = sp.offset(3 as core::ffi::c_int as isize);
                bp = sp;
                continue;
            }
            40 | 41 => {
                val = get_u32(pc);
                pc = pc.offset(4 as core::ffi::c_int as isize);
                if ((stack_end.offset_from(sp) as core::ffi::c_long)
                    < 3 as core::ffi::c_long) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    let mut saved_sp_0: size_t = sp.offset_from((*s).stack_buf)
                        as core::ffi::c_long as size_t;
                    let mut saved_bp_0: size_t = bp.offset_from((*s).stack_buf)
                        as core::ffi::c_long as size_t;
                    if stack_realloc(
                        s,
                        (sp.offset_from((*s).stack_buf) as core::ffi::c_long
                            + 3 as core::ffi::c_long) as size_t,
                    ) != 0
                    {
                        return LRE_RET_MEMORY_ERROR as intptr_t;
                    }
                    stack_end = ((*s).stack_buf).offset((*s).stack_size as isize);
                    sp = ((*s).stack_buf).offset(saved_sp_0 as isize);
                    bp = ((*s).stack_buf).offset(saved_bp_0 as isize);
                }
                let ref mut fresh56 = (*sp.offset(0 as core::ffi::c_int as isize)).ptr;
                *fresh56 = pc.offset(val as core::ffi::c_int as isize) as *mut uint8_t;
                let ref mut fresh57 = (*sp.offset(1 as core::ffi::c_int as isize)).ptr;
                *fresh57 = cptr as *mut uint8_t;
                let ref mut fresh58 = (*sp.offset(2 as core::ffi::c_int as isize)).bp;
                (*fresh58)
                    .set_val(
                        bp.offset_from((*s).stack_buf) as core::ffi::c_long as u64,
                    );
                let ref mut fresh59 = (*sp.offset(2 as core::ffi::c_int as isize)).bp;
                (*fresh59)
                    .set_type_0(
                        (RE_EXEC_STATE_LOOKAHEAD as core::ffi::c_int + opcode
                            - REOP_lookahead as core::ffi::c_int) as u64,
                    );
                sp = sp.offset(3 as core::ffi::c_int as isize);
                bp = sp;
                continue;
            }
            13 => {
                val = get_u32(pc);
                pc = pc
                    .offset((4 as core::ffi::c_int + val as core::ffi::c_int) as isize);
                if lre_poll_timeout(s) != 0 {
                    return LRE_RET_TIMEOUT as intptr_t;
                }
                continue;
            }
            9 | 10 => {
                if cptr == (*s).cbuf {
                    continue;
                }
                if opcode == REOP_line_start as core::ffi::c_int {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as core::ffi::c_int {
                        c = *cptr.offset(-(1 as core::ffi::c_int) as isize) as uint32_t;
                    } else {
                        let mut _p_0: *const uint16_t = (cptr as *const uint16_t)
                            .offset(-(1 as core::ffi::c_int as isize));
                        let mut _start: *const uint16_t = (*s).cbuf as *const uint16_t;
                        c = *_p_0 as uint32_t;
                        if is_lo_surrogate(c) != 0 && cbuf_type == 2 as core::ffi::c_int
                        {
                            if _p_0 > _start
                                && is_hi_surrogate(
                                    *_p_0.offset(-(1 as core::ffi::c_int) as isize) as uint32_t,
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
                if opcode == REOP_line_end as core::ffi::c_int {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as core::ffi::c_int {
                        c = *cptr.offset(0 as core::ffi::c_int as isize) as uint32_t;
                    } else {
                        let mut _p_1: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_0: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh60 = _p_1;
                        _p_1 = _p_1.offset(1);
                        c = *fresh60 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as core::ffi::c_int
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
                    if cbuf_type == 0 as core::ffi::c_int {
                        let fresh61 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh61 as uint32_t;
                    } else {
                        let mut _p_2: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_1: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh62 = _p_2;
                        _p_2 = _p_2.offset(1);
                        c = *fresh62 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as core::ffi::c_int
                        {
                            if _p_2 < _end_1 && is_lo_surrogate(*_p_2 as uint32_t) != 0 {
                                let fresh63 = _p_2;
                                _p_2 = _p_2.offset(1);
                                c = from_surrogate(c, *fresh63 as uint32_t);
                            }
                        }
                        cptr = _p_2 as *const core::ffi::c_void as *const uint8_t;
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
                    if cbuf_type == 0 as core::ffi::c_int {
                        let fresh64 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh64 as uint32_t;
                    } else {
                        let mut _p_3: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_2: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh65 = _p_3;
                        _p_3 = _p_3.offset(1);
                        c = *fresh65 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as core::ffi::c_int
                        {
                            if _p_3 < _end_2 && is_lo_surrogate(*_p_3 as uint32_t) != 0 {
                                let fresh66 = _p_3;
                                _p_3 = _p_3.offset(1);
                                c = from_surrogate(c, *fresh66 as uint32_t);
                            }
                        }
                        cptr = _p_3 as *const core::ffi::c_void as *const uint8_t;
                    }
                    continue;
                }
            }
            7 => {
                if cptr == cbuf_end {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as core::ffi::c_int {
                        let fresh67 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh67 as uint32_t;
                    } else {
                        let mut _p_4: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_3: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh68 = _p_4;
                        _p_4 = _p_4.offset(1);
                        c = *fresh68 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as core::ffi::c_int
                        {
                            if _p_4 < _end_3 && is_lo_surrogate(*_p_4 as uint32_t) != 0 {
                                let fresh69 = _p_4;
                                _p_4 = _p_4.offset(1);
                                c = from_surrogate(c, *fresh69 as uint32_t);
                            }
                        }
                        cptr = _p_4 as *const core::ffi::c_void as *const uint8_t;
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
                    if cbuf_type == 0 as core::ffi::c_int {
                        let fresh70 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh70 as uint32_t;
                    } else {
                        let mut _p_5: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_4: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh71 = _p_5;
                        _p_5 = _p_5.offset(1);
                        c = *fresh71 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as core::ffi::c_int
                        {
                            if _p_5 < _end_4 && is_lo_surrogate(*_p_5 as uint32_t) != 0 {
                                let fresh72 = _p_5;
                                _p_5 = _p_5.offset(1);
                                c = from_surrogate(c, *fresh72 as uint32_t);
                            }
                        }
                        cptr = _p_5 as *const core::ffi::c_void as *const uint8_t;
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
                if val < (*s).capture_count as uint32_t {} else {
                    __assert_fail(
                        b"val < s->capture_count\0" as *const u8
                            as *const core::ffi::c_char,
                        b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                        3034 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 91],
                            [core::ffi::c_char; 91],
                        >(
                            *b"intptr_t lre_exec_backtrack(REExecContext *, uint8_t **, const uint8_t *, const uint8_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_20001: {
                    if val < (*s).capture_count as uint32_t {} else {
                        __assert_fail(
                            b"val < s->capture_count\0" as *const u8
                                as *const core::ffi::c_char,
                            b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                            3034 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 91],
                                [core::ffi::c_char; 91],
                            >(
                                *b"intptr_t lre_exec_backtrack(REExecContext *, uint8_t **, const uint8_t *, const uint8_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                idx = (2 as uint32_t)
                    .wrapping_mul(val)
                    .wrapping_add(opcode as uint32_t)
                    .wrapping_sub(REOP_save_start as core::ffi::c_int as uint32_t);
                if ((stack_end.offset_from(sp) as core::ffi::c_long)
                    < 2 as core::ffi::c_long) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    let mut saved_sp_1: size_t = sp.offset_from((*s).stack_buf)
                        as core::ffi::c_long as size_t;
                    let mut saved_bp_1: size_t = bp.offset_from((*s).stack_buf)
                        as core::ffi::c_long as size_t;
                    if stack_realloc(
                        s,
                        (sp.offset_from((*s).stack_buf) as core::ffi::c_long
                            + 2 as core::ffi::c_long) as size_t,
                    ) != 0
                    {
                        return LRE_RET_MEMORY_ERROR as intptr_t;
                    }
                    stack_end = ((*s).stack_buf).offset((*s).stack_size as isize);
                    sp = ((*s).stack_buf).offset(saved_sp_1 as isize);
                    bp = ((*s).stack_buf).offset(saved_bp_1 as isize);
                }
                (*sp.offset(0 as core::ffi::c_int as isize)).val = idx as intptr_t;
                let ref mut fresh74 = (*sp.offset(1 as core::ffi::c_int as isize)).ptr;
                *fresh74 = *capture.offset(idx as isize);
                sp = sp.offset(2 as core::ffi::c_int as isize);
                let ref mut fresh75 = *capture.offset(idx as isize);
                *fresh75 = cptr as *mut uint8_t;
                continue;
            }
            21 => {
                let mut val2: uint32_t = 0;
                val = *pc.offset(0 as core::ffi::c_int as isize) as uint32_t;
                val2 = *pc.offset(1 as core::ffi::c_int as isize) as uint32_t;
                pc = pc.offset(2 as core::ffi::c_int as isize);
                if val2 < (*s).capture_count as uint32_t {} else {
                    __assert_fail(
                        b"val2 < s->capture_count\0" as *const u8
                            as *const core::ffi::c_char,
                        b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                        3044 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 91],
                            [core::ffi::c_char; 91],
                        >(
                            *b"intptr_t lre_exec_backtrack(REExecContext *, uint8_t **, const uint8_t *, const uint8_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_19786: {
                    if val2 < (*s).capture_count as uint32_t {} else {
                        __assert_fail(
                            b"val2 < s->capture_count\0" as *const u8
                                as *const core::ffi::c_char,
                            b"libregexp.c\0" as *const u8 as *const core::ffi::c_char,
                            3044 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 91],
                                [core::ffi::c_char; 91],
                            >(
                                *b"intptr_t lre_exec_backtrack(REExecContext *, uint8_t **, const uint8_t *, const uint8_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if ((stack_end.offset_from(sp) as core::ffi::c_long)
                    < (2 as uint32_t)
                        .wrapping_mul(val2.wrapping_sub(val).wrapping_add(1 as uint32_t))
                        as core::ffi::c_long) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    let mut saved_sp_2: size_t = sp.offset_from((*s).stack_buf)
                        as core::ffi::c_long as size_t;
                    let mut saved_bp_2: size_t = bp.offset_from((*s).stack_buf)
                        as core::ffi::c_long as size_t;
                    if stack_realloc(
                        s,
                        (sp.offset_from((*s).stack_buf) as core::ffi::c_long
                            + (2 as uint32_t)
                                .wrapping_mul(
                                    val2.wrapping_sub(val).wrapping_add(1 as uint32_t),
                                ) as core::ffi::c_long) as size_t,
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
                    if ((stack_end.offset_from(sp) as core::ffi::c_long)
                        < 2 as core::ffi::c_long) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        let mut saved_sp_3: size_t = sp.offset_from((*s).stack_buf)
                            as core::ffi::c_long as size_t;
                        let mut saved_bp_3: size_t = bp.offset_from((*s).stack_buf)
                            as core::ffi::c_long as size_t;
                        if stack_realloc(
                            s,
                            (sp.offset_from((*s).stack_buf) as core::ffi::c_long
                                + 2 as core::ffi::c_long) as size_t,
                        ) != 0
                        {
                            return LRE_RET_MEMORY_ERROR as intptr_t;
                        }
                        stack_end = ((*s).stack_buf).offset((*s).stack_size as isize);
                        sp = ((*s).stack_buf).offset(saved_sp_3 as isize);
                        bp = ((*s).stack_buf).offset(saved_bp_3 as isize);
                    }
                    (*sp.offset(0 as core::ffi::c_int as isize)).val = idx as intptr_t;
                    let ref mut fresh76 = (*sp.offset(1 as core::ffi::c_int as isize))
                        .ptr;
                    *fresh76 = *capture.offset(idx as isize);
                    sp = sp.offset(2 as core::ffi::c_int as isize);
                    let ref mut fresh77 = *capture.offset(idx as isize);
                    *fresh77 = 0 as *mut uint8_t;
                    idx = (2 as uint32_t).wrapping_mul(val).wrapping_add(1 as uint32_t);
                    if ((stack_end.offset_from(sp) as core::ffi::c_long)
                        < 2 as core::ffi::c_long) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        let mut saved_sp_4: size_t = sp.offset_from((*s).stack_buf)
                            as core::ffi::c_long as size_t;
                        let mut saved_bp_4: size_t = bp.offset_from((*s).stack_buf)
                            as core::ffi::c_long as size_t;
                        if stack_realloc(
                            s,
                            (sp.offset_from((*s).stack_buf) as core::ffi::c_long
                                + 2 as core::ffi::c_long) as size_t,
                        ) != 0
                        {
                            return LRE_RET_MEMORY_ERROR as intptr_t;
                        }
                        stack_end = ((*s).stack_buf).offset((*s).stack_size as isize);
                        sp = ((*s).stack_buf).offset(saved_sp_4 as isize);
                        bp = ((*s).stack_buf).offset(saved_bp_4 as isize);
                    }
                    (*sp.offset(0 as core::ffi::c_int as isize)).val = idx as intptr_t;
                    let ref mut fresh78 = (*sp.offset(1 as core::ffi::c_int as isize))
                        .ptr;
                    *fresh78 = *capture.offset(idx as isize);
                    sp = sp.offset(2 as core::ffi::c_int as isize);
                    let ref mut fresh79 = *capture.offset(idx as isize);
                    *fresh79 = 0 as *mut uint8_t;
                    val = val.wrapping_add(1);
                }
                continue;
            }
            27 => {
                idx = (2 as core::ffi::c_int * (*s).capture_count
                    + *pc.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
                    as uint32_t;
                val = get_u32(pc.offset(1 as core::ffi::c_int as isize));
                pc = pc.offset(5 as core::ffi::c_int as isize);
                let mut sp1_0: *mut StackElem = 0 as *mut StackElem;
                sp1_0 = sp;
                loop {
                    if sp1_0 > bp {
                        if (*sp1_0.offset(-(2 as core::ffi::c_int) as isize)).val
                            == idx as intptr_t
                        {
                            break;
                        }
                        sp1_0 = sp1_0.offset(-(2 as core::ffi::c_int as isize));
                    } else {
                        if ((stack_end.offset_from(sp) as core::ffi::c_long)
                            < 2 as core::ffi::c_long) as core::ffi::c_int
                            as core::ffi::c_long != 0
                        {
                            let mut saved_sp_5: size_t = sp.offset_from((*s).stack_buf)
                                as core::ffi::c_long as size_t;
                            let mut saved_bp_5: size_t = bp.offset_from((*s).stack_buf)
                                as core::ffi::c_long as size_t;
                            if stack_realloc(
                                s,
                                (sp.offset_from((*s).stack_buf) as core::ffi::c_long
                                    + 2 as core::ffi::c_long) as size_t,
                            ) != 0
                            {
                                return LRE_RET_MEMORY_ERROR as intptr_t;
                            }
                            stack_end = ((*s).stack_buf)
                                .offset((*s).stack_size as isize);
                            sp = ((*s).stack_buf).offset(saved_sp_5 as isize);
                            bp = ((*s).stack_buf).offset(saved_bp_5 as isize);
                        }
                        (*sp.offset(0 as core::ffi::c_int as isize)).val = idx
                            as intptr_t;
                        let ref mut fresh80 = (*sp
                            .offset(1 as core::ffi::c_int as isize))
                            .ptr;
                        *fresh80 = *capture.offset(idx as isize);
                        sp = sp.offset(2 as core::ffi::c_int as isize);
                        break;
                    }
                }
                let ref mut fresh81 = *capture.offset(idx as isize);
                *fresh81 = val as uintptr_t as *mut core::ffi::c_void as *mut uint8_t;
                continue;
            }
            22 => {
                let mut val2_0: uint32_t = 0;
                idx = (2 as core::ffi::c_int * (*s).capture_count
                    + *pc.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
                    as uint32_t;
                val = get_u32(pc.offset(1 as core::ffi::c_int as isize));
                pc = pc.offset(5 as core::ffi::c_int as isize);
                val2_0 = (*capture.offset(idx as isize) as uintptr_t)
                    .wrapping_sub(1 as uintptr_t) as uint32_t;
                let mut sp1_1: *mut StackElem = 0 as *mut StackElem;
                sp1_1 = sp;
                loop {
                    if sp1_1 > bp {
                        if (*sp1_1.offset(-(2 as core::ffi::c_int) as isize)).val
                            == idx as intptr_t
                        {
                            break;
                        }
                        sp1_1 = sp1_1.offset(-(2 as core::ffi::c_int as isize));
                    } else {
                        if ((stack_end.offset_from(sp) as core::ffi::c_long)
                            < 2 as core::ffi::c_long) as core::ffi::c_int
                            as core::ffi::c_long != 0
                        {
                            let mut saved_sp_6: size_t = sp.offset_from((*s).stack_buf)
                                as core::ffi::c_long as size_t;
                            let mut saved_bp_6: size_t = bp.offset_from((*s).stack_buf)
                                as core::ffi::c_long as size_t;
                            if stack_realloc(
                                s,
                                (sp.offset_from((*s).stack_buf) as core::ffi::c_long
                                    + 2 as core::ffi::c_long) as size_t,
                            ) != 0
                            {
                                return LRE_RET_MEMORY_ERROR as intptr_t;
                            }
                            stack_end = ((*s).stack_buf)
                                .offset((*s).stack_size as isize);
                            sp = ((*s).stack_buf).offset(saved_sp_6 as isize);
                            bp = ((*s).stack_buf).offset(saved_bp_6 as isize);
                        }
                        (*sp.offset(0 as core::ffi::c_int as isize)).val = idx
                            as intptr_t;
                        let ref mut fresh82 = (*sp
                            .offset(1 as core::ffi::c_int as isize))
                            .ptr;
                        *fresh82 = *capture.offset(idx as isize);
                        sp = sp.offset(2 as core::ffi::c_int as isize);
                        break;
                    }
                }
                let ref mut fresh83 = *capture.offset(idx as isize);
                *fresh83 = val2_0 as uintptr_t as *mut core::ffi::c_void as *mut uint8_t;
                if val2_0 != 0 as uint32_t {
                    pc = pc.offset(val as core::ffi::c_int as isize);
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
                idx = (2 as core::ffi::c_int * (*s).capture_count
                    + *pc.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
                    as uint32_t;
                limit = get_u32(pc.offset(1 as core::ffi::c_int as isize));
                val = get_u32(pc.offset(5 as core::ffi::c_int as isize));
                pc = pc.offset(9 as core::ffi::c_int as isize);
                val2_1 = (*capture.offset(idx as isize) as uintptr_t)
                    .wrapping_sub(1 as uintptr_t) as uint32_t;
                let mut sp1_2: *mut StackElem = 0 as *mut StackElem;
                sp1_2 = sp;
                loop {
                    if sp1_2 > bp {
                        if (*sp1_2.offset(-(2 as core::ffi::c_int) as isize)).val
                            == idx as intptr_t
                        {
                            break;
                        }
                        sp1_2 = sp1_2.offset(-(2 as core::ffi::c_int as isize));
                    } else {
                        if ((stack_end.offset_from(sp) as core::ffi::c_long)
                            < 2 as core::ffi::c_long) as core::ffi::c_int
                            as core::ffi::c_long != 0
                        {
                            let mut saved_sp_7: size_t = sp.offset_from((*s).stack_buf)
                                as core::ffi::c_long as size_t;
                            let mut saved_bp_7: size_t = bp.offset_from((*s).stack_buf)
                                as core::ffi::c_long as size_t;
                            if stack_realloc(
                                s,
                                (sp.offset_from((*s).stack_buf) as core::ffi::c_long
                                    + 2 as core::ffi::c_long) as size_t,
                            ) != 0
                            {
                                return LRE_RET_MEMORY_ERROR as intptr_t;
                            }
                            stack_end = ((*s).stack_buf)
                                .offset((*s).stack_size as isize);
                            sp = ((*s).stack_buf).offset(saved_sp_7 as isize);
                            bp = ((*s).stack_buf).offset(saved_bp_7 as isize);
                        }
                        (*sp.offset(0 as core::ffi::c_int as isize)).val = idx
                            as intptr_t;
                        let ref mut fresh84 = (*sp
                            .offset(1 as core::ffi::c_int as isize))
                            .ptr;
                        *fresh84 = *capture.offset(idx as isize);
                        sp = sp.offset(2 as core::ffi::c_int as isize);
                        break;
                    }
                }
                let ref mut fresh85 = *capture.offset(idx as isize);
                *fresh85 = val2_1 as uintptr_t as *mut core::ffi::c_void as *mut uint8_t;
                if val2_1 > limit {
                    pc = pc.offset(val as core::ffi::c_int as isize);
                    if lre_poll_timeout(s) != 0 {
                        return LRE_RET_TIMEOUT as intptr_t;
                    }
                    continue;
                } else if !((opcode
                    == REOP_loop_check_adv_split_goto_first as core::ffi::c_int
                    || opcode
                        == REOP_loop_check_adv_split_next_first as core::ffi::c_int)
                    && *capture.offset(idx.wrapping_add(1 as uint32_t) as isize)
                        == cptr as *mut uint8_t && val2_1 != limit)
                {
                    if val2_1 != 0 as uint32_t {
                        if opcode == REOP_loop_split_next_first as core::ffi::c_int
                            || opcode
                                == REOP_loop_check_adv_split_next_first as core::ffi::c_int
                        {
                            pc1_0 = pc.offset(val as core::ffi::c_int as isize);
                        } else {
                            pc1_0 = pc;
                            pc = pc.offset(val as core::ffi::c_int as isize);
                        }
                        if ((stack_end.offset_from(sp) as core::ffi::c_long)
                            < 3 as core::ffi::c_long) as core::ffi::c_int
                            as core::ffi::c_long != 0
                        {
                            let mut saved_sp_8: size_t = sp.offset_from((*s).stack_buf)
                                as core::ffi::c_long as size_t;
                            let mut saved_bp_8: size_t = bp.offset_from((*s).stack_buf)
                                as core::ffi::c_long as size_t;
                            if stack_realloc(
                                s,
                                (sp.offset_from((*s).stack_buf) as core::ffi::c_long
                                    + 3 as core::ffi::c_long) as size_t,
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
                            .offset(0 as core::ffi::c_int as isize))
                            .ptr;
                        *fresh86 = pc1_0 as *mut uint8_t;
                        let ref mut fresh87 = (*sp
                            .offset(1 as core::ffi::c_int as isize))
                            .ptr;
                        *fresh87 = cptr as *mut uint8_t;
                        let ref mut fresh88 = (*sp
                            .offset(2 as core::ffi::c_int as isize))
                            .bp;
                        (*fresh88)
                            .set_val(
                                bp.offset_from((*s).stack_buf) as core::ffi::c_long as u64,
                            );
                        let ref mut fresh89 = (*sp
                            .offset(2 as core::ffi::c_int as isize))
                            .bp;
                        (*fresh89)
                            .set_type_0(
                                RE_EXEC_STATE_SPLIT as core::ffi::c_int as u64,
                            );
                        sp = sp.offset(3 as core::ffi::c_int as isize);
                        bp = sp;
                    }
                    continue;
                }
                current_block = 1885734024781174349;
            }
            42 => {
                idx = (2 as core::ffi::c_int * (*s).capture_count
                    + *pc.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
                    as uint32_t;
                pc = pc.offset(1);
                let mut sp1_3: *mut StackElem = 0 as *mut StackElem;
                sp1_3 = sp;
                loop {
                    if sp1_3 > bp {
                        if (*sp1_3.offset(-(2 as core::ffi::c_int) as isize)).val
                            == idx as intptr_t
                        {
                            break;
                        }
                        sp1_3 = sp1_3.offset(-(2 as core::ffi::c_int as isize));
                    } else {
                        if ((stack_end.offset_from(sp) as core::ffi::c_long)
                            < 2 as core::ffi::c_long) as core::ffi::c_int
                            as core::ffi::c_long != 0
                        {
                            let mut saved_sp_9: size_t = sp.offset_from((*s).stack_buf)
                                as core::ffi::c_long as size_t;
                            let mut saved_bp_9: size_t = bp.offset_from((*s).stack_buf)
                                as core::ffi::c_long as size_t;
                            if stack_realloc(
                                s,
                                (sp.offset_from((*s).stack_buf) as core::ffi::c_long
                                    + 2 as core::ffi::c_long) as size_t,
                            ) != 0
                            {
                                return LRE_RET_MEMORY_ERROR as intptr_t;
                            }
                            stack_end = ((*s).stack_buf)
                                .offset((*s).stack_size as isize);
                            sp = ((*s).stack_buf).offset(saved_sp_9 as isize);
                            bp = ((*s).stack_buf).offset(saved_bp_9 as isize);
                        }
                        (*sp.offset(0 as core::ffi::c_int as isize)).val = idx
                            as intptr_t;
                        let ref mut fresh90 = (*sp
                            .offset(1 as core::ffi::c_int as isize))
                            .ptr;
                        *fresh90 = *capture.offset(idx as isize);
                        sp = sp.offset(2 as core::ffi::c_int as isize);
                        break;
                    }
                }
                let ref mut fresh91 = *capture.offset(idx as isize);
                *fresh91 = cptr as *mut uint8_t;
                continue;
            }
            43 => {
                idx = (2 as core::ffi::c_int * (*s).capture_count
                    + *pc.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
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
                let mut ignore_case: core::ffi::c_int = (opcode
                    == REOP_word_boundary_i as core::ffi::c_int
                    || opcode == REOP_not_word_boundary_i as core::ffi::c_int)
                    as core::ffi::c_int;
                let mut is_boundary: BOOL = (opcode
                    == REOP_word_boundary as core::ffi::c_int
                    || opcode == REOP_word_boundary_i as core::ffi::c_int)
                    as core::ffi::c_int;
                if cptr == (*s).cbuf {
                    v1 = FALSE as core::ffi::c_int as BOOL;
                } else {
                    if cbuf_type == 0 as core::ffi::c_int {
                        c = *cptr.offset(-(1 as core::ffi::c_int) as isize) as uint32_t;
                    } else {
                        let mut _p_6: *const uint16_t = (cptr as *const uint16_t)
                            .offset(-(1 as core::ffi::c_int as isize));
                        let mut _start_0: *const uint16_t = (*s).cbuf as *const uint16_t;
                        c = *_p_6 as uint32_t;
                        if is_lo_surrogate(c) != 0 && cbuf_type == 2 as core::ffi::c_int
                        {
                            if _p_6 > _start_0
                                && is_hi_surrogate(
                                    *_p_6.offset(-(1 as core::ffi::c_int) as isize) as uint32_t,
                                ) != 0
                            {
                                _p_6 = _p_6.offset(-1);
                                c = from_surrogate(*_p_6 as uint32_t, c);
                            }
                        }
                    }
                    if c < 256 as uint32_t {
                        v1 = (lre_is_word_byte(c as uint8_t) != 0 as core::ffi::c_int)
                            as core::ffi::c_int as BOOL;
                    } else {
                        v1 = (ignore_case != 0
                            && (c == 0x17f as uint32_t || c == 0x212a as uint32_t))
                            as core::ffi::c_int as BOOL;
                    }
                }
                if cptr >= cbuf_end {
                    v2 = FALSE as core::ffi::c_int as BOOL;
                } else {
                    if cbuf_type == 0 as core::ffi::c_int {
                        c = *cptr.offset(0 as core::ffi::c_int as isize) as uint32_t;
                    } else {
                        let mut _p_7: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_5: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh92 = _p_7;
                        _p_7 = _p_7.offset(1);
                        c = *fresh92 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as core::ffi::c_int
                        {
                            if _p_7 < _end_5 && is_lo_surrogate(*_p_7 as uint32_t) != 0 {
                                c = from_surrogate(c, *_p_7 as uint32_t);
                            }
                        }
                    }
                    if c < 256 as uint32_t {
                        v2 = (lre_is_word_byte(c as uint8_t) != 0 as core::ffi::c_int)
                            as core::ffi::c_int as BOOL;
                    } else {
                        v2 = (ignore_case != 0
                            && (c == 0x17f as uint32_t || c == 0x212a as uint32_t))
                            as core::ffi::c_int as BOOL;
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
                let mut i: core::ffi::c_int = 0;
                let mut n: core::ffi::c_int = 0;
                let fresh93 = pc;
                pc = pc.offset(1);
                n = *fresh93 as core::ffi::c_int;
                pc1_1 = pc;
                pc = pc.offset(n as isize);
                i = 0 as core::ffi::c_int;
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
                        if opcode == REOP_back_reference as core::ffi::c_int
                            || opcode == REOP_back_reference_i as core::ffi::c_int
                        {
                            cptr1 = cptr1_start;
                            loop {
                                if !(cptr1 < cptr1_end) {
                                    continue 's_31;
                                }
                                if cptr >= cbuf_end {
                                    break 's_2002;
                                }
                                if cbuf_type == 0 as core::ffi::c_int {
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
                                        && cbuf_type == 2 as core::ffi::c_int
                                    {
                                        if _p_8 < _end_6 && is_lo_surrogate(*_p_8 as uint32_t) != 0
                                        {
                                            let fresh96 = _p_8;
                                            _p_8 = _p_8.offset(1);
                                            c1 = from_surrogate(c1, *fresh96 as uint32_t);
                                        }
                                    }
                                    cptr1 = _p_8 as *const core::ffi::c_void as *const uint8_t;
                                }
                                if cbuf_type == 0 as core::ffi::c_int {
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
                                        && cbuf_type == 2 as core::ffi::c_int
                                    {
                                        if _p_9 < _end_7 && is_lo_surrogate(*_p_9 as uint32_t) != 0
                                        {
                                            let fresh99 = _p_9;
                                            _p_9 = _p_9.offset(1);
                                            c2 = from_surrogate(c2, *fresh99 as uint32_t);
                                        }
                                    }
                                    cptr = _p_9 as *const core::ffi::c_void as *const uint8_t;
                                }
                                if opcode == REOP_back_reference_i as core::ffi::c_int {
                                    c1 = lre_canonicalize(
                                        c1,
                                        (*s).is_unicode as core::ffi::c_int,
                                    ) as uint32_t;
                                    c2 = lre_canonicalize(
                                        c2,
                                        (*s).is_unicode as core::ffi::c_int,
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
                                if cbuf_type == 0 as core::ffi::c_int {
                                    cptr1 = cptr1.offset(-1);
                                    c1 = *cptr1.offset(0 as core::ffi::c_int as isize)
                                        as uint32_t;
                                } else {
                                    let mut _p_10: *const uint16_t = (cptr1 as *const uint16_t)
                                        .offset(-(1 as core::ffi::c_int as isize));
                                    let mut _start_1: *const uint16_t = cptr1_start
                                        as *const uint16_t;
                                    c1 = *_p_10 as uint32_t;
                                    if is_lo_surrogate(c1) != 0
                                        && cbuf_type == 2 as core::ffi::c_int
                                    {
                                        if _p_10 > _start_1
                                            && is_hi_surrogate(
                                                *_p_10.offset(-(1 as core::ffi::c_int) as isize) as uint32_t,
                                            ) != 0
                                        {
                                            _p_10 = _p_10.offset(-1);
                                            c1 = from_surrogate(*_p_10 as uint32_t, c1);
                                        }
                                    }
                                    cptr1 = _p_10 as *const core::ffi::c_void as *const uint8_t;
                                }
                                if cbuf_type == 0 as core::ffi::c_int {
                                    cptr = cptr.offset(-1);
                                    c2 = *cptr.offset(0 as core::ffi::c_int as isize)
                                        as uint32_t;
                                } else {
                                    let mut _p_11: *const uint16_t = (cptr as *const uint16_t)
                                        .offset(-(1 as core::ffi::c_int as isize));
                                    let mut _start_2: *const uint16_t = (*s).cbuf
                                        as *const uint16_t;
                                    c2 = *_p_11 as uint32_t;
                                    if is_lo_surrogate(c2) != 0
                                        && cbuf_type == 2 as core::ffi::c_int
                                    {
                                        if _p_11 > _start_2
                                            && is_hi_surrogate(
                                                *_p_11.offset(-(1 as core::ffi::c_int) as isize) as uint32_t,
                                            ) != 0
                                        {
                                            _p_11 = _p_11.offset(-1);
                                            c2 = from_surrogate(*_p_11 as uint32_t, c2);
                                        }
                                    }
                                    cptr = _p_11 as *const core::ffi::c_void as *const uint8_t;
                                }
                                if opcode
                                    == REOP_backward_back_reference_i as core::ffi::c_int
                                {
                                    c1 = lre_canonicalize(
                                        c1,
                                        (*s).is_unicode as core::ffi::c_int,
                                    ) as uint32_t;
                                    c2 = lre_canonicalize(
                                        c2,
                                        (*s).is_unicode as core::ffi::c_int,
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
                let mut n_0: core::ffi::c_int = 0;
                let mut low: uint32_t = 0;
                let mut high: uint32_t = 0;
                let mut idx_min: uint32_t = 0;
                let mut idx_max: uint32_t = 0;
                let mut idx_0: uint32_t = 0;
                n_0 = get_u16(pc) as core::ffi::c_int;
                pc = pc.offset(2 as core::ffi::c_int as isize);
                if cptr >= cbuf_end {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as core::ffi::c_int {
                        let fresh100 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh100 as uint32_t;
                    } else {
                        let mut _p_12: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_8: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh101 = _p_12;
                        _p_12 = _p_12.offset(1);
                        c = *fresh101 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as core::ffi::c_int
                        {
                            if _p_12 < _end_8 && is_lo_surrogate(*_p_12 as uint32_t) != 0
                            {
                                let fresh102 = _p_12;
                                _p_12 = _p_12.offset(1);
                                c = from_surrogate(c, *fresh102 as uint32_t);
                            }
                        }
                        cptr = _p_12 as *const core::ffi::c_void as *const uint8_t;
                    }
                    if opcode == REOP_range_i as core::ffi::c_int {
                        c = lre_canonicalize(c, (*s).is_unicode as core::ffi::c_int)
                            as uint32_t;
                    }
                    idx_min = 0 as uint32_t;
                    low = get_u16(
                        pc
                            .offset(
                                (0 as core::ffi::c_int * 4 as core::ffi::c_int) as isize,
                            ),
                    );
                    if c < low {
                        current_block = 1885734024781174349;
                    } else {
                        idx_max = (n_0 - 1 as core::ffi::c_int) as uint32_t;
                        high = get_u16(
                            pc
                                .offset(idx_max.wrapping_mul(4 as uint32_t) as isize)
                                .offset(2 as core::ffi::c_int as isize),
                        );
                        if (c >= 0xffff as uint32_t) as core::ffi::c_int
                            as core::ffi::c_long != 0 && high == 0xffff as uint32_t
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
                                        .offset(2 as core::ffi::c_int as isize),
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
                                pc = pc.offset((4 as core::ffi::c_int * n_0) as isize);
                                continue;
                            }
                        }
                    }
                }
            }
            38 | 39 => {
                let mut n_1: core::ffi::c_int = 0;
                let mut low_0: uint32_t = 0;
                let mut high_0: uint32_t = 0;
                let mut idx_min_0: uint32_t = 0;
                let mut idx_max_0: uint32_t = 0;
                let mut idx_1: uint32_t = 0;
                n_1 = get_u16(pc) as core::ffi::c_int;
                pc = pc.offset(2 as core::ffi::c_int as isize);
                if cptr >= cbuf_end {
                    current_block = 1885734024781174349;
                } else {
                    if cbuf_type == 0 as core::ffi::c_int {
                        let fresh103 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh103 as uint32_t;
                    } else {
                        let mut _p_13: *const uint16_t = cptr as *const uint16_t;
                        let mut _end_9: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh104 = _p_13;
                        _p_13 = _p_13.offset(1);
                        c = *fresh104 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as core::ffi::c_int
                        {
                            if _p_13 < _end_9 && is_lo_surrogate(*_p_13 as uint32_t) != 0
                            {
                                let fresh105 = _p_13;
                                _p_13 = _p_13.offset(1);
                                c = from_surrogate(c, *fresh105 as uint32_t);
                            }
                        }
                        cptr = _p_13 as *const core::ffi::c_void as *const uint8_t;
                    }
                    if opcode == REOP_range32_i as core::ffi::c_int {
                        c = lre_canonicalize(c, (*s).is_unicode as core::ffi::c_int)
                            as uint32_t;
                    }
                    idx_min_0 = 0 as uint32_t;
                    low_0 = get_u32(
                        pc
                            .offset(
                                (0 as core::ffi::c_int * 8 as core::ffi::c_int) as isize,
                            ),
                    );
                    if c < low_0 {
                        current_block = 1885734024781174349;
                    } else {
                        idx_max_0 = (n_1 - 1 as core::ffi::c_int) as uint32_t;
                        high_0 = get_u32(
                            pc
                                .offset(idx_max_0.wrapping_mul(8 as uint32_t) as isize)
                                .offset(4 as core::ffi::c_int as isize),
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
                                        .offset(4 as core::ffi::c_int as isize),
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
                                    pc = pc.offset((8 as core::ffi::c_int * n_1) as isize);
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
                    if cbuf_type == 0 as core::ffi::c_int {
                        cptr = cptr.offset(-1);
                    } else {
                        let mut _p_14: *const uint16_t = (cptr as *const uint16_t)
                            .offset(-(1 as core::ffi::c_int as isize));
                        let mut _start_3: *const uint16_t = (*s).cbuf as *const uint16_t;
                        if is_lo_surrogate(*_p_14 as uint32_t) != 0
                            && cbuf_type == 2 as core::ffi::c_int
                        {
                            if _p_14 > _start_3
                                && is_hi_surrogate(
                                    *_p_14.offset(-(1 as core::ffi::c_int) as isize) as uint32_t,
                                ) != 0
                            {
                                _p_14 = _p_14.offset(-1);
                            }
                        }
                        cptr = _p_14 as *const core::ffi::c_void as *const uint8_t;
                    }
                    continue;
                }
            }
            _ => {
                abort();
            }
        }
        match current_block {
            13538245850655155981 => {
                if !(cptr >= cbuf_end) {
                    if cbuf_type == 0 as core::ffi::c_int {
                        let fresh49 = cptr;
                        cptr = cptr.offset(1);
                        c = *fresh49 as uint32_t;
                    } else {
                        let mut _p: *const uint16_t = cptr as *const uint16_t;
                        let mut _end: *const uint16_t = cbuf_end as *const uint16_t;
                        let fresh50 = _p;
                        _p = _p.offset(1);
                        c = *fresh50 as uint32_t;
                        if is_hi_surrogate(c) != 0 && cbuf_type == 2 as core::ffi::c_int
                        {
                            if _p < _end && is_lo_surrogate(*_p as uint32_t) != 0 {
                                let fresh51 = _p;
                                _p = _p.offset(1);
                                c = from_surrogate(c, *fresh51 as uint32_t);
                            }
                        }
                        cptr = _p as *const core::ffi::c_void as *const uint8_t;
                    }
                    if opcode == REOP_char_i as core::ffi::c_int
                        || opcode == REOP_char32_i as core::ffi::c_int
                    {
                        c = lre_canonicalize(c, (*s).is_unicode as core::ffi::c_int)
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
                        (*sp.offset(-(2 as core::ffi::c_int) as isize)).val as isize,
                    );
                *fresh44 = (*sp.offset(-(1 as core::ffi::c_int) as isize)).ptr;
                sp = sp.offset(-(2 as core::ffi::c_int as isize));
            }
            pc = (*sp.offset(-(3 as core::ffi::c_int) as isize)).ptr;
            cptr = (*sp.offset(-(2 as core::ffi::c_int) as isize)).ptr;
            type_0 = ((*sp.offset(-(1 as core::ffi::c_int) as isize)).bp).type_0()
                as REExecStateEnum;
            bp = ((*s).stack_buf)
                .offset(
                    ((*sp.offset(-(1 as core::ffi::c_int) as isize)).bp).val() as isize,
                );
            sp = sp.offset(-(3 as core::ffi::c_int as isize));
            if type_0 as core::ffi::c_uint
                != RE_EXEC_STATE_LOOKAHEAD as core::ffi::c_int as core::ffi::c_uint
            {
                break;
            }
        }
        if lre_poll_timeout(s) != 0 {
            return LRE_RET_TIMEOUT as intptr_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lre_exec(
    mut capture: *mut *mut uint8_t,
    mut bc_buf: *const uint8_t,
    mut cbuf: *const uint8_t,
    mut cindex: core::ffi::c_int,
    mut clen: core::ffi::c_int,
    mut cbuf_type: core::ffi::c_int,
    mut opaque: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let mut s_s: REExecContext = REExecContext {
        cbuf: 0 as *const uint8_t,
        cbuf_end: 0 as *const uint8_t,
        cbuf_type: 0,
        capture_count: 0,
        is_unicode: 0,
        interrupt_counter: 0,
        opaque: 0 as *mut core::ffi::c_void,
        stack_buf: 0 as *mut StackElem,
        stack_size: 0,
        static_stack_buf: [StackElem {
            ptr: 0 as *mut uint8_t,
        }; 32],
    };
    let mut s: *mut REExecContext = &mut s_s;
    let mut re_flags: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    let mut ret: core::ffi::c_int = 0;
    let mut cptr: *const uint8_t = 0 as *const uint8_t;
    re_flags = lre_get_flags(bc_buf);
    (*s).is_unicode = (re_flags & (LRE_FLAG_UNICODE | LRE_FLAG_UNICODE_SETS)
        != 0 as core::ffi::c_int) as core::ffi::c_int as BOOL;
    (*s).capture_count = *bc_buf.offset(RE_HEADER_CAPTURE_COUNT as isize)
        as core::ffi::c_int;
    (*s).cbuf = cbuf;
    (*s).cbuf_end = cbuf.offset((clen << cbuf_type) as isize);
    (*s).cbuf_type = cbuf_type;
    if (*s).cbuf_type == 1 as core::ffi::c_int && (*s).is_unicode != 0 {
        (*s).cbuf_type = 2 as core::ffi::c_int;
    }
    (*s).interrupt_counter = INTERRUPT_COUNTER_INIT;
    (*s).opaque = opaque;
    (*s).stack_buf = ((*s).static_stack_buf).as_mut_ptr();
    (*s).stack_size = (::core::mem::size_of::<[StackElem; 32]>() as usize)
        .wrapping_div(::core::mem::size_of::<StackElem>() as usize) as size_t;
    i = 0 as core::ffi::c_int;
    while i < (*s).capture_count * 2 as core::ffi::c_int {
        let ref mut fresh42 = *capture.offset(i as isize);
        *fresh42 = 0 as *mut uint8_t;
        i += 1;
    }
    cptr = cbuf.offset((cindex << cbuf_type) as isize);
    if (0 as core::ffi::c_int) < cindex && cindex < clen
        && (*s).cbuf_type == 2 as core::ffi::c_int
    {
        let mut p: *const uint16_t = cptr as *const uint16_t;
        if is_lo_surrogate(*p as uint32_t) != 0
            && is_hi_surrogate(*p.offset(-(1 as core::ffi::c_int) as isize) as uint32_t)
                != 0
        {
            cptr = p.offset(-(1 as core::ffi::c_int as isize)) as *const uint8_t;
        }
    }
    ret = lre_exec_backtrack(s, capture, bc_buf.offset(RE_HEADER_LEN as isize), cptr)
        as core::ffi::c_int;
    if (*s).stack_buf != ((*s).static_stack_buf).as_mut_ptr() {
        lre_realloc((*s).opaque, (*s).stack_buf as *mut core::ffi::c_void, 0 as size_t);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lre_get_alloc_count(
    mut bc_buf: *const uint8_t,
) -> core::ffi::c_int {
    return *bc_buf.offset(RE_HEADER_CAPTURE_COUNT as isize) as core::ffi::c_int
        * 2 as core::ffi::c_int
        + *bc_buf.offset(RE_HEADER_REGISTER_COUNT as isize) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lre_get_capture_count(
    mut bc_buf: *const uint8_t,
) -> core::ffi::c_int {
    return *bc_buf.offset(RE_HEADER_CAPTURE_COUNT as isize) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lre_get_flags(mut bc_buf: *const uint8_t) -> core::ffi::c_int {
    return get_u16(bc_buf.offset(RE_HEADER_FLAGS as isize)) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lre_get_groupnames(
    mut bc_buf: *const uint8_t,
) -> *const core::ffi::c_char {
    let mut re_bytecode_len: uint32_t = 0;
    if lre_get_flags(bc_buf) & LRE_FLAG_NAMED_GROUPS == 0 as core::ffi::c_int {
        return 0 as *const core::ffi::c_char;
    }
    re_bytecode_len = get_u32(bc_buf.offset(RE_HEADER_BYTECODE_LEN as isize));
    return bc_buf.offset(RE_HEADER_LEN as isize).offset(re_bytecode_len as isize)
        as *const core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn cr_add_point(
    mut cr: *mut CharRange,
    mut v: uint32_t,
) -> core::ffi::c_int {
    if (*cr).len >= (*cr).size {
        if cr_realloc(cr, (*cr).len + 1 as core::ffi::c_int) != 0 {
            return -(1 as core::ffi::c_int);
        }
    }
    let fresh38 = (*cr).len;
    (*cr).len = (*cr).len + 1;
    *((*cr).points).offset(fresh38 as isize) = v;
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn cr_add_interval(
    mut cr: *mut CharRange,
    mut c1: uint32_t,
    mut c2: uint32_t,
) -> core::ffi::c_int {
    if (*cr).len + 2 as core::ffi::c_int > (*cr).size {
        if cr_realloc(cr, (*cr).len + 2 as core::ffi::c_int) != 0 {
            return -(1 as core::ffi::c_int);
        }
    }
    let fresh39 = (*cr).len;
    (*cr).len = (*cr).len + 1;
    *((*cr).points).offset(fresh39 as isize) = c1;
    let fresh40 = (*cr).len;
    (*cr).len = (*cr).len + 1;
    *((*cr).points).offset(fresh40 as isize) = c2;
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn cr_union_interval(
    mut cr: *mut CharRange,
    mut c1: uint32_t,
    mut c2: uint32_t,
) -> core::ffi::c_int {
    let mut b_pt: [uint32_t; 2] = [0; 2];
    b_pt[0 as core::ffi::c_int as usize] = c1;
    b_pt[1 as core::ffi::c_int as usize] = c2.wrapping_add(1 as uint32_t);
    return cr_op1(
        cr,
        b_pt.as_mut_ptr(),
        2 as core::ffi::c_int,
        CR_OP_UNION as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn lre_is_space_byte(mut c: uint8_t) -> core::ffi::c_int {
    return lre_ctype_bits[c as usize] as core::ffi::c_int
        & UNICODE_C_SPACE as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn lre_is_id_start_byte(mut c: uint8_t) -> core::ffi::c_int {
    return lre_ctype_bits[c as usize] as core::ffi::c_int
        & (UNICODE_C_UPPER as core::ffi::c_int | UNICODE_C_LOWER as core::ffi::c_int
            | UNICODE_C_UNDER as core::ffi::c_int
            | UNICODE_C_DOLLAR as core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn lre_is_id_continue_byte(mut c: uint8_t) -> core::ffi::c_int {
    return lre_ctype_bits[c as usize] as core::ffi::c_int
        & (UNICODE_C_UPPER as core::ffi::c_int | UNICODE_C_LOWER as core::ffi::c_int
            | UNICODE_C_UNDER as core::ffi::c_int | UNICODE_C_DOLLAR as core::ffi::c_int
            | UNICODE_C_DIGIT as core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn lre_is_word_byte(mut c: uint8_t) -> core::ffi::c_int {
    return lre_ctype_bits[c as usize] as core::ffi::c_int
        & (UNICODE_C_UPPER as core::ffi::c_int | UNICODE_C_LOWER as core::ffi::c_int
            | UNICODE_C_UNDER as core::ffi::c_int | UNICODE_C_DIGIT as core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn lre_is_space(mut c: uint32_t) -> core::ffi::c_int {
    if c < 256 as uint32_t {
        return lre_is_space_byte(c as uint8_t)
    } else {
        return lre_is_space_non_ascii(c)
    };
}
#[inline]
unsafe extern "C" fn lre_js_is_ident_first(mut c: uint32_t) -> core::ffi::c_int {
    if c < 128 as uint32_t {
        return lre_is_id_start_byte(c as uint8_t)
    } else {
        return lre_is_id_start(c)
    };
}
#[inline]
unsafe extern "C" fn lre_js_is_ident_next(mut c: uint32_t) -> core::ffi::c_int {
    if c < 128 as uint32_t {
        return lre_is_id_continue_byte(c as uint8_t)
    } else {
        if c >= 0x200c as uint32_t && c <= 0x200d as uint32_t {
            return TRUE as core::ffi::c_int;
        }
        return lre_is_id_continue(c);
    };
}
pub const LRE_FLAG_DOTALL: core::ffi::c_int = (1 as core::ffi::c_int)
    << 3 as core::ffi::c_int;
pub const LRE_FLAG_UNICODE: core::ffi::c_int = (1 as core::ffi::c_int)
    << 4 as core::ffi::c_int;
pub const LRE_FLAG_STICKY: core::ffi::c_int = (1 as core::ffi::c_int)
    << 5 as core::ffi::c_int;
pub const LRE_FLAG_NAMED_GROUPS: core::ffi::c_int = (1 as core::ffi::c_int)
    << 7 as core::ffi::c_int;
pub const LRE_FLAG_UNICODE_SETS: core::ffi::c_int = (1 as core::ffi::c_int)
    << 8 as core::ffi::c_int;
pub const LRE_RET_MEMORY_ERROR: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const LRE_RET_TIMEOUT: core::ffi::c_int = -(2 as core::ffi::c_int);
pub const LRE_GROUP_NAME_TRAILER_LEN: core::ffi::c_int = 2 as core::ffi::c_int;

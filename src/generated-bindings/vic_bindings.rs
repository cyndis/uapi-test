/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __TIMESIZE: u32 = 64;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 35;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct _OutputConfig {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 16usize], u16>,
}
#[test]
fn bindgen_test_layout__OutputConfig() {
    assert_eq!(
        ::std::mem::size_of::<_OutputConfig>(),
        16usize,
        concat!("Size of: ", stringify!(_OutputConfig))
    );
    assert_eq!(
        ::std::mem::align_of::<_OutputConfig>(),
        8usize,
        concat!("Alignment of ", stringify!(_OutputConfig))
    );
}
impl _OutputConfig {
    #[inline]
    pub fn AlphaFillMode(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 3u8) as u64) }
    }
    #[inline]
    pub fn set_AlphaFillMode(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn AlphaFillSlot(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 3u8) as u64) }
    }
    #[inline]
    pub fn set_AlphaFillSlot(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn BackgroundAlpha(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 10u8) as u64) }
    }
    #[inline]
    pub fn set_BackgroundAlpha(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn BackgroundR(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 10u8) as u64) }
    }
    #[inline]
    pub fn set_BackgroundR(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn BackgroundG(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(26usize, 10u8) as u64) }
    }
    #[inline]
    pub fn set_BackgroundG(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(26usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn BackgroundB(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(36usize, 10u8) as u64) }
    }
    #[inline]
    pub fn set_BackgroundB(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(36usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn RegammaMode(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(46usize, 2u8) as u64) }
    }
    #[inline]
    pub fn set_RegammaMode(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(46usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn OutputFlipX(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(48usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_OutputFlipX(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(48usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn OutputFlipY(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(49usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_OutputFlipY(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(49usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn OutputTranspose(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(50usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_OutputTranspose(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(50usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved1(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(51usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_reserved1(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(51usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved2(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(52usize, 12u8) as u64) }
    }
    #[inline]
    pub fn set_reserved2(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(52usize, 12u8, val as u64)
        }
    }
    #[inline]
    pub fn TargetRectLeft(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(64usize, 14u8) as u64) }
    }
    #[inline]
    pub fn set_TargetRectLeft(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(64usize, 14u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved3(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(78usize, 2u8) as u64) }
    }
    #[inline]
    pub fn set_reserved3(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(78usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn TargetRectRight(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(80usize, 14u8) as u64) }
    }
    #[inline]
    pub fn set_TargetRectRight(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(80usize, 14u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved4(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(94usize, 2u8) as u64) }
    }
    #[inline]
    pub fn set_reserved4(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(94usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn TargetRectTop(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(96usize, 14u8) as u64) }
    }
    #[inline]
    pub fn set_TargetRectTop(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(96usize, 14u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved5(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(110usize, 2u8) as u64) }
    }
    #[inline]
    pub fn set_reserved5(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(110usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn TargetRectBottom(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(112usize, 14u8) as u64) }
    }
    #[inline]
    pub fn set_TargetRectBottom(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(112usize, 14u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved6(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(126usize, 2u8) as u64) }
    }
    #[inline]
    pub fn set_reserved6(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(126usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        AlphaFillMode: u64,
        AlphaFillSlot: u64,
        BackgroundAlpha: u64,
        BackgroundR: u64,
        BackgroundG: u64,
        BackgroundB: u64,
        RegammaMode: u64,
        OutputFlipX: u64,
        OutputFlipY: u64,
        OutputTranspose: u64,
        reserved1: u64,
        reserved2: u64,
        TargetRectLeft: u64,
        reserved3: u64,
        TargetRectRight: u64,
        reserved4: u64,
        TargetRectTop: u64,
        reserved5: u64,
        TargetRectBottom: u64,
        reserved6: u64,
    ) -> __BindgenBitfieldUnit<[u8; 16usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 16usize], u16> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 3u8, {
            let AlphaFillMode: u64 = unsafe { ::std::mem::transmute(AlphaFillMode) };
            AlphaFillMode as u64
        });
        __bindgen_bitfield_unit.set(3usize, 3u8, {
            let AlphaFillSlot: u64 = unsafe { ::std::mem::transmute(AlphaFillSlot) };
            AlphaFillSlot as u64
        });
        __bindgen_bitfield_unit.set(6usize, 10u8, {
            let BackgroundAlpha: u64 = unsafe { ::std::mem::transmute(BackgroundAlpha) };
            BackgroundAlpha as u64
        });
        __bindgen_bitfield_unit.set(16usize, 10u8, {
            let BackgroundR: u64 = unsafe { ::std::mem::transmute(BackgroundR) };
            BackgroundR as u64
        });
        __bindgen_bitfield_unit.set(26usize, 10u8, {
            let BackgroundG: u64 = unsafe { ::std::mem::transmute(BackgroundG) };
            BackgroundG as u64
        });
        __bindgen_bitfield_unit.set(36usize, 10u8, {
            let BackgroundB: u64 = unsafe { ::std::mem::transmute(BackgroundB) };
            BackgroundB as u64
        });
        __bindgen_bitfield_unit.set(46usize, 2u8, {
            let RegammaMode: u64 = unsafe { ::std::mem::transmute(RegammaMode) };
            RegammaMode as u64
        });
        __bindgen_bitfield_unit.set(48usize, 1u8, {
            let OutputFlipX: u64 = unsafe { ::std::mem::transmute(OutputFlipX) };
            OutputFlipX as u64
        });
        __bindgen_bitfield_unit.set(49usize, 1u8, {
            let OutputFlipY: u64 = unsafe { ::std::mem::transmute(OutputFlipY) };
            OutputFlipY as u64
        });
        __bindgen_bitfield_unit.set(50usize, 1u8, {
            let OutputTranspose: u64 = unsafe { ::std::mem::transmute(OutputTranspose) };
            OutputTranspose as u64
        });
        __bindgen_bitfield_unit.set(51usize, 1u8, {
            let reserved1: u64 = unsafe { ::std::mem::transmute(reserved1) };
            reserved1 as u64
        });
        __bindgen_bitfield_unit.set(52usize, 12u8, {
            let reserved2: u64 = unsafe { ::std::mem::transmute(reserved2) };
            reserved2 as u64
        });
        __bindgen_bitfield_unit.set(64usize, 14u8, {
            let TargetRectLeft: u64 = unsafe { ::std::mem::transmute(TargetRectLeft) };
            TargetRectLeft as u64
        });
        __bindgen_bitfield_unit.set(78usize, 2u8, {
            let reserved3: u64 = unsafe { ::std::mem::transmute(reserved3) };
            reserved3 as u64
        });
        __bindgen_bitfield_unit.set(80usize, 14u8, {
            let TargetRectRight: u64 = unsafe { ::std::mem::transmute(TargetRectRight) };
            TargetRectRight as u64
        });
        __bindgen_bitfield_unit.set(94usize, 2u8, {
            let reserved4: u64 = unsafe { ::std::mem::transmute(reserved4) };
            reserved4 as u64
        });
        __bindgen_bitfield_unit.set(96usize, 14u8, {
            let TargetRectTop: u64 = unsafe { ::std::mem::transmute(TargetRectTop) };
            TargetRectTop as u64
        });
        __bindgen_bitfield_unit.set(110usize, 2u8, {
            let reserved5: u64 = unsafe { ::std::mem::transmute(reserved5) };
            reserved5 as u64
        });
        __bindgen_bitfield_unit.set(112usize, 14u8, {
            let TargetRectBottom: u64 = unsafe { ::std::mem::transmute(TargetRectBottom) };
            TargetRectBottom as u64
        });
        __bindgen_bitfield_unit.set(126usize, 2u8, {
            let reserved6: u64 = unsafe { ::std::mem::transmute(reserved6) };
            reserved6 as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type OutputConfig = _OutputConfig;
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct _OutputSurfaceConfig {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 16usize], u16>,
}
#[test]
fn bindgen_test_layout__OutputSurfaceConfig() {
    assert_eq!(
        ::std::mem::size_of::<_OutputSurfaceConfig>(),
        16usize,
        concat!("Size of: ", stringify!(_OutputSurfaceConfig))
    );
    assert_eq!(
        ::std::mem::align_of::<_OutputSurfaceConfig>(),
        8usize,
        concat!("Alignment of ", stringify!(_OutputSurfaceConfig))
    );
}
impl _OutputSurfaceConfig {
    #[inline]
    pub fn OutPixelFormat(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u64) }
    }
    #[inline]
    pub fn set_OutPixelFormat(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn OutChromaLocHoriz(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 2u8) as u64) }
    }
    #[inline]
    pub fn set_OutChromaLocHoriz(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn OutChromaLocVert(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 2u8) as u64) }
    }
    #[inline]
    pub fn set_OutChromaLocVert(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn OutBlkKind(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 4u8) as u64) }
    }
    #[inline]
    pub fn set_OutBlkKind(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn OutBlkHeight(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 4u8) as u64) }
    }
    #[inline]
    pub fn set_OutBlkHeight(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved0(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(19usize, 3u8) as u64) }
    }
    #[inline]
    pub fn set_reserved0(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(19usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved1(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(22usize, 10u8) as u64) }
    }
    #[inline]
    pub fn set_reserved1(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(22usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn OutSurfaceWidth(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(32usize, 14u8) as u64) }
    }
    #[inline]
    pub fn set_OutSurfaceWidth(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(32usize, 14u8, val as u64)
        }
    }
    #[inline]
    pub fn OutSurfaceHeight(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(46usize, 14u8) as u64) }
    }
    #[inline]
    pub fn set_OutSurfaceHeight(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(46usize, 14u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved2(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(60usize, 4u8) as u64) }
    }
    #[inline]
    pub fn set_reserved2(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(60usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn OutLumaWidth(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(64usize, 14u8) as u64) }
    }
    #[inline]
    pub fn set_OutLumaWidth(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(64usize, 14u8, val as u64)
        }
    }
    #[inline]
    pub fn OutLumaHeight(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(78usize, 14u8) as u64) }
    }
    #[inline]
    pub fn set_OutLumaHeight(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(78usize, 14u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved3(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(92usize, 4u8) as u64) }
    }
    #[inline]
    pub fn set_reserved3(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(92usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn OutChromaWidth(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(96usize, 14u8) as u64) }
    }
    #[inline]
    pub fn set_OutChromaWidth(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(96usize, 14u8, val as u64)
        }
    }
    #[inline]
    pub fn OutChromaHeight(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(110usize, 14u8) as u64) }
    }
    #[inline]
    pub fn set_OutChromaHeight(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(110usize, 14u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved4(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(124usize, 4u8) as u64) }
    }
    #[inline]
    pub fn set_reserved4(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(124usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        OutPixelFormat: u64,
        OutChromaLocHoriz: u64,
        OutChromaLocVert: u64,
        OutBlkKind: u64,
        OutBlkHeight: u64,
        reserved0: u64,
        reserved1: u64,
        OutSurfaceWidth: u64,
        OutSurfaceHeight: u64,
        reserved2: u64,
        OutLumaWidth: u64,
        OutLumaHeight: u64,
        reserved3: u64,
        OutChromaWidth: u64,
        OutChromaHeight: u64,
        reserved4: u64,
    ) -> __BindgenBitfieldUnit<[u8; 16usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 16usize], u16> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let OutPixelFormat: u64 = unsafe { ::std::mem::transmute(OutPixelFormat) };
            OutPixelFormat as u64
        });
        __bindgen_bitfield_unit.set(7usize, 2u8, {
            let OutChromaLocHoriz: u64 = unsafe { ::std::mem::transmute(OutChromaLocHoriz) };
            OutChromaLocHoriz as u64
        });
        __bindgen_bitfield_unit.set(9usize, 2u8, {
            let OutChromaLocVert: u64 = unsafe { ::std::mem::transmute(OutChromaLocVert) };
            OutChromaLocVert as u64
        });
        __bindgen_bitfield_unit.set(11usize, 4u8, {
            let OutBlkKind: u64 = unsafe { ::std::mem::transmute(OutBlkKind) };
            OutBlkKind as u64
        });
        __bindgen_bitfield_unit.set(15usize, 4u8, {
            let OutBlkHeight: u64 = unsafe { ::std::mem::transmute(OutBlkHeight) };
            OutBlkHeight as u64
        });
        __bindgen_bitfield_unit.set(19usize, 3u8, {
            let reserved0: u64 = unsafe { ::std::mem::transmute(reserved0) };
            reserved0 as u64
        });
        __bindgen_bitfield_unit.set(22usize, 10u8, {
            let reserved1: u64 = unsafe { ::std::mem::transmute(reserved1) };
            reserved1 as u64
        });
        __bindgen_bitfield_unit.set(32usize, 14u8, {
            let OutSurfaceWidth: u64 = unsafe { ::std::mem::transmute(OutSurfaceWidth) };
            OutSurfaceWidth as u64
        });
        __bindgen_bitfield_unit.set(46usize, 14u8, {
            let OutSurfaceHeight: u64 = unsafe { ::std::mem::transmute(OutSurfaceHeight) };
            OutSurfaceHeight as u64
        });
        __bindgen_bitfield_unit.set(60usize, 4u8, {
            let reserved2: u64 = unsafe { ::std::mem::transmute(reserved2) };
            reserved2 as u64
        });
        __bindgen_bitfield_unit.set(64usize, 14u8, {
            let OutLumaWidth: u64 = unsafe { ::std::mem::transmute(OutLumaWidth) };
            OutLumaWidth as u64
        });
        __bindgen_bitfield_unit.set(78usize, 14u8, {
            let OutLumaHeight: u64 = unsafe { ::std::mem::transmute(OutLumaHeight) };
            OutLumaHeight as u64
        });
        __bindgen_bitfield_unit.set(92usize, 4u8, {
            let reserved3: u64 = unsafe { ::std::mem::transmute(reserved3) };
            reserved3 as u64
        });
        __bindgen_bitfield_unit.set(96usize, 14u8, {
            let OutChromaWidth: u64 = unsafe { ::std::mem::transmute(OutChromaWidth) };
            OutChromaWidth as u64
        });
        __bindgen_bitfield_unit.set(110usize, 14u8, {
            let OutChromaHeight: u64 = unsafe { ::std::mem::transmute(OutChromaHeight) };
            OutChromaHeight as u64
        });
        __bindgen_bitfield_unit.set(124usize, 4u8, {
            let reserved4: u64 = unsafe { ::std::mem::transmute(reserved4) };
            reserved4 as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type OutputSurfaceConfig = _OutputSurfaceConfig;

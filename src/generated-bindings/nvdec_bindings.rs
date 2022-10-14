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
#[derive(Debug, Copy, Clone)]
pub struct _nvdec_display_param_s {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u16>,
    pub OutputTop: [i32; 2usize],
    pub OutputBottom: [i32; 2usize],
    pub _bitfield_2: __BindgenBitfieldUnit<[u8; 8usize], u16>,
}
#[test]
fn bindgen_test_layout__nvdec_display_param_s() {
    assert_eq!(
        ::std::mem::size_of::<_nvdec_display_param_s>(),
        28usize,
        concat!("Size of: ", stringify!(_nvdec_display_param_s))
    );
    assert_eq!(
        ::std::mem::align_of::<_nvdec_display_param_s>(),
        4usize,
        concat!("Alignment of ", stringify!(_nvdec_display_param_s))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_display_param_s>())).OutputTop as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_display_param_s),
            "::",
            stringify!(OutputTop)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_display_param_s>())).OutputBottom as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_display_param_s),
            "::",
            stringify!(OutputBottom)
        )
    );
}
impl _nvdec_display_param_s {
    #[inline]
    pub fn enableTFOutput(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableTFOutput(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn VC1MapYFlag(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_VC1MapYFlag(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn MapYValue(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_MapYValue(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn VC1MapUVFlag(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_VC1MapUVFlag(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn MapUVValue(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_MapUVValue(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn OutStride(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_OutStride(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn TilingFormat(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(17usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_TilingFormat(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(17usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn OutputStructure(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(20usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_OutputStructure(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(20usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved0(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(21usize, 11u8) as u32) }
    }
    #[inline]
    pub fn set_reserved0(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(21usize, 11u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        enableTFOutput: u32,
        VC1MapYFlag: u32,
        MapYValue: u32,
        VC1MapUVFlag: u32,
        MapUVValue: u32,
        OutStride: u32,
        TilingFormat: u32,
        OutputStructure: u32,
        reserved0: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u16> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let enableTFOutput: u32 = unsafe { ::std::mem::transmute(enableTFOutput) };
            enableTFOutput as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let VC1MapYFlag: u32 = unsafe { ::std::mem::transmute(VC1MapYFlag) };
            VC1MapYFlag as u64
        });
        __bindgen_bitfield_unit.set(2usize, 3u8, {
            let MapYValue: u32 = unsafe { ::std::mem::transmute(MapYValue) };
            MapYValue as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let VC1MapUVFlag: u32 = unsafe { ::std::mem::transmute(VC1MapUVFlag) };
            VC1MapUVFlag as u64
        });
        __bindgen_bitfield_unit.set(6usize, 3u8, {
            let MapUVValue: u32 = unsafe { ::std::mem::transmute(MapUVValue) };
            MapUVValue as u64
        });
        __bindgen_bitfield_unit.set(9usize, 8u8, {
            let OutStride: u32 = unsafe { ::std::mem::transmute(OutStride) };
            OutStride as u64
        });
        __bindgen_bitfield_unit.set(17usize, 3u8, {
            let TilingFormat: u32 = unsafe { ::std::mem::transmute(TilingFormat) };
            TilingFormat as u64
        });
        __bindgen_bitfield_unit.set(20usize, 1u8, {
            let OutputStructure: u32 = unsafe { ::std::mem::transmute(OutputStructure) };
            OutputStructure as u64
        });
        __bindgen_bitfield_unit.set(21usize, 11u8, {
            let reserved0: u32 = unsafe { ::std::mem::transmute(reserved0) };
            reserved0 as u64
        });
        __bindgen_bitfield_unit
    }
    #[inline]
    pub fn enableHistogram(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_enableHistogram(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn HistogramStartX(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(1usize, 12u8) as u32) }
    }
    #[inline]
    pub fn set_HistogramStartX(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set(1usize, 12u8, val as u64)
        }
    }
    #[inline]
    pub fn HistogramStartY(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(13usize, 12u8) as u32) }
    }
    #[inline]
    pub fn set_HistogramStartY(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set(13usize, 12u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved1(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(25usize, 7u8) as u32) }
    }
    #[inline]
    pub fn set_reserved1(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set(25usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn HistogramEndX(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(32usize, 12u8) as u32) }
    }
    #[inline]
    pub fn set_HistogramEndX(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set(32usize, 12u8, val as u64)
        }
    }
    #[inline]
    pub fn HistogramEndY(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(44usize, 12u8) as u32) }
    }
    #[inline]
    pub fn set_HistogramEndY(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set(44usize, 12u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved2(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_2.get(56usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_reserved2(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set(56usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_2(
        enableHistogram: u32,
        HistogramStartX: u32,
        HistogramStartY: u32,
        reserved1: u32,
        HistogramEndX: u32,
        HistogramEndY: u32,
        reserved2: u32,
    ) -> __BindgenBitfieldUnit<[u8; 8usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize], u16> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let enableHistogram: u32 = unsafe { ::std::mem::transmute(enableHistogram) };
            enableHistogram as u64
        });
        __bindgen_bitfield_unit.set(1usize, 12u8, {
            let HistogramStartX: u32 = unsafe { ::std::mem::transmute(HistogramStartX) };
            HistogramStartX as u64
        });
        __bindgen_bitfield_unit.set(13usize, 12u8, {
            let HistogramStartY: u32 = unsafe { ::std::mem::transmute(HistogramStartY) };
            HistogramStartY as u64
        });
        __bindgen_bitfield_unit.set(25usize, 7u8, {
            let reserved1: u32 = unsafe { ::std::mem::transmute(reserved1) };
            reserved1 as u64
        });
        __bindgen_bitfield_unit.set(32usize, 12u8, {
            let HistogramEndX: u32 = unsafe { ::std::mem::transmute(HistogramEndX) };
            HistogramEndX as u64
        });
        __bindgen_bitfield_unit.set(44usize, 12u8, {
            let HistogramEndY: u32 = unsafe { ::std::mem::transmute(HistogramEndY) };
            HistogramEndY as u64
        });
        __bindgen_bitfield_unit.set(56usize, 8u8, {
            let reserved2: u32 = unsafe { ::std::mem::transmute(reserved2) };
            reserved2 as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type nvdec_display_param_s = _nvdec_display_param_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _nvdec_mpeg2_pic_s {
    pub reserved0: [u32; 13usize],
    pub eos: [u8; 16usize],
    pub explicitEOSPresentFlag: u8,
    pub reserved1: [u8; 3usize],
    pub stream_len: u32,
    pub slice_count: u32,
    pub gptimer_timeout_value: u32,
    pub FrameWidth: u16,
    pub FrameHeight: u16,
    pub picture_structure: u8,
    pub picture_coding_type: u8,
    pub intra_dc_precision: u8,
    pub frame_pred_frame_dct: i8,
    pub concealment_motion_vectors: i8,
    pub intra_vlc_format: i8,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub reserved2: i8,
    pub f_code: [i8; 4usize],
    pub PicWidthInMbs: u16,
    pub FrameHeightInMbs: u16,
    pub pitch_luma: u32,
    pub pitch_chroma: u32,
    pub luma_top_offset: u32,
    pub luma_bot_offset: u32,
    pub luma_frame_offset: u32,
    pub chroma_top_offset: u32,
    pub chroma_bot_offset: u32,
    pub chroma_frame_offset: u32,
    pub HistBufferSize: u32,
    pub output_memory_layout: u16,
    pub alternate_scan: u16,
    pub secondfield: u16,
    pub rounding_type: u16,
    pub MbInfoSizeInBytes: u32,
    pub q_scale_type: u32,
    pub top_field_first: u32,
    pub full_pel_fwd_vector: u32,
    pub full_pel_bwd_vector: u32,
    pub quant_mat_8x8intra: [u8; 64usize],
    pub quant_mat_8x8nonintra: [u8; 64usize],
    pub ref_memory_layout: [u32; 2usize],
    pub displayPara: nvdec_display_param_s,
    pub reserved3: [u32; 3usize],
}
#[test]
fn bindgen_test_layout__nvdec_mpeg2_pic_s() {
    assert_eq!(
        ::std::mem::size_of::<_nvdec_mpeg2_pic_s>(),
        344usize,
        concat!("Size of: ", stringify!(_nvdec_mpeg2_pic_s))
    );
    assert_eq!(
        ::std::mem::align_of::<_nvdec_mpeg2_pic_s>(),
        4usize,
        concat!("Alignment of ", stringify!(_nvdec_mpeg2_pic_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).reserved0 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(reserved0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).eos as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(eos)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).explicitEOSPresentFlag as *const _
                as usize
        },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(explicitEOSPresentFlag)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).reserved1 as *const _ as usize },
        69usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(reserved1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).stream_len as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(stream_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).slice_count as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(slice_count)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).gptimer_timeout_value as *const _
                as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(gptimer_timeout_value)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).FrameWidth as *const _ as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(FrameWidth)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).FrameHeight as *const _ as usize },
        86usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(FrameHeight)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).picture_structure as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(picture_structure)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).picture_coding_type as *const _ as usize
        },
        89usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(picture_coding_type)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).intra_dc_precision as *const _ as usize
        },
        90usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(intra_dc_precision)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).frame_pred_frame_dct as *const _ as usize
        },
        91usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(frame_pred_frame_dct)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).concealment_motion_vectors as *const _
                as usize
        },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(concealment_motion_vectors)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).intra_vlc_format as *const _ as usize
        },
        93usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(intra_vlc_format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).reserved2 as *const _ as usize },
        95usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(reserved2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).f_code as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(f_code)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).PicWidthInMbs as *const _ as usize
        },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(PicWidthInMbs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).FrameHeightInMbs as *const _ as usize
        },
        102usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(FrameHeightInMbs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).pitch_luma as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(pitch_luma)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).pitch_chroma as *const _ as usize },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(pitch_chroma)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).luma_top_offset as *const _ as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(luma_top_offset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).luma_bot_offset as *const _ as usize
        },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(luma_bot_offset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).luma_frame_offset as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(luma_frame_offset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).chroma_top_offset as *const _ as usize
        },
        124usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(chroma_top_offset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).chroma_bot_offset as *const _ as usize
        },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(chroma_bot_offset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).chroma_frame_offset as *const _ as usize
        },
        132usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(chroma_frame_offset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).HistBufferSize as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(HistBufferSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).output_memory_layout as *const _ as usize
        },
        140usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(output_memory_layout)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).alternate_scan as *const _ as usize
        },
        142usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(alternate_scan)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).secondfield as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(secondfield)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).rounding_type as *const _ as usize
        },
        146usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(rounding_type)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).MbInfoSizeInBytes as *const _ as usize
        },
        148usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(MbInfoSizeInBytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).q_scale_type as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(q_scale_type)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).top_field_first as *const _ as usize
        },
        156usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(top_field_first)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).full_pel_fwd_vector as *const _ as usize
        },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(full_pel_fwd_vector)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).full_pel_bwd_vector as *const _ as usize
        },
        164usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(full_pel_bwd_vector)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).quant_mat_8x8intra as *const _ as usize
        },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(quant_mat_8x8intra)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).quant_mat_8x8nonintra as *const _
                as usize
        },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(quant_mat_8x8nonintra)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).ref_memory_layout as *const _ as usize
        },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(ref_memory_layout)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).displayPara as *const _ as usize },
        304usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(displayPara)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_nvdec_mpeg2_pic_s>())).reserved3 as *const _ as usize },
        332usize,
        concat!(
            "Offset of field: ",
            stringify!(_nvdec_mpeg2_pic_s),
            "::",
            stringify!(reserved3)
        )
    );
}
impl _nvdec_mpeg2_pic_s {
    #[inline]
    pub fn tileFormat(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_tileFormat(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn gob_height(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 3u8) as u8) }
    }
    #[inline]
    pub fn set_gob_height(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn reserverd_surface_format(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 3u8) as u8) }
    }
    #[inline]
    pub fn set_reserverd_surface_format(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        tileFormat: u8,
        gob_height: u8,
        reserverd_surface_format: u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let tileFormat: u8 = unsafe { ::std::mem::transmute(tileFormat) };
            tileFormat as u64
        });
        __bindgen_bitfield_unit.set(2usize, 3u8, {
            let gob_height: u8 = unsafe { ::std::mem::transmute(gob_height) };
            gob_height as u64
        });
        __bindgen_bitfield_unit.set(5usize, 3u8, {
            let reserverd_surface_format: u8 =
                unsafe { ::std::mem::transmute(reserverd_surface_format) };
            reserverd_surface_format as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type nvdec_mpeg2_pic_s = _nvdec_mpeg2_pic_s;

#[repr(C)]
pub struct _IO_wide_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct _IO_codecvt {
    _private: [u8; 0],
}
#[repr(C)]
pub struct _IO_marker {
    _private: [u8; 0],
}
extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn ffi_tramp_is_supported() -> ::core::ffi::c_int;
    fn ffi_tramp_alloc(flags: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn ffi_tramp_get_addr(tramp: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn ffi_tramp_free(tramp: *mut ::core::ffi::c_void);
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn access(__name: *const ::core::ffi::c_char, __type: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn ftruncate(__fd: ::core::ffi::c_int, __length: __off_t) -> ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn __getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn setmntent(
        __file: *const ::core::ffi::c_char,
        __mode: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn getmntent_r(
        __stream: *mut FILE,
        __result: *mut mntent,
        __buffer: *mut ::core::ffi::c_char,
        __bufsize: ::core::ffi::c_int,
    ) -> *mut mntent;
    fn endmntent(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn hasmntopt(
        __mnt: *const mntent,
        __opt: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sched_yield() -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn memfd_create(
        __name: *const ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off_t,
    ) -> *mut ::core::ffi::c_void;
    fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
    fn statfs(__file: *const ::core::ffi::c_char, __buf: *mut statfs) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn mkstemp(__template: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_LAST_ABI: ffi_abi = 2;
pub const FFI_DEFAULT_ABI: ffi_abi = 1;
pub const FFI_V9: ffi_abi = 1;
pub const FFI_FIRST_ABI: ffi_abi = 0;
pub type ptrdiff_t = isize;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ffi_type {
    pub size: size_t,
    pub alignment: ::core::ffi::c_ushort,
    pub type_0: ::core::ffi::c_ushort,
    pub elements: *mut *mut _ffi_type,
}
pub type ffi_type = _ffi_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_cif {
    pub abi: ffi_abi,
    pub nargs: ::core::ffi::c_uint,
    pub arg_types: *mut *mut ffi_type,
    pub rtype: *mut ffi_type,
    pub bytes: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
    pub nfixedargs: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_closure {
    pub c2rust_unnamed: C2RustUnnamed,
    pub cif: *mut ffi_cif,
    pub fun: Option<
        unsafe extern "C" fn(
            *mut ffi_cif,
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub user_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub tramp: [::core::ffi::c_char; 24],
    pub ftramp: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct malloc_state {
    pub smallmap: binmap_t,
    pub treemap: binmap_t,
    pub dvsize: size_t,
    pub topsize: size_t,
    pub least_addr: *mut ::core::ffi::c_char,
    pub dv: mchunkptr,
    pub top: mchunkptr,
    pub trim_check: size_t,
    pub release_checks: size_t,
    pub magic: size_t,
    pub smallbins: [mchunkptr; 66],
    pub treebins: [tbinptr; 32],
    pub footprint: size_t,
    pub max_footprint: size_t,
    pub footprint_limit: size_t,
    pub mflags: flag_t,
    pub mutex: ::core::ffi::c_int,
    pub seg: msegment,
    pub extp: *mut ::core::ffi::c_void,
    pub exts: size_t,
}
pub type msegment = malloc_segment;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct malloc_segment {
    pub base: *mut ::core::ffi::c_char,
    pub size: size_t,
    pub next: *mut malloc_segment,
    pub exec_offset: ptrdiff_t,
}
pub type flag_t = ::core::ffi::c_uint;
pub type tbinptr = *mut malloc_tree_chunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct malloc_tree_chunk {
    pub prev_foot: size_t,
    pub head: size_t,
    pub fd: *mut malloc_tree_chunk,
    pub bk: *mut malloc_tree_chunk,
    pub child: [*mut malloc_tree_chunk; 2],
    pub parent: *mut malloc_tree_chunk,
    pub index: bindex_t,
}
pub type bindex_t = ::core::ffi::c_uint;
pub type mchunkptr = *mut malloc_chunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct malloc_chunk {
    pub prev_foot: size_t,
    pub head: size_t,
    pub fd: *mut malloc_chunk,
    pub bk: *mut malloc_chunk,
}
pub type binmap_t = ::core::ffi::c_uint;
pub type mstate = *mut malloc_state;
pub type msegmentptr = *mut malloc_segment;
pub type tchunkptr = *mut malloc_tree_chunk;
pub type mchunk = malloc_chunk;
pub type sbinptr = *mut malloc_chunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct malloc_params {
    pub magic: size_t,
    pub page_size: size_t,
    pub granularity: size_t,
    pub mmap_threshold: size_t,
    pub trim_threshold: size_t,
    pub default_mflags: flag_t,
}
pub type time_t = __time_t;
pub type __time_t = ::core::ffi::c_long;
pub const _SC_PAGESIZE: C2RustUnnamed_1 = 30;
pub type off_t = __off_t;
pub type __off_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_int,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub func: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub arg: *const ::core::ffi::c_char,
    pub repeat: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mntent {
    pub mnt_fsname: *mut ::core::ffi::c_char,
    pub mnt_dir: *mut ::core::ffi::c_char,
    pub mnt_type: *mut ::core::ffi::c_char,
    pub mnt_opts: *mut ::core::ffi::c_char,
    pub mnt_freq: ::core::ffi::c_int,
    pub mnt_passno: ::core::ffi::c_int,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type ssize_t = isize;
pub type __ssize_t = ::core::ffi::c_long;
pub type __fsword_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
    pub __val: [::core::ffi::c_int; 2],
}
pub type __fsfilcnt_t = ::core::ffi::c_ulong;
pub type __fsblkcnt_t = ::core::ffi::c_ulong;
pub const PAX_EMUTRAMP: C2RustUnnamed_3 = 2;
pub const PAX_MPROTECT: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed_1 = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed_1 = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_1 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_1 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_1 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_1 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_1 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_1 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_1 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_1 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_1 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_1 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_1 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_1 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_1 = 236;
pub const _SC_IPV6: C2RustUnnamed_1 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_1 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_1 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_1 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_1 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_1 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_1 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_1 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_1 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_1 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_1 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_1 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_1 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_1 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_1 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_1 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_1 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_1 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_1 = 182;
pub const _SC_TRACE: C2RustUnnamed_1 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_1 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_1 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_1 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_1 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_1 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_1 = 175;
pub const _SC_STREAMS: C2RustUnnamed_1 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_1 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_1 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_1 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_1 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_1 = 169;
pub const _SC_2_PBS: C2RustUnnamed_1 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_1 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_1 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_1 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_1 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_1 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_1 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_1 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_1 = 160;
pub const _SC_SPAWN: C2RustUnnamed_1 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_1 = 158;
pub const _SC_SHELL: C2RustUnnamed_1 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_1 = 156;
pub const _SC_REGEXP: C2RustUnnamed_1 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_1 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_1 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_1 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_1 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_1 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_1 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_1 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_1 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_1 = 146;
pub const _SC_PIPE: C2RustUnnamed_1 = 145;
pub const _SC_FIFO: C2RustUnnamed_1 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_1 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_1 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_1 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_1 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_1 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_1 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_1 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_1 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_1 = 135;
pub const _SC_BASE: C2RustUnnamed_1 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_1 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_1 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_1 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_1 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_1 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_1 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_1 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_1 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_1 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_1 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_1 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_1 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_1 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_1 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_1 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_1 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_1 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_1 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_1 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_1 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_1 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_1 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_1 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_1 = 110;
pub const _SC_NZERO: C2RustUnnamed_1 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_1 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_1 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_1 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_1 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_1 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_1 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_1 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_1 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_1 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_1 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_1 = 98;
pub const _SC_2_UPE: C2RustUnnamed_1 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_1 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_1 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_1 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_1 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_1 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_1 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_1 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_1 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_1 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_1 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_1 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_1 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_1 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_1 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_1 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_1 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_1 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_1 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_1 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_1 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_1 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_1 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_1 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_1 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_1 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_1 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_1 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_1 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_1 = 68;
pub const _SC_THREADS: C2RustUnnamed_1 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_1 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_1 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_1 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_1 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_1 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_1 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_1 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_1 = 60;
pub const _SC_SELECT: C2RustUnnamed_1 = 59;
pub const _SC_POLL: C2RustUnnamed_1 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_1 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_1 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_1 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_1 = 54;
pub const _SC_PII: C2RustUnnamed_1 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_1 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_1 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_1 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_1 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_1 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_1 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_1 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_1 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_1 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_1 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_1 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_1 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_1 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_1 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_1 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_1 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_1 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_1 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_1 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_1 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_1 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_1 = 31;
pub const _SC_VERSION: C2RustUnnamed_1 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_1 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_1 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_1 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_1 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_1 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_1 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_1 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_1 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_1 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_1 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_1 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_1 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_1 = 16;
pub const _SC_FSYNC: C2RustUnnamed_1 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_1 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_1 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_1 = 12;
pub const _SC_TIMERS: C2RustUnnamed_1 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_1 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_1 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_1 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_1 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_1 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_1 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_1 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_1 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_1 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_1 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_2 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_2 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_2 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_2 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_2 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_2 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_2 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const HAVE_MORECORE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DEFAULT_MMAP_THRESHOLD: size_t = MAX_SIZE_T;
pub const EOPNOTSUPP: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
pub const __O_TMPFILE: ::core::ffi::c_int = 0x2010000 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const O_TMPFILE: ::core::ffi::c_int = __O_TMPFILE;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EISDIR: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const W_OK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut ::core::ffi::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PROT_EXEC: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MAP_SHARED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAP_PRIVATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_ANONYMOUS: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MFD_CLOEXEC: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut selinux_enabled: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C" fn selinux_enabled_check() -> ::core::ffi::c_int {
    let mut sfs: statfs = statfs {
        f_type: 0,
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_fsid: __fsid_t { __val: [0; 2] },
        f_namelen: 0,
        f_frsize: 0,
        f_flags: 0,
        f_spare: [0; 4],
    };
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0 as size_t;
    if statfs(
        b"/selinux\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut sfs,
    ) >= 0 as ::core::ffi::c_int
        && sfs.f_type as ::core::ffi::c_uint == 0xf97cff8c as ::core::ffi::c_uint
    {
        return 1 as ::core::ffi::c_int;
    }
    f = fopen(
        b"/proc/mounts\0" as *const u8 as *const ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if f.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    while getline(&raw mut buf, &raw mut len, f) >= 0 as __ssize_t {
        let mut p: *mut ::core::ffi::c_char = strchr(buf, ' ' as i32);
        if p.is_null() {
            break;
        }
        p = strchr(p.offset(1 as ::core::ffi::c_int as isize), ' ' as i32);
        if p.is_null() {
            break;
        }
        if strncmp(
            p.offset(1 as ::core::ffi::c_int as isize),
            b"selinuxfs \0" as *const u8 as *const ::core::ffi::c_char,
            10 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            free(buf as *mut ::core::ffi::c_void);
            fclose(f);
            return 1 as ::core::ffi::c_int;
        }
    }
    free(buf as *mut ::core::ffi::c_void);
    fclose(f);
    return 0 as ::core::ffi::c_int;
}
static mut cached_pax_flags: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C" fn pax_flags_check() -> ::core::ffi::c_int {
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0 as size_t;
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut ret: ::core::ffi::c_int = 0;
    f = fopen(
        b"/proc/self/status\0" as *const u8 as *const ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if f.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    ret = 0 as ::core::ffi::c_int;
    while getline(&raw mut buf, &raw mut len, f) != -(1 as ::core::ffi::c_int) as __ssize_t {
        if !(strncmp(
            buf,
            b"PaX:\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0)
        {
            continue;
        }
        if !strchr(buf.offset(4 as ::core::ffi::c_int as isize), 'M' as i32).is_null() {
            ret |= PAX_MPROTECT as ::core::ffi::c_int;
        }
        if !strchr(buf.offset(4 as ::core::ffi::c_int as isize), 'E' as i32).is_null() {
            ret |= PAX_EMUTRAMP as ::core::ffi::c_int;
        }
        break;
    }
    free(buf as *mut ::core::ffi::c_void);
    fclose(f);
    return ret;
}
pub const MAX_SIZE_T: size_t = !(0 as ::core::ffi::c_int as size_t);
pub const MALLOC_ALIGNMENT: size_t =
    (2 as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize);
pub const HAVE_MMAP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MORECORE_CONTIGUOUS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DEFAULT_TRIM_THRESHOLD: size_t = (2 as ::core::ffi::c_uint as size_t)
    .wrapping_mul(1024 as ::core::ffi::c_uint as size_t)
    .wrapping_mul(1024 as ::core::ffi::c_uint as size_t);
pub const MAX_RELEASE_CHECK_RATE: ::core::ffi::c_int = 4095 as ::core::ffi::c_int;
pub const NO_SEGMENT_TRAVERSAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIZE_T_SIZE: ::core::ffi::c_ulong =
    ::core::mem::size_of::<size_t>() as ::core::ffi::c_ulong;
pub const SIZE_T_BITSIZE: usize =
    (::core::mem::size_of::<size_t>() as usize) << 3 as ::core::ffi::c_int;
pub const SIZE_T_ONE: size_t = 1 as ::core::ffi::c_int as size_t;
pub const SIZE_T_TWO: size_t = 2 as ::core::ffi::c_int as size_t;
pub const SIZE_T_FOUR: size_t = 4 as ::core::ffi::c_int as size_t;
pub const TWO_SIZE_T_SIZES: ::core::ffi::c_ulong = SIZE_T_SIZE << 1 as ::core::ffi::c_int;
pub const FOUR_SIZE_T_SIZES: usize = (SIZE_T_SIZE as usize) << 2 as ::core::ffi::c_int;
pub const HALF_MAX_SIZE_T: size_t = MAX_SIZE_T.wrapping_div(2 as size_t);
pub const CHUNK_ALIGN_MASK: size_t = MALLOC_ALIGNMENT.wrapping_sub(SIZE_T_ONE);
pub const MFAIL: *mut ::core::ffi::c_void =
    !(0 as ::core::ffi::c_int as size_t) as *mut ::core::ffi::c_void;
pub const CMFAIL: *mut ::core::ffi::c_char =
    !(0 as ::core::ffi::c_int as size_t) as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char;
pub const MMAP_PROT: ::core::ffi::c_int = PROT_READ | PROT_WRITE;
pub const MMAP_FLAGS: ::core::ffi::c_int = MAP_PRIVATE | MAP_ANONYMOUS;
pub const USE_MMAP_BIT: size_t = 1 as ::core::ffi::c_int as size_t;
pub const USE_NONCONTIGUOUS_BIT: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const EXTERN_BIT: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
pub const SPINS_PER_YIELD: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
unsafe extern "C" fn spin_acquire_lock(mut sl: *mut ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut spins: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while crate::atomic_compat::atomic_load_relaxed(sl) != 0 as ::core::ffi::c_int
        || crate::atomic_compat::atomic_xchg_acquire(sl, 1 as ::core::ffi::c_int) != 0
    {
        spins += 1;
        if spins & SPINS_PER_YIELD == 0 as ::core::ffi::c_int {
            sched_yield();
        }
    }
    return 0 as ::core::ffi::c_int;
}
static mut malloc_global_mutex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const USE_LOCK_BIT: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const MCHUNK_SIZE: ::core::ffi::c_ulong =
    ::core::mem::size_of::<mchunk>() as ::core::ffi::c_ulong;
pub const CHUNK_OVERHEAD: ::core::ffi::c_ulong =
    ::core::mem::size_of::<size_t>() as ::core::ffi::c_ulong;
pub const MMAP_FOOT_PAD: usize = (SIZE_T_SIZE as usize) << 2 as ::core::ffi::c_int;
pub const MIN_CHUNK_SIZE: ::core::ffi::c_ulong = MCHUNK_SIZE
    .wrapping_add(CHUNK_ALIGN_MASK as ::core::ffi::c_ulong)
    & !(CHUNK_ALIGN_MASK as ::core::ffi::c_ulong);
pub const MAX_REQUEST: ::core::ffi::c_ulong =
    MIN_CHUNK_SIZE.wrapping_neg() << 2 as ::core::ffi::c_int;
pub const MIN_REQUEST: ::core::ffi::c_ulong = MIN_CHUNK_SIZE
    .wrapping_sub(CHUNK_OVERHEAD)
    .wrapping_sub(SIZE_T_ONE as ::core::ffi::c_ulong);
pub const PINUSE_BIT: size_t = 1 as ::core::ffi::c_int as size_t;
pub const CINUSE_BIT: size_t = 2 as ::core::ffi::c_int as size_t;
pub const FLAG4_BIT: size_t = 4 as ::core::ffi::c_int as size_t;
pub const INUSE_BITS: size_t = PINUSE_BIT | CINUSE_BIT;
pub const FLAG_BITS: size_t = PINUSE_BIT | CINUSE_BIT | FLAG4_BIT;
pub const FENCEPOST_HEAD: ::core::ffi::c_ulong = INUSE_BITS as ::core::ffi::c_ulong | SIZE_T_SIZE;
pub const NSMALLBINS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const NTREEBINS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const SMALLBIN_SHIFT: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TREEBIN_SHIFT: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
pub const MIN_LARGE_SIZE: size_t = SIZE_T_ONE << TREEBIN_SHIFT;
pub const MAX_SMALL_SIZE: size_t = MIN_LARGE_SIZE.wrapping_sub(SIZE_T_ONE);
pub const MAX_SMALL_REQUEST: size_t = MAX_SMALL_SIZE
    .wrapping_sub(CHUNK_ALIGN_MASK)
    .wrapping_sub(CHUNK_OVERHEAD as size_t);
static mut mparams: malloc_params = malloc_params {
    magic: 0,
    page_size: 0,
    granularity: 0,
    mmap_threshold: 0,
    trim_threshold: 0,
    default_mflags: 0,
};
static mut _gm_: malloc_state = malloc_state {
    smallmap: 0,
    treemap: 0,
    dvsize: 0,
    topsize: 0,
    least_addr: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    dv: ::core::ptr::null::<malloc_chunk>() as *mut malloc_chunk,
    top: ::core::ptr::null::<malloc_chunk>() as *mut malloc_chunk,
    trim_check: 0,
    release_checks: 0,
    magic: 0,
    smallbins: [::core::ptr::null::<malloc_chunk>() as *mut malloc_chunk; 66],
    treebins: [::core::ptr::null::<malloc_tree_chunk>() as *mut malloc_tree_chunk; 32],
    footprint: 0,
    max_footprint: 0,
    footprint_limit: 0,
    mflags: 0,
    mutex: 0,
    seg: malloc_segment {
        base: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        size: 0,
        next: ::core::ptr::null::<malloc_segment>() as *mut malloc_segment,
        exec_offset: 0,
    },
    extp: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    exts: 0,
};
unsafe extern "C" fn segment_holding(
    mut m: mstate,
    mut addr: *mut ::core::ffi::c_char,
) -> msegmentptr {
    let mut sp: msegmentptr = &raw mut (*m).seg;
    loop {
        if addr >= (*sp).base && addr < (*sp).base.offset((*sp).size as isize) {
            return sp;
        }
        sp = (*sp).next as msegmentptr;
        if sp.is_null() {
            return ::core::ptr::null_mut::<malloc_segment>();
        }
    }
}
unsafe extern "C" fn has_segment_link(mut m: mstate, mut ss: msegmentptr) -> ::core::ffi::c_int {
    let mut sp: msegmentptr = &raw mut (*m).seg;
    loop {
        if sp as *mut ::core::ffi::c_char >= (*ss).base
            && (sp as *mut ::core::ffi::c_char) < (*ss).base.offset((*ss).size as isize)
        {
            return 1 as ::core::ffi::c_int;
        }
        sp = (*sp).next as msegmentptr;
        if sp.is_null() {
            return 0 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn init_mparams() -> ::core::ffi::c_int {
    if crate::atomic_compat::atomic_xchg_acquire(
        &raw mut malloc_global_mutex,
        1 as ::core::ffi::c_int,
    ) != 0
    {
        spin_acquire_lock(&raw mut malloc_global_mutex);
    } else {
    };
    if mparams.magic == 0 as size_t {
        let mut magic: size_t = 0;
        let mut psize: size_t = 0;
        let mut gsize: size_t = 0;
        psize = sysconf(_SC_PAGESIZE as ::core::ffi::c_int) as size_t;
        gsize = if sysconf(_SC_PAGESIZE as ::core::ffi::c_int) as size_t != 0 as size_t {
            sysconf(_SC_PAGESIZE as ::core::ffi::c_int) as size_t
        } else {
            psize
        };
        if ::core::mem::size_of::<size_t>() as usize
            != ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize
            || MAX_SIZE_T < MIN_CHUNK_SIZE as usize
            || (::core::mem::size_of::<::core::ffi::c_int>() as usize) < 4 as usize
            || MALLOC_ALIGNMENT < 8 as ::core::ffi::c_uint as size_t
            || MALLOC_ALIGNMENT & MALLOC_ALIGNMENT.wrapping_sub(SIZE_T_ONE) != 0 as size_t
            || MCHUNK_SIZE as usize & (MCHUNK_SIZE as usize).wrapping_sub(SIZE_T_ONE) != 0 as usize
            || gsize & gsize.wrapping_sub(SIZE_T_ONE) != 0 as size_t
            || psize & psize.wrapping_sub(SIZE_T_ONE) != 0 as size_t
        {
            abort();
        }
        mparams.granularity = gsize;
        mparams.page_size = psize;
        mparams.mmap_threshold = DEFAULT_MMAP_THRESHOLD;
        mparams.trim_threshold = DEFAULT_TRIM_THRESHOLD;
        mparams.default_mflags =
            (USE_LOCK_BIT as size_t | USE_MMAP_BIT | USE_NONCONTIGUOUS_BIT as size_t) as flag_t;
        _gm_.mflags = mparams.default_mflags;
        _gm_.mutex = 0 as ::core::ffi::c_int;
        magic = time(::core::ptr::null_mut::<time_t>()) as size_t
            ^ 0x55555555 as ::core::ffi::c_uint as size_t;
        magic |= 8 as ::core::ffi::c_uint as size_t;
        magic &= !(7 as ::core::ffi::c_uint as size_t);
        crate::atomic_compat::atomic_store_release(&raw mut mparams.magic, magic);
    }
    crate::atomic_compat::atomic_store_release(&raw mut malloc_global_mutex, 0);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn mmap_alloc(mut m: mstate, mut nb: size_t) -> *mut ::core::ffi::c_void {
    let mut mmsize: size_t = nb
        .wrapping_add(
            ((::core::mem::size_of::<size_t>() as size_t) << 2 as ::core::ffi::c_int).wrapping_add(
                (::core::mem::size_of::<size_t>() as size_t) << 1 as ::core::ffi::c_int,
            ),
        )
        .wrapping_add(
            (2 as usize)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                .wrapping_sub(1 as ::core::ffi::c_int as size_t),
        )
        .wrapping_add(mparams.page_size.wrapping_sub(SIZE_T_ONE))
        & !mparams.page_size.wrapping_sub(SIZE_T_ONE);
    if (*m).footprint_limit != 0 as size_t {
        let mut fp: size_t = (*m).footprint.wrapping_add(mmsize);
        if fp <= (*m).footprint || fp > (*m).footprint_limit {
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
    }
    if mmsize > nb {
        let mut mm: *mut ::core::ffi::c_char = dlmmap(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            mmsize,
            MMAP_PROT,
            MMAP_FLAGS,
            -(1 as ::core::ffi::c_int),
            0 as off_t,
        ) as *mut ::core::ffi::c_char;
        if mm != CMFAIL {
            let mut offset: size_t = if mm.offset(
                ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_void as size_t
                & CHUNK_ALIGN_MASK
                == 0 as size_t
            {
                0 as size_t
            } else {
                MALLOC_ALIGNMENT.wrapping_sub(
                    mm.offset(
                        ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut ::core::ffi::c_void as size_t
                        & CHUNK_ALIGN_MASK,
                ) & CHUNK_ALIGN_MASK
            };
            let mut psize: size_t = mmsize.wrapping_sub(offset).wrapping_sub(MMAP_FOOT_PAD);
            let mut p: mchunkptr = mm.offset(offset as isize) as mchunkptr;
            (*p).prev_foot = offset;
            (*p).head = psize;
            (*((p as *mut ::core::ffi::c_char).offset(psize as isize) as mchunkptr)).head =
                FENCEPOST_HEAD as size_t;
            (*((p as *mut ::core::ffi::c_char)
                .offset(psize.wrapping_add(::core::mem::size_of::<size_t>() as size_t) as isize)
                as mchunkptr))
                .head = 0 as size_t;
            if (*m).least_addr.is_null() || mm < (*m).least_addr {
                (*m).least_addr = mm;
            }
            (*m).footprint = (*m).footprint.wrapping_add(mmsize);
            if (*m).footprint > (*m).max_footprint {
                (*m).max_footprint = (*m).footprint;
            }
            return (p as *mut ::core::ffi::c_char).offset(TWO_SIZE_T_SIZES as usize as isize)
                as *mut ::core::ffi::c_void;
        }
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn mmap_resize(
    mut m: mstate,
    mut oldp: mchunkptr,
    mut nb: size_t,
    mut flags: ::core::ffi::c_int,
) -> mchunkptr {
    let mut oldsize: size_t = (*oldp).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT);
    if nb >> SMALLBIN_SHIFT < NSMALLBINS as size_t {
        return ::core::ptr::null_mut::<malloc_chunk>();
    }
    if oldsize >= nb.wrapping_add(SIZE_T_SIZE as size_t)
        && oldsize.wrapping_sub(nb) <= mparams.granularity << 1 as ::core::ffi::c_int
    {
        return oldp;
    } else {
        let mut offset: size_t = (*oldp).prev_foot;
        let mut oldmmsize: size_t = oldsize.wrapping_add(offset).wrapping_add(MMAP_FOOT_PAD);
        let mut newmmsize: size_t = nb
            .wrapping_add(
                ((::core::mem::size_of::<size_t>() as size_t) << 2 as ::core::ffi::c_int)
                    .wrapping_add(
                        (::core::mem::size_of::<size_t>() as size_t) << 1 as ::core::ffi::c_int,
                    ),
            )
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                    .wrapping_sub(1 as ::core::ffi::c_int as size_t),
            )
            .wrapping_add(mparams.page_size.wrapping_sub(SIZE_T_ONE))
            & !mparams.page_size.wrapping_sub(SIZE_T_ONE);
        let mut cp: *mut ::core::ffi::c_char = !(0 as ::core::ffi::c_int as size_t)
            as *mut ::core::ffi::c_void
            as *mut ::core::ffi::c_char;
        if cp != CMFAIL {
            let mut newp: mchunkptr = cp.offset(offset as isize) as mchunkptr;
            let mut psize: size_t = newmmsize.wrapping_sub(offset).wrapping_sub(MMAP_FOOT_PAD);
            (*newp).head = psize;
            (*((newp as *mut ::core::ffi::c_char).offset(psize as isize) as mchunkptr)).head =
                FENCEPOST_HEAD as size_t;
            (*((newp as *mut ::core::ffi::c_char)
                .offset(psize.wrapping_add(::core::mem::size_of::<size_t>() as size_t) as isize)
                as mchunkptr))
                .head = 0 as size_t;
            if cp < (*m).least_addr {
                (*m).least_addr = cp;
            }
            (*m).footprint = (*m)
                .footprint
                .wrapping_add(newmmsize.wrapping_sub(oldmmsize));
            if (*m).footprint > (*m).max_footprint {
                (*m).max_footprint = (*m).footprint;
            }
            return newp;
        }
    }
    return ::core::ptr::null_mut::<malloc_chunk>();
}
unsafe extern "C" fn init_top(mut m: mstate, mut p: mchunkptr, mut psize: size_t) {
    let mut offset: size_t = if (p as *mut ::core::ffi::c_char)
        .offset(((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_void as size_t
        & CHUNK_ALIGN_MASK
        == 0 as size_t
    {
        0 as size_t
    } else {
        MALLOC_ALIGNMENT.wrapping_sub(
            (p as *mut ::core::ffi::c_char).offset(
                ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_void as size_t
                & CHUNK_ALIGN_MASK,
        ) & CHUNK_ALIGN_MASK
    };
    p = (p as *mut ::core::ffi::c_char).offset(offset as isize) as mchunkptr;
    psize = psize.wrapping_sub(offset);
    (*m).top = p;
    (*m).topsize = psize;
    (*p).head = psize | PINUSE_BIT;
    (*((p as *mut ::core::ffi::c_char).offset(psize as isize) as mchunkptr)).head =
        (if ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
            ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int) as isize,
        ) as *mut ::core::ffi::c_void as size_t
            & CHUNK_ALIGN_MASK
            == 0 as size_t
        {
            0 as size_t
        } else {
            MALLOC_ALIGNMENT.wrapping_sub(
                ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                    ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                        as isize,
                ) as *mut ::core::ffi::c_void as size_t
                    & CHUNK_ALIGN_MASK,
            ) & CHUNK_ALIGN_MASK
        })
        .wrapping_add(
            (::core::mem::size_of::<malloc_segment>() as size_t)
                .wrapping_add(CHUNK_OVERHEAD as size_t)
                .wrapping_add(CHUNK_ALIGN_MASK)
                & !CHUNK_ALIGN_MASK,
        )
        .wrapping_add(MIN_CHUNK_SIZE as size_t);
    (*m).trim_check = mparams.trim_threshold;
}
unsafe extern "C" fn init_bins(mut m: mstate) {
    let mut i: bindex_t = 0;
    i = 0 as bindex_t;
    while i < NSMALLBINS {
        let mut bin: sbinptr = (&raw mut (*m).smallbins as *mut mchunkptr)
            .offset((i << 1 as ::core::ffi::c_int) as isize)
            as *mut mchunkptr as *mut ::core::ffi::c_char as sbinptr;
        (*bin).bk = bin as *mut malloc_chunk;
        (*bin).fd = (*bin).bk;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn prepend_alloc(
    mut m: mstate,
    mut newbase: *mut ::core::ffi::c_char,
    mut oldbase: *mut ::core::ffi::c_char,
    mut nb: size_t,
) -> *mut ::core::ffi::c_void {
    let mut p: mchunkptr = newbase.offset(
        (if newbase.offset(
            ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int) as isize,
        ) as *mut ::core::ffi::c_void as size_t
            & CHUNK_ALIGN_MASK
            == 0 as size_t
        {
            0 as size_t
        } else {
            MALLOC_ALIGNMENT.wrapping_sub(
                newbase.offset(
                    ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                        as isize,
                ) as *mut ::core::ffi::c_void as size_t
                    & CHUNK_ALIGN_MASK,
            ) & CHUNK_ALIGN_MASK
        }) as isize,
    ) as mchunkptr;
    let mut oldfirst: mchunkptr = oldbase.offset(
        (if oldbase.offset(
            ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int) as isize,
        ) as *mut ::core::ffi::c_void as size_t
            & CHUNK_ALIGN_MASK
            == 0 as size_t
        {
            0 as size_t
        } else {
            MALLOC_ALIGNMENT.wrapping_sub(
                oldbase.offset(
                    ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                        as isize,
                ) as *mut ::core::ffi::c_void as size_t
                    & CHUNK_ALIGN_MASK,
            ) & CHUNK_ALIGN_MASK
        }) as isize,
    ) as mchunkptr;
    let mut psize: size_t = (oldfirst as *mut ::core::ffi::c_char)
        .offset_from(p as *mut ::core::ffi::c_char)
        as ::core::ffi::c_long as size_t;
    let mut q: mchunkptr = (p as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr;
    let mut qsize: size_t = psize.wrapping_sub(nb);
    (*p).head = nb | PINUSE_BIT | CINUSE_BIT;
    if oldfirst == (*m).top {
        (*m).topsize = (*m).topsize.wrapping_add(qsize);
        let mut tsize: size_t = (*m).topsize;
        (*m).top = q;
        (*q).head = tsize | PINUSE_BIT;
    } else if oldfirst == (*m).dv {
        (*m).dvsize = (*m).dvsize.wrapping_add(qsize);
        let mut dsize: size_t = (*m).dvsize;
        (*m).dv = q;
        (*q).head = dsize | PINUSE_BIT;
        (*((q as *mut ::core::ffi::c_char).offset(dsize as isize) as mchunkptr)).prev_foot = dsize;
    } else {
        if !((*oldfirst).head & INUSE_BITS != PINUSE_BIT) {
            let mut nsize: size_t = (*oldfirst).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT);
            if nsize >> SMALLBIN_SHIFT < NSMALLBINS as size_t {
                let mut F: mchunkptr = (*oldfirst).fd as mchunkptr;
                let mut B: mchunkptr = (*oldfirst).bk as mchunkptr;
                let mut I: bindex_t = (nsize >> SMALLBIN_SHIFT) as bindex_t;
                if (F
                    == (&raw mut (*m).smallbins as *mut mchunkptr)
                        .offset((I << 1 as ::core::ffi::c_int) as isize)
                        as *mut mchunkptr as *mut ::core::ffi::c_char
                        as sbinptr
                    || F as *mut ::core::ffi::c_char >= (*m).least_addr && (*F).bk == oldfirst)
                    as ::core::ffi::c_int as ::core::ffi::c_long
                    != 0
                {
                    if B == F {
                        (*m).smallmap &= !((1 as ::core::ffi::c_int as binmap_t) << I);
                    } else if (B
                        == (&raw mut (*m).smallbins as *mut mchunkptr)
                            .offset((I << 1 as ::core::ffi::c_int) as isize)
                            as *mut mchunkptr as *mut ::core::ffi::c_char
                            as sbinptr
                        || B as *mut ::core::ffi::c_char >= (*m).least_addr && (*B).fd == oldfirst)
                        as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        (*F).bk = B as *mut malloc_chunk;
                        (*B).fd = F as *mut malloc_chunk;
                    } else {
                        abort();
                    }
                } else {
                    abort();
                }
            } else {
                let mut TP: tchunkptr = oldfirst as tchunkptr;
                let mut XP: tchunkptr = (*TP).parent as tchunkptr;
                let mut R: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
                if (*TP).bk != TP {
                    let mut F_0: tchunkptr = (*TP).fd as tchunkptr;
                    R = (*TP).bk as tchunkptr;
                    if (F_0 as *mut ::core::ffi::c_char >= (*m).least_addr
                        && (*F_0).bk == TP
                        && (*R).fd == TP) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        (*F_0).bk = R as *mut malloc_tree_chunk;
                        (*R).fd = F_0 as *mut malloc_tree_chunk;
                    } else {
                        abort();
                    }
                } else {
                    let mut RP: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                    RP = (&raw mut (*TP).child as *mut *mut malloc_tree_chunk)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut *mut malloc_tree_chunk as *mut tchunkptr;
                    R = *RP;
                    if !R.is_null() || {
                        RP = (&raw mut (*TP).child as *mut *mut malloc_tree_chunk)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut *mut malloc_tree_chunk
                            as *mut tchunkptr;
                        R = *RP;
                        !R.is_null()
                    } {
                        let mut CP: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                        loop {
                            CP = (&raw mut (*R).child as *mut *mut malloc_tree_chunk)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *mut *mut malloc_tree_chunk
                                as *mut tchunkptr;
                            if !(!(*CP).is_null() || {
                                CP = (&raw mut (*R).child as *mut *mut malloc_tree_chunk)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut *mut malloc_tree_chunk
                                    as *mut tchunkptr;
                                !(*CP).is_null()
                            }) {
                                break;
                            }
                            RP = CP;
                            R = *RP;
                        }
                        if (RP as *mut ::core::ffi::c_char >= (*m).least_addr) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            *RP = ::core::ptr::null_mut::<malloc_tree_chunk>();
                        } else {
                            abort();
                        }
                    }
                }
                if !XP.is_null() {
                    let mut H: *mut tbinptr = (&raw mut (*m).treebins as *mut tbinptr)
                        .offset((*TP).index as isize)
                        as *mut tbinptr;
                    if TP == *H {
                        *H = R as tbinptr;
                        if (*H).is_null() {
                            (*m).treemap &= !((1 as ::core::ffi::c_int as binmap_t) << (*TP).index);
                        }
                    } else if (XP as *mut ::core::ffi::c_char >= (*m).least_addr)
                        as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        if (*XP).child[0 as ::core::ffi::c_int as usize] == TP {
                            (*XP).child[0 as ::core::ffi::c_int as usize] =
                                R as *mut malloc_tree_chunk;
                        } else {
                            (*XP).child[1 as ::core::ffi::c_int as usize] =
                                R as *mut malloc_tree_chunk;
                        }
                    } else {
                        abort();
                    }
                    if !R.is_null() {
                        if (R as *mut ::core::ffi::c_char >= (*m).least_addr) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            let mut C0: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
                            let mut C1: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
                            (*R).parent = XP as *mut malloc_tree_chunk;
                            C0 = (*TP).child[0 as ::core::ffi::c_int as usize] as tchunkptr;
                            if !C0.is_null() {
                                if (C0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_long
                                    != 0
                                {
                                    (*R).child[0 as ::core::ffi::c_int as usize] =
                                        C0 as *mut malloc_tree_chunk;
                                    (*C0).parent = R as *mut malloc_tree_chunk;
                                } else {
                                    abort();
                                }
                            }
                            C1 = (*TP).child[1 as ::core::ffi::c_int as usize] as tchunkptr;
                            if !C1.is_null() {
                                if (C1 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_long
                                    != 0
                                {
                                    (*R).child[1 as ::core::ffi::c_int as usize] =
                                        C1 as *mut malloc_tree_chunk;
                                    (*C1).parent = R as *mut malloc_tree_chunk;
                                } else {
                                    abort();
                                }
                            }
                        } else {
                            abort();
                        }
                    }
                }
            }
            oldfirst = (oldfirst as *mut ::core::ffi::c_char).offset(nsize as isize) as mchunkptr;
            qsize = qsize.wrapping_add(nsize);
        }
        (*oldfirst).head &= !PINUSE_BIT;
        (*q).head = qsize | PINUSE_BIT;
        (*((q as *mut ::core::ffi::c_char).offset(qsize as isize) as mchunkptr)).prev_foot = qsize;
        if qsize >> SMALLBIN_SHIFT < NSMALLBINS as size_t {
            let mut I_0: bindex_t = (qsize >> SMALLBIN_SHIFT) as bindex_t;
            let mut B_0: mchunkptr = (&raw mut (*m).smallbins as *mut mchunkptr)
                .offset((I_0 << 1 as ::core::ffi::c_int) as isize)
                as *mut mchunkptr as *mut ::core::ffi::c_char
                as mchunkptr;
            let mut F_1: mchunkptr = B_0;
            if (*m).smallmap & (1 as ::core::ffi::c_int as binmap_t) << I_0 == 0 {
                (*m).smallmap |= (1 as ::core::ffi::c_int as binmap_t) << I_0;
            } else if ((*B_0).fd as *mut ::core::ffi::c_char >= (*m).least_addr)
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                F_1 = (*B_0).fd as mchunkptr;
            } else {
                abort();
            }
            (*B_0).fd = q as *mut malloc_chunk;
            (*F_1).bk = q as *mut malloc_chunk;
            (*q).fd = F_1 as *mut malloc_chunk;
            (*q).bk = B_0 as *mut malloc_chunk;
        } else {
            let mut TP_0: tchunkptr = q as tchunkptr;
            let mut H_0: *mut tbinptr = ::core::ptr::null_mut::<tbinptr>();
            let mut I_1: bindex_t = 0;
            let mut X: ::core::ffi::c_uint = (qsize >> TREEBIN_SHIFT) as ::core::ffi::c_uint;
            if X == 0 as ::core::ffi::c_uint {
                I_1 = 0 as bindex_t;
            } else if X > 0xffff as ::core::ffi::c_uint {
                I_1 = NTREEBINS.wrapping_sub(1 as ::core::ffi::c_uint) as bindex_t;
            } else {
                let mut K: ::core::ffi::c_uint = (::core::mem::size_of::<::core::ffi::c_uint>()
                    as ::core::ffi::c_uint)
                    .wrapping_mul(__CHAR_BIT__ as ::core::ffi::c_uint)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    .wrapping_sub(X.leading_zeros() as i32 as ::core::ffi::c_uint);
                I_1 = ((K << 1 as ::core::ffi::c_int) as size_t).wrapping_add(
                    qsize >> K.wrapping_add(TREEBIN_SHIFT.wrapping_sub(1 as ::core::ffi::c_uint))
                        & 1 as size_t,
                ) as bindex_t;
            }
            H_0 = (&raw mut (*m).treebins as *mut tbinptr).offset(I_1 as isize) as *mut tbinptr;
            (*TP_0).index = I_1;
            (*TP_0).child[1 as ::core::ffi::c_int as usize] =
                ::core::ptr::null_mut::<malloc_tree_chunk>();
            (*TP_0).child[0 as ::core::ffi::c_int as usize] =
                (*TP_0).child[1 as ::core::ffi::c_int as usize];
            if (*m).treemap & (1 as ::core::ffi::c_int as binmap_t) << I_1 == 0 {
                (*m).treemap |= (1 as ::core::ffi::c_int as binmap_t) << I_1;
                *H_0 = TP_0 as tbinptr;
                (*TP_0).parent = H_0 as tchunkptr as *mut malloc_tree_chunk;
                (*TP_0).bk = TP_0 as *mut malloc_tree_chunk;
                (*TP_0).fd = (*TP_0).bk;
            } else {
                let mut T: tchunkptr = *H_0;
                let mut K_0: size_t = qsize
                    << (if I_1 == NTREEBINS.wrapping_sub(1 as ::core::ffi::c_uint) {
                        0 as usize
                    } else {
                        SIZE_T_BITSIZE.wrapping_sub(SIZE_T_ONE).wrapping_sub(
                            (I_1 as ::core::ffi::c_uint >> 1 as ::core::ffi::c_int)
                                .wrapping_add(TREEBIN_SHIFT)
                                .wrapping_sub(2 as ::core::ffi::c_uint)
                                as usize,
                        )
                    });
                loop {
                    if (*T).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT) != qsize {
                        let mut C: *mut tchunkptr =
                            (&raw mut (*T).child as *mut *mut malloc_tree_chunk).offset(
                                (K_0 >> SIZE_T_BITSIZE.wrapping_sub(SIZE_T_ONE) & 1 as size_t)
                                    as isize,
                            ) as *mut tchunkptr;
                        K_0 <<= 1 as ::core::ffi::c_int;
                        if !(*C).is_null() {
                            T = *C;
                        } else if (C as *mut ::core::ffi::c_char >= (*m).least_addr)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            *C = TP_0;
                            (*TP_0).parent = T as *mut malloc_tree_chunk;
                            (*TP_0).bk = TP_0 as *mut malloc_tree_chunk;
                            (*TP_0).fd = (*TP_0).bk;
                            break;
                        } else {
                            abort();
                        }
                    } else {
                        let mut F_2: tchunkptr = (*T).fd as tchunkptr;
                        if (T as *mut ::core::ffi::c_char >= (*m).least_addr
                            && F_2 as *mut ::core::ffi::c_char >= (*m).least_addr)
                            as ::core::ffi::c_int as ::core::ffi::c_long
                            != 0
                        {
                            (*F_2).bk = TP_0 as *mut malloc_tree_chunk;
                            (*T).fd = (*F_2).bk;
                            (*TP_0).fd = F_2 as *mut malloc_tree_chunk;
                            (*TP_0).bk = T as *mut malloc_tree_chunk;
                            (*TP_0).parent = ::core::ptr::null_mut::<malloc_tree_chunk>();
                            break;
                        } else {
                            abort();
                        }
                    }
                }
            }
        }
    }
    return (p as *mut ::core::ffi::c_char).offset(TWO_SIZE_T_SIZES as usize as isize)
        as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn add_segment(
    mut m: mstate,
    mut tbase: *mut ::core::ffi::c_char,
    mut tsize: size_t,
    mut mmapped: flag_t,
) {
    let mut old_top: *mut ::core::ffi::c_char = (*m).top as *mut ::core::ffi::c_char;
    let mut oldsp: msegmentptr = segment_holding(m, old_top);
    let mut old_end: *mut ::core::ffi::c_char = (*oldsp).base.offset((*oldsp).size as isize);
    let mut ssize: size_t = (::core::mem::size_of::<malloc_segment>() as size_t)
        .wrapping_add(CHUNK_OVERHEAD as size_t)
        .wrapping_add(CHUNK_ALIGN_MASK)
        & !CHUNK_ALIGN_MASK;
    let mut rawsp: *mut ::core::ffi::c_char = old_end.offset(
        -(ssize
            .wrapping_add(FOUR_SIZE_T_SIZES)
            .wrapping_add(CHUNK_ALIGN_MASK) as isize),
    );
    let mut offset: size_t = if rawsp
        .offset(((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_void as size_t
        & CHUNK_ALIGN_MASK
        == 0 as size_t
    {
        0 as size_t
    } else {
        MALLOC_ALIGNMENT.wrapping_sub(
            rawsp.offset(
                ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_void as size_t
                & CHUNK_ALIGN_MASK,
        ) & CHUNK_ALIGN_MASK
    };
    let mut asp: *mut ::core::ffi::c_char = rawsp.offset(offset as isize);
    let mut csp: *mut ::core::ffi::c_char =
        if asp < old_top.offset(MIN_CHUNK_SIZE as usize as isize) {
            old_top
        } else {
            asp
        };
    let mut sp: mchunkptr = csp as mchunkptr;
    let mut ss: msegmentptr = (sp as *mut ::core::ffi::c_char)
        .offset(TWO_SIZE_T_SIZES as usize as isize)
        as *mut ::core::ffi::c_void as msegmentptr;
    let mut tnext: mchunkptr = (sp as *mut ::core::ffi::c_char).offset(ssize as isize) as mchunkptr;
    let mut p: mchunkptr = tnext;
    let mut nfences: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    init_top(
        m,
        tbase as mchunkptr,
        tsize.wrapping_sub(
            (if ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_void as size_t
                & CHUNK_ALIGN_MASK
                == 0 as size_t
            {
                0 as size_t
            } else {
                MALLOC_ALIGNMENT.wrapping_sub(
                    ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                        ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut ::core::ffi::c_void as size_t
                        & CHUNK_ALIGN_MASK,
                ) & CHUNK_ALIGN_MASK
            })
            .wrapping_add(
                (::core::mem::size_of::<malloc_segment>() as size_t)
                    .wrapping_add(CHUNK_OVERHEAD as size_t)
                    .wrapping_add(CHUNK_ALIGN_MASK)
                    & !CHUNK_ALIGN_MASK,
            )
            .wrapping_add(MIN_CHUNK_SIZE as size_t),
        ),
    );
    (*sp).head = ssize | PINUSE_BIT | CINUSE_BIT;
    *ss = (*m).seg as malloc_segment;
    (*m).seg.base = tbase;
    (*m).seg.size = tsize;
    if mmapped as size_t != USE_MMAP_BIT {
        abort();
    } else {
        (*m).seg.exec_offset = *((*m)
            .seg
            .base
            .offset((*m).seg.size as isize)
            .offset(-(::core::mem::size_of::<ptrdiff_t>() as usize as isize))
            as *mut ptrdiff_t);
        if *((*m)
            .seg
            .base
            .offset((*m).seg.exec_offset as isize)
            .offset((*m).seg.size as isize)
            .offset(-(::core::mem::size_of::<ptrdiff_t>() as usize as isize))
            as *mut ptrdiff_t)
            != (*m).seg.exec_offset
        {
            abort();
        } else {
            *((*m)
                .seg
                .base
                .offset((*m).seg.size as isize)
                .offset(-(::core::mem::size_of::<ptrdiff_t>() as usize as isize))
                as *mut ptrdiff_t) = 0 as ptrdiff_t;
        };
    };
    (*m).seg.next = ss as *mut malloc_segment;
    loop {
        let mut nextp: mchunkptr = (p as *mut ::core::ffi::c_char)
            .offset(::core::mem::size_of::<size_t>() as usize as isize)
            as mchunkptr;
        (*p).head = FENCEPOST_HEAD as size_t;
        nfences += 1;
        if !((&raw mut (*nextp).head as *mut ::core::ffi::c_char) < old_end) {
            break;
        }
        p = nextp;
    }
    if csp != old_top {
        let mut q: mchunkptr = old_top as mchunkptr;
        let mut psize: size_t = csp.offset_from(old_top) as ::core::ffi::c_long as size_t;
        let mut tn: mchunkptr = (q as *mut ::core::ffi::c_char).offset(psize as isize) as mchunkptr;
        (*tn).head &= !PINUSE_BIT;
        (*q).head = psize | PINUSE_BIT;
        (*((q as *mut ::core::ffi::c_char).offset(psize as isize) as mchunkptr)).prev_foot = psize;
        if psize >> SMALLBIN_SHIFT < NSMALLBINS as size_t {
            let mut I: bindex_t = (psize >> SMALLBIN_SHIFT) as bindex_t;
            let mut B: mchunkptr = (&raw mut (*m).smallbins as *mut mchunkptr)
                .offset((I << 1 as ::core::ffi::c_int) as isize)
                as *mut mchunkptr as *mut ::core::ffi::c_char
                as mchunkptr;
            let mut F: mchunkptr = B;
            if (*m).smallmap & (1 as ::core::ffi::c_int as binmap_t) << I == 0 {
                (*m).smallmap |= (1 as ::core::ffi::c_int as binmap_t) << I;
            } else if ((*B).fd as *mut ::core::ffi::c_char >= (*m).least_addr) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
            {
                F = (*B).fd as mchunkptr;
            } else {
                abort();
            }
            (*B).fd = q as *mut malloc_chunk;
            (*F).bk = q as *mut malloc_chunk;
            (*q).fd = F as *mut malloc_chunk;
            (*q).bk = B as *mut malloc_chunk;
        } else {
            let mut TP: tchunkptr = q as tchunkptr;
            let mut H: *mut tbinptr = ::core::ptr::null_mut::<tbinptr>();
            let mut I_0: bindex_t = 0;
            let mut X: ::core::ffi::c_uint = (psize >> TREEBIN_SHIFT) as ::core::ffi::c_uint;
            if X == 0 as ::core::ffi::c_uint {
                I_0 = 0 as bindex_t;
            } else if X > 0xffff as ::core::ffi::c_uint {
                I_0 = NTREEBINS.wrapping_sub(1 as ::core::ffi::c_uint) as bindex_t;
            } else {
                let mut K: ::core::ffi::c_uint = (::core::mem::size_of::<::core::ffi::c_uint>()
                    as ::core::ffi::c_uint)
                    .wrapping_mul(__CHAR_BIT__ as ::core::ffi::c_uint)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    .wrapping_sub(X.leading_zeros() as i32 as ::core::ffi::c_uint);
                I_0 = ((K << 1 as ::core::ffi::c_int) as size_t).wrapping_add(
                    psize >> K.wrapping_add(TREEBIN_SHIFT.wrapping_sub(1 as ::core::ffi::c_uint))
                        & 1 as size_t,
                ) as bindex_t;
            }
            H = (&raw mut (*m).treebins as *mut tbinptr).offset(I_0 as isize) as *mut tbinptr;
            (*TP).index = I_0;
            (*TP).child[1 as ::core::ffi::c_int as usize] =
                ::core::ptr::null_mut::<malloc_tree_chunk>();
            (*TP).child[0 as ::core::ffi::c_int as usize] =
                (*TP).child[1 as ::core::ffi::c_int as usize];
            if (*m).treemap & (1 as ::core::ffi::c_int as binmap_t) << I_0 == 0 {
                (*m).treemap |= (1 as ::core::ffi::c_int as binmap_t) << I_0;
                *H = TP as tbinptr;
                (*TP).parent = H as tchunkptr as *mut malloc_tree_chunk;
                (*TP).bk = TP as *mut malloc_tree_chunk;
                (*TP).fd = (*TP).bk;
            } else {
                let mut T: tchunkptr = *H;
                let mut K_0: size_t = psize
                    << (if I_0 == NTREEBINS.wrapping_sub(1 as ::core::ffi::c_uint) {
                        0 as usize
                    } else {
                        SIZE_T_BITSIZE.wrapping_sub(SIZE_T_ONE).wrapping_sub(
                            (I_0 as ::core::ffi::c_uint >> 1 as ::core::ffi::c_int)
                                .wrapping_add(TREEBIN_SHIFT)
                                .wrapping_sub(2 as ::core::ffi::c_uint)
                                as usize,
                        )
                    });
                loop {
                    if (*T).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT) != psize {
                        let mut C: *mut tchunkptr =
                            (&raw mut (*T).child as *mut *mut malloc_tree_chunk).offset(
                                (K_0 >> SIZE_T_BITSIZE.wrapping_sub(SIZE_T_ONE) & 1 as size_t)
                                    as isize,
                            ) as *mut tchunkptr;
                        K_0 <<= 1 as ::core::ffi::c_int;
                        if !(*C).is_null() {
                            T = *C;
                        } else if (C as *mut ::core::ffi::c_char >= (*m).least_addr)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            *C = TP;
                            (*TP).parent = T as *mut malloc_tree_chunk;
                            (*TP).bk = TP as *mut malloc_tree_chunk;
                            (*TP).fd = (*TP).bk;
                            break;
                        } else {
                            abort();
                        }
                    } else {
                        let mut F_0: tchunkptr = (*T).fd as tchunkptr;
                        if (T as *mut ::core::ffi::c_char >= (*m).least_addr
                            && F_0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                            as ::core::ffi::c_int as ::core::ffi::c_long
                            != 0
                        {
                            (*F_0).bk = TP as *mut malloc_tree_chunk;
                            (*T).fd = (*F_0).bk;
                            (*TP).fd = F_0 as *mut malloc_tree_chunk;
                            (*TP).bk = T as *mut malloc_tree_chunk;
                            (*TP).parent = ::core::ptr::null_mut::<malloc_tree_chunk>();
                            break;
                        } else {
                            abort();
                        }
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn sys_alloc(mut m: mstate, mut nb: size_t) -> *mut ::core::ffi::c_void {
    let mut tbase: *mut ::core::ffi::c_char = CMFAIL;
    let mut tsize: size_t = 0 as size_t;
    let mut mmap_flag: flag_t = 0 as flag_t;
    let mut asize: size_t = 0;
    (crate::atomic_compat::atomic_load_acquire(&raw mut mparams.magic) != 0 as size_t
        || init_mparams() != 0) as ::core::ffi::c_int;
    if (*m).mflags as size_t & USE_MMAP_BIT != 0
        && nb >= mparams.mmap_threshold
        && (*m).topsize != 0 as size_t
    {
        let mut mem: *mut ::core::ffi::c_void = mmap_alloc(m, nb);
        if !mem.is_null() {
            return mem;
        }
    }
    asize = nb
        .wrapping_add(
            (if ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_void as size_t
                & (2 as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                    .wrapping_sub(1 as ::core::ffi::c_int as size_t)
                == 0 as size_t
            {
                0 as size_t
            } else {
                (2 as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                    .wrapping_sub(
                        ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                            ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut ::core::ffi::c_void as size_t
                            & (2 as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                                )
                                .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                    )
                    & (2 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                        .wrapping_sub(1 as ::core::ffi::c_int as size_t)
            })
            .wrapping_add(
                (::core::mem::size_of::<malloc_segment>() as size_t)
                    .wrapping_add(::core::mem::size_of::<size_t>() as size_t)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                            )
                            .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                    )
                    & !(2 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                        .wrapping_sub(1 as ::core::ffi::c_int as size_t),
            )
            .wrapping_add(
                (::core::mem::size_of::<mchunk>() as size_t).wrapping_add(
                    (2 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                        .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                ) & !(2 as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                    .wrapping_sub(1 as ::core::ffi::c_int as size_t),
            )
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize),
            ),
        )
        .wrapping_add(mparams.granularity.wrapping_sub(SIZE_T_ONE))
        & !mparams.granularity.wrapping_sub(SIZE_T_ONE);
    if asize <= nb {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*m).footprint_limit != 0 as size_t {
        let mut fp: size_t = (*m).footprint.wrapping_add(asize);
        if fp <= (*m).footprint || fp > (*m).footprint_limit {
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
    }
    if MORECORE_CONTIGUOUS != 0 && (*m).mflags as ::core::ffi::c_uint & USE_NONCONTIGUOUS_BIT == 0 {
        let mut br: *mut ::core::ffi::c_char = CMFAIL;
        let mut ssize: size_t = asize;
        let mut ss: msegmentptr = if (*m).top.is_null() {
            ::core::ptr::null_mut::<malloc_segment>()
        } else {
            segment_holding(m, (*m).top as *mut ::core::ffi::c_char)
        };
        if crate::atomic_compat::atomic_xchg_acquire(
            &raw mut malloc_global_mutex,
            1 as ::core::ffi::c_int,
        ) != 0
        {
            spin_acquire_lock(&raw mut malloc_global_mutex);
        } else {
        };
        if ss.is_null() {
            let mut base: *mut ::core::ffi::c_char = !(0 as ::core::ffi::c_int as size_t)
                as *mut ::core::ffi::c_void
                as *mut ::core::ffi::c_char;
            if base != CMFAIL {
                let mut fp_0: size_t = 0;
                if !(base as size_t & mparams.page_size.wrapping_sub(SIZE_T_ONE) == 0 as size_t) {
                    ssize = ssize.wrapping_add(
                        ((base as size_t).wrapping_add(mparams.page_size.wrapping_sub(SIZE_T_ONE))
                            & !mparams.page_size.wrapping_sub(SIZE_T_ONE))
                        .wrapping_sub(base as size_t),
                    );
                }
                fp_0 = (*m).footprint.wrapping_add(ssize);
                if ssize > nb
                    && ssize < HALF_MAX_SIZE_T
                    && ((*m).footprint_limit == 0 as size_t
                        || fp_0 > (*m).footprint && fp_0 <= (*m).footprint_limit)
                    && {
                        br = !(0 as ::core::ffi::c_int as size_t) as *mut ::core::ffi::c_void
                            as *mut ::core::ffi::c_char;
                        br == base
                    }
                {
                    tbase = base;
                    tsize = ssize;
                }
            }
        } else {
            ssize = nb
                .wrapping_sub((*m).topsize)
                .wrapping_add(
                    (if ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                        ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut ::core::ffi::c_void as size_t
                        & (2 as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                            )
                            .wrapping_sub(1 as ::core::ffi::c_int as size_t)
                        == 0 as size_t
                    {
                        0 as size_t
                    } else {
                        (2 as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                            )
                            .wrapping_sub(
                                ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                                    ((::core::mem::size_of::<size_t>() as usize)
                                        << 1 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut ::core::ffi::c_void
                                    as size_t
                                    & (2 as usize)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                                                as usize,
                                        )
                                        .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                            )
                            & (2 as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                                )
                                .wrapping_sub(1 as ::core::ffi::c_int as size_t)
                    })
                    .wrapping_add(
                        (::core::mem::size_of::<malloc_segment>() as size_t)
                            .wrapping_add(::core::mem::size_of::<size_t>() as size_t)
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize,
                                    )
                                    .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                            )
                            & !(2 as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                                )
                                .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                    )
                    .wrapping_add(
                        (::core::mem::size_of::<mchunk>() as size_t).wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                                )
                                .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                        ) & !(2 as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
                            )
                            .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                    )
                    .wrapping_add(
                        (2 as usize).wrapping_mul(
                            ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize,
                        ),
                    ),
                )
                .wrapping_add(mparams.granularity.wrapping_sub(SIZE_T_ONE))
                & !mparams.granularity.wrapping_sub(SIZE_T_ONE);
            if ssize < HALF_MAX_SIZE_T && {
                br = !(0 as ::core::ffi::c_int as size_t) as *mut ::core::ffi::c_void
                    as *mut ::core::ffi::c_char;
                br == (*ss).base.offset((*ss).size as isize)
            } {
                tbase = br;
                tsize = ssize;
            }
        }
        if tbase == CMFAIL {
            if br != CMFAIL {
                if ssize < HALF_MAX_SIZE_T
                    && ssize
                        < nb.wrapping_add(
                            (if ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                                ((::core::mem::size_of::<size_t>() as usize)
                                    << 1 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut ::core::ffi::c_void as size_t
                                & CHUNK_ALIGN_MASK
                                == 0 as size_t
                            {
                                0 as size_t
                            } else {
                                MALLOC_ALIGNMENT.wrapping_sub(
                                    ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                                        ((::core::mem::size_of::<size_t>() as usize)
                                            << 1 as ::core::ffi::c_int)
                                            as isize,
                                    )
                                        as *mut ::core::ffi::c_void
                                        as size_t
                                        & CHUNK_ALIGN_MASK,
                                ) & CHUNK_ALIGN_MASK
                            })
                            .wrapping_add(
                                (::core::mem::size_of::<malloc_segment>() as size_t)
                                    .wrapping_add(CHUNK_OVERHEAD as size_t)
                                    .wrapping_add(CHUNK_ALIGN_MASK)
                                    & !CHUNK_ALIGN_MASK,
                            )
                            .wrapping_add(MIN_CHUNK_SIZE as size_t)
                            .wrapping_add(MALLOC_ALIGNMENT),
                        )
                {
                    let mut esize: size_t = nb
                        .wrapping_add(
                            (if ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                                ((::core::mem::size_of::<size_t>() as usize)
                                    << 1 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut ::core::ffi::c_void as size_t
                                & (2 as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize,
                                    )
                                    .wrapping_sub(1 as ::core::ffi::c_int as size_t)
                                == 0 as size_t
                            {
                                0 as size_t
                            } else {
                                (2 as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize,
                                    )
                                    .wrapping_sub(
                                        ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                                            ((::core::mem::size_of::<size_t>() as usize)
                                                << 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as *mut ::core::ffi::c_void
                                            as size_t
                                            & (2 as usize)
                                                .wrapping_mul(::core::mem::size_of::<
                                                    *mut ::core::ffi::c_void,
                                                >(
                                                )
                                                    as usize)
                                                .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                                    )
                                    & (2 as usize)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                                                as usize,
                                        )
                                        .wrapping_sub(1 as ::core::ffi::c_int as size_t)
                            })
                            .wrapping_add(
                                (::core::mem::size_of::<malloc_segment>() as size_t)
                                    .wrapping_add(::core::mem::size_of::<size_t>() as size_t)
                                    .wrapping_add(
                                        (2 as usize)
                                            .wrapping_mul(::core::mem::size_of::<
                                                *mut ::core::ffi::c_void,
                                            >(
                                            )
                                                as usize)
                                            .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                                    )
                                    & !(2 as usize)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                                                as usize,
                                        )
                                        .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<mchunk>() as size_t).wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                                                as usize,
                                        )
                                        .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                                ) & !(2 as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize,
                                    )
                                    .wrapping_sub(1 as ::core::ffi::c_int as size_t),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize,
                                    ),
                            ),
                        )
                        .wrapping_sub(ssize)
                        .wrapping_add(mparams.granularity.wrapping_sub(SIZE_T_ONE))
                        & !mparams.granularity.wrapping_sub(SIZE_T_ONE);
                    if esize < HALF_MAX_SIZE_T {
                        let mut end: *mut ::core::ffi::c_char = !(0 as ::core::ffi::c_int as size_t)
                            as *mut ::core::ffi::c_void
                            as *mut ::core::ffi::c_char;
                        if end != CMFAIL {
                            ssize = ssize.wrapping_add(esize);
                        } else {
                            !(0 as ::core::ffi::c_int as size_t);
                            br = CMFAIL;
                        }
                    }
                }
            }
            if br != CMFAIL {
                tbase = br;
                tsize = ssize;
            } else {
                (*m).mflags |= USE_NONCONTIGUOUS_BIT;
            }
        }
        crate::atomic_compat::atomic_store_release(&raw mut malloc_global_mutex, 0);
    }
    if HAVE_MMAP != 0 && tbase == CMFAIL {
        let mut mp: *mut ::core::ffi::c_char = dlmmap(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            asize,
            MMAP_PROT,
            MMAP_FLAGS,
            -(1 as ::core::ffi::c_int),
            0 as off_t,
        ) as *mut ::core::ffi::c_char;
        if mp != CMFAIL {
            tbase = mp;
            tsize = asize;
            mmap_flag = USE_MMAP_BIT as flag_t;
        }
    }
    if HAVE_MORECORE != 0 && tbase == CMFAIL {
        if asize < HALF_MAX_SIZE_T {
            let mut br_0: *mut ::core::ffi::c_char = CMFAIL;
            let mut end_0: *mut ::core::ffi::c_char = CMFAIL;
            if crate::atomic_compat::atomic_xchg_acquire(
                &raw mut malloc_global_mutex,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                spin_acquire_lock(&raw mut malloc_global_mutex);
            } else {
            };
            br_0 = !(0 as ::core::ffi::c_int as size_t) as *mut ::core::ffi::c_void
                as *mut ::core::ffi::c_char;
            end_0 = !(0 as ::core::ffi::c_int as size_t) as *mut ::core::ffi::c_void
                as *mut ::core::ffi::c_char;
            crate::atomic_compat::atomic_store_release(&raw mut malloc_global_mutex, 0);
            if br_0 != CMFAIL && end_0 != CMFAIL && br_0 < end_0 {
                let mut ssize_0: size_t = end_0.offset_from(br_0) as ::core::ffi::c_long as size_t;
                if ssize_0
                    > nb.wrapping_add(
                        (if ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                            ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut ::core::ffi::c_void as size_t
                            & CHUNK_ALIGN_MASK
                            == 0 as size_t
                        {
                            0 as size_t
                        } else {
                            MALLOC_ALIGNMENT.wrapping_sub(
                                ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                                    ((::core::mem::size_of::<size_t>() as usize)
                                        << 1 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut ::core::ffi::c_void
                                    as size_t
                                    & CHUNK_ALIGN_MASK,
                            ) & CHUNK_ALIGN_MASK
                        })
                        .wrapping_add(
                            (::core::mem::size_of::<malloc_segment>() as size_t)
                                .wrapping_add(CHUNK_OVERHEAD as size_t)
                                .wrapping_add(CHUNK_ALIGN_MASK)
                                & !CHUNK_ALIGN_MASK,
                        )
                        .wrapping_add(MIN_CHUNK_SIZE as size_t),
                    )
                {
                    tbase = br_0;
                    tsize = ssize_0;
                }
            }
        }
    }
    if tbase != CMFAIL {
        (*m).footprint = (*m).footprint.wrapping_add(tsize);
        if (*m).footprint > (*m).max_footprint {
            (*m).max_footprint = (*m).footprint;
        }
        if (*m).top.is_null() {
            if (*m).least_addr.is_null() || tbase < (*m).least_addr {
                (*m).least_addr = tbase;
            }
            (*m).seg.base = tbase;
            (*m).seg.size = tsize;
            if mmap_flag as size_t != USE_MMAP_BIT {
                abort();
            } else {
                (*m).seg.exec_offset = *((*m)
                    .seg
                    .base
                    .offset((*m).seg.size as isize)
                    .offset(-(::core::mem::size_of::<ptrdiff_t>() as usize as isize))
                    as *mut ptrdiff_t);
                if *((*m)
                    .seg
                    .base
                    .offset((*m).seg.exec_offset as isize)
                    .offset((*m).seg.size as isize)
                    .offset(-(::core::mem::size_of::<ptrdiff_t>() as usize as isize))
                    as *mut ptrdiff_t)
                    != (*m).seg.exec_offset
                {
                    abort();
                } else {
                    *((*m)
                        .seg
                        .base
                        .offset((*m).seg.size as isize)
                        .offset(-(::core::mem::size_of::<ptrdiff_t>() as usize as isize))
                        as *mut ptrdiff_t) = 0 as ptrdiff_t;
                };
            };
            (*m).magic = mparams.magic;
            (*m).release_checks = MAX_RELEASE_CHECK_RATE as size_t;
            init_bins(m);
            if m == &raw mut _gm_ {
                init_top(
                    m,
                    tbase as mchunkptr,
                    tsize.wrapping_sub(
                        (if ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                            ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut ::core::ffi::c_void as size_t
                            & CHUNK_ALIGN_MASK
                            == 0 as size_t
                        {
                            0 as size_t
                        } else {
                            MALLOC_ALIGNMENT.wrapping_sub(
                                ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                                    ((::core::mem::size_of::<size_t>() as usize)
                                        << 1 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut ::core::ffi::c_void
                                    as size_t
                                    & CHUNK_ALIGN_MASK,
                            ) & CHUNK_ALIGN_MASK
                        })
                        .wrapping_add(
                            (::core::mem::size_of::<malloc_segment>() as size_t)
                                .wrapping_add(CHUNK_OVERHEAD as size_t)
                                .wrapping_add(CHUNK_ALIGN_MASK)
                                & !CHUNK_ALIGN_MASK,
                        )
                        .wrapping_add(MIN_CHUNK_SIZE as size_t),
                    ),
                );
            } else {
                let mut mn: mchunkptr = ((m as *mut ::core::ffi::c_char).offset(
                    -(((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                        as isize),
                ) as mchunkptr as *mut ::core::ffi::c_char)
                    .offset(
                        ((*((m as *mut ::core::ffi::c_char).offset(
                            -(((::core::mem::size_of::<size_t>() as usize)
                                << 1 as ::core::ffi::c_int) as isize),
                        ) as mchunkptr))
                            .head
                            & !FLAG_BITS) as isize,
                    ) as mchunkptr;
                init_top(
                    m,
                    mn,
                    (tbase
                        .offset(tsize as isize)
                        .offset_from(mn as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long as size_t)
                        .wrapping_sub(
                            (if ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                                ((::core::mem::size_of::<size_t>() as usize)
                                    << 1 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut ::core::ffi::c_void as size_t
                                & CHUNK_ALIGN_MASK
                                == 0 as size_t
                            {
                                0 as size_t
                            } else {
                                MALLOC_ALIGNMENT.wrapping_sub(
                                    ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                                        ((::core::mem::size_of::<size_t>() as usize)
                                            << 1 as ::core::ffi::c_int)
                                            as isize,
                                    )
                                        as *mut ::core::ffi::c_void
                                        as size_t
                                        & CHUNK_ALIGN_MASK,
                                ) & CHUNK_ALIGN_MASK
                            })
                            .wrapping_add(
                                (::core::mem::size_of::<malloc_segment>() as size_t)
                                    .wrapping_add(CHUNK_OVERHEAD as size_t)
                                    .wrapping_add(CHUNK_ALIGN_MASK)
                                    & !CHUNK_ALIGN_MASK,
                            )
                            .wrapping_add(MIN_CHUNK_SIZE as size_t),
                        ),
                );
            }
        } else {
            let mut sp: msegmentptr = &raw mut (*m).seg;
            while !sp.is_null() && tbase != (*sp).base.offset((*sp).size as isize) {
                sp = (if 0 as ::core::ffi::c_int != 0 {
                    ::core::ptr::null_mut::<malloc_segment>()
                } else {
                    (*sp).next
                }) as msegmentptr;
            }
            if !sp.is_null()
                && 1 as ::core::ffi::c_int as size_t & EXTERN_BIT as size_t == 0
                && *(tbase
                    .offset(tsize as isize)
                    .offset(-(::core::mem::size_of::<ptrdiff_t>() as usize as isize))
                    as *mut ptrdiff_t)
                    == (*sp).exec_offset
                && 1 as ::core::ffi::c_int as size_t & USE_MMAP_BIT == mmap_flag as size_t
                && ((*m).top as *mut ::core::ffi::c_char >= (*sp).base
                    && ((*m).top as *mut ::core::ffi::c_char)
                        < (*sp).base.offset((*sp).size as isize))
            {
                (*sp).size = (*sp).size.wrapping_add(tsize);
                init_top(m, (*m).top, (*m).topsize.wrapping_add(tsize));
            } else {
                if tbase < (*m).least_addr {
                    (*m).least_addr = tbase;
                }
                sp = &raw mut (*m).seg as msegmentptr;
                while !sp.is_null() && (*sp).base != tbase.offset(tsize as isize) {
                    sp = (if 0 as ::core::ffi::c_int != 0 {
                        ::core::ptr::null_mut::<malloc_segment>()
                    } else {
                        (*sp).next
                    }) as msegmentptr;
                }
                if !sp.is_null()
                    && 1 as ::core::ffi::c_int as size_t & EXTERN_BIT as size_t == 0
                    && *(tbase
                        .offset(tsize as isize)
                        .offset(-(::core::mem::size_of::<ptrdiff_t>() as usize as isize))
                        as *mut ptrdiff_t)
                        == (*sp).exec_offset
                    && 1 as ::core::ffi::c_int as size_t & USE_MMAP_BIT == mmap_flag as size_t
                {
                    let mut oldbase: *mut ::core::ffi::c_char = (*sp).base;
                    (*sp).base = tbase;
                    (*sp).size = (*sp).size.wrapping_add(tsize);
                    return prepend_alloc(m, tbase, oldbase, nb);
                } else {
                    add_segment(m, tbase, tsize, mmap_flag);
                }
            }
        }
        if nb < (*m).topsize {
            (*m).topsize = (*m).topsize.wrapping_sub(nb);
            let mut rsize: size_t = (*m).topsize;
            let mut p: mchunkptr = (*m).top;
            (*m).top = (p as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr;
            let mut r: mchunkptr = (*m).top;
            (*r).head = rsize | PINUSE_BIT;
            (*p).head = nb | PINUSE_BIT | CINUSE_BIT;
            return (p as *mut ::core::ffi::c_char).offset(TWO_SIZE_T_SIZES as usize as isize)
                as *mut ::core::ffi::c_void;
        }
    }
    *__errno_location() = ENOMEM;
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn release_unused_segments(mut m: mstate) -> size_t {
    let mut released: size_t = 0 as size_t;
    let mut nsegs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pred: msegmentptr = &raw mut (*m).seg;
    let mut sp: msegmentptr = (*pred).next as msegmentptr;
    while !sp.is_null() {
        let mut base: *mut ::core::ffi::c_char = (*sp).base;
        let mut size: size_t = (*sp).size;
        let mut next: msegmentptr = (*sp).next as msegmentptr;
        nsegs += 1;
        if 1 as ::core::ffi::c_int as size_t & USE_MMAP_BIT != 0
            && 1 as ::core::ffi::c_int as size_t & EXTERN_BIT as size_t == 0
        {
            let mut p: mchunkptr = base.offset(
                (if base.offset(
                    ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                        as isize,
                ) as *mut ::core::ffi::c_void as size_t
                    & CHUNK_ALIGN_MASK
                    == 0 as size_t
                {
                    0 as size_t
                } else {
                    MALLOC_ALIGNMENT.wrapping_sub(
                        base.offset(
                            ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut ::core::ffi::c_void as size_t
                            & CHUNK_ALIGN_MASK,
                    ) & CHUNK_ALIGN_MASK
                }) as isize,
            ) as mchunkptr;
            let mut psize: size_t = (*p).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT);
            if !((*p).head & INUSE_BITS != PINUSE_BIT)
                && (p as *mut ::core::ffi::c_char).offset(psize as isize)
                    >= base.offset(size as isize).offset(
                        -((if ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                            ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut ::core::ffi::c_void as size_t
                            & CHUNK_ALIGN_MASK
                            == 0 as size_t
                        {
                            0 as size_t
                        } else {
                            MALLOC_ALIGNMENT.wrapping_sub(
                                ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                                    ((::core::mem::size_of::<size_t>() as usize)
                                        << 1 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut ::core::ffi::c_void
                                    as size_t
                                    & CHUNK_ALIGN_MASK,
                            ) & CHUNK_ALIGN_MASK
                        })
                        .wrapping_add(
                            (::core::mem::size_of::<malloc_segment>() as size_t)
                                .wrapping_add(CHUNK_OVERHEAD as size_t)
                                .wrapping_add(CHUNK_ALIGN_MASK)
                                & !CHUNK_ALIGN_MASK,
                        )
                        .wrapping_add(MIN_CHUNK_SIZE as size_t) as isize),
                    )
            {
                let mut tp: tchunkptr = p as tchunkptr;
                if p == (*m).dv {
                    (*m).dv = ::core::ptr::null_mut::<malloc_chunk>();
                    (*m).dvsize = 0 as size_t;
                } else {
                    let mut XP: tchunkptr = (*tp).parent as tchunkptr;
                    let mut R: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
                    if (*tp).bk != tp {
                        let mut F: tchunkptr = (*tp).fd as tchunkptr;
                        R = (*tp).bk as tchunkptr;
                        if (F as *mut ::core::ffi::c_char >= (*m).least_addr
                            && (*F).bk == tp
                            && (*R).fd == tp) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            (*F).bk = R as *mut malloc_tree_chunk;
                            (*R).fd = F as *mut malloc_tree_chunk;
                        } else {
                            abort();
                        }
                    } else {
                        let mut RP: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                        RP = (&raw mut (*tp).child as *mut *mut malloc_tree_chunk)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut *mut malloc_tree_chunk
                            as *mut tchunkptr;
                        R = *RP;
                        if !R.is_null() || {
                            RP = (&raw mut (*tp).child as *mut *mut malloc_tree_chunk)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut *mut malloc_tree_chunk
                                as *mut tchunkptr;
                            R = *RP;
                            !R.is_null()
                        } {
                            let mut CP: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                            loop {
                                CP = (&raw mut (*R).child as *mut *mut malloc_tree_chunk)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as *mut *mut malloc_tree_chunk
                                    as *mut tchunkptr;
                                if !(!(*CP).is_null() || {
                                    CP = (&raw mut (*R).child as *mut *mut malloc_tree_chunk)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut *mut malloc_tree_chunk
                                        as *mut tchunkptr;
                                    !(*CP).is_null()
                                }) {
                                    break;
                                }
                                RP = CP;
                                R = *RP;
                            }
                            if (RP as *mut ::core::ffi::c_char >= (*m).least_addr)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                *RP = ::core::ptr::null_mut::<malloc_tree_chunk>();
                            } else {
                                abort();
                            }
                        }
                    }
                    if !XP.is_null() {
                        let mut H: *mut tbinptr = (&raw mut (*m).treebins as *mut tbinptr)
                            .offset((*tp).index as isize)
                            as *mut tbinptr;
                        if tp == *H {
                            *H = R as tbinptr;
                            if (*H).is_null() {
                                (*m).treemap &=
                                    !((1 as ::core::ffi::c_int as binmap_t) << (*tp).index);
                            }
                        } else if (XP as *mut ::core::ffi::c_char >= (*m).least_addr)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            if (*XP).child[0 as ::core::ffi::c_int as usize] == tp {
                                (*XP).child[0 as ::core::ffi::c_int as usize] =
                                    R as *mut malloc_tree_chunk;
                            } else {
                                (*XP).child[1 as ::core::ffi::c_int as usize] =
                                    R as *mut malloc_tree_chunk;
                            }
                        } else {
                            abort();
                        }
                        if !R.is_null() {
                            if (R as *mut ::core::ffi::c_char >= (*m).least_addr)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                let mut C0: tchunkptr =
                                    ::core::ptr::null_mut::<malloc_tree_chunk>();
                                let mut C1: tchunkptr =
                                    ::core::ptr::null_mut::<malloc_tree_chunk>();
                                (*R).parent = XP as *mut malloc_tree_chunk;
                                C0 = (*tp).child[0 as ::core::ffi::c_int as usize] as tchunkptr;
                                if !C0.is_null() {
                                    if (C0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        (*R).child[0 as ::core::ffi::c_int as usize] =
                                            C0 as *mut malloc_tree_chunk;
                                        (*C0).parent = R as *mut malloc_tree_chunk;
                                    } else {
                                        abort();
                                    }
                                }
                                C1 = (*tp).child[1 as ::core::ffi::c_int as usize] as tchunkptr;
                                if !C1.is_null() {
                                    if (C1 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        (*R).child[1 as ::core::ffi::c_int as usize] =
                                            C1 as *mut malloc_tree_chunk;
                                        (*C1).parent = R as *mut malloc_tree_chunk;
                                    } else {
                                        abort();
                                    }
                                }
                            } else {
                                abort();
                            }
                        }
                    }
                }
                if dlmunmap(base as *mut ::core::ffi::c_void, size) == 0 as ::core::ffi::c_int {
                    released = released.wrapping_add(size);
                    (*m).footprint = (*m).footprint.wrapping_sub(size);
                    sp = pred;
                    (*sp).next = next as *mut malloc_segment;
                } else {
                    let mut H_0: *mut tbinptr = ::core::ptr::null_mut::<tbinptr>();
                    let mut I: bindex_t = 0;
                    let mut X: ::core::ffi::c_uint =
                        (psize >> TREEBIN_SHIFT) as ::core::ffi::c_uint;
                    if X == 0 as ::core::ffi::c_uint {
                        I = 0 as bindex_t;
                    } else if X > 0xffff as ::core::ffi::c_uint {
                        I = NTREEBINS.wrapping_sub(1 as ::core::ffi::c_uint) as bindex_t;
                    } else {
                        let mut K: ::core::ffi::c_uint =
                            (::core::mem::size_of::<::core::ffi::c_uint>() as ::core::ffi::c_uint)
                                .wrapping_mul(__CHAR_BIT__ as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                                .wrapping_sub(X.leading_zeros() as i32 as ::core::ffi::c_uint);
                        I = ((K << 1 as ::core::ffi::c_int) as size_t).wrapping_add(
                            psize
                                >> K.wrapping_add(
                                    TREEBIN_SHIFT.wrapping_sub(1 as ::core::ffi::c_uint),
                                )
                                & 1 as size_t,
                        ) as bindex_t;
                    }
                    H_0 =
                        (&raw mut (*m).treebins as *mut tbinptr).offset(I as isize) as *mut tbinptr;
                    (*tp).index = I;
                    (*tp).child[1 as ::core::ffi::c_int as usize] =
                        ::core::ptr::null_mut::<malloc_tree_chunk>();
                    (*tp).child[0 as ::core::ffi::c_int as usize] =
                        (*tp).child[1 as ::core::ffi::c_int as usize];
                    if (*m).treemap & (1 as ::core::ffi::c_int as binmap_t) << I == 0 {
                        (*m).treemap |= (1 as ::core::ffi::c_int as binmap_t) << I;
                        *H_0 = tp as tbinptr;
                        (*tp).parent = H_0 as tchunkptr as *mut malloc_tree_chunk;
                        (*tp).bk = tp as *mut malloc_tree_chunk;
                        (*tp).fd = (*tp).bk;
                    } else {
                        let mut T: tchunkptr = *H_0;
                        let mut K_0: size_t = psize
                            << (if I == NTREEBINS.wrapping_sub(1 as ::core::ffi::c_uint) {
                                0 as usize
                            } else {
                                SIZE_T_BITSIZE.wrapping_sub(SIZE_T_ONE).wrapping_sub(
                                    (I as ::core::ffi::c_uint >> 1 as ::core::ffi::c_int)
                                        .wrapping_add(TREEBIN_SHIFT)
                                        .wrapping_sub(2 as ::core::ffi::c_uint)
                                        as usize,
                                )
                            });
                        loop {
                            if (*T).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT) != psize {
                                let mut C: *mut tchunkptr =
                                    (&raw mut (*T).child as *mut *mut malloc_tree_chunk).offset(
                                        (K_0 >> SIZE_T_BITSIZE.wrapping_sub(SIZE_T_ONE)
                                            & 1 as size_t)
                                            as isize,
                                    ) as *mut tchunkptr;
                                K_0 <<= 1 as ::core::ffi::c_int;
                                if !(*C).is_null() {
                                    T = *C;
                                } else if (C as *mut ::core::ffi::c_char >= (*m).least_addr)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_long
                                    != 0
                                {
                                    *C = tp;
                                    (*tp).parent = T as *mut malloc_tree_chunk;
                                    (*tp).bk = tp as *mut malloc_tree_chunk;
                                    (*tp).fd = (*tp).bk;
                                    break;
                                } else {
                                    abort();
                                }
                            } else {
                                let mut F_0: tchunkptr = (*T).fd as tchunkptr;
                                if (T as *mut ::core::ffi::c_char >= (*m).least_addr
                                    && F_0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_long
                                    != 0
                                {
                                    (*F_0).bk = tp as *mut malloc_tree_chunk;
                                    (*T).fd = (*F_0).bk;
                                    (*tp).fd = F_0 as *mut malloc_tree_chunk;
                                    (*tp).bk = T as *mut malloc_tree_chunk;
                                    (*tp).parent = ::core::ptr::null_mut::<malloc_tree_chunk>();
                                    break;
                                } else {
                                    abort();
                                }
                            }
                        }
                    }
                }
            }
        }
        pred = sp;
        sp = next;
    }
    (*m).release_checks = if nsegs as size_t > MAX_RELEASE_CHECK_RATE as size_t {
        nsegs as size_t
    } else {
        MAX_RELEASE_CHECK_RATE as size_t
    };
    return released;
}
unsafe extern "C" fn sys_trim(mut m: mstate, mut pad: size_t) -> ::core::ffi::c_int {
    let mut released: size_t = 0 as size_t;
    (crate::atomic_compat::atomic_load_acquire(&raw mut mparams.magic) != 0 as size_t
        || init_mparams() != 0) as ::core::ffi::c_int;
    if pad < MAX_REQUEST as usize && !(*m).top.is_null() {
        pad = (pad as ::core::ffi::c_ulong).wrapping_add(
            (if ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_void as size_t
                & CHUNK_ALIGN_MASK
                == 0 as size_t
            {
                0 as size_t
            } else {
                MALLOC_ALIGNMENT.wrapping_sub(
                    ::core::ptr::null_mut::<::core::ffi::c_char>().offset(
                        ((::core::mem::size_of::<size_t>() as usize) << 1 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut ::core::ffi::c_void as size_t
                        & CHUNK_ALIGN_MASK,
                ) & CHUNK_ALIGN_MASK
            })
            .wrapping_add(
                (::core::mem::size_of::<malloc_segment>() as size_t)
                    .wrapping_add(CHUNK_OVERHEAD as size_t)
                    .wrapping_add(CHUNK_ALIGN_MASK)
                    & !CHUNK_ALIGN_MASK,
            )
            .wrapping_add(MIN_CHUNK_SIZE as size_t) as ::core::ffi::c_ulong,
        ) as size_t as size_t;
        if (*m).topsize > pad {
            let mut unit: size_t = mparams.granularity;
            let mut extra: size_t = (*m)
                .topsize
                .wrapping_sub(pad)
                .wrapping_add(unit.wrapping_sub(SIZE_T_ONE))
                .wrapping_div(unit)
                .wrapping_sub(SIZE_T_ONE)
                .wrapping_mul(unit);
            let mut sp: msegmentptr = segment_holding(m, (*m).top as *mut ::core::ffi::c_char);
            if sp.is_null() {
                abort();
            }
            if 1 as ::core::ffi::c_int as size_t & EXTERN_BIT as size_t == 0 {
                if 1 as ::core::ffi::c_int as size_t & USE_MMAP_BIT != 0 {
                    if HAVE_MMAP != 0 && (*sp).size >= extra && has_segment_link(m, sp) == 0 {
                        let mut newsize: size_t = (*sp).size.wrapping_sub(extra);
                        if !(0 as ::core::ffi::c_int as size_t) as *mut ::core::ffi::c_void != MFAIL
                            || dlmunmap(
                                (*sp).base.offset(newsize as isize) as *mut ::core::ffi::c_void,
                                extra,
                            ) == 0 as ::core::ffi::c_int
                        {
                            released = extra;
                        }
                    }
                }
            }
            if released != 0 as size_t {
                (*sp).size = (*sp).size.wrapping_sub(released);
                (*m).footprint = (*m).footprint.wrapping_sub(released);
                init_top(m, (*m).top, (*m).topsize.wrapping_sub(released));
            }
        }
        released = released.wrapping_add(release_unused_segments(m));
        if released == 0 as size_t && (*m).topsize > (*m).trim_check {
            (*m).trim_check = MAX_SIZE_T;
        }
    }
    return if released != 0 as size_t {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn dispose_chunk(mut m: mstate, mut p: mchunkptr, mut psize: size_t) {
    let mut next: mchunkptr = (p as *mut ::core::ffi::c_char).offset(psize as isize) as mchunkptr;
    if (*p).head & PINUSE_BIT == 0 {
        let mut prev: mchunkptr = ::core::ptr::null_mut::<malloc_chunk>();
        let mut prevsize: size_t = (*p).prev_foot;
        if (*p).head & INUSE_BITS == 0 as size_t {
            psize = (psize as ::core::ffi::c_ulong)
                .wrapping_add(prevsize.wrapping_add(MMAP_FOOT_PAD) as ::core::ffi::c_ulong)
                as size_t as size_t;
            if dlmunmap(
                (p as *mut ::core::ffi::c_char).offset(-(prevsize as isize))
                    as *mut ::core::ffi::c_void,
                psize,
            ) == 0 as ::core::ffi::c_int
            {
                (*m).footprint = (*m).footprint.wrapping_sub(psize);
            }
            return;
        }
        prev = (p as *mut ::core::ffi::c_char).offset(-(prevsize as isize)) as mchunkptr;
        psize = psize.wrapping_add(prevsize);
        p = prev;
        if (prev as *mut ::core::ffi::c_char >= (*m).least_addr) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            if p != (*m).dv {
                if prevsize >> SMALLBIN_SHIFT < NSMALLBINS as size_t {
                    let mut F: mchunkptr = (*p).fd as mchunkptr;
                    let mut B: mchunkptr = (*p).bk as mchunkptr;
                    let mut I: bindex_t = (prevsize >> SMALLBIN_SHIFT) as bindex_t;
                    if (F
                        == (&raw mut (*m).smallbins as *mut mchunkptr)
                            .offset((I << 1 as ::core::ffi::c_int) as isize)
                            as *mut mchunkptr as *mut ::core::ffi::c_char
                            as sbinptr
                        || F as *mut ::core::ffi::c_char >= (*m).least_addr && (*F).bk == p)
                        as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        if B == F {
                            (*m).smallmap &= !((1 as ::core::ffi::c_int as binmap_t) << I);
                        } else if (B
                            == (&raw mut (*m).smallbins as *mut mchunkptr)
                                .offset((I << 1 as ::core::ffi::c_int) as isize)
                                as *mut mchunkptr
                                as *mut ::core::ffi::c_char
                                as sbinptr
                            || B as *mut ::core::ffi::c_char >= (*m).least_addr && (*B).fd == p)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            (*F).bk = B as *mut malloc_chunk;
                            (*B).fd = F as *mut malloc_chunk;
                        } else {
                            abort();
                        }
                    } else {
                        abort();
                    }
                } else {
                    let mut TP: tchunkptr = p as tchunkptr;
                    let mut XP: tchunkptr = (*TP).parent as tchunkptr;
                    let mut R: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
                    if (*TP).bk != TP {
                        let mut F_0: tchunkptr = (*TP).fd as tchunkptr;
                        R = (*TP).bk as tchunkptr;
                        if (F_0 as *mut ::core::ffi::c_char >= (*m).least_addr
                            && (*F_0).bk == TP
                            && (*R).fd == TP) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            (*F_0).bk = R as *mut malloc_tree_chunk;
                            (*R).fd = F_0 as *mut malloc_tree_chunk;
                        } else {
                            abort();
                        }
                    } else {
                        let mut RP: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                        RP = (&raw mut (*TP).child as *mut *mut malloc_tree_chunk)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut *mut malloc_tree_chunk
                            as *mut tchunkptr;
                        R = *RP;
                        if !R.is_null() || {
                            RP = (&raw mut (*TP).child as *mut *mut malloc_tree_chunk)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut *mut malloc_tree_chunk
                                as *mut tchunkptr;
                            R = *RP;
                            !R.is_null()
                        } {
                            let mut CP: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                            loop {
                                CP = (&raw mut (*R).child as *mut *mut malloc_tree_chunk)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as *mut *mut malloc_tree_chunk
                                    as *mut tchunkptr;
                                if !(!(*CP).is_null() || {
                                    CP = (&raw mut (*R).child as *mut *mut malloc_tree_chunk)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut *mut malloc_tree_chunk
                                        as *mut tchunkptr;
                                    !(*CP).is_null()
                                }) {
                                    break;
                                }
                                RP = CP;
                                R = *RP;
                            }
                            if (RP as *mut ::core::ffi::c_char >= (*m).least_addr)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                *RP = ::core::ptr::null_mut::<malloc_tree_chunk>();
                            } else {
                                abort();
                            }
                        }
                    }
                    if !XP.is_null() {
                        let mut H: *mut tbinptr = (&raw mut (*m).treebins as *mut tbinptr)
                            .offset((*TP).index as isize)
                            as *mut tbinptr;
                        if TP == *H {
                            *H = R as tbinptr;
                            if (*H).is_null() {
                                (*m).treemap &=
                                    !((1 as ::core::ffi::c_int as binmap_t) << (*TP).index);
                            }
                        } else if (XP as *mut ::core::ffi::c_char >= (*m).least_addr)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            if (*XP).child[0 as ::core::ffi::c_int as usize] == TP {
                                (*XP).child[0 as ::core::ffi::c_int as usize] =
                                    R as *mut malloc_tree_chunk;
                            } else {
                                (*XP).child[1 as ::core::ffi::c_int as usize] =
                                    R as *mut malloc_tree_chunk;
                            }
                        } else {
                            abort();
                        }
                        if !R.is_null() {
                            if (R as *mut ::core::ffi::c_char >= (*m).least_addr)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                let mut C0: tchunkptr =
                                    ::core::ptr::null_mut::<malloc_tree_chunk>();
                                let mut C1: tchunkptr =
                                    ::core::ptr::null_mut::<malloc_tree_chunk>();
                                (*R).parent = XP as *mut malloc_tree_chunk;
                                C0 = (*TP).child[0 as ::core::ffi::c_int as usize] as tchunkptr;
                                if !C0.is_null() {
                                    if (C0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        (*R).child[0 as ::core::ffi::c_int as usize] =
                                            C0 as *mut malloc_tree_chunk;
                                        (*C0).parent = R as *mut malloc_tree_chunk;
                                    } else {
                                        abort();
                                    }
                                }
                                C1 = (*TP).child[1 as ::core::ffi::c_int as usize] as tchunkptr;
                                if !C1.is_null() {
                                    if (C1 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        (*R).child[1 as ::core::ffi::c_int as usize] =
                                            C1 as *mut malloc_tree_chunk;
                                        (*C1).parent = R as *mut malloc_tree_chunk;
                                    } else {
                                        abort();
                                    }
                                }
                            } else {
                                abort();
                            }
                        }
                    }
                }
            } else if (*next).head & INUSE_BITS == INUSE_BITS {
                (*m).dvsize = psize;
                (*next).head &= !PINUSE_BIT;
                (*p).head = psize | PINUSE_BIT;
                (*((p as *mut ::core::ffi::c_char).offset(psize as isize) as mchunkptr))
                    .prev_foot = psize;
                return;
            }
        } else {
            abort();
        }
    }
    if (next as *mut ::core::ffi::c_char >= (*m).least_addr) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        if (*next).head & CINUSE_BIT == 0 {
            if next == (*m).top {
                (*m).topsize = (*m).topsize.wrapping_add(psize);
                let mut tsize: size_t = (*m).topsize;
                (*m).top = p;
                (*p).head = tsize | PINUSE_BIT;
                if p == (*m).dv {
                    (*m).dv = ::core::ptr::null_mut::<malloc_chunk>();
                    (*m).dvsize = 0 as size_t;
                }
                return;
            } else if next == (*m).dv {
                (*m).dvsize = (*m).dvsize.wrapping_add(psize);
                let mut dsize: size_t = (*m).dvsize;
                (*m).dv = p;
                (*p).head = dsize | PINUSE_BIT;
                (*((p as *mut ::core::ffi::c_char).offset(dsize as isize) as mchunkptr))
                    .prev_foot = dsize;
                return;
            } else {
                let mut nsize: size_t = (*next).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT);
                psize = psize.wrapping_add(nsize);
                if nsize >> SMALLBIN_SHIFT < NSMALLBINS as size_t {
                    let mut F_1: mchunkptr = (*next).fd as mchunkptr;
                    let mut B_0: mchunkptr = (*next).bk as mchunkptr;
                    let mut I_0: bindex_t = (nsize >> SMALLBIN_SHIFT) as bindex_t;
                    if (F_1
                        == (&raw mut (*m).smallbins as *mut mchunkptr)
                            .offset((I_0 << 1 as ::core::ffi::c_int) as isize)
                            as *mut mchunkptr as *mut ::core::ffi::c_char
                            as sbinptr
                        || F_1 as *mut ::core::ffi::c_char >= (*m).least_addr && (*F_1).bk == next)
                        as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        if B_0 == F_1 {
                            (*m).smallmap &= !((1 as ::core::ffi::c_int as binmap_t) << I_0);
                        } else if (B_0
                            == (&raw mut (*m).smallbins as *mut mchunkptr)
                                .offset((I_0 << 1 as ::core::ffi::c_int) as isize)
                                as *mut mchunkptr
                                as *mut ::core::ffi::c_char
                                as sbinptr
                            || B_0 as *mut ::core::ffi::c_char >= (*m).least_addr
                                && (*B_0).fd == next)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            (*F_1).bk = B_0 as *mut malloc_chunk;
                            (*B_0).fd = F_1 as *mut malloc_chunk;
                        } else {
                            abort();
                        }
                    } else {
                        abort();
                    }
                } else {
                    let mut TP_0: tchunkptr = next as tchunkptr;
                    let mut XP_0: tchunkptr = (*TP_0).parent as tchunkptr;
                    let mut R_0: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
                    if (*TP_0).bk != TP_0 {
                        let mut F_2: tchunkptr = (*TP_0).fd as tchunkptr;
                        R_0 = (*TP_0).bk as tchunkptr;
                        if (F_2 as *mut ::core::ffi::c_char >= (*m).least_addr
                            && (*F_2).bk == TP_0
                            && (*R_0).fd == TP_0) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            (*F_2).bk = R_0 as *mut malloc_tree_chunk;
                            (*R_0).fd = F_2 as *mut malloc_tree_chunk;
                        } else {
                            abort();
                        }
                    } else {
                        let mut RP_0: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                        RP_0 = (&raw mut (*TP_0).child as *mut *mut malloc_tree_chunk)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut *mut malloc_tree_chunk
                            as *mut tchunkptr;
                        R_0 = *RP_0;
                        if !R_0.is_null() || {
                            RP_0 = (&raw mut (*TP_0).child as *mut *mut malloc_tree_chunk)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut *mut malloc_tree_chunk
                                as *mut tchunkptr;
                            R_0 = *RP_0;
                            !R_0.is_null()
                        } {
                            let mut CP_0: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                            loop {
                                CP_0 = (&raw mut (*R_0).child as *mut *mut malloc_tree_chunk)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as *mut *mut malloc_tree_chunk
                                    as *mut tchunkptr;
                                if !(!(*CP_0).is_null() || {
                                    CP_0 = (&raw mut (*R_0).child as *mut *mut malloc_tree_chunk)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut *mut malloc_tree_chunk
                                        as *mut tchunkptr;
                                    !(*CP_0).is_null()
                                }) {
                                    break;
                                }
                                RP_0 = CP_0;
                                R_0 = *RP_0;
                            }
                            if (RP_0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                *RP_0 = ::core::ptr::null_mut::<malloc_tree_chunk>();
                            } else {
                                abort();
                            }
                        }
                    }
                    if !XP_0.is_null() {
                        let mut H_0: *mut tbinptr = (&raw mut (*m).treebins as *mut tbinptr)
                            .offset((*TP_0).index as isize)
                            as *mut tbinptr;
                        if TP_0 == *H_0 {
                            *H_0 = R_0 as tbinptr;
                            if (*H_0).is_null() {
                                (*m).treemap &=
                                    !((1 as ::core::ffi::c_int as binmap_t) << (*TP_0).index);
                            }
                        } else if (XP_0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            if (*XP_0).child[0 as ::core::ffi::c_int as usize] == TP_0 {
                                (*XP_0).child[0 as ::core::ffi::c_int as usize] =
                                    R_0 as *mut malloc_tree_chunk;
                            } else {
                                (*XP_0).child[1 as ::core::ffi::c_int as usize] =
                                    R_0 as *mut malloc_tree_chunk;
                            }
                        } else {
                            abort();
                        }
                        if !R_0.is_null() {
                            if (R_0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                let mut C0_0: tchunkptr =
                                    ::core::ptr::null_mut::<malloc_tree_chunk>();
                                let mut C1_0: tchunkptr =
                                    ::core::ptr::null_mut::<malloc_tree_chunk>();
                                (*R_0).parent = XP_0 as *mut malloc_tree_chunk;
                                C0_0 = (*TP_0).child[0 as ::core::ffi::c_int as usize] as tchunkptr;
                                if !C0_0.is_null() {
                                    if (C0_0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        (*R_0).child[0 as ::core::ffi::c_int as usize] =
                                            C0_0 as *mut malloc_tree_chunk;
                                        (*C0_0).parent = R_0 as *mut malloc_tree_chunk;
                                    } else {
                                        abort();
                                    }
                                }
                                C1_0 = (*TP_0).child[1 as ::core::ffi::c_int as usize] as tchunkptr;
                                if !C1_0.is_null() {
                                    if (C1_0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        (*R_0).child[1 as ::core::ffi::c_int as usize] =
                                            C1_0 as *mut malloc_tree_chunk;
                                        (*C1_0).parent = R_0 as *mut malloc_tree_chunk;
                                    } else {
                                        abort();
                                    }
                                }
                            } else {
                                abort();
                            }
                        }
                    }
                }
                (*p).head = psize | PINUSE_BIT;
                (*((p as *mut ::core::ffi::c_char).offset(psize as isize) as mchunkptr))
                    .prev_foot = psize;
                if p == (*m).dv {
                    (*m).dvsize = psize;
                    return;
                }
            }
        } else {
            (*next).head &= !PINUSE_BIT;
            (*p).head = psize | PINUSE_BIT;
            (*((p as *mut ::core::ffi::c_char).offset(psize as isize) as mchunkptr)).prev_foot =
                psize;
        }
        if psize >> SMALLBIN_SHIFT < NSMALLBINS as size_t {
            let mut I_1: bindex_t = (psize >> SMALLBIN_SHIFT) as bindex_t;
            let mut B_1: mchunkptr = (&raw mut (*m).smallbins as *mut mchunkptr)
                .offset((I_1 << 1 as ::core::ffi::c_int) as isize)
                as *mut mchunkptr as *mut ::core::ffi::c_char
                as mchunkptr;
            let mut F_3: mchunkptr = B_1;
            if (*m).smallmap & (1 as ::core::ffi::c_int as binmap_t) << I_1 == 0 {
                (*m).smallmap |= (1 as ::core::ffi::c_int as binmap_t) << I_1;
            } else if ((*B_1).fd as *mut ::core::ffi::c_char >= (*m).least_addr)
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                F_3 = (*B_1).fd as mchunkptr;
            } else {
                abort();
            }
            (*B_1).fd = p as *mut malloc_chunk;
            (*F_3).bk = p as *mut malloc_chunk;
            (*p).fd = F_3 as *mut malloc_chunk;
            (*p).bk = B_1 as *mut malloc_chunk;
        } else {
            let mut TP_1: tchunkptr = p as tchunkptr;
            let mut H_1: *mut tbinptr = ::core::ptr::null_mut::<tbinptr>();
            let mut I_2: bindex_t = 0;
            let mut X: ::core::ffi::c_uint = (psize >> TREEBIN_SHIFT) as ::core::ffi::c_uint;
            if X == 0 as ::core::ffi::c_uint {
                I_2 = 0 as bindex_t;
            } else if X > 0xffff as ::core::ffi::c_uint {
                I_2 = NTREEBINS.wrapping_sub(1 as ::core::ffi::c_uint) as bindex_t;
            } else {
                let mut K: ::core::ffi::c_uint = (::core::mem::size_of::<::core::ffi::c_uint>()
                    as ::core::ffi::c_uint)
                    .wrapping_mul(__CHAR_BIT__ as ::core::ffi::c_uint)
                    .wrapping_sub(1 as ::core::ffi::c_uint)
                    .wrapping_sub(X.leading_zeros() as i32 as ::core::ffi::c_uint);
                I_2 = ((K << 1 as ::core::ffi::c_int) as size_t).wrapping_add(
                    psize >> K.wrapping_add(TREEBIN_SHIFT.wrapping_sub(1 as ::core::ffi::c_uint))
                        & 1 as size_t,
                ) as bindex_t;
            }
            H_1 = (&raw mut (*m).treebins as *mut tbinptr).offset(I_2 as isize) as *mut tbinptr;
            (*TP_1).index = I_2;
            (*TP_1).child[1 as ::core::ffi::c_int as usize] =
                ::core::ptr::null_mut::<malloc_tree_chunk>();
            (*TP_1).child[0 as ::core::ffi::c_int as usize] =
                (*TP_1).child[1 as ::core::ffi::c_int as usize];
            if (*m).treemap & (1 as ::core::ffi::c_int as binmap_t) << I_2 == 0 {
                (*m).treemap |= (1 as ::core::ffi::c_int as binmap_t) << I_2;
                *H_1 = TP_1 as tbinptr;
                (*TP_1).parent = H_1 as tchunkptr as *mut malloc_tree_chunk;
                (*TP_1).bk = TP_1 as *mut malloc_tree_chunk;
                (*TP_1).fd = (*TP_1).bk;
            } else {
                let mut T: tchunkptr = *H_1;
                let mut K_0: size_t = psize
                    << (if I_2 == NTREEBINS.wrapping_sub(1 as ::core::ffi::c_uint) {
                        0 as usize
                    } else {
                        SIZE_T_BITSIZE.wrapping_sub(SIZE_T_ONE).wrapping_sub(
                            (I_2 as ::core::ffi::c_uint >> 1 as ::core::ffi::c_int)
                                .wrapping_add(TREEBIN_SHIFT)
                                .wrapping_sub(2 as ::core::ffi::c_uint)
                                as usize,
                        )
                    });
                loop {
                    if (*T).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT) != psize {
                        let mut C: *mut tchunkptr =
                            (&raw mut (*T).child as *mut *mut malloc_tree_chunk).offset(
                                (K_0 >> SIZE_T_BITSIZE.wrapping_sub(SIZE_T_ONE) & 1 as size_t)
                                    as isize,
                            ) as *mut tchunkptr;
                        K_0 <<= 1 as ::core::ffi::c_int;
                        if !(*C).is_null() {
                            T = *C;
                        } else if (C as *mut ::core::ffi::c_char >= (*m).least_addr)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            *C = TP_1;
                            (*TP_1).parent = T as *mut malloc_tree_chunk;
                            (*TP_1).bk = TP_1 as *mut malloc_tree_chunk;
                            (*TP_1).fd = (*TP_1).bk;
                            break;
                        } else {
                            abort();
                        }
                    } else {
                        let mut F_4: tchunkptr = (*T).fd as tchunkptr;
                        if (T as *mut ::core::ffi::c_char >= (*m).least_addr
                            && F_4 as *mut ::core::ffi::c_char >= (*m).least_addr)
                            as ::core::ffi::c_int as ::core::ffi::c_long
                            != 0
                        {
                            (*F_4).bk = TP_1 as *mut malloc_tree_chunk;
                            (*T).fd = (*F_4).bk;
                            (*TP_1).fd = F_4 as *mut malloc_tree_chunk;
                            (*TP_1).bk = T as *mut malloc_tree_chunk;
                            (*TP_1).parent = ::core::ptr::null_mut::<malloc_tree_chunk>();
                            break;
                        } else {
                            abort();
                        }
                    }
                }
            }
        }
    } else {
        abort();
    };
}
unsafe extern "C" fn tmalloc_large(mut m: mstate, mut nb: size_t) -> *mut ::core::ffi::c_void {
    let mut v: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
    let mut rsize: size_t = nb.wrapping_neg();
    let mut t: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
    let mut idx: bindex_t = 0;
    let mut X: ::core::ffi::c_uint = (nb >> TREEBIN_SHIFT) as ::core::ffi::c_uint;
    if X == 0 as ::core::ffi::c_uint {
        idx = 0 as bindex_t;
    } else if X > 0xffff as ::core::ffi::c_uint {
        idx = NTREEBINS.wrapping_sub(1 as ::core::ffi::c_uint) as bindex_t;
    } else {
        let mut K: ::core::ffi::c_uint = (::core::mem::size_of::<::core::ffi::c_uint>()
            as ::core::ffi::c_uint)
            .wrapping_mul(__CHAR_BIT__ as ::core::ffi::c_uint)
            .wrapping_sub(1 as ::core::ffi::c_uint)
            .wrapping_sub(X.leading_zeros() as i32 as ::core::ffi::c_uint);
        idx = ((K << 1 as ::core::ffi::c_int) as size_t).wrapping_add(
            nb >> K.wrapping_add(TREEBIN_SHIFT.wrapping_sub(1 as ::core::ffi::c_uint))
                & 1 as size_t,
        ) as bindex_t;
    }
    t = (*m).treebins[idx as usize] as tchunkptr;
    if !t.is_null() {
        let mut sizebits: size_t = nb
            << (if idx == NTREEBINS.wrapping_sub(1 as ::core::ffi::c_uint) {
                0 as usize
            } else {
                SIZE_T_BITSIZE.wrapping_sub(SIZE_T_ONE).wrapping_sub(
                    (idx as ::core::ffi::c_uint >> 1 as ::core::ffi::c_int)
                        .wrapping_add(TREEBIN_SHIFT)
                        .wrapping_sub(2 as ::core::ffi::c_uint) as usize,
                )
            });
        let mut rst: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
        loop {
            let mut rt: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
            let mut trem: size_t =
                ((*t).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT)).wrapping_sub(nb);
            if trem < rsize {
                v = t;
                rsize = trem;
                if rsize == 0 as size_t {
                    break;
                }
            }
            rt = (*t).child[1 as ::core::ffi::c_int as usize] as tchunkptr;
            t = (*t).child
                [(sizebits >> SIZE_T_BITSIZE.wrapping_sub(SIZE_T_ONE) & 1 as size_t) as usize]
                as tchunkptr;
            if !rt.is_null() && rt != t {
                rst = rt;
            }
            if t.is_null() {
                t = rst;
                break;
            } else {
                sizebits <<= 1 as ::core::ffi::c_int;
            }
        }
    }
    if t.is_null() && v.is_null() {
        let mut leftbits: binmap_t =
            ((1 as ::core::ffi::c_int as binmap_t) << idx << 1 as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int as binmap_t) << idx << 1 as ::core::ffi::c_int)
                    .wrapping_neg())
                & (*m).treemap;
        if leftbits != 0 as binmap_t {
            let mut i: bindex_t = 0;
            let mut leastbit: binmap_t = leftbits & leftbits.wrapping_neg();
            let mut J: ::core::ffi::c_uint = 0;
            J = leastbit.trailing_zeros() as i32 as ::core::ffi::c_uint;
            i = J;
            t = (*m).treebins[i as usize] as tchunkptr;
        }
    }
    while !t.is_null() {
        let mut trem_0: size_t =
            ((*t).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT)).wrapping_sub(nb);
        if trem_0 < rsize {
            rsize = trem_0;
            v = t;
        }
        t = (if !(*t).child[0 as ::core::ffi::c_int as usize].is_null() {
            (*t).child[0 as ::core::ffi::c_int as usize]
        } else {
            (*t).child[1 as ::core::ffi::c_int as usize]
        }) as tchunkptr;
    }
    if !v.is_null() && rsize < (*m).dvsize.wrapping_sub(nb) {
        if (v as *mut ::core::ffi::c_char >= (*m).least_addr) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            let mut r: mchunkptr = (v as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr;
            if ((v as *mut ::core::ffi::c_char) < r as *mut ::core::ffi::c_char)
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                let mut XP: tchunkptr = (*v).parent as tchunkptr;
                let mut R: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
                if (*v).bk != v {
                    let mut F: tchunkptr = (*v).fd as tchunkptr;
                    R = (*v).bk as tchunkptr;
                    if (F as *mut ::core::ffi::c_char >= (*m).least_addr
                        && (*F).bk == v
                        && (*R).fd == v) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        (*F).bk = R as *mut malloc_tree_chunk;
                        (*R).fd = F as *mut malloc_tree_chunk;
                    } else {
                        abort();
                    }
                } else {
                    let mut RP: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                    RP = (&raw mut (*v).child as *mut *mut malloc_tree_chunk)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut *mut malloc_tree_chunk as *mut tchunkptr;
                    R = *RP;
                    if !R.is_null() || {
                        RP = (&raw mut (*v).child as *mut *mut malloc_tree_chunk)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut *mut malloc_tree_chunk
                            as *mut tchunkptr;
                        R = *RP;
                        !R.is_null()
                    } {
                        let mut CP: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                        loop {
                            CP = (&raw mut (*R).child as *mut *mut malloc_tree_chunk)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *mut *mut malloc_tree_chunk
                                as *mut tchunkptr;
                            if !(!(*CP).is_null() || {
                                CP = (&raw mut (*R).child as *mut *mut malloc_tree_chunk)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut *mut malloc_tree_chunk
                                    as *mut tchunkptr;
                                !(*CP).is_null()
                            }) {
                                break;
                            }
                            RP = CP;
                            R = *RP;
                        }
                        if (RP as *mut ::core::ffi::c_char >= (*m).least_addr) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            *RP = ::core::ptr::null_mut::<malloc_tree_chunk>();
                        } else {
                            abort();
                        }
                    }
                }
                if !XP.is_null() {
                    let mut H: *mut tbinptr = (&raw mut (*m).treebins as *mut tbinptr)
                        .offset((*v).index as isize)
                        as *mut tbinptr;
                    if v == *H {
                        *H = R as tbinptr;
                        if (*H).is_null() {
                            (*m).treemap &= !((1 as ::core::ffi::c_int as binmap_t) << (*v).index);
                        }
                    } else if (XP as *mut ::core::ffi::c_char >= (*m).least_addr)
                        as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        if (*XP).child[0 as ::core::ffi::c_int as usize] == v {
                            (*XP).child[0 as ::core::ffi::c_int as usize] =
                                R as *mut malloc_tree_chunk;
                        } else {
                            (*XP).child[1 as ::core::ffi::c_int as usize] =
                                R as *mut malloc_tree_chunk;
                        }
                    } else {
                        abort();
                    }
                    if !R.is_null() {
                        if (R as *mut ::core::ffi::c_char >= (*m).least_addr) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            let mut C0: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
                            let mut C1: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
                            (*R).parent = XP as *mut malloc_tree_chunk;
                            C0 = (*v).child[0 as ::core::ffi::c_int as usize] as tchunkptr;
                            if !C0.is_null() {
                                if (C0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_long
                                    != 0
                                {
                                    (*R).child[0 as ::core::ffi::c_int as usize] =
                                        C0 as *mut malloc_tree_chunk;
                                    (*C0).parent = R as *mut malloc_tree_chunk;
                                } else {
                                    abort();
                                }
                            }
                            C1 = (*v).child[1 as ::core::ffi::c_int as usize] as tchunkptr;
                            if !C1.is_null() {
                                if (C1 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_long
                                    != 0
                                {
                                    (*R).child[1 as ::core::ffi::c_int as usize] =
                                        C1 as *mut malloc_tree_chunk;
                                    (*C1).parent = R as *mut malloc_tree_chunk;
                                } else {
                                    abort();
                                }
                            }
                        } else {
                            abort();
                        }
                    }
                }
                if rsize < MIN_CHUNK_SIZE as usize {
                    (*v).head = rsize.wrapping_add(nb) | PINUSE_BIT | CINUSE_BIT;
                    (*((v as *mut ::core::ffi::c_char).offset(rsize.wrapping_add(nb) as isize)
                        as mchunkptr))
                        .head |= PINUSE_BIT;
                } else {
                    (*v).head = nb | PINUSE_BIT | CINUSE_BIT;
                    (*r).head = rsize | PINUSE_BIT;
                    (*((r as *mut ::core::ffi::c_char).offset(rsize as isize) as mchunkptr))
                        .prev_foot = rsize;
                    if rsize >> SMALLBIN_SHIFT < NSMALLBINS as size_t {
                        let mut I: bindex_t = (rsize >> SMALLBIN_SHIFT) as bindex_t;
                        let mut B: mchunkptr = (&raw mut (*m).smallbins as *mut mchunkptr)
                            .offset((I << 1 as ::core::ffi::c_int) as isize)
                            as *mut mchunkptr
                            as *mut ::core::ffi::c_char
                            as mchunkptr;
                        let mut F_0: mchunkptr = B;
                        if (*m).smallmap & (1 as ::core::ffi::c_int as binmap_t) << I == 0 {
                            (*m).smallmap |= (1 as ::core::ffi::c_int as binmap_t) << I;
                        } else if ((*B).fd as *mut ::core::ffi::c_char >= (*m).least_addr)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            F_0 = (*B).fd as mchunkptr;
                        } else {
                            abort();
                        }
                        (*B).fd = r as *mut malloc_chunk;
                        (*F_0).bk = r as *mut malloc_chunk;
                        (*r).fd = F_0 as *mut malloc_chunk;
                        (*r).bk = B as *mut malloc_chunk;
                    } else {
                        let mut TP: tchunkptr = r as tchunkptr;
                        let mut H_0: *mut tbinptr = ::core::ptr::null_mut::<tbinptr>();
                        let mut I_0: bindex_t = 0;
                        let mut X_0: ::core::ffi::c_uint =
                            (rsize >> TREEBIN_SHIFT) as ::core::ffi::c_uint;
                        if X_0 == 0 as ::core::ffi::c_uint {
                            I_0 = 0 as bindex_t;
                        } else if X_0 > 0xffff as ::core::ffi::c_uint {
                            I_0 = NTREEBINS.wrapping_sub(1 as ::core::ffi::c_uint) as bindex_t;
                        } else {
                            let mut K_0: ::core::ffi::c_uint = (::core::mem::size_of::<
                                ::core::ffi::c_uint,
                            >()
                                as ::core::ffi::c_uint)
                                .wrapping_mul(__CHAR_BIT__ as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                                .wrapping_sub(X_0.leading_zeros() as i32 as ::core::ffi::c_uint);
                            I_0 = ((K_0 << 1 as ::core::ffi::c_int) as size_t).wrapping_add(
                                rsize
                                    >> K_0.wrapping_add(
                                        TREEBIN_SHIFT.wrapping_sub(1 as ::core::ffi::c_uint),
                                    )
                                    & 1 as size_t,
                            ) as bindex_t;
                        }
                        H_0 = (&raw mut (*m).treebins as *mut tbinptr).offset(I_0 as isize)
                            as *mut tbinptr;
                        (*TP).index = I_0;
                        (*TP).child[1 as ::core::ffi::c_int as usize] =
                            ::core::ptr::null_mut::<malloc_tree_chunk>();
                        (*TP).child[0 as ::core::ffi::c_int as usize] =
                            (*TP).child[1 as ::core::ffi::c_int as usize];
                        if (*m).treemap & (1 as ::core::ffi::c_int as binmap_t) << I_0 == 0 {
                            (*m).treemap |= (1 as ::core::ffi::c_int as binmap_t) << I_0;
                            *H_0 = TP as tbinptr;
                            (*TP).parent = H_0 as tchunkptr as *mut malloc_tree_chunk;
                            (*TP).bk = TP as *mut malloc_tree_chunk;
                            (*TP).fd = (*TP).bk;
                        } else {
                            let mut T: tchunkptr = *H_0;
                            let mut K_1: size_t = rsize
                                << (if I_0 == NTREEBINS.wrapping_sub(1 as ::core::ffi::c_uint) {
                                    0 as usize
                                } else {
                                    SIZE_T_BITSIZE.wrapping_sub(SIZE_T_ONE).wrapping_sub(
                                        (I_0 as ::core::ffi::c_uint >> 1 as ::core::ffi::c_int)
                                            .wrapping_add(TREEBIN_SHIFT)
                                            .wrapping_sub(2 as ::core::ffi::c_uint)
                                            as usize,
                                    )
                                });
                            loop {
                                if (*T).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT) != rsize {
                                    let mut C: *mut tchunkptr = (&raw mut (*T).child
                                        as *mut *mut malloc_tree_chunk)
                                        .offset(
                                            (K_1 >> SIZE_T_BITSIZE.wrapping_sub(SIZE_T_ONE)
                                                & 1 as size_t)
                                                as isize,
                                        )
                                        as *mut tchunkptr;
                                    K_1 <<= 1 as ::core::ffi::c_int;
                                    if !(*C).is_null() {
                                        T = *C;
                                    } else if (C as *mut ::core::ffi::c_char >= (*m).least_addr)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        *C = TP;
                                        (*TP).parent = T as *mut malloc_tree_chunk;
                                        (*TP).bk = TP as *mut malloc_tree_chunk;
                                        (*TP).fd = (*TP).bk;
                                        break;
                                    } else {
                                        abort();
                                    }
                                } else {
                                    let mut F_1: tchunkptr = (*T).fd as tchunkptr;
                                    if (T as *mut ::core::ffi::c_char >= (*m).least_addr
                                        && F_1 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        (*F_1).bk = TP as *mut malloc_tree_chunk;
                                        (*T).fd = (*F_1).bk;
                                        (*TP).fd = F_1 as *mut malloc_tree_chunk;
                                        (*TP).bk = T as *mut malloc_tree_chunk;
                                        (*TP).parent = ::core::ptr::null_mut::<malloc_tree_chunk>();
                                        break;
                                    } else {
                                        abort();
                                    }
                                }
                            }
                        }
                    }
                }
                return (v as *mut ::core::ffi::c_char).offset(TWO_SIZE_T_SIZES as usize as isize)
                    as *mut ::core::ffi::c_void;
            }
        }
        abort();
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn tmalloc_small(mut m: mstate, mut nb: size_t) -> *mut ::core::ffi::c_void {
    let mut t: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
    let mut v: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
    let mut rsize: size_t = 0;
    let mut i: bindex_t = 0;
    let mut leastbit: binmap_t = (*m).treemap & (*m).treemap.wrapping_neg();
    let mut J: ::core::ffi::c_uint = 0;
    J = leastbit.trailing_zeros() as i32 as ::core::ffi::c_uint;
    i = J;
    t = (*m).treebins[i as usize] as tchunkptr;
    v = t;
    rsize = ((*t).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT)).wrapping_sub(nb);
    loop {
        t = (if !(*t).child[0 as ::core::ffi::c_int as usize].is_null() {
            (*t).child[0 as ::core::ffi::c_int as usize]
        } else {
            (*t).child[1 as ::core::ffi::c_int as usize]
        }) as tchunkptr;
        if t.is_null() {
            break;
        }
        let mut trem: size_t =
            ((*t).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT)).wrapping_sub(nb);
        if trem < rsize {
            rsize = trem;
            v = t;
        }
    }
    if (v as *mut ::core::ffi::c_char >= (*m).least_addr) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        let mut r: mchunkptr = (v as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr;
        if ((v as *mut ::core::ffi::c_char) < r as *mut ::core::ffi::c_char) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            let mut XP: tchunkptr = (*v).parent as tchunkptr;
            let mut R: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
            if (*v).bk != v {
                let mut F: tchunkptr = (*v).fd as tchunkptr;
                R = (*v).bk as tchunkptr;
                if (F as *mut ::core::ffi::c_char >= (*m).least_addr
                    && (*F).bk == v
                    && (*R).fd == v) as ::core::ffi::c_int as ::core::ffi::c_long
                    != 0
                {
                    (*F).bk = R as *mut malloc_tree_chunk;
                    (*R).fd = F as *mut malloc_tree_chunk;
                } else {
                    abort();
                }
            } else {
                let mut RP: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                RP = (&raw mut (*v).child as *mut *mut malloc_tree_chunk)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut *mut malloc_tree_chunk as *mut tchunkptr;
                R = *RP;
                if !R.is_null() || {
                    RP = (&raw mut (*v).child as *mut *mut malloc_tree_chunk)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut *mut malloc_tree_chunk as *mut tchunkptr;
                    R = *RP;
                    !R.is_null()
                } {
                    let mut CP: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                    loop {
                        CP = (&raw mut (*R).child as *mut *mut malloc_tree_chunk)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut *mut malloc_tree_chunk
                            as *mut tchunkptr;
                        if !(!(*CP).is_null() || {
                            CP = (&raw mut (*R).child as *mut *mut malloc_tree_chunk)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut *mut malloc_tree_chunk
                                as *mut tchunkptr;
                            !(*CP).is_null()
                        }) {
                            break;
                        }
                        RP = CP;
                        R = *RP;
                    }
                    if (RP as *mut ::core::ffi::c_char >= (*m).least_addr) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        *RP = ::core::ptr::null_mut::<malloc_tree_chunk>();
                    } else {
                        abort();
                    }
                }
            }
            if !XP.is_null() {
                let mut H: *mut tbinptr = (&raw mut (*m).treebins as *mut tbinptr)
                    .offset((*v).index as isize)
                    as *mut tbinptr;
                if v == *H {
                    *H = R as tbinptr;
                    if (*H).is_null() {
                        (*m).treemap &= !((1 as ::core::ffi::c_int as binmap_t) << (*v).index);
                    }
                } else if (XP as *mut ::core::ffi::c_char >= (*m).least_addr) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    if (*XP).child[0 as ::core::ffi::c_int as usize] == v {
                        (*XP).child[0 as ::core::ffi::c_int as usize] = R as *mut malloc_tree_chunk;
                    } else {
                        (*XP).child[1 as ::core::ffi::c_int as usize] = R as *mut malloc_tree_chunk;
                    }
                } else {
                    abort();
                }
                if !R.is_null() {
                    if (R as *mut ::core::ffi::c_char >= (*m).least_addr) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        let mut C0: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
                        let mut C1: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
                        (*R).parent = XP as *mut malloc_tree_chunk;
                        C0 = (*v).child[0 as ::core::ffi::c_int as usize] as tchunkptr;
                        if !C0.is_null() {
                            if (C0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                (*R).child[0 as ::core::ffi::c_int as usize] =
                                    C0 as *mut malloc_tree_chunk;
                                (*C0).parent = R as *mut malloc_tree_chunk;
                            } else {
                                abort();
                            }
                        }
                        C1 = (*v).child[1 as ::core::ffi::c_int as usize] as tchunkptr;
                        if !C1.is_null() {
                            if (C1 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                (*R).child[1 as ::core::ffi::c_int as usize] =
                                    C1 as *mut malloc_tree_chunk;
                                (*C1).parent = R as *mut malloc_tree_chunk;
                            } else {
                                abort();
                            }
                        }
                    } else {
                        abort();
                    }
                }
            }
            if rsize < MIN_CHUNK_SIZE as usize {
                (*v).head = rsize.wrapping_add(nb) | PINUSE_BIT | CINUSE_BIT;
                (*((v as *mut ::core::ffi::c_char).offset(rsize.wrapping_add(nb) as isize)
                    as mchunkptr))
                    .head |= PINUSE_BIT;
            } else {
                (*v).head = nb | PINUSE_BIT | CINUSE_BIT;
                (*r).head = rsize | PINUSE_BIT;
                (*((r as *mut ::core::ffi::c_char).offset(rsize as isize) as mchunkptr))
                    .prev_foot = rsize;
                let mut DVS: size_t = (*m).dvsize;
                if DVS != 0 as size_t {
                    let mut DV: mchunkptr = (*m).dv;
                    let mut I: bindex_t = (DVS >> SMALLBIN_SHIFT) as bindex_t;
                    let mut B: mchunkptr = (&raw mut (*m).smallbins as *mut mchunkptr)
                        .offset((I << 1 as ::core::ffi::c_int) as isize)
                        as *mut mchunkptr
                        as *mut ::core::ffi::c_char
                        as mchunkptr;
                    let mut F_0: mchunkptr = B;
                    if (*m).smallmap & (1 as ::core::ffi::c_int as binmap_t) << I == 0 {
                        (*m).smallmap |= (1 as ::core::ffi::c_int as binmap_t) << I;
                    } else if ((*B).fd as *mut ::core::ffi::c_char >= (*m).least_addr)
                        as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        F_0 = (*B).fd as mchunkptr;
                    } else {
                        abort();
                    }
                    (*B).fd = DV as *mut malloc_chunk;
                    (*F_0).bk = DV as *mut malloc_chunk;
                    (*DV).fd = F_0 as *mut malloc_chunk;
                    (*DV).bk = B as *mut malloc_chunk;
                }
                (*m).dvsize = rsize;
                (*m).dv = r;
            }
            return (v as *mut ::core::ffi::c_char).offset(TWO_SIZE_T_SIZES as usize as isize)
                as *mut ::core::ffi::c_void;
        }
    }
    abort();
}
unsafe extern "C" fn dlmalloc(mut bytes: size_t) -> *mut ::core::ffi::c_void {
    let mut current_block: u64;
    (crate::atomic_compat::atomic_load_acquire(&raw mut mparams.magic) != 0 as size_t
        || init_mparams() != 0) as ::core::ffi::c_int;
    if if _gm_.mflags as ::core::ffi::c_uint & USE_LOCK_BIT != 0 {
        if crate::atomic_compat::atomic_xchg_acquire(&raw mut _gm_.mutex, 1 as ::core::ffi::c_int)
            != 0
        {
            spin_acquire_lock(&raw mut _gm_.mutex)
        } else {
            0 as ::core::ffi::c_int
        }
    } else {
        0 as ::core::ffi::c_int
    } == 0
    {
        let mut mem: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut nb: size_t = 0;
        if bytes <= MAX_SMALL_REQUEST {
            let mut idx: bindex_t = 0;
            let mut smallbits: binmap_t = 0;
            nb = (if bytes < MIN_REQUEST as usize {
                MIN_CHUNK_SIZE as usize
            } else {
                (bytes as usize)
                    .wrapping_add(CHUNK_OVERHEAD as usize)
                    .wrapping_add(CHUNK_ALIGN_MASK)
                    & !CHUNK_ALIGN_MASK
            }) as size_t;
            idx = (nb >> SMALLBIN_SHIFT) as bindex_t;
            smallbits = _gm_.smallmap >> idx;
            if smallbits as ::core::ffi::c_uint & 0x3 as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                let mut b: mchunkptr = ::core::ptr::null_mut::<malloc_chunk>();
                let mut p: mchunkptr = ::core::ptr::null_mut::<malloc_chunk>();
                idx = (idx as ::core::ffi::c_uint)
                    .wrapping_add((!smallbits & 1 as binmap_t) as ::core::ffi::c_uint)
                    as bindex_t as bindex_t;
                b = (&raw mut _gm_.smallbins as *mut mchunkptr)
                    .offset((idx << 1 as ::core::ffi::c_int) as isize)
                    as *mut mchunkptr as *mut ::core::ffi::c_char as sbinptr
                    as mchunkptr;
                p = (*b).fd as mchunkptr;
                let mut F: mchunkptr = (*p).fd as mchunkptr;
                if b == F {
                    _gm_.smallmap &= !((1 as ::core::ffi::c_int as binmap_t) << idx);
                } else if (F as *mut ::core::ffi::c_char >= _gm_.least_addr && (*F).bk == p)
                    as ::core::ffi::c_int as ::core::ffi::c_long
                    != 0
                {
                    (*F).bk = b as *mut malloc_chunk;
                    (*b).fd = F as *mut malloc_chunk;
                } else {
                    abort();
                }
                (*p).head = (idx << 3 as ::core::ffi::c_uint) as size_t | PINUSE_BIT | CINUSE_BIT;
                (*((p as *mut ::core::ffi::c_char)
                    .offset((idx << 3 as ::core::ffi::c_uint) as isize)
                    as mchunkptr))
                    .head |= PINUSE_BIT;
                mem = (p as *mut ::core::ffi::c_char).offset(TWO_SIZE_T_SIZES as usize as isize)
                    as *mut ::core::ffi::c_void;
                current_block = 3828701184648723886;
            } else if nb > _gm_.dvsize {
                if smallbits != 0 as binmap_t {
                    let mut b_0: mchunkptr = ::core::ptr::null_mut::<malloc_chunk>();
                    let mut p_0: mchunkptr = ::core::ptr::null_mut::<malloc_chunk>();
                    let mut r: mchunkptr = ::core::ptr::null_mut::<malloc_chunk>();
                    let mut rsize: size_t = 0;
                    let mut i: bindex_t = 0;
                    let mut leftbits: binmap_t = smallbits << idx
                        & ((1 as ::core::ffi::c_int as binmap_t) << idx << 1 as ::core::ffi::c_int
                            | ((1 as ::core::ffi::c_int as binmap_t)
                                << idx
                                << 1 as ::core::ffi::c_int)
                                .wrapping_neg());
                    let mut leastbit: binmap_t = leftbits & leftbits.wrapping_neg();
                    let mut J: ::core::ffi::c_uint = 0;
                    J = leastbit.trailing_zeros() as i32 as ::core::ffi::c_uint;
                    i = J;
                    b_0 = (&raw mut _gm_.smallbins as *mut mchunkptr)
                        .offset((i << 1 as ::core::ffi::c_int) as isize)
                        as *mut mchunkptr as *mut ::core::ffi::c_char
                        as sbinptr as mchunkptr;
                    p_0 = (*b_0).fd as mchunkptr;
                    let mut F_0: mchunkptr = (*p_0).fd as mchunkptr;
                    if b_0 == F_0 {
                        _gm_.smallmap &= !((1 as ::core::ffi::c_int as binmap_t) << i);
                    } else if (F_0 as *mut ::core::ffi::c_char >= _gm_.least_addr
                        && (*F_0).bk == p_0) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        (*F_0).bk = b_0 as *mut malloc_chunk;
                        (*b_0).fd = F_0 as *mut malloc_chunk;
                    } else {
                        abort();
                    }
                    rsize = ((i << SMALLBIN_SHIFT) as size_t).wrapping_sub(nb);
                    if SIZE_T_SIZE as usize != 4 as usize && rsize < MIN_CHUNK_SIZE as usize {
                        (*p_0).head =
                            (i << 3 as ::core::ffi::c_uint) as size_t | PINUSE_BIT | CINUSE_BIT;
                        (*((p_0 as *mut ::core::ffi::c_char)
                            .offset((i << 3 as ::core::ffi::c_uint) as isize)
                            as mchunkptr))
                            .head |= PINUSE_BIT;
                    } else {
                        (*p_0).head = nb | PINUSE_BIT | CINUSE_BIT;
                        r = (p_0 as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr;
                        (*r).head = rsize | PINUSE_BIT;
                        (*((r as *mut ::core::ffi::c_char).offset(rsize as isize) as mchunkptr))
                            .prev_foot = rsize;
                        let mut DVS: size_t = _gm_.dvsize;
                        if DVS != 0 as size_t {
                            let mut DV: mchunkptr = _gm_.dv;
                            let mut I: bindex_t = (DVS >> SMALLBIN_SHIFT) as bindex_t;
                            let mut B: mchunkptr = (&raw mut _gm_.smallbins as *mut mchunkptr)
                                .offset((I << 1 as ::core::ffi::c_int) as isize)
                                as *mut mchunkptr
                                as *mut ::core::ffi::c_char
                                as mchunkptr;
                            let mut F_1: mchunkptr = B;
                            if _gm_.smallmap & (1 as ::core::ffi::c_int as binmap_t) << I == 0 {
                                _gm_.smallmap |= (1 as ::core::ffi::c_int as binmap_t) << I;
                            } else if ((*B).fd as *mut ::core::ffi::c_char >= _gm_.least_addr)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                F_1 = (*B).fd as mchunkptr;
                            } else {
                                abort();
                            }
                            (*B).fd = DV as *mut malloc_chunk;
                            (*F_1).bk = DV as *mut malloc_chunk;
                            (*DV).fd = F_1 as *mut malloc_chunk;
                            (*DV).bk = B as *mut malloc_chunk;
                        }
                        _gm_.dvsize = rsize;
                        _gm_.dv = r;
                    }
                    mem = (p_0 as *mut ::core::ffi::c_char)
                        .offset(TWO_SIZE_T_SIZES as usize as isize)
                        as *mut ::core::ffi::c_void;
                    current_block = 3828701184648723886;
                } else if _gm_.treemap != 0 as binmap_t && {
                    mem = tmalloc_small(&raw mut _gm_, nb);
                    !mem.is_null()
                } {
                    current_block = 3828701184648723886;
                } else {
                    current_block = 7419121793134201633;
                }
            } else {
                current_block = 7419121793134201633;
            }
        } else if bytes >= MAX_REQUEST as usize {
            nb = MAX_SIZE_T;
            current_block = 7419121793134201633;
        } else {
            nb = bytes
                .wrapping_add(CHUNK_OVERHEAD as size_t)
                .wrapping_add(CHUNK_ALIGN_MASK)
                & !CHUNK_ALIGN_MASK;
            if _gm_.treemap != 0 as binmap_t && {
                mem = tmalloc_large(&raw mut _gm_, nb);
                !mem.is_null()
            } {
                current_block = 3828701184648723886;
            } else {
                current_block = 7419121793134201633;
            }
        }
        match current_block {
            7419121793134201633 => {
                if nb <= _gm_.dvsize {
                    let mut rsize_0: size_t = _gm_.dvsize.wrapping_sub(nb);
                    let mut p_1: mchunkptr = _gm_.dv;
                    if rsize_0 >= MIN_CHUNK_SIZE as usize {
                        _gm_.dv =
                            (p_1 as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr;
                        let mut r_0: mchunkptr = _gm_.dv;
                        _gm_.dvsize = rsize_0;
                        (*r_0).head = rsize_0 | PINUSE_BIT;
                        (*((r_0 as *mut ::core::ffi::c_char).offset(rsize_0 as isize)
                            as mchunkptr))
                            .prev_foot = rsize_0;
                        (*p_1).head = nb | PINUSE_BIT | CINUSE_BIT;
                    } else {
                        let mut dvs: size_t = _gm_.dvsize;
                        _gm_.dvsize = 0 as size_t;
                        _gm_.dv = ::core::ptr::null_mut::<malloc_chunk>();
                        (*p_1).head = dvs | PINUSE_BIT | CINUSE_BIT;
                        (*((p_1 as *mut ::core::ffi::c_char).offset(dvs as isize) as mchunkptr))
                            .head |= PINUSE_BIT;
                    }
                    mem = (p_1 as *mut ::core::ffi::c_char)
                        .offset(TWO_SIZE_T_SIZES as usize as isize)
                        as *mut ::core::ffi::c_void;
                } else if nb < _gm_.topsize {
                    _gm_.topsize = _gm_.topsize.wrapping_sub(nb);
                    let mut rsize_1: size_t = _gm_.topsize;
                    let mut p_2: mchunkptr = _gm_.top;
                    _gm_.top = (p_2 as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr;
                    let mut r_1: mchunkptr = _gm_.top;
                    (*r_1).head = rsize_1 | PINUSE_BIT;
                    (*p_2).head = nb | PINUSE_BIT | CINUSE_BIT;
                    mem = (p_2 as *mut ::core::ffi::c_char)
                        .offset(TWO_SIZE_T_SIZES as usize as isize)
                        as *mut ::core::ffi::c_void;
                } else {
                    mem = sys_alloc(&raw mut _gm_, nb);
                }
            }
            _ => {}
        }
        if _gm_.mflags as ::core::ffi::c_uint & USE_LOCK_BIT != 0 {
            crate::atomic_compat::atomic_store_release(&raw mut _gm_.mutex, 0);
        }
        return mem;
    } else {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    };
}
unsafe extern "C" fn dlfree(mut mem: *mut ::core::ffi::c_void) {
    let mut current_block: u64;
    if !mem.is_null() {
        let mut p: mchunkptr = (mem as *mut ::core::ffi::c_char)
            .offset(-(TWO_SIZE_T_SIZES as usize as isize))
            as mchunkptr;
        if if _gm_.mflags as ::core::ffi::c_uint & USE_LOCK_BIT != 0 {
            if crate::atomic_compat::atomic_xchg_acquire(
                &raw mut _gm_.mutex,
                1 as ::core::ffi::c_int,
            ) != 0
            {
                spin_acquire_lock(&raw mut _gm_.mutex)
            } else {
                0 as ::core::ffi::c_int
            }
        } else {
            0 as ::core::ffi::c_int
        } == 0
        {
            if (p as *mut ::core::ffi::c_char >= _gm_.least_addr
                && (*p).head
                    & (1 as ::core::ffi::c_int as size_t | 2 as ::core::ffi::c_int as size_t)
                    != 1 as ::core::ffi::c_int as size_t) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
            {
                let mut psize: size_t = (*p).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT);
                let mut next: mchunkptr =
                    (p as *mut ::core::ffi::c_char).offset(psize as isize) as mchunkptr;
                if (*p).head & PINUSE_BIT == 0 {
                    let mut prevsize: size_t = (*p).prev_foot;
                    if (*p).head & INUSE_BITS == 0 as size_t {
                        psize = (psize as ::core::ffi::c_ulong).wrapping_add(
                            prevsize.wrapping_add(MMAP_FOOT_PAD) as ::core::ffi::c_ulong,
                        ) as size_t as size_t;
                        if dlmunmap(
                            (p as *mut ::core::ffi::c_char).offset(-(prevsize as isize))
                                as *mut ::core::ffi::c_void,
                            psize,
                        ) == 0 as ::core::ffi::c_int
                        {
                            _gm_.footprint = _gm_.footprint.wrapping_sub(psize);
                        }
                        current_block = 14836311167396004099;
                    } else {
                        let mut prev: mchunkptr = (p as *mut ::core::ffi::c_char)
                            .offset(-(prevsize as isize))
                            as mchunkptr;
                        psize = psize.wrapping_add(prevsize);
                        p = prev;
                        if (prev as *mut ::core::ffi::c_char >= _gm_.least_addr)
                            as ::core::ffi::c_int as ::core::ffi::c_long
                            != 0
                        {
                            if p != _gm_.dv {
                                if prevsize >> SMALLBIN_SHIFT < NSMALLBINS as size_t {
                                    let mut F: mchunkptr = (*p).fd as mchunkptr;
                                    let mut B: mchunkptr = (*p).bk as mchunkptr;
                                    let mut I: bindex_t = (prevsize >> SMALLBIN_SHIFT) as bindex_t;
                                    if (F
                                        == (&raw mut _gm_.smallbins as *mut mchunkptr)
                                            .offset((I << 1 as ::core::ffi::c_int) as isize)
                                            as *mut mchunkptr
                                            as *mut ::core::ffi::c_char
                                            as sbinptr
                                        || F as *mut ::core::ffi::c_char >= _gm_.least_addr
                                            && (*F).bk == p)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        if B == F {
                                            _gm_.smallmap &=
                                                !((1 as ::core::ffi::c_int as binmap_t) << I);
                                        } else if (B
                                            == (&raw mut _gm_.smallbins as *mut mchunkptr)
                                                .offset((I << 1 as ::core::ffi::c_int) as isize)
                                                as *mut mchunkptr
                                                as *mut ::core::ffi::c_char
                                                as sbinptr
                                            || B as *mut ::core::ffi::c_char >= _gm_.least_addr
                                                && (*B).fd == p)
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                            (*F).bk = B as *mut malloc_chunk;
                                            (*B).fd = F as *mut malloc_chunk;
                                        } else {
                                            abort();
                                        }
                                    } else {
                                        abort();
                                    }
                                } else {
                                    let mut TP: tchunkptr = p as tchunkptr;
                                    let mut XP: tchunkptr = (*TP).parent as tchunkptr;
                                    let mut R: tchunkptr =
                                        ::core::ptr::null_mut::<malloc_tree_chunk>();
                                    if (*TP).bk != TP {
                                        let mut F_0: tchunkptr = (*TP).fd as tchunkptr;
                                        R = (*TP).bk as tchunkptr;
                                        if (F_0 as *mut ::core::ffi::c_char >= _gm_.least_addr
                                            && (*F_0).bk == TP
                                            && (*R).fd == TP)
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                            (*F_0).bk = R as *mut malloc_tree_chunk;
                                            (*R).fd = F_0 as *mut malloc_tree_chunk;
                                        } else {
                                            abort();
                                        }
                                    } else {
                                        let mut RP: *mut tchunkptr =
                                            ::core::ptr::null_mut::<tchunkptr>();
                                        RP = (&raw mut (*TP).child as *mut *mut malloc_tree_chunk)
                                            .offset(1 as ::core::ffi::c_int as isize)
                                            as *mut *mut malloc_tree_chunk
                                            as *mut tchunkptr;
                                        R = *RP;
                                        if !R.is_null() || {
                                            RP = (&raw mut (*TP).child
                                                as *mut *mut malloc_tree_chunk)
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as *mut *mut malloc_tree_chunk
                                                as *mut tchunkptr;
                                            R = *RP;
                                            !R.is_null()
                                        } {
                                            let mut CP: *mut tchunkptr =
                                                ::core::ptr::null_mut::<tchunkptr>();
                                            loop {
                                                CP = (&raw mut (*R).child
                                                    as *mut *mut malloc_tree_chunk)
                                                    .offset(1 as ::core::ffi::c_int as isize)
                                                    as *mut *mut malloc_tree_chunk
                                                    as *mut tchunkptr;
                                                if !(!(*CP).is_null() || {
                                                    CP = (&raw mut (*R).child
                                                        as *mut *mut malloc_tree_chunk)
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as *mut *mut malloc_tree_chunk
                                                        as *mut tchunkptr;
                                                    !(*CP).is_null()
                                                }) {
                                                    break;
                                                }
                                                RP = CP;
                                                R = *RP;
                                            }
                                            if (RP as *mut ::core::ffi::c_char >= _gm_.least_addr)
                                                as ::core::ffi::c_int
                                                as ::core::ffi::c_long
                                                != 0
                                            {
                                                *RP = ::core::ptr::null_mut::<malloc_tree_chunk>();
                                            } else {
                                                abort();
                                            }
                                        }
                                    }
                                    if !XP.is_null() {
                                        let mut H: *mut tbinptr = (&raw mut _gm_.treebins
                                            as *mut tbinptr)
                                            .offset((*TP).index as isize)
                                            as *mut tbinptr;
                                        if TP == *H {
                                            *H = R as tbinptr;
                                            if (*H).is_null() {
                                                _gm_.treemap &= !((1 as ::core::ffi::c_int
                                                    as binmap_t)
                                                    << (*TP).index);
                                            }
                                        } else if (XP as *mut ::core::ffi::c_char
                                            >= _gm_.least_addr)
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                            if (*XP).child[0 as ::core::ffi::c_int as usize] == TP {
                                                (*XP).child[0 as ::core::ffi::c_int as usize] =
                                                    R as *mut malloc_tree_chunk;
                                            } else {
                                                (*XP).child[1 as ::core::ffi::c_int as usize] =
                                                    R as *mut malloc_tree_chunk;
                                            }
                                        } else {
                                            abort();
                                        }
                                        if !R.is_null() {
                                            if (R as *mut ::core::ffi::c_char >= _gm_.least_addr)
                                                as ::core::ffi::c_int
                                                as ::core::ffi::c_long
                                                != 0
                                            {
                                                let mut C0: tchunkptr =
                                                    ::core::ptr::null_mut::<malloc_tree_chunk>();
                                                let mut C1: tchunkptr =
                                                    ::core::ptr::null_mut::<malloc_tree_chunk>();
                                                (*R).parent = XP as *mut malloc_tree_chunk;
                                                C0 = (*TP).child[0 as ::core::ffi::c_int as usize]
                                                    as tchunkptr;
                                                if !C0.is_null() {
                                                    if (C0 as *mut ::core::ffi::c_char
                                                        >= _gm_.least_addr)
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_long
                                                        != 0
                                                    {
                                                        (*R).child
                                                            [0 as ::core::ffi::c_int as usize] =
                                                            C0 as *mut malloc_tree_chunk;
                                                        (*C0).parent = R as *mut malloc_tree_chunk;
                                                    } else {
                                                        abort();
                                                    }
                                                }
                                                C1 = (*TP).child[1 as ::core::ffi::c_int as usize]
                                                    as tchunkptr;
                                                if !C1.is_null() {
                                                    if (C1 as *mut ::core::ffi::c_char
                                                        >= _gm_.least_addr)
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_long
                                                        != 0
                                                    {
                                                        (*R).child
                                                            [1 as ::core::ffi::c_int as usize] =
                                                            C1 as *mut malloc_tree_chunk;
                                                        (*C1).parent = R as *mut malloc_tree_chunk;
                                                    } else {
                                                        abort();
                                                    }
                                                }
                                            } else {
                                                abort();
                                            }
                                        }
                                    }
                                }
                                current_block = 10778260831612459202;
                            } else if (*next).head & INUSE_BITS == INUSE_BITS {
                                _gm_.dvsize = psize;
                                (*next).head &= !PINUSE_BIT;
                                (*p).head = psize | PINUSE_BIT;
                                (*((p as *mut ::core::ffi::c_char).offset(psize as isize)
                                    as mchunkptr))
                                    .prev_foot = psize;
                                current_block = 14836311167396004099;
                            } else {
                                current_block = 10778260831612459202;
                            }
                        } else {
                            current_block = 1766691779471037599;
                        }
                    }
                } else {
                    current_block = 10778260831612459202;
                }
                match current_block {
                    1766691779471037599 => {}
                    _ => {
                        match current_block {
                            10778260831612459202 => {
                                if ((p as *mut ::core::ffi::c_char)
                                    < next as *mut ::core::ffi::c_char
                                    && (*next).head & 1 as ::core::ffi::c_int as size_t != 0)
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_long
                                    != 0
                                {
                                    if (*next).head & CINUSE_BIT == 0 {
                                        if next == _gm_.top {
                                            _gm_.topsize = _gm_.topsize.wrapping_add(psize);
                                            let mut tsize: size_t = _gm_.topsize;
                                            _gm_.top = p;
                                            (*p).head = tsize | PINUSE_BIT;
                                            if p == _gm_.dv {
                                                _gm_.dv = ::core::ptr::null_mut::<malloc_chunk>();
                                                _gm_.dvsize = 0 as size_t;
                                            }
                                            if tsize > _gm_.trim_check {
                                                sys_trim(&raw mut _gm_, 0 as size_t);
                                            }
                                            current_block = 14836311167396004099;
                                        } else if next == _gm_.dv {
                                            _gm_.dvsize = _gm_.dvsize.wrapping_add(psize);
                                            let mut dsize: size_t = _gm_.dvsize;
                                            _gm_.dv = p;
                                            (*p).head = dsize | PINUSE_BIT;
                                            (*((p as *mut ::core::ffi::c_char)
                                                .offset(dsize as isize)
                                                as mchunkptr))
                                                .prev_foot = dsize;
                                            current_block = 14836311167396004099;
                                        } else {
                                            let mut nsize: size_t = (*next).head
                                                & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT);
                                            psize = psize.wrapping_add(nsize);
                                            if nsize >> SMALLBIN_SHIFT < NSMALLBINS as size_t {
                                                let mut F_1: mchunkptr = (*next).fd as mchunkptr;
                                                let mut B_0: mchunkptr = (*next).bk as mchunkptr;
                                                let mut I_0: bindex_t =
                                                    (nsize >> SMALLBIN_SHIFT) as bindex_t;
                                                if (F_1
                                                    == (&raw mut _gm_.smallbins as *mut mchunkptr)
                                                        .offset(
                                                            (I_0 << 1 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        as *mut mchunkptr
                                                        as *mut ::core::ffi::c_char
                                                        as sbinptr
                                                    || F_1 as *mut ::core::ffi::c_char
                                                        >= _gm_.least_addr
                                                        && (*F_1).bk == next)
                                                    as ::core::ffi::c_int
                                                    as ::core::ffi::c_long
                                                    != 0
                                                {
                                                    if B_0 == F_1 {
                                                        _gm_.smallmap &= !((1 as ::core::ffi::c_int
                                                            as binmap_t)
                                                            << I_0);
                                                    } else if (B_0
                                                        == (&raw mut _gm_.smallbins
                                                            as *mut mchunkptr)
                                                            .offset(
                                                                (I_0 << 1 as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                            as *mut mchunkptr
                                                            as *mut ::core::ffi::c_char
                                                            as sbinptr
                                                        || B_0 as *mut ::core::ffi::c_char
                                                            >= _gm_.least_addr
                                                            && (*B_0).fd == next)
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_long
                                                        != 0
                                                    {
                                                        (*F_1).bk = B_0 as *mut malloc_chunk;
                                                        (*B_0).fd = F_1 as *mut malloc_chunk;
                                                    } else {
                                                        abort();
                                                    }
                                                } else {
                                                    abort();
                                                }
                                            } else {
                                                let mut TP_0: tchunkptr = next as tchunkptr;
                                                let mut XP_0: tchunkptr =
                                                    (*TP_0).parent as tchunkptr;
                                                let mut R_0: tchunkptr =
                                                    ::core::ptr::null_mut::<malloc_tree_chunk>();
                                                if (*TP_0).bk != TP_0 {
                                                    let mut F_2: tchunkptr =
                                                        (*TP_0).fd as tchunkptr;
                                                    R_0 = (*TP_0).bk as tchunkptr;
                                                    if (F_2 as *mut ::core::ffi::c_char
                                                        >= _gm_.least_addr
                                                        && (*F_2).bk == TP_0
                                                        && (*R_0).fd == TP_0)
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_long
                                                        != 0
                                                    {
                                                        (*F_2).bk = R_0 as *mut malloc_tree_chunk;
                                                        (*R_0).fd = F_2 as *mut malloc_tree_chunk;
                                                    } else {
                                                        abort();
                                                    }
                                                } else {
                                                    let mut RP_0: *mut tchunkptr =
                                                        ::core::ptr::null_mut::<tchunkptr>();
                                                    RP_0 = (&raw mut (*TP_0).child
                                                        as *mut *mut malloc_tree_chunk)
                                                        .offset(1 as ::core::ffi::c_int as isize)
                                                        as *mut *mut malloc_tree_chunk
                                                        as *mut tchunkptr;
                                                    R_0 = *RP_0;
                                                    if !R_0.is_null() || {
                                                        RP_0 = (&raw mut (*TP_0).child
                                                            as *mut *mut malloc_tree_chunk)
                                                            .offset(
                                                                0 as ::core::ffi::c_int as isize,
                                                            )
                                                            as *mut *mut malloc_tree_chunk
                                                            as *mut tchunkptr;
                                                        R_0 = *RP_0;
                                                        !R_0.is_null()
                                                    } {
                                                        let mut CP_0: *mut tchunkptr =
                                                            ::core::ptr::null_mut::<tchunkptr>();
                                                        loop {
                                                            CP_0 = (&raw mut (*R_0).child
                                                                as *mut *mut malloc_tree_chunk)
                                                                .offset(
                                                                    1 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                as *mut *mut malloc_tree_chunk
                                                                as *mut tchunkptr;
                                                            if !(!(*CP_0).is_null() || {
                                                                CP_0 = (&raw mut (*R_0).child
                                                                    as *mut *mut malloc_tree_chunk)
                                                                    .offset(
                                                                        0 as ::core::ffi::c_int
                                                                            as isize,
                                                                    )
                                                                    as *mut *mut malloc_tree_chunk
                                                                    as *mut tchunkptr;
                                                                !(*CP_0).is_null()
                                                            }) {
                                                                break;
                                                            }
                                                            RP_0 = CP_0;
                                                            R_0 = *RP_0;
                                                        }
                                                        if (RP_0 as *mut ::core::ffi::c_char
                                                            >= _gm_.least_addr)
                                                            as ::core::ffi::c_int
                                                            as ::core::ffi::c_long
                                                            != 0
                                                        {
                                                            *RP_0 = ::core::ptr::null_mut::<
                                                                malloc_tree_chunk,
                                                            >(
                                                            );
                                                        } else {
                                                            abort();
                                                        }
                                                    }
                                                }
                                                if !XP_0.is_null() {
                                                    let mut H_0: *mut tbinptr =
                                                        (&raw mut _gm_.treebins as *mut tbinptr)
                                                            .offset((*TP_0).index as isize)
                                                            as *mut tbinptr;
                                                    if TP_0 == *H_0 {
                                                        *H_0 = R_0 as tbinptr;
                                                        if (*H_0).is_null() {
                                                            _gm_.treemap &= !((1
                                                                as ::core::ffi::c_int
                                                                as binmap_t)
                                                                << (*TP_0).index);
                                                        }
                                                    } else if (XP_0 as *mut ::core::ffi::c_char
                                                        >= _gm_.least_addr)
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_long
                                                        != 0
                                                    {
                                                        if (*XP_0).child
                                                            [0 as ::core::ffi::c_int as usize]
                                                            == TP_0
                                                        {
                                                            (*XP_0).child[0 as ::core::ffi::c_int
                                                                as usize] =
                                                                R_0 as *mut malloc_tree_chunk;
                                                        } else {
                                                            (*XP_0).child[1 as ::core::ffi::c_int
                                                                as usize] =
                                                                R_0 as *mut malloc_tree_chunk;
                                                        }
                                                    } else {
                                                        abort();
                                                    }
                                                    if !R_0.is_null() {
                                                        if (R_0 as *mut ::core::ffi::c_char
                                                            >= _gm_.least_addr)
                                                            as ::core::ffi::c_int
                                                            as ::core::ffi::c_long
                                                            != 0
                                                        {
                                                            let mut C0_0: tchunkptr =
                                                                ::core::ptr::null_mut::<
                                                                    malloc_tree_chunk,
                                                                >(
                                                                );
                                                            let mut C1_0: tchunkptr =
                                                                ::core::ptr::null_mut::<
                                                                    malloc_tree_chunk,
                                                                >(
                                                                );
                                                            (*R_0).parent =
                                                                XP_0 as *mut malloc_tree_chunk;
                                                            C0_0 = (*TP_0).child
                                                                [0 as ::core::ffi::c_int as usize]
                                                                as tchunkptr;
                                                            if !C0_0.is_null() {
                                                                if (C0_0
                                                                    as *mut ::core::ffi::c_char
                                                                    >= _gm_.least_addr)
                                                                    as ::core::ffi::c_int
                                                                    as ::core::ffi::c_long
                                                                    != 0
                                                                {
                                                                    (*R_0).child[0
                                                                        as ::core::ffi::c_int
                                                                        as usize] = C0_0
                                                                        as *mut malloc_tree_chunk;
                                                                    (*C0_0).parent = R_0
                                                                        as *mut malloc_tree_chunk;
                                                                } else {
                                                                    abort();
                                                                }
                                                            }
                                                            C1_0 = (*TP_0).child
                                                                [1 as ::core::ffi::c_int as usize]
                                                                as tchunkptr;
                                                            if !C1_0.is_null() {
                                                                if (C1_0
                                                                    as *mut ::core::ffi::c_char
                                                                    >= _gm_.least_addr)
                                                                    as ::core::ffi::c_int
                                                                    as ::core::ffi::c_long
                                                                    != 0
                                                                {
                                                                    (*R_0).child[1
                                                                        as ::core::ffi::c_int
                                                                        as usize] = C1_0
                                                                        as *mut malloc_tree_chunk;
                                                                    (*C1_0).parent = R_0
                                                                        as *mut malloc_tree_chunk;
                                                                } else {
                                                                    abort();
                                                                }
                                                            }
                                                        } else {
                                                            abort();
                                                        }
                                                    }
                                                }
                                            }
                                            (*p).head = psize | PINUSE_BIT;
                                            (*((p as *mut ::core::ffi::c_char)
                                                .offset(psize as isize)
                                                as mchunkptr))
                                                .prev_foot = psize;
                                            if p == _gm_.dv {
                                                _gm_.dvsize = psize;
                                                current_block = 14836311167396004099;
                                            } else {
                                                current_block = 2415422468722899689;
                                            }
                                        }
                                    } else {
                                        (*next).head &= !PINUSE_BIT;
                                        (*p).head = psize | PINUSE_BIT;
                                        (*((p as *mut ::core::ffi::c_char).offset(psize as isize)
                                            as mchunkptr))
                                            .prev_foot = psize;
                                        current_block = 2415422468722899689;
                                    }
                                    match current_block {
                                        14836311167396004099 => {}
                                        _ => {
                                            if psize >> SMALLBIN_SHIFT < NSMALLBINS as size_t {
                                                let mut I_1: bindex_t =
                                                    (psize >> SMALLBIN_SHIFT) as bindex_t;
                                                let mut B_1: mchunkptr = (&raw mut _gm_.smallbins
                                                    as *mut mchunkptr)
                                                    .offset(
                                                        (I_1 << 1 as ::core::ffi::c_int) as isize,
                                                    )
                                                    as *mut mchunkptr
                                                    as *mut ::core::ffi::c_char
                                                    as mchunkptr;
                                                let mut F_3: mchunkptr = B_1;
                                                if _gm_.smallmap
                                                    & (1 as ::core::ffi::c_int as binmap_t) << I_1
                                                    == 0
                                                {
                                                    _gm_.smallmap |= (1 as ::core::ffi::c_int
                                                        as binmap_t)
                                                        << I_1;
                                                } else if ((*B_1).fd as *mut ::core::ffi::c_char
                                                    >= _gm_.least_addr)
                                                    as ::core::ffi::c_int
                                                    as ::core::ffi::c_long
                                                    != 0
                                                {
                                                    F_3 = (*B_1).fd as mchunkptr;
                                                } else {
                                                    abort();
                                                }
                                                (*B_1).fd = p as *mut malloc_chunk;
                                                (*F_3).bk = p as *mut malloc_chunk;
                                                (*p).fd = F_3 as *mut malloc_chunk;
                                                (*p).bk = B_1 as *mut malloc_chunk;
                                            } else {
                                                let mut tp: tchunkptr = p as tchunkptr;
                                                let mut H_1: *mut tbinptr =
                                                    ::core::ptr::null_mut::<tbinptr>();
                                                let mut I_2: bindex_t = 0;
                                                let mut X: ::core::ffi::c_uint =
                                                    (psize >> TREEBIN_SHIFT) as ::core::ffi::c_uint;
                                                if X == 0 as ::core::ffi::c_uint {
                                                    I_2 = 0 as bindex_t;
                                                } else if X > 0xffff as ::core::ffi::c_uint {
                                                    I_2 = NTREEBINS
                                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                                        as bindex_t;
                                                } else {
                                                    let mut K: ::core::ffi::c_uint = (::core::mem::size_of::<
                                                        ::core::ffi::c_uint,
                                                    >() as ::core::ffi::c_uint)
                                                        .wrapping_mul(__CHAR_BIT__ as ::core::ffi::c_uint)
                                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                                        .wrapping_sub(
                                                            X.leading_zeros() as i32 as ::core::ffi::c_uint,
                                                        );
                                                    I_2 = ((K << 1 as ::core::ffi::c_int) as size_t)
                                                        .wrapping_add(
                                                            psize
                                                                >> K.wrapping_add(
                                                                    TREEBIN_SHIFT.wrapping_sub(
                                                                        1 as ::core::ffi::c_uint,
                                                                    ),
                                                                )
                                                                & 1 as size_t,
                                                        )
                                                        as bindex_t;
                                                }
                                                H_1 = (&raw mut _gm_.treebins as *mut tbinptr)
                                                    .offset(I_2 as isize)
                                                    as *mut tbinptr;
                                                (*tp).index = I_2;
                                                (*tp).child[1 as ::core::ffi::c_int as usize] =
                                                    ::core::ptr::null_mut::<malloc_tree_chunk>();
                                                (*tp).child[0 as ::core::ffi::c_int as usize] =
                                                    (*tp).child[1 as ::core::ffi::c_int as usize];
                                                if _gm_.treemap
                                                    & (1 as ::core::ffi::c_int as binmap_t) << I_2
                                                    == 0
                                                {
                                                    _gm_.treemap |= (1 as ::core::ffi::c_int
                                                        as binmap_t)
                                                        << I_2;
                                                    *H_1 = tp as tbinptr;
                                                    (*tp).parent =
                                                        H_1 as tchunkptr as *mut malloc_tree_chunk;
                                                    (*tp).bk = tp as *mut malloc_tree_chunk;
                                                    (*tp).fd = (*tp).bk;
                                                } else {
                                                    let mut T: tchunkptr = *H_1;
                                                    let mut K_0: size_t = psize
                                                        << (if I_2
                                                            == NTREEBINS.wrapping_sub(
                                                                1 as ::core::ffi::c_uint,
                                                            ) {
                                                            0 as usize
                                                        } else {
                                                            SIZE_T_BITSIZE
                                                                .wrapping_sub(SIZE_T_ONE)
                                                                .wrapping_sub(
                                                                (I_2 as ::core::ffi::c_uint
                                                                    >> 1 as ::core::ffi::c_int)
                                                                    .wrapping_add(TREEBIN_SHIFT)
                                                                    .wrapping_sub(
                                                                        2 as ::core::ffi::c_uint,
                                                                    )
                                                                    as usize,
                                                            )
                                                        });
                                                    loop {
                                                        if (*T).head
                                                            & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT)
                                                            != psize
                                                        {
                                                            let mut C: *mut tchunkptr =
                                                                (&raw mut (*T).child
                                                                    as *mut *mut malloc_tree_chunk)
                                                                    .offset(
                                                                        (K_0 >> SIZE_T_BITSIZE
                                                                            .wrapping_sub(
                                                                                SIZE_T_ONE,
                                                                            )
                                                                            & 1 as size_t)
                                                                            as isize,
                                                                    )
                                                                    as *mut tchunkptr;
                                                            K_0 <<= 1 as ::core::ffi::c_int;
                                                            if !(*C).is_null() {
                                                                T = *C;
                                                            } else if (C
                                                                as *mut ::core::ffi::c_char
                                                                >= _gm_.least_addr)
                                                                as ::core::ffi::c_int
                                                                as ::core::ffi::c_long
                                                                != 0
                                                            {
                                                                *C = tp;
                                                                (*tp).parent =
                                                                    T as *mut malloc_tree_chunk;
                                                                (*tp).bk =
                                                                    tp as *mut malloc_tree_chunk;
                                                                (*tp).fd = (*tp).bk;
                                                                break;
                                                            } else {
                                                                abort();
                                                            }
                                                        } else {
                                                            let mut F_4: tchunkptr =
                                                                (*T).fd as tchunkptr;
                                                            if (T as *mut ::core::ffi::c_char
                                                                >= _gm_.least_addr
                                                                && F_4 as *mut ::core::ffi::c_char
                                                                    >= _gm_.least_addr)
                                                                as ::core::ffi::c_int
                                                                as ::core::ffi::c_long
                                                                != 0
                                                            {
                                                                (*F_4).bk =
                                                                    tp as *mut malloc_tree_chunk;
                                                                (*T).fd = (*F_4).bk;
                                                                (*tp).fd =
                                                                    F_4 as *mut malloc_tree_chunk;
                                                                (*tp).bk =
                                                                    T as *mut malloc_tree_chunk;
                                                                (*tp).parent =
                                                                    ::core::ptr::null_mut::<
                                                                        malloc_tree_chunk,
                                                                    >(
                                                                    );
                                                                break;
                                                            } else {
                                                                abort();
                                                            }
                                                        }
                                                    }
                                                }
                                                _gm_.release_checks =
                                                    _gm_.release_checks.wrapping_sub(1);
                                                if _gm_.release_checks == 0 as size_t {
                                                    release_unused_segments(&raw mut _gm_);
                                                }
                                            }
                                            current_block = 14836311167396004099;
                                        }
                                    }
                                } else {
                                    current_block = 1766691779471037599;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            1766691779471037599 => {}
                            _ => {
                                if _gm_.mflags as ::core::ffi::c_uint & USE_LOCK_BIT != 0 {
                                    crate::atomic_compat::atomic_store_release(
                                        &raw mut _gm_.mutex,
                                        0,
                                    );
                                }
                                current_block = 17419946814775977432;
                            }
                        }
                    }
                }
            } else {
                current_block = 1766691779471037599;
            }
            match current_block {
                17419946814775977432 => {}
                _ => {
                    abort();
                }
            }
        }
    }
}
unsafe extern "C" fn try_realloc_chunk(
    mut m: mstate,
    mut p: mchunkptr,
    mut nb: size_t,
    mut can_move: ::core::ffi::c_int,
) -> mchunkptr {
    let mut newp: mchunkptr = ::core::ptr::null_mut::<malloc_chunk>();
    let mut oldsize: size_t = (*p).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT);
    let mut next: mchunkptr = (p as *mut ::core::ffi::c_char).offset(oldsize as isize) as mchunkptr;
    if (p as *mut ::core::ffi::c_char >= (*m).least_addr
        && (*p).head & (1 as ::core::ffi::c_int as size_t | 2 as ::core::ffi::c_int as size_t)
            != 1 as ::core::ffi::c_int as size_t
        && (p as *mut ::core::ffi::c_char) < next as *mut ::core::ffi::c_char
        && (*next).head & 1 as ::core::ffi::c_int as size_t != 0) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        if (*p).head & INUSE_BITS == 0 as size_t {
            newp = mmap_resize(m, p, nb, can_move);
        } else if oldsize >= nb {
            let mut rsize: size_t = oldsize.wrapping_sub(nb);
            if rsize >= MIN_CHUNK_SIZE as usize {
                let mut r: mchunkptr =
                    (p as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr;
                (*p).head = (*p).head & PINUSE_BIT | nb | CINUSE_BIT;
                (*((p as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr)).head |=
                    PINUSE_BIT;
                (*r).head = (*r).head & PINUSE_BIT | rsize | CINUSE_BIT;
                (*((r as *mut ::core::ffi::c_char).offset(rsize as isize) as mchunkptr)).head |=
                    PINUSE_BIT;
                dispose_chunk(m, r, rsize);
            }
            newp = p;
        } else if next == (*m).top {
            if oldsize.wrapping_add((*m).topsize) > nb {
                let mut newsize: size_t = oldsize.wrapping_add((*m).topsize);
                let mut newtopsize: size_t = newsize.wrapping_sub(nb);
                let mut newtop: mchunkptr =
                    (p as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr;
                (*p).head = (*p).head & PINUSE_BIT | nb | CINUSE_BIT;
                (*((p as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr)).head |=
                    PINUSE_BIT;
                (*newtop).head = newtopsize | PINUSE_BIT;
                (*m).top = newtop;
                (*m).topsize = newtopsize;
                newp = p;
            }
        } else if next == (*m).dv {
            let mut dvs: size_t = (*m).dvsize;
            if oldsize.wrapping_add(dvs) >= nb {
                let mut dsize: size_t = oldsize.wrapping_add(dvs).wrapping_sub(nb);
                if dsize >= MIN_CHUNK_SIZE as usize {
                    let mut r_0: mchunkptr =
                        (p as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr;
                    let mut n: mchunkptr =
                        (r_0 as *mut ::core::ffi::c_char).offset(dsize as isize) as mchunkptr;
                    (*p).head = (*p).head & PINUSE_BIT | nb | CINUSE_BIT;
                    (*((p as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr)).head |=
                        PINUSE_BIT;
                    (*r_0).head = dsize | PINUSE_BIT;
                    (*((r_0 as *mut ::core::ffi::c_char).offset(dsize as isize) as mchunkptr))
                        .prev_foot = dsize;
                    (*n).head &= !PINUSE_BIT;
                    (*m).dvsize = dsize;
                    (*m).dv = r_0;
                } else {
                    let mut newsize_0: size_t = oldsize.wrapping_add(dvs);
                    (*p).head = (*p).head & PINUSE_BIT | newsize_0 | CINUSE_BIT;
                    (*((p as *mut ::core::ffi::c_char).offset(newsize_0 as isize) as mchunkptr))
                        .head |= PINUSE_BIT;
                    (*m).dvsize = 0 as size_t;
                    (*m).dv = ::core::ptr::null_mut::<malloc_chunk>();
                }
                newp = p;
            }
        } else if (*next).head & CINUSE_BIT == 0 {
            let mut nextsize: size_t = (*next).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT);
            if oldsize.wrapping_add(nextsize) >= nb {
                let mut rsize_0: size_t = oldsize.wrapping_add(nextsize).wrapping_sub(nb);
                if nextsize >> SMALLBIN_SHIFT < NSMALLBINS as size_t {
                    let mut F: mchunkptr = (*next).fd as mchunkptr;
                    let mut B: mchunkptr = (*next).bk as mchunkptr;
                    let mut I: bindex_t = (nextsize >> SMALLBIN_SHIFT) as bindex_t;
                    if (F
                        == (&raw mut (*m).smallbins as *mut mchunkptr)
                            .offset((I << 1 as ::core::ffi::c_int) as isize)
                            as *mut mchunkptr as *mut ::core::ffi::c_char
                            as sbinptr
                        || F as *mut ::core::ffi::c_char >= (*m).least_addr && (*F).bk == next)
                        as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        if B == F {
                            (*m).smallmap &= !((1 as ::core::ffi::c_int as binmap_t) << I);
                        } else if (B
                            == (&raw mut (*m).smallbins as *mut mchunkptr)
                                .offset((I << 1 as ::core::ffi::c_int) as isize)
                                as *mut mchunkptr
                                as *mut ::core::ffi::c_char
                                as sbinptr
                            || B as *mut ::core::ffi::c_char >= (*m).least_addr && (*B).fd == next)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            (*F).bk = B as *mut malloc_chunk;
                            (*B).fd = F as *mut malloc_chunk;
                        } else {
                            abort();
                        }
                    } else {
                        abort();
                    }
                } else {
                    let mut TP: tchunkptr = next as tchunkptr;
                    let mut XP: tchunkptr = (*TP).parent as tchunkptr;
                    let mut R: tchunkptr = ::core::ptr::null_mut::<malloc_tree_chunk>();
                    if (*TP).bk != TP {
                        let mut F_0: tchunkptr = (*TP).fd as tchunkptr;
                        R = (*TP).bk as tchunkptr;
                        if (F_0 as *mut ::core::ffi::c_char >= (*m).least_addr
                            && (*F_0).bk == TP
                            && (*R).fd == TP) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            (*F_0).bk = R as *mut malloc_tree_chunk;
                            (*R).fd = F_0 as *mut malloc_tree_chunk;
                        } else {
                            abort();
                        }
                    } else {
                        let mut RP: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                        RP = (&raw mut (*TP).child as *mut *mut malloc_tree_chunk)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut *mut malloc_tree_chunk
                            as *mut tchunkptr;
                        R = *RP;
                        if !R.is_null() || {
                            RP = (&raw mut (*TP).child as *mut *mut malloc_tree_chunk)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut *mut malloc_tree_chunk
                                as *mut tchunkptr;
                            R = *RP;
                            !R.is_null()
                        } {
                            let mut CP: *mut tchunkptr = ::core::ptr::null_mut::<tchunkptr>();
                            loop {
                                CP = (&raw mut (*R).child as *mut *mut malloc_tree_chunk)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as *mut *mut malloc_tree_chunk
                                    as *mut tchunkptr;
                                if !(!(*CP).is_null() || {
                                    CP = (&raw mut (*R).child as *mut *mut malloc_tree_chunk)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut *mut malloc_tree_chunk
                                        as *mut tchunkptr;
                                    !(*CP).is_null()
                                }) {
                                    break;
                                }
                                RP = CP;
                                R = *RP;
                            }
                            if (RP as *mut ::core::ffi::c_char >= (*m).least_addr)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                *RP = ::core::ptr::null_mut::<malloc_tree_chunk>();
                            } else {
                                abort();
                            }
                        }
                    }
                    if !XP.is_null() {
                        let mut H: *mut tbinptr = (&raw mut (*m).treebins as *mut tbinptr)
                            .offset((*TP).index as isize)
                            as *mut tbinptr;
                        if TP == *H {
                            *H = R as tbinptr;
                            if (*H).is_null() {
                                (*m).treemap &=
                                    !((1 as ::core::ffi::c_int as binmap_t) << (*TP).index);
                            }
                        } else if (XP as *mut ::core::ffi::c_char >= (*m).least_addr)
                            as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            if (*XP).child[0 as ::core::ffi::c_int as usize] == TP {
                                (*XP).child[0 as ::core::ffi::c_int as usize] =
                                    R as *mut malloc_tree_chunk;
                            } else {
                                (*XP).child[1 as ::core::ffi::c_int as usize] =
                                    R as *mut malloc_tree_chunk;
                            }
                        } else {
                            abort();
                        }
                        if !R.is_null() {
                            if (R as *mut ::core::ffi::c_char >= (*m).least_addr)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                let mut C0: tchunkptr =
                                    ::core::ptr::null_mut::<malloc_tree_chunk>();
                                let mut C1: tchunkptr =
                                    ::core::ptr::null_mut::<malloc_tree_chunk>();
                                (*R).parent = XP as *mut malloc_tree_chunk;
                                C0 = (*TP).child[0 as ::core::ffi::c_int as usize] as tchunkptr;
                                if !C0.is_null() {
                                    if (C0 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        (*R).child[0 as ::core::ffi::c_int as usize] =
                                            C0 as *mut malloc_tree_chunk;
                                        (*C0).parent = R as *mut malloc_tree_chunk;
                                    } else {
                                        abort();
                                    }
                                }
                                C1 = (*TP).child[1 as ::core::ffi::c_int as usize] as tchunkptr;
                                if !C1.is_null() {
                                    if (C1 as *mut ::core::ffi::c_char >= (*m).least_addr)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_long
                                        != 0
                                    {
                                        (*R).child[1 as ::core::ffi::c_int as usize] =
                                            C1 as *mut malloc_tree_chunk;
                                        (*C1).parent = R as *mut malloc_tree_chunk;
                                    } else {
                                        abort();
                                    }
                                }
                            } else {
                                abort();
                            }
                        }
                    }
                }
                if rsize_0 < MIN_CHUNK_SIZE as usize {
                    let mut newsize_1: size_t = oldsize.wrapping_add(nextsize);
                    (*p).head = (*p).head & PINUSE_BIT | newsize_1 | CINUSE_BIT;
                    (*((p as *mut ::core::ffi::c_char).offset(newsize_1 as isize) as mchunkptr))
                        .head |= PINUSE_BIT;
                } else {
                    let mut r_1: mchunkptr =
                        (p as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr;
                    (*p).head = (*p).head & PINUSE_BIT | nb | CINUSE_BIT;
                    (*((p as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr)).head |=
                        PINUSE_BIT;
                    (*r_1).head = (*r_1).head & PINUSE_BIT | rsize_0 | CINUSE_BIT;
                    (*((r_1 as *mut ::core::ffi::c_char).offset(rsize_0 as isize) as mchunkptr))
                        .head |= PINUSE_BIT;
                    dispose_chunk(m, r_1, rsize_0);
                }
                newp = p;
            }
        }
    } else {
        abort();
    }
    return newp;
}
unsafe extern "C" fn internal_memalign(
    mut m: mstate,
    mut alignment: size_t,
    mut bytes: size_t,
) -> *mut ::core::ffi::c_void {
    let mut mem: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if alignment < MIN_CHUNK_SIZE as usize {
        alignment = MIN_CHUNK_SIZE as usize as size_t;
    }
    if alignment & alignment.wrapping_sub(SIZE_T_ONE) != 0 as size_t {
        let mut a: size_t = MALLOC_ALIGNMENT << 1 as ::core::ffi::c_int;
        while a < alignment {
            a <<= 1 as ::core::ffi::c_int;
        }
        alignment = a;
    }
    if bytes >= (MAX_REQUEST as usize).wrapping_sub(alignment as usize) {
        if !m.is_null() {
            *__errno_location() = ENOMEM;
        }
    } else {
        let mut nb: size_t = if bytes < MIN_REQUEST as usize {
            MIN_CHUNK_SIZE as size_t
        } else {
            bytes
                .wrapping_add(CHUNK_OVERHEAD as size_t)
                .wrapping_add(CHUNK_ALIGN_MASK)
                & !CHUNK_ALIGN_MASK
        };
        let mut req: size_t = nb
            .wrapping_add(alignment)
            .wrapping_add(MIN_CHUNK_SIZE as size_t)
            .wrapping_sub(CHUNK_OVERHEAD as size_t);
        mem = dlmalloc(req);
        if !mem.is_null() {
            let mut p: mchunkptr = (mem as *mut ::core::ffi::c_char)
                .offset(-(TWO_SIZE_T_SIZES as usize as isize))
                as mchunkptr;
            if if (*m).mflags as ::core::ffi::c_uint & USE_LOCK_BIT != 0 {
                if crate::atomic_compat::atomic_xchg_acquire(
                    &raw mut (*m).mutex,
                    1 as ::core::ffi::c_int,
                ) != 0
                {
                    spin_acquire_lock(&raw mut (*m).mutex)
                } else {
                    0 as ::core::ffi::c_int
                }
            } else {
                0 as ::core::ffi::c_int
            } != 0
            {
                return ::core::ptr::null_mut::<::core::ffi::c_void>();
            }
            if mem as size_t & alignment.wrapping_sub(1 as size_t) != 0 as size_t {
                let mut br: *mut ::core::ffi::c_char = (((mem as *mut ::core::ffi::c_char)
                    .offset(alignment as isize)
                    .offset(-(1 as ::core::ffi::c_int as size_t as isize))
                    as size_t
                    & alignment.wrapping_neg())
                    as *mut ::core::ffi::c_char)
                    .offset(-(TWO_SIZE_T_SIZES as usize as isize))
                    as mchunkptr
                    as *mut ::core::ffi::c_char;
                let mut pos: *mut ::core::ffi::c_char =
                    if br.offset_from(p as *mut ::core::ffi::c_char) as ::core::ffi::c_long
                        as size_t
                        >= MIN_CHUNK_SIZE as usize
                    {
                        br
                    } else {
                        br.offset(alignment as isize)
                    };
                let mut newp: mchunkptr = pos as mchunkptr;
                let mut leadsize: size_t =
                    pos.offset_from(p as *mut ::core::ffi::c_char) as ::core::ffi::c_long as size_t;
                let mut newsize: size_t =
                    ((*p).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT)).wrapping_sub(leadsize);
                if (*p).head & INUSE_BITS == 0 as size_t {
                    (*newp).prev_foot = (*p).prev_foot.wrapping_add(leadsize);
                    (*newp).head = newsize;
                } else {
                    (*newp).head = (*newp).head & PINUSE_BIT | newsize | CINUSE_BIT;
                    (*((newp as *mut ::core::ffi::c_char).offset(newsize as isize)
                        as mchunkptr))
                        .head |= PINUSE_BIT;
                    (*p).head = (*p).head & PINUSE_BIT | leadsize | CINUSE_BIT;
                    (*((p as *mut ::core::ffi::c_char).offset(leadsize as isize) as mchunkptr))
                        .head |= PINUSE_BIT;
                    dispose_chunk(m, p, leadsize);
                }
                p = newp;
            }
            if !((*p).head & INUSE_BITS == 0 as size_t) {
                let mut size: size_t = (*p).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT);
                if size > nb.wrapping_add(MIN_CHUNK_SIZE as size_t) {
                    let mut remainder_size: size_t = size.wrapping_sub(nb);
                    let mut remainder: mchunkptr =
                        (p as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr;
                    (*p).head = (*p).head & PINUSE_BIT | nb | CINUSE_BIT;
                    (*((p as *mut ::core::ffi::c_char).offset(nb as isize) as mchunkptr)).head |=
                        PINUSE_BIT;
                    (*remainder).head =
                        (*remainder).head & PINUSE_BIT | remainder_size | CINUSE_BIT;
                    (*((remainder as *mut ::core::ffi::c_char).offset(remainder_size as isize)
                        as mchunkptr))
                        .head |= PINUSE_BIT;
                    dispose_chunk(m, remainder, remainder_size);
                }
            }
            mem = (p as *mut ::core::ffi::c_char).offset(TWO_SIZE_T_SIZES as usize as isize)
                as *mut ::core::ffi::c_void;
            if (*m).mflags as ::core::ffi::c_uint & USE_LOCK_BIT != 0 {
                crate::atomic_compat::atomic_store_release(&raw mut (*m).mutex, 0);
            }
        }
    }
    return mem;
}
unsafe extern "C" fn internal_bulk_free(
    mut m: mstate,
    mut array: *mut *mut ::core::ffi::c_void,
    mut nelem: size_t,
) -> size_t {
    let mut unfreed: size_t = 0 as size_t;
    if if (*m).mflags as ::core::ffi::c_uint & USE_LOCK_BIT != 0 {
        if crate::atomic_compat::atomic_xchg_acquire(&raw mut (*m).mutex, 1 as ::core::ffi::c_int)
            != 0
        {
            spin_acquire_lock(&raw mut (*m).mutex)
        } else {
            0 as ::core::ffi::c_int
        }
    } else {
        0 as ::core::ffi::c_int
    } == 0
    {
        let mut a: *mut *mut ::core::ffi::c_void =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
        let mut fence: *mut *mut ::core::ffi::c_void =
            array.offset(nelem as isize) as *mut *mut ::core::ffi::c_void;
        a = array as *mut *mut ::core::ffi::c_void;
        while a != fence {
            let mut mem: *mut ::core::ffi::c_void = *a;
            if !mem.is_null() {
                let mut p: mchunkptr = (mem as *mut ::core::ffi::c_char)
                    .offset(-(TWO_SIZE_T_SIZES as usize as isize))
                    as mchunkptr;
                let mut psize: size_t = (*p).head & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT);
                *a = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if (p as *mut ::core::ffi::c_char >= (*m).least_addr
                    && (*p).head
                        & (1 as ::core::ffi::c_int as size_t | 2 as ::core::ffi::c_int as size_t)
                        != 1 as ::core::ffi::c_int as size_t)
                    as ::core::ffi::c_int as ::core::ffi::c_long
                    != 0
                {
                    let mut b: *mut *mut ::core::ffi::c_void =
                        a.offset(1 as ::core::ffi::c_int as isize);
                    let mut next: mchunkptr = (p as *mut ::core::ffi::c_char)
                        .offset(((*p).head & !FLAG_BITS) as isize)
                        as mchunkptr;
                    if b != fence
                        && *b
                            == (next as *mut ::core::ffi::c_char)
                                .offset(TWO_SIZE_T_SIZES as usize as isize)
                                as *mut ::core::ffi::c_void
                    {
                        let mut newsize: size_t = ((*next).head
                            & !(PINUSE_BIT | CINUSE_BIT | FLAG4_BIT))
                            .wrapping_add(psize);
                        (*p).head = (*p).head & PINUSE_BIT | newsize | CINUSE_BIT;
                        (*((p as *mut ::core::ffi::c_char).offset(newsize as isize)
                            as mchunkptr))
                            .head |= PINUSE_BIT;
                        *b = (p as *mut ::core::ffi::c_char)
                            .offset(TWO_SIZE_T_SIZES as usize as isize)
                            as *mut ::core::ffi::c_void;
                    } else {
                        dispose_chunk(m, p, psize);
                    }
                } else {
                    abort();
                }
            }
            a = a.offset(1);
        }
        if (*m).topsize > (*m).trim_check {
            sys_trim(m, 0 as size_t);
        }
        if (*m).mflags as ::core::ffi::c_uint & USE_LOCK_BIT != 0 {
            crate::atomic_compat::atomic_store_release(&raw mut (*m).mutex, 0);
        }
    }
    return unfreed;
}
#[no_mangle]
pub unsafe extern "C" fn dlrealloc_in_place(
    mut oldmem: *mut ::core::ffi::c_void,
    mut bytes: size_t,
) -> *mut ::core::ffi::c_void {
    let mut mem: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if !oldmem.is_null() {
        if bytes >= MAX_REQUEST as usize {
            *__errno_location() = ENOMEM;
        } else {
            let mut nb: size_t = if bytes < MIN_REQUEST as usize {
                MIN_CHUNK_SIZE as size_t
            } else {
                bytes
                    .wrapping_add(CHUNK_OVERHEAD as size_t)
                    .wrapping_add(CHUNK_ALIGN_MASK)
                    & !CHUNK_ALIGN_MASK
            };
            let mut oldp: mchunkptr = (oldmem as *mut ::core::ffi::c_char)
                .offset(-(TWO_SIZE_T_SIZES as usize as isize))
                as mchunkptr;
            let mut m: mstate = &raw mut _gm_;
            if if (*m).mflags as ::core::ffi::c_uint & USE_LOCK_BIT != 0 {
                if crate::atomic_compat::atomic_xchg_acquire(
                    &raw mut (*m).mutex,
                    1 as ::core::ffi::c_int,
                ) != 0
                {
                    spin_acquire_lock(&raw mut (*m).mutex)
                } else {
                    0 as ::core::ffi::c_int
                }
            } else {
                0 as ::core::ffi::c_int
            } == 0
            {
                let mut newp: mchunkptr = try_realloc_chunk(m, oldp, nb, 0 as ::core::ffi::c_int);
                if (*m).mflags as ::core::ffi::c_uint & USE_LOCK_BIT != 0 {
                    crate::atomic_compat::atomic_store_release(&raw mut (*m).mutex, 0);
                }
                if newp == oldp {
                    mem = oldmem;
                }
            }
        }
    }
    return mem;
}
#[no_mangle]
pub unsafe extern "C" fn dlposix_memalign(
    mut pp: *mut *mut ::core::ffi::c_void,
    mut alignment: size_t,
    mut bytes: size_t,
) -> ::core::ffi::c_int {
    let mut mem: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if alignment == MALLOC_ALIGNMENT {
        mem = dlmalloc(bytes);
    } else {
        let mut d: size_t =
            alignment.wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t);
        let mut r: size_t =
            alignment.wrapping_rem(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t);
        if r != 0 as size_t || d == 0 as size_t || d & d.wrapping_sub(SIZE_T_ONE) != 0 as size_t {
            return EINVAL;
        } else if bytes <= (MAX_REQUEST as usize).wrapping_sub(alignment as usize) {
            if alignment < MIN_CHUNK_SIZE as usize {
                alignment = MIN_CHUNK_SIZE as usize as size_t;
            }
            mem = internal_memalign(&raw mut _gm_, alignment, bytes);
        }
    }
    if mem.is_null() {
        return ENOMEM;
    } else {
        *pp = mem;
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn dlbulk_free(
    mut array: *mut *mut ::core::ffi::c_void,
    mut nelem: size_t,
) -> size_t {
    return internal_bulk_free(&raw mut _gm_, array, nelem);
}
#[no_mangle]
pub unsafe extern "C" fn dlmalloc_footprint_limit() -> size_t {
    let mut maf: size_t = _gm_.footprint_limit;
    return if maf == 0 as size_t { MAX_SIZE_T } else { maf };
}
#[no_mangle]
pub unsafe extern "C" fn dlmalloc_set_footprint_limit(mut bytes: size_t) -> size_t {
    let mut result: size_t = 0;
    if bytes == 0 as size_t {
        result = (1 as size_t).wrapping_add(mparams.granularity.wrapping_sub(SIZE_T_ONE))
            & !mparams.granularity.wrapping_sub(SIZE_T_ONE);
    }
    if bytes == MAX_SIZE_T {
        result = 0 as size_t;
    } else {
        result = bytes.wrapping_add(mparams.granularity.wrapping_sub(SIZE_T_ONE))
            & !mparams.granularity.wrapping_sub(SIZE_T_ONE);
    }
    _gm_.footprint_limit = result;
    return _gm_.footprint_limit;
}
static mut open_temp_exec_file_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_int,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
            __next: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
        },
    },
};
static mut execfd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut execsize: size_t = 0 as size_t;
unsafe extern "C" fn open_temp_exec_file_memfd(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    fd = memfd_create(name, MFD_CLOEXEC);
    return fd;
}
unsafe extern "C" fn open_temp_exec_file_name(
    mut name: *mut ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    fd = mkstemp(name);
    if fd != -(1 as ::core::ffi::c_int) {
        unlink(name);
    }
    return fd;
}
unsafe extern "C" fn open_temp_exec_file_dir(
    mut dir: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    static mut suffix: [::core::ffi::c_char; 11] =
        unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"/ffiXXXXXX\0") };
    let mut lendir: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut tempname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fd: ::core::ffi::c_int = 0;
    flags = O_CLOEXEC;
    fd = open(
        dir,
        flags | O_RDWR | O_EXCL | O_TMPFILE,
        0o700 as ::core::ffi::c_int,
    );
    if fd != -(1 as ::core::ffi::c_int)
        || *__errno_location() != EINVAL
            && *__errno_location() != EISDIR
            && *__errno_location() != EOPNOTSUPP
    {
        return fd;
    } else {
        *__errno_location() = 0 as ::core::ffi::c_int;
    }
    lendir = strlen(dir) as ::core::ffi::c_int;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (lendir as usize).wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 11]>() as usize)
            as usize,
    ));
    tempname = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    if tempname.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    memcpy(
        tempname as *mut ::core::ffi::c_void,
        dir as *const ::core::ffi::c_void,
        lendir as size_t,
    );
    memcpy(
        tempname.offset(lendir as isize) as *mut ::core::ffi::c_void,
        &raw const suffix as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 11]>() as size_t,
    );
    return open_temp_exec_file_name(tempname, flags);
}
unsafe extern "C" fn open_temp_exec_file_env(
    mut envvar: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut value: *const ::core::ffi::c_char = getenv(envvar);
    if value.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return open_temp_exec_file_dir(value);
}
unsafe extern "C" fn open_temp_exec_file_mnt(
    mut mounts: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut last_mounts: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    static mut last_mntent: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
    if mounts != last_mounts {
        if !last_mntent.is_null() {
            endmntent(last_mntent);
        }
        last_mounts = mounts;
        if !mounts.is_null() {
            last_mntent = setmntent(mounts, b"r\0" as *const u8 as *const ::core::ffi::c_char);
        } else {
            last_mntent = ::core::ptr::null_mut::<FILE>();
        }
    }
    if last_mntent.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    loop {
        let mut fd: ::core::ffi::c_int = 0;
        let mut mnt: mntent = mntent {
            mnt_fsname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            mnt_dir: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            mnt_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            mnt_opts: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            mnt_freq: 0,
            mnt_passno: 0,
        };
        let mut buf: [::core::ffi::c_char; 12288] = [0; 12288];
        if getmntent_r(
            last_mntent,
            &raw mut mnt,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 12288]>() as ::core::ffi::c_int,
        )
        .is_null()
        {
            return -(1 as ::core::ffi::c_int);
        }
        if !hasmntopt(
            &raw mut mnt,
            b"ro\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
            || !hasmntopt(
                &raw mut mnt,
                b"noexec\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
            || access(mnt.mnt_dir, W_OK) != 0
        {
            continue;
        }
        fd = open_temp_exec_file_dir(mnt.mnt_dir);
        if fd != -(1 as ::core::ffi::c_int) {
            return fd;
        }
    }
}
static mut open_temp_exec_file_opts: [C2RustUnnamed_0; 9] = unsafe {
    [
        C2RustUnnamed_0 {
            func: Some(
                open_temp_exec_file_memfd
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            ),
            arg: b"libffi\0" as *const u8 as *const ::core::ffi::c_char,
            repeat: 0 as ::core::ffi::c_int,
        },
        C2RustUnnamed_0 {
            func: Some(
                open_temp_exec_file_env
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            ),
            arg: b"LIBFFI_TMPDIR\0" as *const u8 as *const ::core::ffi::c_char,
            repeat: 0 as ::core::ffi::c_int,
        },
        C2RustUnnamed_0 {
            func: Some(
                open_temp_exec_file_env
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            ),
            arg: b"TMPDIR\0" as *const u8 as *const ::core::ffi::c_char,
            repeat: 0 as ::core::ffi::c_int,
        },
        C2RustUnnamed_0 {
            func: Some(
                open_temp_exec_file_dir
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            ),
            arg: b"/tmp\0" as *const u8 as *const ::core::ffi::c_char,
            repeat: 0 as ::core::ffi::c_int,
        },
        C2RustUnnamed_0 {
            func: Some(
                open_temp_exec_file_dir
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            ),
            arg: b"/var/tmp\0" as *const u8 as *const ::core::ffi::c_char,
            repeat: 0 as ::core::ffi::c_int,
        },
        C2RustUnnamed_0 {
            func: Some(
                open_temp_exec_file_dir
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            ),
            arg: b"/dev/shm\0" as *const u8 as *const ::core::ffi::c_char,
            repeat: 0 as ::core::ffi::c_int,
        },
        C2RustUnnamed_0 {
            func: Some(
                open_temp_exec_file_env
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            ),
            arg: b"HOME\0" as *const u8 as *const ::core::ffi::c_char,
            repeat: 0 as ::core::ffi::c_int,
        },
        C2RustUnnamed_0 {
            func: Some(
                open_temp_exec_file_mnt
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            ),
            arg: b"/etc/mtab\0" as *const u8 as *const ::core::ffi::c_char,
            repeat: 1 as ::core::ffi::c_int,
        },
        C2RustUnnamed_0 {
            func: Some(
                open_temp_exec_file_mnt
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            ),
            arg: b"/proc/mounts\0" as *const u8 as *const ::core::ffi::c_char,
            repeat: 1 as ::core::ffi::c_int,
        },
    ]
};
static mut open_temp_exec_file_opts_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn open_temp_exec_file_opts_next() -> ::core::ffi::c_int {
    if open_temp_exec_file_opts[open_temp_exec_file_opts_idx as usize].repeat != 0 {
        open_temp_exec_file_opts[open_temp_exec_file_opts_idx as usize]
            .func
            .expect("non-null function pointer")(::core::ptr::null::<::core::ffi::c_char>());
    }
    open_temp_exec_file_opts_idx += 1;
    if open_temp_exec_file_opts_idx as usize
        == (::core::mem::size_of::<[C2RustUnnamed_0; 9]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as usize)
    {
        open_temp_exec_file_opts_idx = 0 as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn open_temp_exec_file() -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    loop {
        fd = open_temp_exec_file_opts[open_temp_exec_file_opts_idx as usize]
            .func
            .expect("non-null function pointer")(
            open_temp_exec_file_opts[open_temp_exec_file_opts_idx as usize].arg,
        );
        if open_temp_exec_file_opts[open_temp_exec_file_opts_idx as usize].repeat == 0
            || fd == -(1 as ::core::ffi::c_int)
        {
            if open_temp_exec_file_opts_next() != 0 {
                break;
            }
        }
        if !(fd == -(1 as ::core::ffi::c_int)) {
            break;
        }
    }
    return fd;
}
unsafe extern "C" fn allocate_space(
    mut fd: ::core::ffi::c_int,
    mut len: off_t,
) -> ::core::ffi::c_int {
    static mut page_size: ::core::ffi::c_long = 0;
    if page_size == 0 {
        page_size = sysconf(_SC_PAGESIZE as ::core::ffi::c_int);
    }
    let vla = page_size as usize;
    let mut buf: Vec<::core::ffi::c_uchar> = ::std::vec::from_elem(0, vla);
    memset(
        buf.as_mut_ptr() as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        page_size as size_t,
    );
    while len > 0 as off_t {
        let mut to_write: off_t = if len < page_size {
            len
        } else {
            page_size as off_t
        };
        if write(
            fd,
            buf.as_mut_ptr() as *const ::core::ffi::c_void,
            to_write as size_t,
        ) < to_write as ssize_t
        {
            return -(1 as ::core::ffi::c_int);
        }
        len -= to_write;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn dlmmap_locked(
    mut start: *mut ::core::ffi::c_void,
    mut length: size_t,
    mut prot: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut offset: off_t,
) -> *mut ::core::ffi::c_void {
    let mut current_block: u64;
    let mut ptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if execfd == -(1 as ::core::ffi::c_int) {
        open_temp_exec_file_opts_idx = 0 as ::core::ffi::c_int;
        current_block = 10470113670290408892;
    } else {
        current_block = 735147466149431745;
    }
    loop {
        match current_block {
            10470113670290408892 => {
                execfd = open_temp_exec_file();
                if execfd == -(1 as ::core::ffi::c_int) {
                    return MFAIL;
                }
                current_block = 735147466149431745;
            }
            _ => {
                offset = execsize as off_t;
                if allocate_space(execfd, length as off_t) != 0 {
                    return MFAIL;
                }
                flags &= !(MAP_PRIVATE | MAP_ANONYMOUS);
                flags |= MAP_SHARED;
                ptr = mmap(
                    NULL,
                    length,
                    prot & !PROT_WRITE | PROT_EXEC,
                    flags,
                    execfd,
                    offset as __off_t,
                );
                if ptr == MFAIL {
                    if offset == 0 {
                        close(execfd);
                        current_block = 10470113670290408892;
                    } else {
                        ftruncate(execfd, offset as __off_t) != 0 as ::core::ffi::c_int;
                        return MFAIL;
                    }
                } else {
                    if offset == 0
                        && open_temp_exec_file_opts[open_temp_exec_file_opts_idx as usize].repeat
                            != 0
                    {
                        open_temp_exec_file_opts_next();
                    }
                    start = mmap(start, length, prot, flags, execfd, offset as __off_t);
                    if start == MFAIL {
                        munmap(ptr, length);
                        ftruncate(execfd, offset as __off_t) != 0 as ::core::ffi::c_int;
                        return start;
                    }
                    *((start as *mut ::core::ffi::c_char)
                        .offset(length as isize)
                        .offset(-(::core::mem::size_of::<ptrdiff_t>() as usize as isize))
                        as *mut ptrdiff_t) = (ptr as *mut ::core::ffi::c_char)
                        .offset_from(start as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        as ptrdiff_t;
                    execsize = execsize.wrapping_add(length);
                    return start;
                }
            }
        }
    }
}
unsafe extern "C" fn dlmmap(
    mut start: *mut ::core::ffi::c_void,
    mut length: size_t,
    mut prot: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut fd: ::core::ffi::c_int,
    mut offset: off_t,
) -> *mut ::core::ffi::c_void {
    let mut ptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if execfd == -(1 as ::core::ffi::c_int) && ffi_tramp_is_supported() != 0 {
        ptr = mmap(
            start,
            length,
            prot & !PROT_EXEC,
            flags,
            fd,
            offset as __off_t,
        );
        return ptr;
    }
    if !(execfd == -(1 as ::core::ffi::c_int)
        && PAX_MPROTECT as ::core::ffi::c_int
            == PAX_MPROTECT as ::core::ffi::c_int
                & (if cached_pax_flags >= 0 as ::core::ffi::c_int {
                    cached_pax_flags
                } else {
                    cached_pax_flags = pax_flags_check();
                    cached_pax_flags
                }))
    {
        if execfd == -(1 as ::core::ffi::c_int)
            && (if selinux_enabled >= 0 as ::core::ffi::c_int {
                selinux_enabled
            } else {
                selinux_enabled = selinux_enabled_check();
                selinux_enabled
            }) == 0
        {
            ptr = mmap(
                start,
                length,
                prot | PROT_EXEC,
                flags,
                fd,
                offset as __off_t,
            );
            if ptr != MFAIL || *__errno_location() != EPERM && *__errno_location() != EACCES {
                return ptr;
            }
        }
    }
    pthread_mutex_lock(&raw mut open_temp_exec_file_mutex);
    ptr = dlmmap_locked(start, length, prot, flags, offset);
    pthread_mutex_unlock(&raw mut open_temp_exec_file_mutex);
    return ptr;
}
unsafe extern "C" fn dlmunmap(
    mut start: *mut ::core::ffi::c_void,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut seg: msegmentptr = segment_holding(&raw mut _gm_, start as *mut ::core::ffi::c_char);
    let mut code: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if !seg.is_null() && {
        code = (start as *mut ::core::ffi::c_char).offset((*seg).exec_offset as isize)
            as *mut ::core::ffi::c_void;
        code != start
    } {
        let mut ret: ::core::ffi::c_int = munmap(code, length);
        if ret != 0 {
            return ret;
        }
    }
    return munmap(start, length);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_closure_alloc(
    mut size: size_t,
    mut code: *mut *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut ptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ftramp: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if code.is_null() {
        return NULL;
    }
    ptr = dlmalloc(size);
    if !ptr.is_null() {
        let mut seg: msegmentptr = segment_holding(&raw mut _gm_, ptr as *mut ::core::ffi::c_char);
        *code = ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            *mut ::core::ffi::c_void,
        >(::core::mem::transmute::<
            *mut ::core::ffi::c_char,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            (ptr as *mut ::core::ffi::c_char).offset((*seg).exec_offset as isize),
        ));
        if ffi_tramp_is_supported() == 0 {
            return ptr;
        }
        ftramp = ffi_tramp_alloc(0 as ::core::ffi::c_int);
        if ftramp.is_null() {
            dlfree(ptr);
            return NULL;
        }
        *code = ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            *mut ::core::ffi::c_void,
        >(::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn() -> ()>,
        >(ffi_tramp_get_addr(ftramp)));
        let ref mut fresh0 = (*(ptr as *mut ffi_closure)).c2rust_unnamed.ftramp;
        *fresh0 = ftramp;
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_data_to_code_pointer(
    mut data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut seg: msegmentptr = segment_holding(&raw mut _gm_, data as *mut ::core::ffi::c_char);
    if !seg.is_null() {
        if ffi_tramp_is_supported() == 0 {
            return (data as *mut ::core::ffi::c_char).offset((*seg).exec_offset as isize)
                as *mut ::core::ffi::c_void;
        }
        return ffi_tramp_get_addr((*(data as *mut ffi_closure)).c2rust_unnamed.ftramp);
    } else {
        return data;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ffi_closure_free(mut ptr: *mut ::core::ffi::c_void) {
    if ffi_tramp_is_supported() != 0 {
        ffi_tramp_free((*(ptr as *mut ffi_closure)).c2rust_unnamed.ftramp);
    }
    dlfree(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_is_present(
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut seg: msegmentptr = segment_holding(&raw mut _gm_, ptr as *mut ::core::ffi::c_char);
    return (!seg.is_null() && ffi_tramp_is_supported() != 0) as ::core::ffi::c_int;
}
pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __ATOMIC_ACQUIRE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __ATOMIC_RELEASE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const __CHAR_BIT__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

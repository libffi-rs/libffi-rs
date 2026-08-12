extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn open_temp_exec_file() -> ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn getpid() -> __pid_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off_t,
    ) -> *mut ::core::ffi::c_void;
    fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
    fn ffi_tramp_arch(tramp_size: *mut size_t, map_size: *mut size_t) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __ssize_t = ::core::ffi::c_long;
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
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __elision: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type uintptr_t = usize;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
pub type tramp_globals_status = ::core::ffi::c_uint;
pub const TRAMP_GLOBALS_FAILED: tramp_globals_status = 2;
pub const TRAMP_GLOBALS_PASSED: tramp_globals_status = 1;
pub const TRAMP_GLOBALS_UNINITIALIZED: tramp_globals_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tramp_globals {
    pub fd: ::core::ffi::c_int,
    pub offset: off_t,
    pub text: *mut ::core::ffi::c_void,
    pub map_size: size_t,
    pub size: size_t,
    pub ntramp: ::core::ffi::c_int,
    pub free_tables: *mut tramp_table,
    pub nfree_tables: ::core::ffi::c_int,
    pub status: tramp_globals_status,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tramp_table {
    pub prev: *mut tramp_table,
    pub next: *mut tramp_table,
    pub code_table: *mut ::core::ffi::c_void,
    pub parm_table: *mut ::core::ffi::c_void,
    pub array: *mut tramp,
    pub free: *mut tramp,
    pub nfree: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tramp {
    pub prev: *mut tramp,
    pub next: *mut tramp,
    pub table: *mut tramp_table,
    pub code: *mut ::core::ffi::c_void,
    pub parm: *mut tramp_parm,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tramp_parm {
    pub data: *mut ::core::ffi::c_void,
    pub target: *mut ::core::ffi::c_void,
}
pub const PATH_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PROT_EXEC: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MAP_PRIVATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_FIXED: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MAP_ANONYMOUS: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MAP_FAILED: *mut ::core::ffi::c_void =
    -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
static mut tramp_globals: tramp_globals = tramp_globals {
    fd: 0,
    offset: 0,
    text: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    map_size: 0,
    size: 0,
    ntramp: 0,
    free_tables: ::core::ptr::null::<tramp_table>() as *mut tramp_table,
    nfree_tables: 0,
    status: TRAMP_GLOBALS_UNINITIALIZED,
};
unsafe extern "C" fn ffi_tramp_get_libffi() -> ::core::ffi::c_int {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut file: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut line: [::core::ffi::c_char; 4196] = [0; 4196];
    let mut perm: [::core::ffi::c_char; 10] = [0; 10];
    let mut dev: [::core::ffi::c_char; 10] = [0; 10];
    let mut start: ::core::ffi::c_ulong = 0;
    let mut end: ::core::ffi::c_ulong = 0;
    let mut offset: ::core::ffi::c_ulong = 0;
    let mut inode: ::core::ffi::c_ulong = 0;
    let mut addr: uintptr_t = tramp_globals.text as uintptr_t;
    let mut nfields: ::core::ffi::c_int = 0;
    let mut found: ::core::ffi::c_int = 0;
    let mut open_flags: ::core::ffi::c_int = O_RDONLY;
    open_flags |= O_CLOEXEC;
    snprintf(
        &raw mut file as *mut ::core::ffi::c_char,
        PATH_MAX as size_t,
        b"/proc/%d/maps\0" as *const u8 as *const ::core::ffi::c_char,
        getpid(),
    );
    fp = fopen(
        &raw mut file as *mut ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if fp.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    found = 0 as ::core::ffi::c_int;
    while feof(fp) == 0 as ::core::ffi::c_int {
        if fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4196]>() as ::core::ffi::c_int,
            fp,
        )
        .is_null()
        {
            break;
        }
        nfields = sscanf(
            &raw mut line as *mut ::core::ffi::c_char,
            b"%lx-%lx %9s %lx %9s %ld %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut start,
            &raw mut end,
            &raw mut perm as *mut ::core::ffi::c_char,
            &raw mut offset,
            &raw mut dev as *mut ::core::ffi::c_char,
            &raw mut inode,
            &raw mut file as *mut ::core::ffi::c_char,
        );
        if nfields != 7 as ::core::ffi::c_int {
            continue;
        }
        if !(addr >= start as uintptr_t && addr < end as uintptr_t) {
            continue;
        }
        tramp_globals.offset =
            (offset as uintptr_t).wrapping_add(addr.wrapping_sub(start as uintptr_t)) as off_t;
        found = 1 as ::core::ffi::c_int;
        break;
    }
    fclose(fp);
    if found == 0 {
        return 0 as ::core::ffi::c_int;
    }
    tramp_globals.fd = open(&raw mut file as *mut ::core::ffi::c_char, open_flags);
    if tramp_globals.fd == -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    if tramp_table_alloc() == 0 {
        close(tramp_globals.fd);
        tramp_globals.fd = -(1 as ::core::ffi::c_int);
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn ffi_tramp_get_temp_file() -> ::core::ffi::c_int {
    let mut count: ssize_t = 0;
    tramp_globals.offset = 0 as off_t;
    tramp_globals.fd = open_temp_exec_file();
    count = write(tramp_globals.fd, tramp_globals.text, tramp_globals.map_size);
    if count >= 0 as ssize_t
        && count as size_t == tramp_globals.map_size
        && tramp_table_alloc() != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    close(tramp_globals.fd);
    tramp_globals.fd = -(1 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ffi_tramp_init_os() -> ::core::ffi::c_int {
    if ffi_tramp_get_libffi() != 0 {
        return 1 as ::core::ffi::c_int;
    }
    return ffi_tramp_get_temp_file();
}
static mut tramp_globals_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __elision: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
            __next: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
        },
    },
};
unsafe extern "C" fn ffi_tramp_lock() {
    pthread_mutex_lock(&raw mut tramp_globals_mutex);
}
unsafe extern "C" fn ffi_tramp_unlock() {
    pthread_mutex_unlock(&raw mut tramp_globals_mutex);
}
unsafe extern "C" fn tramp_table_map(mut table: *mut tramp_table) -> ::core::ffi::c_int {
    let mut addr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    addr = mmap(
        NULL,
        tramp_globals.map_size.wrapping_mul(2 as size_t),
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -(1 as ::core::ffi::c_int),
        0 as __off_t,
    ) as *mut ::core::ffi::c_char;
    if addr == MAP_FAILED as *mut ::core::ffi::c_char {
        return 0 as ::core::ffi::c_int;
    }
    (*table).code_table = mmap(
        addr as *mut ::core::ffi::c_void,
        tramp_globals.map_size,
        PROT_READ | PROT_EXEC,
        MAP_PRIVATE | MAP_FIXED,
        tramp_globals.fd,
        tramp_globals.offset as __off_t,
    );
    if (*table).code_table == MAP_FAILED {
        munmap(
            addr as *mut ::core::ffi::c_void,
            tramp_globals.map_size.wrapping_mul(2 as size_t),
        );
        return 0 as ::core::ffi::c_int;
    }
    (*table).parm_table = (*table).code_table.offset(tramp_globals.map_size as isize);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn tramp_table_unmap(mut table: *mut tramp_table) {
    munmap((*table).code_table, tramp_globals.map_size);
    munmap((*table).parm_table, tramp_globals.map_size);
}
unsafe extern "C" fn ffi_tramp_init() -> ::core::ffi::c_int {
    let mut page_size: ::core::ffi::c_long = 0;
    if tramp_globals.status as ::core::ffi::c_uint
        == TRAMP_GLOBALS_PASSED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 1 as ::core::ffi::c_int;
    }
    if tramp_globals.status as ::core::ffi::c_uint
        == TRAMP_GLOBALS_FAILED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if Some(
        ffi_tramp_arch
            as unsafe extern "C" fn(*mut size_t, *mut size_t) -> *mut ::core::ffi::c_void,
    )
    .is_none()
    {
        tramp_globals.status = TRAMP_GLOBALS_FAILED;
        return 0 as ::core::ffi::c_int;
    }
    tramp_globals.free_tables = ::core::ptr::null_mut::<tramp_table>();
    tramp_globals.nfree_tables = 0 as ::core::ffi::c_int;
    tramp_globals.text =
        ffi_tramp_arch(&raw mut tramp_globals.size, &raw mut tramp_globals.map_size);
    tramp_globals.ntramp =
        tramp_globals.map_size.wrapping_div(tramp_globals.size) as ::core::ffi::c_int;
    page_size = sysconf(_SC_PAGESIZE as ::core::ffi::c_int);
    if page_size >= 0 as ::core::ffi::c_long && page_size as size_t > tramp_globals.map_size {
        return 0 as ::core::ffi::c_int;
    }
    if ffi_tramp_init_os() != 0 {
        tramp_globals.status = TRAMP_GLOBALS_PASSED;
        return 1 as ::core::ffi::c_int;
    }
    tramp_globals.status = TRAMP_GLOBALS_FAILED;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn tramp_table_alloc() -> ::core::ffi::c_int {
    let mut table: *mut tramp_table = ::core::ptr::null_mut::<tramp_table>();
    let mut tramp_array: *mut tramp = ::core::ptr::null_mut::<tramp>();
    let mut tramp: *mut tramp = ::core::ptr::null_mut::<tramp>();
    let mut size: size_t = 0;
    let mut code: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parm: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if tramp_globals.nfree_tables > 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    table = malloc(::core::mem::size_of::<tramp_table>() as size_t) as *mut tramp_table;
    if table.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    tramp_array = malloc(
        (::core::mem::size_of::<tramp>() as size_t).wrapping_mul(tramp_globals.ntramp as size_t),
    ) as *mut tramp;
    if !tramp_array.is_null() {
        if tramp_table_map(table) == 0 {
            free(tramp_array as *mut ::core::ffi::c_void);
        } else {
            (*table).array = tramp_array;
            (*table).free = ::core::ptr::null_mut::<tramp>();
            (*table).nfree = 0 as ::core::ffi::c_int;
            size = tramp_globals.size;
            code = (*table).code_table as *mut ::core::ffi::c_char;
            parm = (*table).parm_table as *mut ::core::ffi::c_char;
            i = 0 as ::core::ffi::c_int;
            while i < tramp_globals.ntramp {
                tramp = tramp_array.offset(i as isize) as *mut tramp;
                (*tramp).table = table;
                (*tramp).code = code as *mut ::core::ffi::c_void;
                (*tramp).parm = parm as *mut tramp_parm;
                tramp_add(tramp);
                code = code.offset(size as isize);
                parm = parm.offset(size as isize);
                i += 1;
            }
            return 1 as ::core::ffi::c_int;
        }
    }
    free(table as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn tramp_table_free(mut table: *mut tramp_table) {
    tramp_table_unmap(table);
    free((*table).array as *mut ::core::ffi::c_void);
    free(table as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn tramp_table_add(mut table: *mut tramp_table) {
    (*table).next = tramp_globals.free_tables;
    (*table).prev = ::core::ptr::null_mut::<tramp_table>();
    if !tramp_globals.free_tables.is_null() {
        (*tramp_globals.free_tables).prev = table;
    }
    tramp_globals.free_tables = table;
    tramp_globals.nfree_tables += 1;
}
unsafe extern "C" fn tramp_table_del(mut table: *mut tramp_table) {
    tramp_globals.nfree_tables -= 1;
    if !(*table).prev.is_null() {
        (*(*table).prev).next = (*table).next;
    }
    if !(*table).next.is_null() {
        (*(*table).next).prev = (*table).prev;
    }
    if tramp_globals.free_tables == table {
        tramp_globals.free_tables = (*table).next;
    }
}
unsafe extern "C" fn tramp_add(mut tramp: *mut tramp) {
    let mut table: *mut tramp_table = (*tramp).table;
    (*tramp).next = (*table).free;
    (*tramp).prev = ::core::ptr::null_mut::<tramp>();
    if !(*table).free.is_null() {
        (*(*table).free).prev = tramp;
    }
    (*table).free = tramp;
    (*table).nfree += 1;
    if (*table).nfree == 1 as ::core::ffi::c_int {
        tramp_table_add(table);
    }
    if (*table).nfree == tramp_globals.ntramp
        && tramp_globals.nfree_tables > 1 as ::core::ffi::c_int
    {
        tramp_table_del(table);
        tramp_table_free(table);
    }
}
unsafe extern "C" fn tramp_del(mut tramp: *mut tramp) {
    let mut table: *mut tramp_table = (*tramp).table;
    (*table).nfree -= 1;
    if !(*tramp).prev.is_null() {
        (*(*tramp).prev).next = (*tramp).next;
    }
    if !(*tramp).next.is_null() {
        (*(*tramp).next).prev = (*tramp).prev;
    }
    if (*table).free == tramp {
        (*table).free = (*tramp).next;
    }
    if (*table).nfree == 0 as ::core::ffi::c_int {
        tramp_table_del(table);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_is_supported() -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ffi_tramp_lock();
    ret = ffi_tramp_init();
    ffi_tramp_unlock();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_alloc(
    mut flags: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut tramp: *mut tramp = ::core::ptr::null_mut::<tramp>();
    ffi_tramp_lock();
    if ffi_tramp_init() == 0 || flags != 0 as ::core::ffi::c_int {
        ffi_tramp_unlock();
        return NULL;
    }
    if tramp_table_alloc() == 0 {
        ffi_tramp_unlock();
        return NULL;
    }
    tramp = (*tramp_globals.free_tables).free;
    tramp_del(tramp);
    ffi_tramp_unlock();
    return tramp as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_set_parms(
    mut arg: *mut ::core::ffi::c_void,
    mut target: *mut ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut tramp: *mut tramp = arg as *mut tramp;
    ffi_tramp_lock();
    (*(*tramp).parm).target = target;
    (*(*tramp).parm).data = data;
    ffi_tramp_unlock();
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_get_addr(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut tramp: *mut tramp = arg as *mut tramp;
    let mut addr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ffi_tramp_lock();
    addr = (*tramp).code;
    ffi_tramp_unlock();
    return addr;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_tramp_free(mut arg: *mut ::core::ffi::c_void) {
    let mut tramp: *mut tramp = arg as *mut tramp;
    ffi_tramp_lock();
    tramp_add(tramp);
    ffi_tramp_unlock();
}

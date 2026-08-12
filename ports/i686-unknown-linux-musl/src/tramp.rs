#[repr(C)]
pub struct _IO_FILE {
    _private: [u8; 0],
}

extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn open_temp_exec_file() -> ::core::ffi::c_int;
    fn fopen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn feof(_: *mut FILE) -> ::core::ffi::c_int;
    fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn close(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn write(_: ::core::ffi::c_int, _: *const ::core::ffi::c_void, _: size_t) -> ssize_t;
    fn getpid() -> pid_t;
    fn sysconf(_: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn open(_: *const ::core::ffi::c_char, _: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn mmap(
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: off_t,
    ) -> *mut ::core::ffi::c_void;
    fn munmap(_: *mut ::core::ffi::c_void, _: size_t) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn ffi_tramp_arch(tramp_size: *mut size_t, map_size: *mut size_t) -> *mut ::core::ffi::c_void;
}
pub type size_t = ::core::ffi::c_uint;
pub type ssize_t = ::core::ffi::c_int;
pub type off_t = ::core::ffi::c_longlong;
pub type FILE = _IO_FILE;
pub type pid_t = ::core::ffi::c_int;
pub type uintptr_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutex_t {
    pub __u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __i: [::core::ffi::c_int; 6],
    pub __vi: [::core::ffi::c_int; 6],
    pub __p: [*mut ::core::ffi::c_void; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [::core::ffi::c_ulong; 32],
}
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
#[inline]
unsafe extern "C" fn __CPU_AND_S(
    mut __size: size_t,
    mut __dest: *mut cpu_set_t,
    mut __src1: *const cpu_set_t,
    mut __src2: *const cpu_set_t,
) {
    let mut __i: size_t = 0;
    __i = 0 as size_t;
    while (__i as usize)
        < (__size as usize).wrapping_div(::core::mem::size_of::<::core::ffi::c_long>() as usize)
    {
        *(__dest as *mut ::core::ffi::c_ulong).offset(__i as isize) =
            *(__src1 as *mut ::core::ffi::c_ulong).offset(__i as isize)
                & *(__src2 as *mut ::core::ffi::c_ulong).offset(__i as isize);
        __i = __i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn __CPU_OR_S(
    mut __size: size_t,
    mut __dest: *mut cpu_set_t,
    mut __src1: *const cpu_set_t,
    mut __src2: *const cpu_set_t,
) {
    let mut __i: size_t = 0;
    __i = 0 as size_t;
    while (__i as usize)
        < (__size as usize).wrapping_div(::core::mem::size_of::<::core::ffi::c_long>() as usize)
    {
        *(__dest as *mut ::core::ffi::c_ulong).offset(__i as isize) =
            *(__src1 as *mut ::core::ffi::c_ulong).offset(__i as isize)
                | *(__src2 as *mut ::core::ffi::c_ulong).offset(__i as isize);
        __i = __i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn __CPU_XOR_S(
    mut __size: size_t,
    mut __dest: *mut cpu_set_t,
    mut __src1: *const cpu_set_t,
    mut __src2: *const cpu_set_t,
) {
    let mut __i: size_t = 0;
    __i = 0 as size_t;
    while (__i as usize)
        < (__size as usize).wrapping_div(::core::mem::size_of::<::core::ffi::c_long>() as usize)
    {
        *(__dest as *mut ::core::ffi::c_ulong).offset(__i as isize) =
            *(__src1 as *mut ::core::ffi::c_ulong).offset(__i as isize)
                ^ *(__src2 as *mut ::core::ffi::c_ulong).offset(__i as isize);
        __i = __i.wrapping_add(1);
    }
}
pub const _SC_PAGESIZE: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAP_FAILED: *mut ::core::ffi::c_void =
    -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
pub const MAP_PRIVATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_FIXED: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MAP_ANON: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MAP_ANONYMOUS: ::core::ffi::c_int = MAP_ANON;
pub const PROT_READ: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PROT_EXEC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
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
    );
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
        if !(addr as ::core::ffi::c_ulong >= start && (addr as ::core::ffi::c_ulong) < end) {
            continue;
        }
        tramp_globals.offset =
            offset.wrapping_add((addr as ::core::ffi::c_ulong).wrapping_sub(start)) as off_t;
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
    if count >= 0 as ::core::ffi::c_int
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
    __u: C2RustUnnamed {
        __i: [0 as ::core::ffi::c_int; 6],
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
        0 as off_t,
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
        tramp_globals.offset,
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
    page_size = sysconf(_SC_PAGESIZE);
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
pub const PATH_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;

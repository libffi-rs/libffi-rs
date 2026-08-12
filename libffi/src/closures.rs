extern "C" {
    fn vm_allocate(
        target_task: vm_map_t,
        address: *mut vm_address_t,
        size: vm_size_t,
        flags: ::core::ffi::c_int,
    ) -> kern_return_t;
    fn vm_deallocate(
        target_task: vm_map_t,
        address: vm_address_t,
        size: vm_size_t,
    ) -> kern_return_t;
    fn vm_protect(
        target_task: vm_map_t,
        address: vm_address_t,
        size: vm_size_t,
        set_maximum: boolean_t,
        new_protection: vm_prot_t,
    ) -> kern_return_t;
    fn vm_remap(
        target_task: vm_map_t,
        target_address: *mut vm_address_t,
        size: vm_size_t,
        mask: vm_address_t,
        flags: ::core::ffi::c_int,
        src_task: vm_map_t,
        src_address: vm_address_t,
        copy: boolean_t,
        cur_protection: *mut vm_prot_t,
        max_protection: *mut vm_prot_t,
        inheritance: vm_inherit_t,
    ) -> kern_return_t;
    static mut mach_task_self_: mach_port_t;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    static mut ffi_closure_trampoline_table_page: *mut ::core::ffi::c_void;
}
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_DEFAULT_ABI: ffi_abi = 1;
pub const FFI_LAST_ABI: ffi_abi = 3;
pub const FFI_WIN64: ffi_abi = 2;
pub const FFI_SYSV: ffi_abi = 1;
pub const FFI_FIRST_ABI: ffi_abi = 0;
pub type __darwin_natural_t = ::core::ffi::c_uint;
pub type __darwin_size_t = usize;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 56],
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
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
    pub aarch64_nfixedargs: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_closure {
    pub trampoline_table: *mut ::core::ffi::c_void,
    pub trampoline_table_entry: *mut ::core::ffi::c_void,
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
pub struct ffi_trampoline_table_entry {
    pub trampoline: Option<unsafe extern "C" fn() -> *mut ::core::ffi::c_void>,
    pub next: *mut ffi_trampoline_table_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_trampoline_table {
    pub config_page: vm_address_t,
    pub free_count: uint16_t,
    pub free_list: *mut ffi_trampoline_table_entry,
    pub free_list_pool: *mut ffi_trampoline_table_entry,
    pub prev: *mut ffi_trampoline_table,
    pub next: *mut ffi_trampoline_table,
}
pub type uint16_t = u16;
pub type vm_address_t = vm_offset_t;
pub type vm_offset_t = uintptr_t;
pub type pthread_mutex_t = __darwin_pthread_mutex_t;
pub type kern_return_t = ::core::ffi::c_int;
pub type vm_size_t = uintptr_t;
pub type mach_port_t = __darwin_mach_port_t;
pub type vm_map_t = mach_port_t;
pub type vm_prot_t = ::core::ffi::c_int;
pub type boolean_t = ::core::ffi::c_int;
pub type vm_inherit_t = ::core::ffi::c_uint;
pub const FFI_TRAMPOLINE_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KERN_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VM_FLAGS_ANYWHERE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const VM_FLAGS_OVERWRITE: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const VM_PROT_EXECUTE: vm_prot_t = 0x4 as ::core::ffi::c_int;
pub const VM_INHERIT_SHARE: vm_inherit_t = 0 as ::core::ffi::c_int as vm_inherit_t;
pub const PAGE_MAX_SHIFT: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const PAGE_MAX_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << PAGE_MAX_SHIFT;
pub const _PTHREAD_MUTEX_SIG_init: ::core::ffi::c_int = 0x32aaaba7 as ::core::ffi::c_int;
pub const FFI_TRAMPOLINE_COUNT: ::core::ffi::c_int = PAGE_MAX_SIZE / FFI_TRAMPOLINE_SIZE;
static mut ffi_trampoline_lock: pthread_mutex_t = _opaque_pthread_mutex_t {
    __sig: _PTHREAD_MUTEX_SIG_init as ::core::ffi::c_long,
    __opaque: [
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
};
static mut ffi_trampoline_tables: *mut ffi_trampoline_table =
    ::core::ptr::null::<ffi_trampoline_table>() as *mut ffi_trampoline_table;
unsafe extern "C" fn ffi_trampoline_table_alloc() -> *mut ffi_trampoline_table {
    let mut table: *mut ffi_trampoline_table = ::core::ptr::null_mut::<ffi_trampoline_table>();
    let mut config_page: vm_address_t = 0;
    let mut trampoline_page: vm_address_t = 0;
    let mut trampoline_page_template: vm_address_t = 0;
    let mut cur_prot: vm_prot_t = 0;
    let mut max_prot: vm_prot_t = 0;
    let mut kt: kern_return_t = 0;
    let mut i: uint16_t = 0;
    config_page = 0 as vm_address_t;
    kt = vm_allocate(
        mach_task_self_ as vm_map_t,
        &raw mut config_page,
        (PAGE_MAX_SIZE * 2 as ::core::ffi::c_int) as vm_size_t,
        VM_FLAGS_ANYWHERE,
    );
    if kt != KERN_SUCCESS {
        return ::core::ptr::null_mut::<ffi_trampoline_table>();
    }
    trampoline_page = config_page.wrapping_add(PAGE_MAX_SIZE as vm_address_t);
    trampoline_page_template = &raw mut ffi_closure_trampoline_table_page as vm_address_t;
    kt = vm_remap(
        mach_task_self_ as vm_map_t,
        &raw mut trampoline_page,
        PAGE_MAX_SIZE as vm_size_t,
        0 as vm_address_t,
        VM_FLAGS_OVERWRITE,
        mach_task_self_ as vm_map_t,
        trampoline_page_template,
        FALSE,
        &raw mut cur_prot,
        &raw mut max_prot,
        VM_INHERIT_SHARE,
    );
    if kt != KERN_SUCCESS {
        vm_deallocate(
            mach_task_self_ as vm_map_t,
            config_page,
            (PAGE_MAX_SIZE * 2 as ::core::ffi::c_int) as vm_size_t,
        );
        return ::core::ptr::null_mut::<ffi_trampoline_table>();
    }
    if cur_prot & VM_PROT_EXECUTE == 0 {
        kt = vm_protect(
            mach_task_self_ as vm_map_t,
            trampoline_page,
            PAGE_MAX_SIZE as vm_size_t,
            FALSE,
            cur_prot | VM_PROT_EXECUTE,
        );
        if kt != KERN_SUCCESS {
            vm_deallocate(
                mach_task_self_ as vm_map_t,
                config_page,
                (PAGE_MAX_SIZE * 2 as ::core::ffi::c_int) as vm_size_t,
            );
            return ::core::ptr::null_mut::<ffi_trampoline_table>();
        }
    }
    table = calloc(
        1 as size_t,
        ::core::mem::size_of::<ffi_trampoline_table>() as size_t,
    ) as *mut ffi_trampoline_table;
    (*table).free_count = FFI_TRAMPOLINE_COUNT as uint16_t;
    (*table).config_page = config_page;
    (*table).free_list_pool = calloc(
        FFI_TRAMPOLINE_COUNT as size_t,
        ::core::mem::size_of::<ffi_trampoline_table_entry>() as size_t,
    ) as *mut ffi_trampoline_table_entry;
    i = 0 as uint16_t;
    while (i as ::core::ffi::c_int) < (*table).free_count as ::core::ffi::c_int {
        let mut entry: *mut ffi_trampoline_table_entry =
            (*table).free_list_pool.offset(i as isize) as *mut ffi_trampoline_table_entry;
        (*entry).trampoline = ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn() -> *mut ::core::ffi::c_void>,
        >(
            trampoline_page
                .wrapping_add((i as ::core::ffi::c_int * FFI_TRAMPOLINE_SIZE) as vm_address_t)
                as *mut ::core::ffi::c_void,
        );
        if (i as ::core::ffi::c_int)
            < (*table).free_count as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            (*entry).next = (*table)
                .free_list_pool
                .offset((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                as *mut ffi_trampoline_table_entry;
        }
        i = i.wrapping_add(1);
    }
    (*table).free_list = (*table).free_list_pool;
    return table;
}
unsafe extern "C" fn ffi_trampoline_table_free(mut table: *mut ffi_trampoline_table) {
    if !(*table).prev.is_null() {
        (*(*table).prev).next = (*table).next;
    }
    if !(*table).next.is_null() {
        (*(*table).next).prev = (*table).prev;
    }
    vm_deallocate(
        mach_task_self_ as vm_map_t,
        (*table).config_page,
        (PAGE_MAX_SIZE * 2 as ::core::ffi::c_int) as vm_size_t,
    );
    free((*table).free_list_pool as *mut ::core::ffi::c_void);
    free(table as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ffi_closure_alloc(
    mut size: size_t,
    mut code: *mut *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut closure: *mut ffi_closure = malloc(size) as *mut ffi_closure;
    if closure.is_null() {
        return NULL;
    }
    pthread_mutex_lock(&raw mut ffi_trampoline_lock);
    let mut table: *mut ffi_trampoline_table = ffi_trampoline_tables;
    if table.is_null() || (*table).free_list.is_null() {
        table = ffi_trampoline_table_alloc();
        if table.is_null() {
            pthread_mutex_unlock(&raw mut ffi_trampoline_lock);
            free(closure as *mut ::core::ffi::c_void);
            return NULL;
        }
        (*table).next = ffi_trampoline_tables;
        if !(*table).next.is_null() {
            (*(*table).next).prev = table;
        }
        ffi_trampoline_tables = table;
    }
    let mut entry: *mut ffi_trampoline_table_entry = (*ffi_trampoline_tables).free_list;
    (*ffi_trampoline_tables).free_list = (*entry).next;
    (*ffi_trampoline_tables).free_count = (*ffi_trampoline_tables).free_count.wrapping_sub(1);
    (*entry).next = ::core::ptr::null_mut::<ffi_trampoline_table_entry>();
    pthread_mutex_unlock(&raw mut ffi_trampoline_lock);
    *code = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> *mut ::core::ffi::c_void>,
        *mut ::core::ffi::c_void,
    >((*entry).trampoline);
    (*closure).trampoline_table = table as *mut ::core::ffi::c_void;
    (*closure).trampoline_table_entry = entry as *mut ::core::ffi::c_void;
    return closure as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn ffi_closure_free(mut ptr: *mut ::core::ffi::c_void) {
    let mut closure: *mut ffi_closure = ptr as *mut ffi_closure;
    pthread_mutex_lock(&raw mut ffi_trampoline_lock);
    let mut table: *mut ffi_trampoline_table =
        (*closure).trampoline_table as *mut ffi_trampoline_table;
    let mut entry: *mut ffi_trampoline_table_entry =
        (*closure).trampoline_table_entry as *mut ffi_trampoline_table_entry;
    (*entry).next = (*table).free_list;
    (*table).free_list = entry;
    (*table).free_count = (*table).free_count.wrapping_add(1);
    if (*table).free_count as ::core::ffi::c_int == FFI_TRAMPOLINE_COUNT
        && ffi_trampoline_tables != table
    {
        ffi_trampoline_table_free(table);
    } else if ffi_trampoline_tables != table {
        (*table).prev = ::core::ptr::null_mut::<ffi_trampoline_table>();
        (*table).next = ffi_trampoline_tables;
        if !ffi_trampoline_tables.is_null() {
            (*ffi_trampoline_tables).prev = table;
        }
        ffi_trampoline_tables = table;
    }
    pthread_mutex_unlock(&raw mut ffi_trampoline_lock);
    free(closure as *mut ::core::ffi::c_void);
}

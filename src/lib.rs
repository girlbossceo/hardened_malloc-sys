#![no_std]

use core::ffi::{c_int, c_void};

extern crate libc;

extern "C" {
    /*
        TODO: implement this

        #ifdef __ANDROID__
    #define H_MALLOC_USABLE_SIZE_CONST const
    #else
    #define H_MALLOC_USABLE_SIZE_CONST
    #endif

        for:
        // glibc extensions
    size_t h_malloc_usable_size(H_MALLOC_USABLE_SIZE_CONST void *ptr);
     */

    /* C standard */

    pub fn h_malloc(size: usize) -> *mut c_void;
    pub fn h_calloc(nmemb: usize, size: usize) -> *mut c_void;
    pub fn h_realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn h_aligned_malloc(alignment: usize, size: usize) -> *mut c_void;
    pub fn h_free(ptr: *mut c_void);

    /* POSIX */

    pub fn h_posix_memalign(memptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;

    /* glibc extensions */

    pub fn h_malloc_usable_size(ptr: *const c_void) -> usize;
    pub fn h_mallopt(param: c_int, value: c_int) -> c_int;
    pub fn h_malloc_trim(pad: usize) -> c_int;
    pub fn h_malloc_stats(void: c_void) -> c_void;

    /* obsolete glibc extensions */

    pub fn h_memalign(alignment: usize, size: usize) -> *mut c_void;
    pub fn h_pvalloc(size: usize) -> *mut c_void;
    pub fn h_cfree(ptr: *mut c_void) -> c_void;
    pub fn h_malloc_get_state(void: c_void) -> c_void;
    pub fn h_malloc_set_state(ptr: *mut c_void) -> c_int;

    /*TODO: implement this see the top:
        #if defined(__GLIBC__) || defined(__ANDROID__)
    struct mallinfo h_mallinfo(void);
    #endif
    #ifndef __ANDROID__
    int h_malloc_info(int options, FILE *fp);
    #endif
     */

    /* hardened_malloc extensions */

    /// return an upper bound on object size for any pointer based on malloc metadata
    pub fn h_malloc_object_size(ptr: *const c_void) -> usize;

    /// similar to malloc_object_size, but avoids locking so the results are much more limited
    pub fn h_malloc_object_size_fast(ptr: *const c_void) -> usize;

    /// The free function with an extra parameter for passing the size requested at
    /// allocation time.
    ///
    /// This offers the same functionality as C++14 sized deallocation and can be
    /// used to implement it.
    ///
    /// A performance-oriented allocator would use this as a performance
    /// enhancement with undefined behavior on a mismatch. Instead, this hardened
    /// allocator implementation uses it to improve security by checking that the
    /// passed size matches the allocated size.
    pub fn h_free_sized(ptr: *mut c_void, expected_size: usize) -> c_void;
}

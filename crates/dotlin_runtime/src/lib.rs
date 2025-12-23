#[no_mangle]
pub extern "C" fn println_i64(val: i64) {
    println!("{}", val);
}

#[no_mangle]
pub extern "C" fn println_str(ptr: *const u8) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let len = *(ptr as *const u64);
        let data = ptr.add(8);
        let s = std::slice::from_raw_parts(data, len as usize);
        let s = std::str::from_utf8(s).unwrap_or("invalid utf8");
        println!("{}", s);
    }
}

#[no_mangle]
pub extern "C" fn dotlin_string_concat(s1: *const u8, s2: *const u8) -> *const u8 {
    if s1.is_null() {
        return s2;
    }
    if s2.is_null() {
        return s1;
    }
    unsafe {
        let len1 = *(s1 as *const u64);
        let len2 = *(s2 as *const u64);
        let total_len = len1 + len2;

        let layout = std::alloc::Layout::from_size_align((total_len + 8) as usize, 8).unwrap();
        let new_ptr = std::alloc::alloc(layout);

        *(new_ptr as *mut u64) = total_len;
        std::ptr::copy_nonoverlapping(s1.add(8), new_ptr.add(8), len1 as usize);
        std::ptr::copy_nonoverlapping(s2.add(8), new_ptr.add(8 + len1 as usize), len2 as usize);

        new_ptr
    }
}

#[no_mangle]
pub extern "C" fn println_f64(val: f64) {
    println!("{}", val);
}

#[no_mangle]
pub extern "C" fn dotlin_string_compare(s1: *const u8, s2: *const u8) -> i64 {
    if s1.is_null() || s2.is_null() {
        if s1 == s2 {
            return 0;
        }
        return if s1.is_null() { -1 } else { 1 };
    }
    unsafe {
        let len1 = *(s1 as *const u64);
        let len2 = *(s2 as *const u64);
        let slice1 = std::slice::from_raw_parts(s1.add(8), len1 as usize);
        let slice2 = std::slice::from_raw_parts(s2.add(8), len2 as usize);
        match slice1.cmp(slice2) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

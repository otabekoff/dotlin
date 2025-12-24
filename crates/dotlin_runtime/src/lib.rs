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

// Array runtime functions
use std::collections::HashMap as StdHashMap;

#[repr(C)]
pub struct DotlinArray {
    data: *mut [u64; 1000], // Fixed-size array for now, will be replaced with dynamic allocation
    size: u64,
    capacity: u64,
}

#[no_mangle]
pub extern "C" fn dotlin_array_new(_element_size: u64, capacity: u64) -> *mut DotlinArray {
    let vec: Vec<u64> = vec![0; capacity as usize];
    let boxed = vec.into_boxed_slice();
    let array = DotlinArray {
        data: Box::into_raw(boxed) as *mut [u64; 1000],
        size: 0,
        capacity,
    };
    
    Box::into_raw(Box::new(array))
}

#[no_mangle]
pub extern "C" fn dotlin_array_get(array_ptr: *mut DotlinArray, index: u64) -> u64 {
    if array_ptr.is_null() {
        return 0; // Error value
    }
    
    unsafe {
        let array = &*array_ptr;
        if index >= array.size {
            return 0; // Error value
        }
        
        let slice = &(*array.data);
        slice[index as usize]
    }
}

#[no_mangle]
pub extern "C" fn dotlin_array_set(array_ptr: *mut DotlinArray, index: u64, value: u64) {
    if array_ptr.is_null() {
        return;
    }
    
    unsafe {
        let array = &mut *array_ptr;
        if index >= array.capacity {
            return; // Out of bounds
        }
        
        let slice = &mut (*array.data);
        slice[index as usize] = value;
        
        if index >= array.size {
            array.size = index + 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn dotlin_array_length(array_ptr: *mut DotlinArray) -> u64 {
    if array_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let array = &*array_ptr;
        array.size
    }
}

// HashMap runtime functions
#[repr(C)]
pub struct DotlinHashMap {
    data: *mut StdHashMap<String, u64>, // For now, we'll use String keys and u64 values
    key_type_size: u64,
    value_type_size: u64,
}

#[no_mangle]
pub extern "C" fn dotlin_map_new() -> *mut DotlinHashMap {
    let map: StdHashMap<String, u64> = StdHashMap::new();
    let boxed_map = Box::new(map);
    
    let hash_map = DotlinHashMap {
        data: Box::into_raw(boxed_map),
        key_type_size: 8, // Assuming string key size
        value_type_size: 8, // u64 value size
    };
    
    Box::into_raw(Box::new(hash_map))
}

#[no_mangle]
pub extern "C" fn dotlin_map_get(map_ptr: *mut DotlinHashMap, key_ptr: *const u8) -> u64 {
    if map_ptr.is_null() || key_ptr.is_null() {
        return 0; // Default value for missing key
    }
    
    unsafe {
        let map = &*(*map_ptr).data;
        
        let len = *(key_ptr as *const u64);
        let data = key_ptr.add(8);
        let s = std::slice::from_raw_parts(data, len as usize);
        let key_str = std::str::from_utf8(s).unwrap_or("invalid_key");
        
        *map.get(key_str).unwrap_or(&0)
    }
}

#[no_mangle]
pub extern "C" fn dotlin_map_set(map_ptr: *mut DotlinHashMap, key_ptr: *const u8, value: u64) {
    if map_ptr.is_null() || key_ptr.is_null() {
        return;
    }
    
    unsafe {
        let map = &mut *(*map_ptr).data;
        
        let len = *(key_ptr as *const u64);
        let data = key_ptr.add(8);
        let s = std::slice::from_raw_parts(data, len as usize);
        let key_str = std::str::from_utf8(s).unwrap_or("invalid_key").to_string();
        
        map.insert(key_str, value);
    }
}

#[no_mangle]
pub extern "C" fn dotlin_map_remove(map_ptr: *mut DotlinHashMap, key_ptr: *const u8) -> u64 {
    if map_ptr.is_null() || key_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let map = &mut *(*map_ptr).data;
        
        let len = *(key_ptr as *const u64);
        let data = key_ptr.add(8);
        let s = std::slice::from_raw_parts(data, len as usize);
        let key_str = std::str::from_utf8(s).unwrap_or("invalid_key").to_string();
        
        map.remove(&key_str).unwrap_or(0)
    }
}

#[no_mangle]
pub extern "C" fn dotlin_map_contains(map_ptr: *mut DotlinHashMap, key_ptr: *const u8) -> u64 {
    if map_ptr.is_null() || key_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let map = &*(*map_ptr).data;
        
        let len = *(key_ptr as *const u64);
        let data = key_ptr.add(8);
        let s = std::slice::from_raw_parts(data, len as usize);
        let key_str = std::str::from_utf8(s).unwrap_or("invalid_key");
        
        if map.contains_key(key_str) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn dotlin_map_free(map_ptr: *mut DotlinHashMap) {
    if map_ptr.is_null() {
        return;
    }
    
    unsafe {
        let _boxed_map = Box::from_raw((*map_ptr).data);
        let _boxed_hash_map = Box::from_raw(map_ptr);
    }
}

// HashMap iteration functions
#[no_mangle]
pub extern "C" fn dotlin_map_keys(map_ptr: *mut DotlinHashMap) -> *mut DotlinArray {
    if map_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let map = &*(*map_ptr).data;
        let keys: Vec<u64> = map.keys()
            .map(|k| {
                // Convert string key to a string object in our format
                // This is a simplified approach - in a real implementation we'd need to create proper string objects
                k.as_bytes().iter().fold(0u64, |acc, &b| acc ^ (b as u64)) // Simple hash as placeholder
            })
            .collect();
            
        let capacity = keys.len() as u64;
        let mut vec: Vec<u64> = vec![0; capacity as usize];
        for (i, &key_hash) in keys.iter().enumerate() {
            vec[i] = key_hash;
        }
        let boxed = vec.into_boxed_slice();
        let array = DotlinArray {
            data: Box::into_raw(boxed) as *mut [u64; 1000],
            size: capacity,
            capacity,
        };
        
        Box::into_raw(Box::new(array))
    }
}

#[no_mangle]
pub extern "C" fn dotlin_map_values(map_ptr: *mut DotlinHashMap) -> *mut DotlinArray {
    if map_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let map = &*(*map_ptr).data;
        let values: Vec<u64> = map.values().cloned().collect();
        
        let capacity = values.len() as u64;
        let mut vec: Vec<u64> = vec![0; capacity as usize];
        for (i, &value) in values.iter().enumerate() {
            vec[i] = value;
        }
        let boxed = vec.into_boxed_slice();
        let array = DotlinArray {
            data: Box::into_raw(boxed) as *mut [u64; 1000],
            size: capacity,
            capacity,
        };
        
        Box::into_raw(Box::new(array))
    }
}

#[no_mangle]
pub extern "C" fn dotlin_map_size(map_ptr: *mut DotlinHashMap) -> u64 {
    if map_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let map = &*(*map_ptr).data;
        map.len() as u64
    }
}

use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap as StdHashMap;

#[unsafe(no_mangle)]
pub extern "C" fn println_i64(val: i64) {
    println!("{}", val);
}

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
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

        let layout = Layout::from_size_align((total_len + 8) as usize, 8).unwrap();
        let new_ptr = alloc(layout);

        *(new_ptr as *mut u64) = total_len;
        std::ptr::copy_nonoverlapping(s1.add(8), new_ptr.add(8), len1 as usize);
        std::ptr::copy_nonoverlapping(s2.add(8), new_ptr.add(8 + len1 as usize), len2 as usize);

        new_ptr
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn println_f64(val: f64) {
    println!("{}", val);
}

#[unsafe(no_mangle)]
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

#[repr(C)]
pub struct DotlinArray {
    data: *mut u64, // Dynamic array of u64 values
    size: u64,
    capacity: u64,
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_array_new(_element_size: u64, initial_capacity: u64) -> *mut DotlinArray {
    let capacity = if initial_capacity == 0 { 1 } else { initial_capacity }; // Ensure at least 1 element capacity
    let vec: Vec<u64> = vec![0; capacity as usize];
    let boxed = vec.into_boxed_slice();
    let array = DotlinArray {
        data: Box::into_raw(boxed) as *mut u64,
        size: 0,
        capacity,
    };
    
    Box::into_raw(Box::new(array))
}

unsafe fn grow_array(old_ptr: *mut u64, old_size: usize, new_size: usize) -> *mut u64 {
    let old_layout = Layout::array::<u64>(old_size).unwrap();
    let new_layout = Layout::array::<u64>(new_size).unwrap();

    let new_ptr = unsafe { alloc(new_layout) } as *mut u64;
    if !new_ptr.is_null() {
        unsafe {
            std::ptr::copy_nonoverlapping(old_ptr, new_ptr, old_size);
            for i in old_size..new_size {
                std::ptr::write(new_ptr.add(i), 0);
            }
            dealloc(old_ptr as *mut u8, old_layout);
        }
    }

    new_ptr
}

// Math runtime functions
#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_abs(val: f64) -> f64 {
    val.abs()
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_min(a: f64, b: f64) -> f64 {
    a.min(b)
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_max(a: f64, b: f64) -> f64 {
    a.max(b)
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_sqrt(val: f64) -> f64 {
    if val < 0.0 {
        std::f64::NAN
    } else {
        val.sqrt()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_pow(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_sin(val: f64) -> f64 {
    val.sin()
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_cos(val: f64) -> f64 {
    val.cos()
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_tan(val: f64) -> f64 {
    val.tan()
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_floor(val: f64) -> f64 {
    val.floor()
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_ceil(val: f64) -> f64 {
    val.ceil()
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_round(val: f64) -> f64 {
    val.round()
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_log(val: f64) -> f64 {
    if val <= 0.0 {
        std::f64::NAN
    } else {
        val.ln()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_exp(val: f64) -> f64 {
    val.exp()
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_pi() -> f64 {
    std::f64::consts::PI
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_math_e() -> f64 {
    std::f64::consts::E
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_array_get(array_ptr: *mut DotlinArray, index: u64) -> u64 {
    if array_ptr.is_null() {
        return 0; // Error value
    }
    
    unsafe {
        let array = &*array_ptr;
        if index >= array.size {
            return 0; // Error value
        }
        
        let data_ptr = array.data;
        std::ptr::read(data_ptr.add(index as usize))
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_array_set(array_ptr: *mut DotlinArray, index: u64, value: u64) {
    if array_ptr.is_null() {
        return;
    }
    
    unsafe {
        let array = &mut *array_ptr;
        
        // If we need to grow the array to accommodate this index
        if index >= array.capacity {
            // Ensure capacity is sufficient
            let new_capacity = std::cmp::max(array.capacity * 2, index + 1);
            let new_data = grow_array(array.data, array.capacity as usize, new_capacity as usize);
            array.data = new_data;
            array.capacity = new_capacity;
        }
        
        let data_ptr = array.data;
        std::ptr::write(data_ptr.add(index as usize), value);
        
        if index >= array.size {
            array.size = index + 1;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_array_length(array_ptr: *mut DotlinArray) -> u64 {
    if array_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let array = &*array_ptr;
        array.size
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_array_push(array_ptr: *mut DotlinArray, value: u64) {
    if array_ptr.is_null() {
        return;
    }
    
    unsafe {
        let array = &mut *array_ptr;
        
        // Check if we need to grow the array
        if array.size >= array.capacity {
            let new_capacity = if array.capacity == 0 { 1 } else { array.capacity * 2 };
            let new_data = grow_array(array.data, array.capacity as usize, new_capacity as usize);
            array.data = new_data;
            array.capacity = new_capacity;
        }
        
        let data_ptr = array.data;
        std::ptr::write(data_ptr.add(array.size as usize), value);
        array.size += 1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_array_pop(array_ptr: *mut DotlinArray) -> u64 {
    if array_ptr.is_null() {
        return 0; // Error value
    }
    
    unsafe {
        let array = &mut *array_ptr;
        if array.size == 0 {
            return 0; // Error value - array is empty
        }
        
        array.size -= 1;
        let data_ptr = array.data;
        std::ptr::read(data_ptr.add(array.size as usize)) // Return the popped value
    }
}

// HashMap runtime functions
#[repr(C)]
pub struct DotlinHashMap {
    data: *mut StdHashMap<String, u64>, // For now, we'll use String keys and u64 values
    key_type_size: u64,
    value_type_size: u64,
}

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_map_free(map_ptr: *mut DotlinHashMap) {
    if map_ptr.is_null() {
        return;
    }
    
    unsafe {
        let _boxed_map = Box::from_raw((*map_ptr).data);
        let _boxed_hash_map = Box::from_raw(map_ptr);
    }
}

// Type conversion runtime functions
#[unsafe(no_mangle)]
pub extern "C" fn dotlin_string_to_int(str_ptr: *const u8) -> i64 {
    if str_ptr.is_null() {
        return 0; // Default value for error
    }
    
    unsafe {
        let len = *(str_ptr as *const u64);
        let data = str_ptr.add(8);
        let s = std::slice::from_raw_parts(data, len as usize);
        let s = std::str::from_utf8(s).unwrap_or("0");
        
        s.parse::<i64>().unwrap_or(0)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_string_to_float(str_ptr: *const u8) -> f64 {
    if str_ptr.is_null() {
        return 0.0; // Default value for error
    }
    
    unsafe {
        let len = *(str_ptr as *const u64);
        let data = str_ptr.add(8);
        let s = std::slice::from_raw_parts(data, len as usize);
        let s = std::str::from_utf8(s).unwrap_or("0.0");
        
        s.parse::<f64>().unwrap_or(0.0)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_int_to_float(val: i64) -> f64 {
    val as f64
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_float_to_int(val: f64) -> i64 {
    val as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_to_string(val: i64) -> *const u8 {
    let s = val.to_string();
    
    unsafe {
        let total_len = s.len() as u64;
        let layout = std::alloc::Layout::from_size_align((total_len + 8) as usize, 8).unwrap();
        let new_ptr = std::alloc::alloc(layout);
        
        *(new_ptr as *mut u64) = total_len;
        std::ptr::copy_nonoverlapping(s.as_ptr(), new_ptr.add(8), s.len());
        
        new_ptr
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_float_to_string(val: f64) -> *const u8 {
    let s = val.to_string();
    
    unsafe {
        let total_len = s.len() as u64;
        let layout = std::alloc::Layout::from_size_align((total_len + 8) as usize, 8).unwrap();
        let new_ptr = std::alloc::alloc(layout);
        
        *(new_ptr as *mut u64) = total_len;
        std::ptr::copy_nonoverlapping(s.as_ptr(), new_ptr.add(8), s.len());
        
        new_ptr
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_bool_to_string(val: i8) -> *const u8 {
    let s = if val != 0 { "true".to_string() } else { "false".to_string() };
    
    unsafe {
        let total_len = s.len() as u64;
        let layout = std::alloc::Layout::from_size_align((total_len + 8) as usize, 8).unwrap();
        let new_ptr = std::alloc::alloc(layout);
        
        *(new_ptr as *mut u64) = total_len;
        std::ptr::copy_nonoverlapping(s.as_ptr(), new_ptr.add(8), s.len());
        
        new_ptr
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_char_to_string(val: i64) -> *const u8 {
    let c = val as u8 as char;
    let s = c.to_string();
    
    unsafe {
        let total_len = s.len() as u64;
        let layout = std::alloc::Layout::from_size_align((total_len + 8) as usize, 8).unwrap();
        let new_ptr = std::alloc::alloc(layout);
        
        *(new_ptr as *mut u64) = total_len;
        std::ptr::copy_nonoverlapping(s.as_ptr(), new_ptr.add(8), s.len());
        
        new_ptr
    }
}

// HashMap iteration functions
#[unsafe(no_mangle)]
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
            data: Box::into_raw(boxed) as *mut u64,
            size: capacity,
            capacity,
        };
        
        Box::into_raw(Box::new(array))
    }
}

#[unsafe(no_mangle)]
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
            data: Box::into_raw(boxed) as *mut u64,
            size: capacity,
            capacity,
        };
        
        Box::into_raw(Box::new(array))
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_map_size(map_ptr: *mut DotlinHashMap) -> u64 {
    if map_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let map = &*(*map_ptr).data;
        map.len() as u64
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_map_entries(map_ptr: *mut DotlinHashMap) -> *mut DotlinArray {
    if map_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let map = &*(*map_ptr).data;
        // For now, return an array of alternating key-value pairs
        // In a real implementation, we would create proper entry objects
        let mut entries: Vec<u64> = Vec::new();
        for (key, &value) in map.iter() {
            // Convert key to a simple hash representation (placeholder)
            let key_hash = key.as_bytes().iter().fold(0u64, |acc, &b| acc ^ (b as u64));
            entries.push(key_hash);
            entries.push(value);
        }
        
        let capacity = entries.len() as u64;
        let mut vec: Vec<u64> = vec![0; capacity as usize];
        for (i, &entry) in entries.iter().enumerate() {
            vec[i] = entry;
        }
        let boxed = vec.into_boxed_slice();
        let array = DotlinArray {
            data: Box::into_raw(boxed) as *mut u64,
            size: capacity,
            capacity,
        };
        
        Box::into_raw(Box::new(array))
    }
}

#[repr(C)]
pub struct DotlinIterator {
    entries: *mut DotlinArray,
    pos: u64,
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_map_iter_new(map_ptr: *mut DotlinHashMap) -> *mut DotlinIterator {
    if map_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let entries = dotlin_map_entries(map_ptr);
    let it = DotlinIterator { entries, pos: 0 };
    Box::into_raw(Box::new(it))
}

#[unsafe(no_mangle)]
pub extern "C" fn dotlin_iterator_next(it_ptr: *mut DotlinIterator) -> u64 {
    if it_ptr.is_null() {
        return 0;
    }

    unsafe {
        let it = &mut *it_ptr;
        if it.entries.is_null() {
            return 0;
        }
        let arr = &*it.entries;
        if it.pos >= arr.size {
            return 0;
        }
        let val = std::ptr::read(arr.data.add(it.pos as usize));
        it.pos += 1;
        val
    }
}

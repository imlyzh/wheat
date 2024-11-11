use std::ptr::NonNull;

use crate::{object_model::*, scope_model::Scope};

#[derive(Debug, Clone)]
pub struct SemiSpaceMemory {
    pub pool0: *mut u8,
    pub pool1: *mut u8,
    pub start_pointer: *mut u8,

    // pointer_compress: bool,
    pub alloc_count: usize, // alloc_count <= pool_size
    // align: usize,       // Must be greater than default, default == 4B(32bit)

    // if pointer_compress enable, pool size <= 4GB
    size_limit: usize,
}

pub fn vm_align(pointer_compress: bool, page_size: usize) -> usize {
    if pointer_compress {
        page_size::_4G
    } else {
        page_size
    }
}

impl Default for SemiSpaceMemory {
    fn default() -> Self {
        Self {
            pool0: std::ptr::null_mut(),
            pool1: std::ptr::null_mut(),
            start_pointer: std::ptr::null_mut(),
            // pointer_compress: false,
            alloc_count: 0,
            size_limit: 0,
        }
    }
}

impl SemiSpaceMemory {
    pub fn init(pool_size: usize, pointer_compress: bool) -> Self {
        // assert!(align >= 8);
        // let pool0 = alloc_page(std::ptr::null_mut(), pool_size, pointer_compress);
        // let pool1 = alloc_page(std::ptr::null_mut(), pool_size, pointer_compress);
        let pool0 = Vec::<u8>::with_capacity(vm_align(pointer_compress, pool_size)).as_ptr();
        let pool1 = Vec::<u8>::with_capacity(vm_align(pointer_compress, pool_size)).as_ptr();
        let pool0 = pool0 as *mut u8;
        let pool1 = pool1 as *mut u8;
        SemiSpaceMemory {
            pool0,
            pool1,
            start_pointer: pool0,

            // pointer_compress,
            alloc_count: 0,
            size_limit: pool_size,
        }
    }
}

impl SemiSpaceMemory {
    pub unsafe fn alloc(
        &mut self,
        current: NonNull<Scope>,
        alloc_size: usize,
    ) -> NonNull<ObjectHead> {
        if !self.alloc_count + alloc_size >= self.size_limit {
            self.gc(current);
        }
        if !self.alloc_count + alloc_size >= self.size_limit {
            panic!("out of memory")
        }
        let ret_ptr = self.start_pointer.add(self.alloc_count);
        self.alloc_count += alloc_size;
        NonNull::new_unchecked(ret_ptr as *mut ObjectHead)
    }

    pub unsafe fn gc(&mut self, current: NonNull<Scope>) {
        let free = if self.start_pointer == self.pool0 {
            self.pool1
        } else {
            self.pool0
        };
        let mut alloc_cur: usize = 0;
        let mut current = Some(current);
        loop {
            if current.is_none() {
                break;
            }
            let mut cur = current.unwrap();
            let r = self.copy(free, &mut alloc_cur, cur.as_ref().pointer);
            cur.as_mut().pointer = r;
            current = cur.as_ref().prev;
        }
        self.start_pointer = free;
        self.alloc_count = alloc_cur;
    }

    unsafe fn copy(&mut self, free: *mut u8, alloc_cur: &mut usize, obj: Slot) -> Slot {
        if !(*obj).moved {
            let coped_obj = self.copy_data(free, alloc_cur, obj);
            (*obj).moved = true;
            match get_tag(coped_obj) {
                ObjectTag::Pair => {
                    (*(coped_obj as *mut Pair)).car =
                        self.copy(free, alloc_cur, (*(coped_obj as *mut Pair)).car);
                    (*(coped_obj as *mut Pair)).cdr =
                        self.copy(free, alloc_cur, (*(coped_obj as *mut Pair)).cdr);
                }
                ObjectTag::Vector => {
                    let len = (*(coped_obj as *mut Vector)).length;
                    let data =
                        &mut (*(coped_obj as *mut Vector)).instance[0] as *mut *mut ObjectHead;
                    for i in 0..(len as usize) {
                        const PTRSIZE: usize = std::mem::size_of::<usize>();
                        *data.add(i * PTRSIZE) = self.copy(free, alloc_cur, *data.add(i * PTRSIZE));
                    }
                }
                _ => {}
            }
            (*(obj as *mut SingleData)).value = coped_obj as u64;
            return coped_obj;
        }
        (*(obj as *mut SingleData)).value as Slot
    }

    unsafe fn copy_data(&mut self, free: *mut u8, alloc_cur: &mut usize, obj: Slot) -> Slot {
        let obj_size = match get_tag(obj) {
            ObjectTag::Null | ObjectTag::Bool | ObjectTag::Char => (obj as *mut SingleData)
                .as_ref()
                .unwrap_unchecked()
                .length(),
            ObjectTag::Number => (obj as *mut Number).as_ref().unwrap_unchecked().length(),
            ObjectTag::Pair => (obj as *mut Pair).as_ref().unwrap_unchecked().length(),
            ObjectTag::Vector => (obj as *mut Vector).as_ref().unwrap_unchecked().length(),
            ObjectTag::String => (obj as *mut SingleByteString)
                .as_ref()
                .unwrap_unchecked()
                .length(),
            ObjectTag::Symbol => (obj as *mut Symbol).as_ref().unwrap_unchecked().length(),
            ObjectTag::Closure => todo!(),
            ObjectTag::NativeFunction => unimplemented!(),
            ObjectTag::Opaque => unimplemented!(),
        };
        if *alloc_cur + obj_size >= self.size_limit {
            panic!("gc: out of memory")
        }
        let dst = free.add(*alloc_cur);
        std::ptr::copy(dst, obj as *mut u8, obj_size);
        *alloc_cur = *alloc_cur + obj_size;
        dst as Slot
    }
}

pub mod page_size {
    pub const _4K: usize = 4096;
    pub const _64K: usize = _4K * 16;
    pub const _512K: usize = _64K * 8;
    pub const _1M: usize = _512K * 2;
    pub const _2M: usize = _1M * 2;
    pub const _4M: usize = _2M * 2;
    pub const _8M: usize = _4M * 2;
    pub const _16M: usize = _8M * 2;
    pub const _32M: usize = _16M * 2;
    pub const _256M: usize = _32M * 8;
    pub const _512M: usize = _256M * 2;
    pub const _1G: usize = _512M * 2;
    pub const _2G: usize = _1G * 2;
    pub const _4G: usize = _2G * 2;
    pub const _8G: usize = _4G * 2;
    pub const _16G: usize = _8G * 2;
}

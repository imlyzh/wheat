

// pub struct Region {
//     pub
// }

/*
pub unsafe fn alloc(size: usize) {
    use libc::*;
    let ptr = mmap(
        std::ptr::null_mut(),
        size,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    );
    let mut r = (ptr as u64);
    print!("addr: ",);
    for i in 1..64 {
        print!("{}", if (r & 1) == 1 { "1" } else { "0" } );
        r = r >> 1;
    }
    println!("");
}
*/
/// Bump Allocator
/// The simplest possible heap allocator.
/// Memory only moves forward — allocations are instant,
/// but memory is never freed individually (only reset wholesale).
///
/// Layout in memory:
/// [HEAP_START ][used...........][free..................][HEAP_END]
///              ^                ^
///              bump_start       bump_ptr (moves right on each alloc)

const HEAP_SIZE: usize = 1024 * 1024; // 1 MB heap

/// The actual heap memory — a static byte array
#[used]
static mut HEAP: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

/// Current position in the heap (how many bytes used)
static mut BUMP_PTR: usize = 0;

/// Total number of allocations made
static mut ALLOC_COUNT: usize = 0;

/// Allocate `size` bytes aligned to `align` bytes.
/// Returns a pointer to the allocated region, or None if OOM.
pub fn alloc(size: usize, align: usize) -> Option<*mut u8> {
    unsafe {
        // Round up bump pointer to the required alignment
        let start = (BUMP_PTR + align - 1) & !(align - 1);
        let end   = start + size;

        if end > HEAP_SIZE {
            return None; // Out of memory
        }

        BUMP_PTR   = end;
        ALLOC_COUNT += 1;
        Some(HEAP.as_mut_ptr().add(start))
    }
}

/// Reset the entire heap — frees all allocations at once.
/// Only safe to call when you're sure nothing is using heap memory.
pub fn reset() {
    unsafe {
        BUMP_PTR    = 0;
        ALLOC_COUNT = 0;
    }
}

/// How many bytes have been allocated
pub fn used_bytes() -> usize {
    unsafe { BUMP_PTR }
}

/// How many bytes are still free
pub fn free_bytes() -> usize {
    HEAP_SIZE - used_bytes()
}

/// How many allocations have been made
pub fn alloc_count() -> usize {
    unsafe { ALLOC_COUNT }
}

/// Total heap size
pub fn total_bytes() -> usize {
    HEAP_SIZE
}

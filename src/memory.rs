use x86_64::{
    structures::paging::{PageTable, OffsetPageTable, Page, PhysFrame, Mapper, Size4KiB, FrameAllocator},
    VirtAddr,
    PhysAddr,
};

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn available_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let available_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = available_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.available_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) 
    -> &'static mut PageTable 
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let paddr = level_4_table_frame.start_address();
    let vaddr = physical_memory_offset + paddr.as_u64();
    let pt_ptr: *mut PageTable = vaddr.as_mut_ptr();

    &mut *pt_ptr
}

pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // FIXME: this is not safe, we do it only for testing
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

// pub unsafe fn vaddr_to_paddr(vaddr: VirtAddr, physical_memory_offset: VirtAddr) 
//     -> Option<PhysAddr>
// {
//     v2p_safe(vaddr, physical_memory_offset)
// }

// fn v2p_safe(vaddr: VirtAddr, physical_memory_offset: VirtAddr)
//     -> Option<PhysAddr>
// {
//     use x86_64::structures::paging::page_table::FrameError;
//     use x86_64::registers::control::Cr3;

//     let (level_4_table_frame, _) = Cr3::read();

//     let table_indices = [
//         vaddr.p4_index(), vaddr.p3_index(), vaddr.p2_index(), vaddr.p1_index()
//     ];

//     let mut frame = level_4_table_frame;

//     for &index in &table_indices {
//         // convert the frame into a page table reference
//         let base_vaddr = physical_memory_offset + frame.start_address().as_u64();
//         let table_ptr: *const PageTable = base_vaddr.as_ptr();
//         let table = unsafe {&*table_ptr};

//         // read the page table entry and update `frame`
//         let entry = &table[index];
//         frame = match entry.frame() {
//             Ok(frame) => frame,
//             Err(FrameError::FrameNotPresent) => return None,
//             Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
//         };
//     }

//     Some(frame.start_address() + u64::from(vaddr.page_offset()))
// }

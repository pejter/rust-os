use core::ptr::addr_of_mut;

use conquer_once::spin::OnceCell;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

pub(crate) struct Segments {
    gdt: GlobalDescriptorTable,
    kernel: SegmentSelector,
    tss: SegmentSelector,
}

pub const DOUBLE_FAULT_STACK_INDEX: u16 = 0;

pub(crate) static TSS: OnceCell<TaskStateSegment> = OnceCell::uninit();
pub(crate) static GDT: OnceCell<Segments> = OnceCell::uninit();

pub(crate) fn init() {
    let tss = TSS.get_or_init(|| {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[usize::from(DOUBLE_FAULT_STACK_INDEX)] = {
            const STACK_SIZE: usize = 5 * 4 * 1024; // 5 * 4kB page
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { addr_of_mut!(STACK) });
            let stack_end = stack_start + STACK_SIZE.try_into().unwrap();

            stack_end
        };
        tss
    });

    let Segments { gdt, kernel, tss } = GDT.get_or_init(|| {
        let mut gdt = GlobalDescriptorTable::new();
        let kernel_segment = gdt.append(Descriptor::kernel_code_segment());
        let tss_segment = gdt.append(Descriptor::tss_segment(tss));
        Segments {
            gdt,
            kernel: kernel_segment,
            tss: tss_segment,
        }
    });

    gdt.load();
    unsafe {
        CS::set_reg(*kernel);
        load_tss(*tss);
    }
}

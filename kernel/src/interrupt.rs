use conquer_once::spin::OnceCell;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::gdt;

pub(crate) static IDT: OnceCell<InterruptDescriptorTable> = OnceCell::uninit();

pub(crate) fn init() {
    let idt = IDT.get_or_init(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        let double_fault_handler = idt.double_fault.set_handler_fn(double_fault_handler);
        unsafe { double_fault_handler.set_stack_index(gdt::DOUBLE_FAULT_STACK_INDEX) };
        idt
    });
    log::info!("Loading IDT");
    idt.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack: InterruptStackFrame) {
    log::warn!("BREAKPOINT: {stack:?}");
}

extern "x86-interrupt" fn double_fault_handler(stack: InterruptStackFrame, code: u64) -> ! {
    panic!("DOUBLE FAULT(code {code}): {stack:#?}");
}

use conquer_once::spin::OnceCell;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub(crate) static IDT: OnceCell<InterruptDescriptorTable> = OnceCell::uninit();

pub(crate) fn init() {
    IDT.get_or_init(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    })
    .load();
}

extern "x86-interrupt" fn breakpoint_handler(stack: InterruptStackFrame) {
    log::warn!("BREAKPOINT: {stack:?}");
}

extern "x86-interrupt" fn double_fault_handler(stack: InterruptStackFrame, code: u64) -> ! {
    panic!("DOUBLE FAULT(code {code}): {stack:#?}");
}

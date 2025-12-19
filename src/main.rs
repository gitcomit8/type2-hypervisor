use std::error;
use crate::vmm::Vmm;

mod devices;
mod kvm_api;
mod cpu;
mod memory;
mod vmm;

fn main()-> Result<(),Box<dyn error::Error>> {
    println!("Booting Type 2 Hypervisor...");

    // Init VMM
    let mut vmm = Vmm::new()?;


    // Define payload aka DummyBIOS
    // This is 16-bit x86 Real Mode asm

    // MOV DX, 0x3F8    ; Point DX to COM1 Serial Port
    // MOV AL, 'K'      ; Move character 'K' into AL
    // OUT DX, AL       ; Write AL to port DX
    // HLT              ; Halt the CPU
    let bios_payload: [u8;7]=[
        0xBA, 0xF8, 0x03,
        0xB0, 0x4B,
        0xEE,
        0xF4
    ];

    // Load payload into Reset Vector
    // X86 Real Mode CPUs start executing at address 0xFFFF0
    // We map 1MB of RAM, so offset is 0x100000 - 0x10 (16)
    let reset_vector_addr=0xFFFF0;

    println!("Loading Payload to Guest Physical Address {:#x}",reset_vector_addr);
    vmm.load_code(&bios_payload,reset_vector_addr);

    // Run the Machine
    println!("Starting VMM loop");
    vmm.run()?;

    println!("\nHypervisor Shutdown Cleanly");
    Ok(())
}

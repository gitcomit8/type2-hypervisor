use std::error::Error;
use kvm_ioctls::{Kvm, VcpuExit, VmFd};
use crate::cpu::Cpu;
use crate::memory::GuestMemory;
use crate::devices::serial::SerialDev;
use crate::kvm_api::Hypervisor;

pub struct Vmm{
	vm_fd: VmFd,
	memory: GuestMemory,
	serial: SerialDev,
	cpu: Cpu,
}

impl Vmm{
	pub fn new()-> Result<Self, Box<dyn Error>>{

		// Init Kvm (System)
		let kvm = Hypervisor::new()?;

		// create the vm
		let vm_fd=kvm.create_vm()?;

		// Setup memory (1MB)
		// map 1MB at 0x0. This covers Reset Vector at 0xFFFF0
		let memory = GuestMemory::new(&vm_fd,0x0,0x100000)?;

		// Create devices
		let serial=SerialDev::new();

		// Create vCPU
		let cpu = Cpu::new(&vm_fd, 0)?;

		Ok(Vmm{
			vm_fd,
			memory,
			serial,
			cpu,
		})
	}

	// Load raw code into guest memory at specified offset
	pub fn load_code(&mut self, code: &[u8], offset: usize){
		self.memory.write_slice(code, offset);
	}

	// Main loop: runs cpu and handles exits
	pub fn run(&mut self)-> Result<(), Box<dyn Error>>{
		println!("VMM: Starting execution look...");

		loop{
			// Run CPU until it exits
			// Borrow cpu mutably, and access serial later
			// it is safe because exit is handled outside the cpu.run() call
			let exit_reason= self.cpu.run_once()?;

			match exit_reason{
				// Handle IO Out (Write)
				VcpuExit::IoOut(port,data)=>{
					// Check if it is our serial port (COM1 usually 0x3F8)
					if port == 0x3f8{
						if let Some(&byte) =data.first(){
							self.serial.write(byte);
						}
					}
				}

				VcpuExit::Hlt => {
					println!("VMM: Guest Halted. Exiting");
					break;
				}

				//Failures
				r=>{
					eprintln!("VMM: Unexpected exit reason: {:?}", r);
					break;
				}
			}
		}
		Ok(())
	}
}
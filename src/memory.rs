use kvm_ioctls::{Error, VmFd};
use kvm_bindings::kvm_userspace_memory_region;

pub struct GuestMemory {
	// Keep vec alive so pointer remains valid
	pub mem: Vec<u8>,
}

impl GuestMemory {
	pub fn new(vm: &VmFd, guest_addr: u64, size: usize) -> Result<Self,Error>{
		// Allocate memory in process
		// Should use mmap for page alignment
		let mut mem = vec![0u8; size];

		// Prepare kvm struct to map it
		let region = kvm_userspace_memory_region{
			slot: 0, // we need only 1 for now
			guest_phys_addr: guest_addr,
			memory_size: size as u64,
			userspace_addr: mem.as_ptr() as u64, // Pointer to Host RAM
			flags: 0,
		};

		// Tell KVM to map it
		// Safety: we are passing raw pointer to kernel
		//  pointer is valid since mem is owned by this fn
		unsafe{
			vm.set_user_memory_region(region)?;
		}
		Ok(GuestMemory{mem})
	}
}
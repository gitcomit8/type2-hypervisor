use kvm_ioctls::{Error, VmFd};
use kvm_bindings::kvm_userspace_memory_region;
use std::ptr;

pub struct GuestMemory {
	pub mem_ptr: *mut u8,
	pub len: usize,
}

impl GuestMemory {
	pub fn new(vm: &VmFd, guest_addr: u64, size: usize) -> Result<Self,Error>{
		// Allocate memory in process
		let ptr = unsafe{
			libc::mmap(
				ptr::null_mut(),
				size,
				libc::PROT_READ | libc::PROT_WRITE,
				libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
				-1,
				0,
			)
		};

		if ptr == libc::MAP_FAILED {
			return Err(Error::new(std::io::Error::last_os_error().raw_os_error().unwrap()))
		}
		let mem_ptr=ptr as *mut u8;

		// Prepare KVM struct
		let region = kvm_userspace_memory_region{
			slot:0,
			guest_phys_addr: size as u64,
			memory_size: size as u64,
			userspace_addr: mem_ptr as u64, // Guarantees 4k aligned
			flags: 0,
		};

		// Register with kvm
		unsafe{
			vm.set_user_memory_region(region)?;
		}
		Ok(GuestMemory{mem_ptr, len:size})
	}

	//Helper to write slice to memory safely
	pub fn write_slice(&mut self, data: &[u8], offset: usize){
		if offset+data.len()>self.len{
			panic!("Write out of bounds");
		}
		unsafe {
			// Calculate dest pointer
			let dest = self.mem_ptr.add(offset);
			ptr::copy_nonoverlapping(data.as_ptr(), dest, data.len());
		}
	}
}

// RAII: Automatically free mem when GuestMemory goes out of scope
impl Drop for GuestMemory {
	fn drop(&mut self) {
		unsafe{
			libc::munmap(self.mem_ptr as *mut _, self.len);
		}
	}
}
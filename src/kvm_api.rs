use kvm_ioctls::{Kvm, Error, VmFd};

pub(crate) struct Hypervisor {
	//Hold the kvm instance to keep /dev/kvm open
	kvm: Kvm,
}

impl Hypervisor {

	pub fn new()-> Result<Self,Error>{
		// Open /dev/kvm
		let kvm = Kvm::new()?;
		//Using ? instead of unwrap() to propagate error on failure
		Ok(Self{ kvm, })
	}

	pub fn get_vcpu_mmap_size(&self) ->Result<usize,Error>{
		// Call helper method on self.kvm
		self.kvm.get_vcpu_mmap_size()
	}

	pub fn create_vm(&self) -> Result<VmFd,Error> {
		// Returns specialized fd that represents te VM
		self.kvm.create_vm()
	}
}

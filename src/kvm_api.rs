use kvm_ioctls::{Kvm,Error};

struct Hypervisor {
	kvm: Kvm,
}

impl Hypervisor {
	pub fn new()-> Result<Self,Error>{
		// Open /dev/kvm
		let kvm = Kvm::new()?;
		//Using ? instead of unwrap() to propagate error on failure
		Ok(Self{ kvm, })
	}
}
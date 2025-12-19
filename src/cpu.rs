use kvm_ioctls::{VmFd,VcpuFd,Error};


pub struct Cpu{
	fd: VcpuFd,
	id: u64,
}

impl Cpu{
	pub fn new(vm: &VmFd, id: u64)->Result<Self,Error>{
		// Create new vCPU attached to given vm
		let fd = vm.create_vcpu(id)?;
		Ok(Cpu{fd,id})
	}

	pub fn run_once(&mut self)-> Result<kvm_ioctls::VcpuExit,Error>{
		self.fd.run()
	}
}
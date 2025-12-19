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

	pub fn run(&mut self)->Result<(),Error>{
		loop{
			// Switch step
			// block here while hw runs guest
			match self.fd.run(){
				Ok(exit_reason)=>{
					match exit_reason{
						// TODO: handle specific exit reasons
						kvm_ioctls::VcpuExit::IoIn(addr,data)=>{
							println!("IO IN at {:#x}",addr);
						}
						kvm_ioctls::VcpuExit::IoOut(addr,data)=>{
							println!("IO OUT at {:#x}",addr);
						}
						kvm_ioctls::VcpuExit::Hlt => {
							println!("vCPU {} Halted",self.id);
							break;
						}
						_ =>{
							println!("unexpected exit reason");
							break;
						}
					}
				}
				Err(e)=> return Err(e),
			}
		}
		Ok(())
	}
}
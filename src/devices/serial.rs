use std::io::{self,Write};

// Emulate standard 8250 UART serial port (simplified)
pub struct SerialDev{
	// might buffer this later
	// just pass it to stdout for now
}

impl SerialDev {
	pub fn new()->Self{
		SerialDev{}
	}

	// Handle writes to serial port data reg
	// Returns true if write was handled successfully
	pub fn write(&mut self, data: u8){
		// act as passthrough dev
		// when guest writes to COM1, write to Host's terminal
		print!("{}", data as char);
		// Flush immediately so it is seen
		let _ = io::stdout().flush();
	}
}
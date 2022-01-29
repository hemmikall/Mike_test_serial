use std::error::Error;
use std::thread;
use std::time::Duration;
use rpi_embedded::uart::{Parity, Uart};

fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the primary UART and configure it for 115.2 kbit/s, no
    // parity bit, 8 data bits and 1 stop bit.
    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
println!("UART Initialized");
    // Configure read() to block until at least 1 byte is received.
    uart.set_read_mode(1, Duration::default())?;

    let mut n1:u8 = 1;
    let mut n2:u8;
    let a:u8 = 1;
    let mut buffer = [0u8;1];

    
    loop {
        n2 = n1;
        println!("Pi sends {}", n2);
        buffer[0] = n2;
        uart.write_bytes(&mut buffer).unwrap();
        thread::sleep(Duration::from_millis(200));

        uart.read_bytes(&mut buffer).unwrap(); 
        n1 = buffer[0];       
        println!("Pi gets {}", n1);
        if n1 > 244 { n1 = 1; }
        n1 = n1 + a;      
    }
}
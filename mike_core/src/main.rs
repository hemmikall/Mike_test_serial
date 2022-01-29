use std::error::Error;
use std::thread;
use std::char;
use std::time::Duration;
use rpi_embedded::uart::{Parity, Uart};
use rpi_embedded::i2c::I2c;
//extern crate pid;
//use pid::Pid;
pub const PI: f64 = 3.14159265358979323846264338327950288f64;


fn main() -> Result<(), Box<dyn Error>> {

    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
    let mut i2c = I2c::new()?;
    let mut i2c_imu = I2c::new()?;
    i2c.set_slave_address(0x53)?;
    i2c_imu.set_slave_address(0x57)?;
    let mut v: f64;
    println!("State 1");
    //let mut pidx = Pid::new(2.50, 0.005, 0.02, 97.0, 97.0, 97.0, 97.0, 0.0);

    loop {
        println!("State 2");
        thread::sleep(Duration::from_millis(1000));
        uart.set_read_mode(0, Duration::default())?;
        uart.set_write_mode(false)?;
/*
        let s = uart.read_line().unwrap_or_default();
        if s.trim().is_empty() == false {
        let spl = s.trim().split(",");
        let vectstr: Vec<&str> = spl.collect();
        if vectstr[0] == "hld" && vectstr[4] == "end"
        {
        let heading = vectstr[1].parse::<f64>().unwrap_or_default();
        let leaning = vectstr[2].parse::<f64>().unwrap_or_default();}}
        //let direction = vectstr[3].parse::<f64>().unwrap_or_default();
        
       
        
       
        let cams = uart.read().unwrap_or_default();
        if cams.trim().is_empty() == false {
        let camspl = cams.trim().split(",");
        let camvectstr: Vec<&str> = camspl.collect();
        if camvectstr[0] == "D" && camvectstr[1] == "R" && camvectstr[5] == "\r" && camvectstr[6] == "\n"
        {
            v = camvectstr[4].parse::<f64>().unwrap_or_default();
        }
        if camvectstr[0] == "R" && camvectstr[1] == "O" && camvectstr[5] == "\r" && camvectstr[6] == "\n"
        {

        }}
        */ 

        let mut direction = 0.0;

        let mut angle1 = PI/3.0+direction*PI/1800.0;
        let mut angle2 = PI/3.0-direction*PI/1800.0;
        let mut angle3 = direction*PI/1800.0;

        println!("State 3");
        //let outputx = pidx.next_control_output(leaning_xpart);
        //let mut vx = outputx.output;

        v = 50.0;
        let vc = v*(angle1.cos())+80.0;
        let va = v*(angle2.cos())+80.0;
        let vb = -1.0*v*(angle3.cos())+80.0;

        //println!("{} {} {}", vc,va,vb);
        
        //let mut buffer_w = [251,vc as u8,252,va as u8,253,vb as u8,0xA,0xD];
        //i2c_imu.block_write(0x01, &mut buffer_w).unwrap_or_default();

        let mut buffer_r = [0u8;7];
        i2c_imu.block_read(0x1E,&mut buffer_r).unwrap_or_default();
        println!("block read with length {} using command 0x1E -> {:?} ", buffer_r.len(), buffer_r);
        //println!("Lx: {} Vx: {}", direction, v);
        


    }
}
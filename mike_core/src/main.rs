//use core::num::dec2flt::float;
use std::error::Error;
//use std::hash::Hasher;
//use std::result;
use std::thread;
use std::char;
use std::time::Duration;
use rpi_embedded::uart::{Parity, Uart};
use rpi_embedded::i2c::I2c;
//extern crate pid;
//use pid::Pid;
pub const PI: f64 = 3.14159265358979323846264338327950288;


fn main() -> Result<(), Box<dyn Error>> {

    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
    let mut i2c = I2c::new()?;
    let mut i2c_imu = I2c::new()?;
    i2c.set_slave_address(0x53)?;
    i2c_imu.set_slave_address(0x57)?;
    let mut v: f64;
    //println!("State 1");
    //let mut pidx = Pid::new(2.50, 0.005, 0.02, 97.0, 97.0, 97.0, 97.0, 0.0);
    let s= uart.set_read_mode(0, Duration::new(0,0));
    match s{
        Ok(_n)=>{
            println!("Serial port mode set:");
        }
        Err(err) =>{
            println!("Error writing to serial port: {}",err);
        }
    }
    let mut current_mode: u8 = 0;
    let mut ringbuffer: [char;256] = [0 as char;256];
    let buffer: &mut [u8;256] = &mut [0;256];
    let mut readpos: u8 = 0;
    let mut writepos: u8 = 0;
    let mut line_available:bool = false;

    let mut v = 0;
    let mut direction = 0;
    let mut v_rot = 0;
    let mut direction_rot = 0;
    loop {
        //println!("State 2");
        thread::sleep(Duration::from_millis(1000));

        
        // read uart
        uart.set_read_mode(0, Duration::default())?;
        uart.set_write_mode(false)?;

        let mut charcount: u8 = 0;
        let s = uart.read_bytes(buffer);
        match s{
            Ok(n) => {
                if n>0 as usize{
                    println!("Read {} bytes from serial port",n);
                }
            while n as u8 > charcount{
                ringbuffer[writepos as usize]= buffer[charcount as usize] as char;
                if ringbuffer[charcount as usize]=='\n'{
                    line_available=true;
                }
                charcount+=1;
                writepos+=1;
            }

            },
            Err(err) =>{
                println!("Error reading from serial port: {}",err);
            }
        }
        // parse uart if line is available
        if line_available{
            line_available = false;
            let ban= String::from("Line read:\n");
            let t=uart.write(ban);
            match t{
                Ok(n)=>{
                    //println!("Wrote {} bytes to serial port",n);
                }
                Err(err) =>{
                    //println!("Error writing to serial port: {}",err);
                }
            }
            while readpos!=writepos{
                // drive command received
                if ringbuffer[readpos as usize] == 'D' && ringbuffer[(readpos+1) as usize] == 'R' {
                    direction = ((ringbuffer[(readpos+2) as usize]as i16*256) + (ringbuffer[(readpos+3) as usize]as i16)) as i32;
                    v = ringbuffer[(readpos+4) as usize] as i32;
                    println!("drive dir: {} \nspeed: {}",direction,v);
                }
                // rotate command received
                if ringbuffer[readpos as usize] == 'R' && ringbuffer[(readpos+1) as usize] == 'O' {
                    direction_rot = ((ringbuffer[(readpos+2) as usize]as i16*256) + (ringbuffer[(readpos+3) as usize]as i16)) as i32;
                    v_rot = ringbuffer[(readpos+4) as usize] as i32;
                    println!("rotate dir: {} \nspeed: {}",direction_rot,v_rot);
                }
                // set mode command received
                if ringbuffer[readpos as usize] == 'S' && ringbuffer[(readpos+1) as usize] == 'M' {
                    current_mode = ringbuffer[(readpos+2) as usize] as u8;
                    println!("set to mode: {}",current_mode);
                }
                readpos+=1;
            }
            
        }

        // put code for each mode withing statements below
        match current_mode{
            0 =>{
                // state is 0
            }
            1 =>{
                // state is 1
            }
            2 =>{
                // state is 2
            }
            3 =>{
                // state is 3
            }
            4 =>{
                // state is 4
            }
            5 =>{
                // state is 5
            }
            6 =>{
                // state is 6
            }
            7 =>{
                // state is 7
            }
            8 =>{
                // state is 8
            }
            9 =>{
                // state is 9
            }
            10 =>{
                // state is 10
            }
            11 =>{
                // state is 11
            }
            12 =>{
                // state is 12
            }
            13 =>{
                // state is 13
            }
            14 =>{
                // state is 14
            }
            15 =>{
                // state is 15
            }
            16 =>{
                // state is 16
            }
            17 =>{
                // state is 17
            }
            18 =>{
                // state is 18
            }
            19 =>{
                // state is 19
            }
            20 =>{
                // state is 20
            }
            21 =>{
                // state is 21
            }
            22 =>{
                // state is 22
            }
            23 =>{
                // state is 23
            }
            24 =>{
                // state is 24
            }
            25 =>{
                // state is 25
            }
            26 =>{
                // state is 26
            }
            27 =>{
                // state is 27
            }
            28 =>{
                // state is 28
            }
            29 =>{
                // state is 29
            }
            _ =>{
                // everything else
            }
        }
        
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

        /*
        let mut angle1 = 0.0;
        let mut angle2 = 0.0;
        let mut angle3 = 0.0;
        unsafe{

            angle1 = (PI*10000.0/30000.0 as i64)+direction*((PI*1000.0/1800000.0) as i64);
            angle2 = PI/3.0-(direction as f64)*PI/1800.0;
            angle3 = (direction as f64)*PI/1800.0;
    
        }
        //println!("State 3");
        //let outputx = pidx.next_control_output(leaning_xpart);
        //let mut vx = outputx.output;
        
        //calculate motor values
        v = 50.0;
        let vc = v*(angle1.cos())+80.0;
        let va = v*(angle2.cos())+80.0;
        let vb = -1.0*v*(angle3.cos())+80.0;

        //println!("{} {} {}", vc,va,vb);
        */
        //write to motors 
//        let mut buffer_w = [251,vc as u8,252,va as u8,253,vb as u8,0xA,0xD];
//        i2c_imu.block_write(0x01, &mut buffer_w).unwrap_or_default();

//        let mut buffer_r = [0u8;7];
//        i2c_imu.block_read(0x1E,&mut buffer_r).unwrap_or_default();
//        println!("block read with length {} using command 0x1E -> {:?} ", buffer_r.len(), buffer_r);
        //println!("Lx: {} Vx: {}", direction, v);
        


    }
}
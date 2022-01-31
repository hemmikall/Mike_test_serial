//use core::num::dec2flt::float;
use std::error::Error;
//use std::hash::Hasher;
//use std::result;
//use std::thread;
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
    let mut i2c_crane = I2c::new()?;
    i2c.set_slave_address(0x53)?;
    i2c_crane.set_slave_address(0x51)?;
    let mut liftdone = 0;
    let mut liftreport = 0;
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
    let mut direction: i16 = 0;
    let mut camdirection: i16 = 0;
    let mut v_rot:i32;
    let mut direction_rot:i16;
    let mut imu_h:i16;
    let mut imu_l:i16;
    let mut imu_d:i16;
    let mut distance_sensor: u8=0;
    loop {
        //println!("State 2");
        //thread::sleep(Duration::from_millis(1000));

        
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
                if ringbuffer[writepos as usize]=='\n'{
                    line_available=true;
                }
                charcount+=1;
                if writepos != 255{
                    writepos+=1;
                }
                else{
                    writepos=0;
                }
            }

            },
            Err(err) =>{
                println!("Error reading from serial port: {}",err);
            }
        }
        // parse uart if line is available
        if line_available{
            line_available = false;
            let mut read:[char;256] = [0 as char;256];
            let mut len = 0;
            while readpos!=writepos{
                read[len as usize] = ringbuffer[readpos as usize];
                len+=1;
                if readpos !=255{
                    readpos+=1;

                }
                else{
                    readpos=0;
                }
            }
            // drive command received
            if read[0 as usize] == 'D' && read[(1) as usize] == 'R' {
                camdirection = (((read[(2 as u8) as usize] as i16 )<<8) + (read[(3) as usize]as i16))*10;
                v = read[(4 as u8) as usize] as i32;
                println!("drive dir: {} \nspeed: {}",direction,v);
            }
            // rotate command received
            if read[0 as usize] == 'R' && read[(1 as u8) as usize] == 'O' {
                direction_rot = ((read[(2 as u8) as usize] as i16 )<<8) + (read[(3) as usize]as i16);
                v_rot = read[(4 as u8) as usize] as i32;
                println!("rotate dir: {} \nspeed: {}",direction_rot,v_rot);
            }
            // set mode command received
            if read[0 as usize] == 'S' && read[(1) as usize] == 'M' {
                current_mode = read[(2 as u8) as usize] as u8-1;
                println!("set to mode: {}",current_mode);
            }// set mode command received
            if read[0 as usize] == 'D' && read[(1) as usize] == 'S' {
                distance_sensor = read[(2 as u8) as usize] as u8;
                println!("Distance sensor: {}",distance_sensor);
            }// rotate command received
            if read[0 as usize] == 'I' && read[(1 as u8) as usize] == 'M' {
                imu_h = ((read[(2 as u8) as usize] as i16 )<<8) + (read[(3) as usize]as i16);
                imu_l = ((read[(4 as u8) as usize] as i16 )<<8) + (read[(6) as usize]as i16);
                imu_d = ((read[(6 as u8) as usize] as i16 )<<8) + (read[(9) as usize]as i16);
                println!("H: {} \nL: {}\nD: {}",imu_h,imu_l,imu_d);
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
        direction = 1500-camdirection;         // when it reads the direction from pixy arduino - flip it and deduct 300.0
        let angle1 = PI/3.0+(direction as f64)*(PI/1800.0);
        let angle2 = PI/3.0-(direction as f64)*PI/1800.0;
        let angle3 = (direction as f64)*PI/1800.0;
        //println!("State 3");
        //let outputx = pidx.next_control_output(leaning_xpart);
        //let mut vx = outputx.output;
        
        //calculate motor value
        let va = (v as f64)*(angle1.cos())+80.0;
        let vb = (v as f64)*(angle2.cos())+80.0;
        let vc = -1.0*(v as f64)*(angle3.cos())+80.0;

        //println!("{} {} {}", vc,va,vb);
        
        let mut buffer_w = [0x01,251,va as u8,252,vb as u8,253,vc as u8,0xA,0xD];
        i2c.write(&mut buffer_w).unwrap_or_default();




        let mut cbuffer_r = [0;5];

        i2c_crane.read(&mut cbuffer_r)?;
        //println!("write read with length {} -> {:?} ", cbuffer_r.len(), cbuffer_r);

        if cbuffer_r[0] == 1 && cbuffer_r[1] == 112 && cbuffer_r[3] == 13 && cbuffer_r[4] == 10{
            liftreport = cbuffer_r[2];
        }
        if liftreport == 1 {
            liftdone = 1;
            liftreport = 0;
        }

        let mut cbuffer_w = [0x01,241,liftdone as u8,242,40 as u8,243,40 as u8,0xA,0xD];
        i2c_crane.write(&mut cbuffer_w).unwrap_or_default();
        


    }
}
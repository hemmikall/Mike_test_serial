//use core::num::dec2flt::float;
use std::error::Error;
//use std::hash::Hasher;
//use std::result;
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
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
    let mut i2c_crane = I2c::new()?;
    i2c.set_slave_address(0x53)?;
    i2c_crane.set_slave_address(0x51)?;
    let mut liftdone = 1;
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
    let mut normal_distance = 0;
    let mut set_normal_distance = false;
    let mut normal_distance2 = 0;
    let mut set_normal_distance2 = false;

    let mut v = 0;
    let mut direction: i16;
    let mut camdirection: i16 = 0;
    let mut v_rot:i32;
    let mut direction_rot:i16 = 0;
    let mut distance_sensor_1: u8=0;
    let mut distance_sensor_2: u8=0;
// thread communications
    let (tx_i16,rx_i16): (Sender<String>,Receiver<String>) = channel();
    let (tx_i16_2,rx_i16_2): (Sender<String>,Receiver<String>) = channel();
    let (tx_i32,rx_i32): (Sender<String>,Receiver<String>) = channel();
    let (tx_str,rx_str): (Sender<String>,Receiver<String>) = channel();
    create_input_thread(tx_i16,tx_i16_2,tx_i32,tx_str);

// generic count
    let mut loopcount = 0;

    // Set serial mode
    uart.set_read_mode(0, Duration::default())?;
    uart.set_write_mode(true)?;
    loop {
        //println!("State 2");
        //thread::sleep(Duration::from_millis(1000));

// get inputs from terminal

        let received_serial=rx_i16.try_recv();
        match received_serial{
            Ok(val)=>{

                println!("{}",val);
                if val.chars().nth(0).unwrap()>='0' && val.chars().nth(0).unwrap()<='9'{
                    camdirection = val.parse().unwrap();
                    println!("Set direction to: {}",camdirection);
                }
            }
            Err(err)=>{
                if err == TryRecvError::Empty{

                }
                if err == TryRecvError::Disconnected{
                    println!("Error: direction manual input channel disconnected {}",err);
                }
            }
        }
        let received_rot=rx_i16_2.try_recv();
        match received_rot{
            Ok(val)=>{

                println!("{}",val);
                if val.chars().nth(0).unwrap()>='0' && val.chars().nth(0).unwrap()<='9'{
                    direction_rot = val.parse().unwrap();
                    println!("Set direction to: {}",direction_rot);
                }
            }
            Err(err)=>{
                if err == TryRecvError::Empty{

                }
                if err == TryRecvError::Disconnected{
                    println!("Error: direction manual input channel disconnected {}",err);
                }
            }
        }
        let received_v=rx_i32.try_recv();
        match received_v{
            Ok(val)=>{
                println!("{}",val);
                if val.chars().nth(0).unwrap()>='0' && val.chars().nth(0).unwrap()<='9'{
                    v = val.parse().unwrap();
                    println!("Set velocity to: {}",v);
                }
            }
            Err(err)=>{
                if err == TryRecvError::Empty{

                }
                if err == TryRecvError::Disconnected{
                    println!("Error: velocity manual input channel disconnected {}",err);
                }
            }
        }
        let received_string=rx_str.try_recv();
        match received_string{
            Ok(val)=>{
                let mut modify_string = val;
                modify_string.pop();
                modify_string.push_str("\r\n");
                let u_sent =uart.write(modify_string); 
                match u_sent{
                    Ok(_val)=>{
                    }
                    Err(err)=>{
                        println!("Error writing to serial port: {}",err);
                    }
                }
            }
            Err(err)=>{
                if err == TryRecvError::Empty{

                }
                if err == TryRecvError::Disconnected{
                    println!("Error: velocity manual input channel disconnected {}",err);
                }
            }
        }

// read uart
        let mut charcount: u8 = 0;
        let s = uart.read_bytes(buffer);
        match s{
            Ok(n) => {
                if n>0 as usize{
                   // println!("Read {} bytes from serial port",n);
                }
            while n as u8 > charcount{
                ringbuffer[writepos as usize]= buffer[charcount as usize] as char;
                if writepos>0{ 
                    if ringbuffer[writepos as usize]=='\n' && ringbuffer[(writepos-1) as usize]=='\r'{
                        line_available=true;
                    }   
                }
                else{
                    if ringbuffer[writepos as usize]=='\n' && ringbuffer[(255) as usize]=='\r'{
                        line_available=true;
                    }
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
            for i in 0..len{
                // drive command received
                let mut i_1 = i+1;
                let mut i_2 = i+2;
                let mut i_3 = i+3;
                let mut i_4 = i+4;
                let mut i_5 = i+5;
                let mut i_6 = i+6;
                let mut i_7 = i+7;
                let mut i_8 = i+8;
                let mut i_9 = i+9;
                if i_1 >255{
                    i_1 = i_1-256;
                }
                if i_2 >255{
                    i_2 = i_2-256;
                }
                if i_3 >255{
                    i_3 = i_3-256;
                }
                if i_4 >255{
                    i_4 = i_4-256;
                }
                if i_5 >255{
                    i_5 = i_5-256;
                }
                if i_6 >255{
                    i_6 = i_6-256;
                }
                if i_7 >255{
                    i_7 = i_7-256;
                }
                if i_8 >255{
                    i_8 = i_8-256;
                }
                if i_9 >255{
                    i_9 = i_9-256;
                }
                if read[i as usize] == 'D' && read[(i_1) as usize] == 'R' {
                    //    println!("Fist byte {} second byte {} " ,read[(2 as u8) as usize] as u8,read[(3 as u8) as usize] as u8);
                    camdirection = ((read[((i_2) as u8) as usize] as i16  -1)<<8) + (read[(i_3) as usize]as i16-1);
                    v = read[(i_4 as u8) as usize] as i32-1;
                    let sign = read[((i_5) as u8) as usize];
                    if sign == '-'{
                        camdirection = camdirection.abs()*(-1);
                       // println!("right");
                    }
                    //println!("drive dir: {} \nspeed: {}",camdirection,v);
                }

            
                // rotate command received
                else if read[i as usize] == 'R' && read[((i_1) as u8) as usize] == 'O' {
                    direction_rot = ((read[((i_2) as u8) as usize] as i16 )<<8) + (read[(i_3) as usize]as i16);
                    v_rot = read[((i_4) as u8) as usize] as i32;
                    //println!("rotate dir: {} \nspeed: {}",direction_rot,v_rot);
                }
            
                // set mode command received
                else if read[i as usize] == 'S' && read[(i_1) as usize] == 'M' {
                    if read[((i_2) as u8) as usize] as u8 >= 1{
                        current_mode = read[((i_2) as u8) as usize] as u8-1;
                        println!("set to mode: {}",current_mode);
                    }
                }
                /*
                // set mode command received
                else if read[i as usize] == 'I' && read[((i_1) as u8) as usize] == 'M' {
                    imu_h = ((read[((i_2) as u8) as usize] as i16 )<<8) + (read[(i_3) as usize]as i16);
                    imu_l = ((read[((i_4) as u8) as usize] as i16 )<<8) + (read[(i_5) as usize]as i16);
                    imu_d = ((read[((i_6) as u8) as usize] as i16 )<<8) + (read[(i_7) as usize]as i16);
                    imu_h = imu_h - h_ref;
                    if imu_refFlag == 1 {          // let mut imu_h = 0; has to be declared before the loop
                        h_ref = imu_h;  // <-- look at the imu_h = imu_h - h_ref before
                        imu_refFlag = 2;
                        println!("IMU calibrated");
                    }
                      println!("H: {}",imu_h);
                }*/

                else if read[i as usize] == 'X' && read[(i_1) as usize] == '1' {
                    distance_sensor_1 = read[((i_2) as u8) as usize] as u8;
                    if set_normal_distance {
                        normal_distance = distance_sensor_1;
                        set_normal_distance = false;
                    }
                    println!("Distance sensor 1: {}",distance_sensor_1);
                }// rotate command received

                else if read[i as usize] == 'X' && read[(i_1) as usize] == '2' {
                    distance_sensor_2 = read[((i_2) as u8) as usize] as u8;
                    if set_normal_distance2 {
                        normal_distance2 = distance_sensor_2;
                        set_normal_distance2 = false;
                    }
                    println!("Distance sensor 2: {}",distance_sensor_2);
                }// rotate command received
            }
        }

// put code for each mode withing statements below
        match current_mode{
            0 =>{
                // state is 0
                // default off mode
                if normal_distance == 0{
                    uart.write("distance1i\r\n".to_string())?;
                    set_normal_distance = true;
                    thread::sleep(Duration::from_millis(500));
                }
                v=0;
            }
    
            1 =>{
                let mut cbuffer_w = [0x01,241,liftdone as u8,242,10 as u8,243,90 as u8,0xA,0xD];
                i2c_crane.write(&mut cbuffer_w).unwrap_or_default();
                // state is 1
                // drive up to starting line
                // driven by camera arduino
            }
            2 =>{
                // state is 2
                // drive up to button and back
                // driven by camera arduino
            }
            3 =>{
                // state is 3
                // originally planned as find intersection
                // not used
            }
            4 =>{
                // state is 4
                // wait for traffic light
                // do nothing
            }
            5 =>{
                // state is 5
                // follow line up to bridge - original plan drive to intersection

                // read distance sensor
                // when distance sesnor triggers change modes to mode 7 and send to camera
            }
            6 =>{
                
                if distance_sensor_1 < 150 && distance_sensor_1 > 20{
                    uart.write("setmode 7c\r\n".to_string())?;
                    thread::sleep(Duration::from_millis(12));
                }
                uart.write("distance1i\r\n".to_string())?;
                // state is 6
                // orignial plan - drive to bridge
            }

            7 =>{
                // state is 7
                // deploy crane// crane operation
                liftdone = 0;
                v=0;
                direction_rot = 0;
                uart.write("setmode 8c\r\n".to_string())?;
                thread::sleep(Duration::from_millis(12));
            }
            8 =>{
                

                let mut cbuffer_w = [0x01,241,liftdone as u8,242,0 as u8,243,0 as u8,0xA,0xD];
                i2c_crane.write(&mut cbuffer_w).unwrap_or_default();
            
                v=0;
                direction_rot = 0;
                if liftdone == 1{
                    uart.write("setmode 9c\r\n".to_string())?;
                    thread::sleep(Duration::from_millis(12));
                    loopcount = 0;
                }
                // state is 8
                // wait for crane to finish
            }
            9 =>{
                // state is 9
                // read distance senor to verify bridge is up
                // if not, try again then verify and if failed again, jump to 17
                    uart.write("distance1i\r\n".to_string())?;
                    loopcount +=1;
                    thread::sleep(Duration::from_millis(12));
                    if distance_sensor_1 > 150{
                        uart.write("setmode 44c\r\n".to_string())?;
                        thread::sleep(Duration::from_millis(12));
                    }
                    else if loopcount>5{
                        uart.write("setmode 7c\r\n".to_string())?;
                        thread::sleep(Duration::from_millis(12));
                    }
            }
            44 =>{
                // state is 10
                // follow line up to intersection if going for brige and hill
                // driven by camera
                let mut cbuffer_w = [0x01,241,liftdone as u8,242,90 as u8,243,90 as u8,0xA,0xD];
                i2c_crane.write(&mut cbuffer_w).unwrap_or_default();
            }
            11 =>{
                // state is 11
                // camera switching between modes to see if button is there
            }
            12 =>{
                // state is 12
                // drive from raspberry pi
                // rotate 90°
                // use IMU to rotate +90°
            }
            13 =>{
                // state is 13
                // drive from raspberry pi
                // drive in circle
                // read IMU to read current heading
                // switch to mode 15 when IMU is within 90° of original heading

                //uart.write("distance2i\r\n".to_string())?;
                //if(distance_sensor_2 < 190 && distance_sensor_2 > 170){

                    let mut buffer_w = [0x01,251,5 as u8,252,80 as u8,253,104 as u8,0xA,0xD];
                    i2c.write(&mut buffer_w).unwrap_or_default();
                /* }
                else if(distance_sensor_2 > 190){
                    let mut buffer_w = [0x01,251,50 as u8,252,80 as u8,253,97 as u8,0xA,0xD];
                    i2c.write(&mut buffer_w).unwrap_or_default();
                }
                else{
                    let mut buffer_w = [0x01,251,30 as u8,252,80 as u8,253,97 as u8,0xA,0xD];
                    i2c.write(&mut buffer_w).unwrap_or_default();
                }*/
            }
            14 =>{
                // state is 14
                // not used - original plan to wait for IMU
            }
            15 =>{
                // state is 15
                // driven by camera
                // follow line towards button
            }
            17 =>{
                let mut cbuffer_w = [0x01,241,liftdone as u8,242,90 as u8,243,30 as u8,0xA,0xD];
                i2c_crane.write(&mut cbuffer_w).unwrap_or_default();
                thread::sleep(Duration::from_millis(500));
                let mut cbuffer_w = [0x01,241,liftdone as u8,242,90 as u8,243,90 as u8,0xA,0xD];
                i2c_crane.write(&mut cbuffer_w).unwrap_or_default();
                // state is 15
                // driven by camera
                // follow line towards button
            }
            18 =>{
                thread::sleep(Duration::from_millis(5000));
                let mut cbuffer_w = [0x01,241,liftdone as u8,242,30 as u8,243,90 as u8,0xA,0xD];
                i2c_crane.write(&mut cbuffer_w).unwrap_or_default();
                thread::sleep(Duration::from_millis(500));
                let mut cbuffer_w = [0x01,241,liftdone as u8,242,90 as u8,243,90 as u8,0xA,0xD];
                i2c_crane.write(&mut cbuffer_w).unwrap_or_default();
                // state is 15
                // driven by camera
                // follow line towards button
            }
            36 =>{
                // state is 16
                // press button to end
                    //if distance_sensor_1 > 150{
                    //    uart.write("setmode 40c\r\n".to_string())?;
                    //}
                
            }

            23 =>{
                // state is 23
                // find when on bottom of hill
                // rotate to align with track (IMU)
                let mut cbuffer_w = [0x01,241,liftdone as u8,242,90 as u8,243,90 as u8,0xA,0xD];
                i2c_crane.write(&mut cbuffer_w).unwrap_or_default();
            }
            24 =>{
        let mut cbuffer_w = [0x01,241,liftdone as u8,242,30 as u8,243,30 as u8,0xA,0xD];
        i2c_crane.write(&mut cbuffer_w).unwrap_or_default();
                // state is 24
                // jump to 11
            }
            _ =>{
                // everything else
            }
        }
        
// calculate motor angles
       // println!("{}",v);
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
        if current_mode != 13{ // if not in mode to drive
            if direction_rot > 20 {
                let mut buffer_w = [0x01,251,(va as u8)-10,252,(vb as u8)-10,253,(vc as u8)-10,0xA,0xD];
            i2c.write(&mut buffer_w).unwrap_or_default();
            }
            else if direction_rot < -20 {
                let mut buffer_w = [0x01,251,(va as u8)+10,252,(vb as u8)+10,253,(vc as u8)+10,0xA,0xD];
            i2c.write(&mut buffer_w).unwrap_or_default();
            }
            else{
                let mut buffer_w = [0x01,251,va as u8,252,vb as u8,253,vc as u8,0xA,0xD];
                i2c.write(&mut buffer_w).unwrap_or_default();
            }
        }
        //println!("{} {} {}", vc,va,vb);
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
    }
}

fn create_input_thread(tx_i16: Sender<String>,tx_i16_2: Sender<String>,tx_i32: Sender<String>,tx_str: Sender<String>){
    thread::spawn(move||{
        loop{
            println!("Enter command type: \ns -serial command\nd -set drive direction\nv -set drive velocity");
            let mut line = String::new();
            let mut line2 = String::new();
            let mut line3 = String::new();
            let mut line4 = String::new();
            let mut line5 = String::new();
/*
            std::io::stdin().read_line(&mut line).unwrap();

            if line.chars().nth(0).unwrap()>='d' && line.chars().nth(1).unwrap()<=' '{
                line.remove(0);
                line.remove(1);
                line.pop();
                tx_i16.send(line).expect("error sending value between threads");
            }
            else if line.chars().nth(0).unwrap()>='r' && line.chars().nth(1).unwrap()<=' '{
                line.remove(0);
                line.pop();
                tx_i16_2.send(line).expect("error sending value between threads");
            }
            else if line.chars().nth(0).unwrap()>='v' && line.chars().nth(1).unwrap()<=' '{
                line.remove(0);
                line.pop();
                tx_i32.send(line).expect("error sending value between threads");
            }
            else{
                tx_str.send(line).expect("error sending value between threads");
            }*/
             //old code
            std::io::stdin().read_line(&mut line).unwrap();
            if line.chars().nth(0).unwrap() == 's'{
                println!("Enter string to send over serial: ");
                std::io::stdin().read_line(&mut line2).unwrap();
                tx_str.send(line2).expect("error sending value between threads");
            }
            else if line.chars().nth(0).unwrap() == 'd'{
                println!("Enter direction value from 0 to 3600: ");
                std::io::stdin().read_line(&mut line3).unwrap();
                line3.pop();
                tx_i16.send(line3).expect("error sending value between threads");
            }
            else if line.chars().nth(0).unwrap() == 'r'{
                println!("Enter rotation value from 0 to 3600: ");
                std::io::stdin().read_line(&mut line5).unwrap();
                line5.pop();
                tx_i16_2.send(line5).expect("error sending value between threads");
            }
            else if line.chars().nth(0).unwrap() == 'v'{
                println!("Enter velocity value from 0 to 50: ");
                std::io::stdin().read_line(&mut line4).unwrap();
                line4.pop();
                tx_i32.send(line4).expect("error sending value between threads");
            }
        }
    });
}
extern crate serial;

use std::io;
use std::time::Duration;
use serial::prelude::*;
use clap::{App, Arg, ArgMatches};

fn print_hex_ascii(data: &[u8]) {
    for i in (0..data.len()).step_by(8) {
        let end = std::cmp::min(i + 8, data.len());
        let hex = &data[i..end].iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
        let ascii = &data[i..end].iter().map(|b| if *b >= 32 && *b < 127 { *b as char } else { '.' }).collect::<String>();
        println!("{}\t{}", hex, ascii);
    }
}

fn get_arguments() -> ArgMatches {
    App::new("UV-5R Tool")
        .version("0.1")
        .author("Bogdan Necula <bogdan@himeko.ro>")
        .about("A small tool to dump or upload the memory of Baofeng UV-5R radios")
        .arg(Arg::with_name("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("Sets the port number [eg. COM3 on Windows, /dev/ttyUSB0 on Linux]")
            .required(true))
        .arg(Arg::with_name("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("Sets the image file")
            .default_value("dump.img")
            .required(false))
        .arg(Arg::with_name("mode")
            .short('m')
            .long("mode")
            .value_name("MODE")
            .possible_values(&["dump", "upload"])
            .default_value("dump")
            .help("Sets the mode to dump or upload the image")
            .required(false))
        .get_matches()
}

fn dump_memory<T: SerialPort>(port: &mut T, input_file: &str) {
    let ident = init(port).unwrap();
    println!("Ident:");
    print_hex_ascii(&ident);
    println!();
        
    let mut memory = vec![];
    memory.extend_from_slice(&ident);

    // dump first part from 0x0000 to 0x1800
    for i in 0..0x60 {
        let chunk = read_block(port, i * 0x40, 0x40, i == 0).unwrap();
        println!("0x{:04x}", i * 0x40);
        print_hex_ascii(&chunk);
        println!();
        memory.extend_from_slice(&chunk);
    }

    // dump aux area from 0x1EC0 to 0x2000
    for i in 0x7B..0x80 {
        let chunk = read_block(port, i * 0x40, 0x40, i == 0).unwrap();
        println!("0x{:04x}", i * 0x40);
        print_hex_ascii(&chunk);
        println!();
        memory.extend_from_slice(&chunk);
    }
    std::fs::write(input_file, &memory).unwrap(); 
}

fn upload_memory<T: SerialPort>(port: &mut T, input_file: &str) {
    let ident = init(port).unwrap();
    println!("Ident:");
    print_hex_ascii(&ident);
    println!();

    let memory = std::fs::read(input_file).unwrap();

    // compare ident to the first 8 bytes in the file, and exit if they don't match
    if &memory[0..8] != &ident {
        eprintln!("Error: file does not match ident");
        return;
    }

    // check if size of memory is larger or equal to 0x1800
    if memory.len() < 0x1800 {
        eprintln!("Error: file is too small");
        return;
    }

    // upload first part from 0x0000 to 0x1800
    println!("Uploading main memory range...\n");
    for (i, chunk) in memory[0x0008..0x1808].chunks(0x10).enumerate() {
        let offset = i * 0x10;
        println!("0x{:04x}", offset);
        print_hex_ascii(chunk);

        let result = send_block(port, offset as u16, chunk);
        match result {
            Ok(true) => println!("OK\n"),
            Ok(false) => println!("FAILED\n"),
            Err(e) => {
                println!("Error sending block: {}", e);
                break;
            }
        }
    }

    if memory.len() >= 0x1948 {
        println!("Uploading aux memory range...\n");
        for (i, chunk) in memory[0x1808..0x1948].chunks(0x10).enumerate() {
            let offset = 0x1EC0 + i * 0x10;
            println!("0x{:04x}", offset);
            print_hex_ascii(chunk);

            let result = send_block(port, offset as u16, chunk);
            match result {
                Ok(true) => println!("OK\n"),
                Ok(false) => println!("FAILED\n"),
                Err(e) => {
                    println!("Error sending block: {}", e);
                    break;
                }
            }
        }
    }
    else {
        println!("Skipping aux memory range because it was not found in file...");
    }
}

fn configure_port<T: SerialPort>(port: &mut T) {
    match port.reconfigure(&|settings| {
        match settings.set_baud_rate(serial::Baud9600) {
            Ok(()) => {
            }
            Err(e) => {
                eprintln!("Error setting baud rate: {:?}", e);
            }
        }
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }) {
        Ok(()) => {
        }
        Err(e) => {
            eprintln!("Error reconfiguring port: {:?}", e);
        }
    }

    match port.set_timeout(Duration::from_millis(1000)) {
        Ok(()) => {
        }
        Err(e) => {
            eprintln!("Error setting timeout: {:?}", e);
        }
    }
}

fn main() {
    let matches = get_arguments();

    let port_num = matches.value_of("port").unwrap();
    let input_file = matches.value_of("file").unwrap();
    let mode = matches.value_of("mode").unwrap();

    println!("Port: {}", port_num);
    println!("Input file: {}", input_file);
    println!("Mode: {}", mode);
    println!();

    let mut port = serial::open(port_num).unwrap();
    configure_port(&mut port);

    if mode == "dump" {
        dump_memory(&mut port, input_file);
    } else {
        upload_memory(&mut port, input_file);
    }

    println!("Done!");
}

fn read_block<T: SerialPort>(port: &mut T, address: u16, size: u8, first_command: bool) -> io::Result<Vec<u8>> {
    let buf = vec![0x53, (address >> 8) as u8, (address & 0xff) as u8, size];
    port.write_all(&buf)?;
    if !first_command {
        let mut ack = [0; 1];
        port.read_exact(&mut ack).unwrap();
        if ack[0] != 0x06 {
            return Err(io::Error::new(io::ErrorKind::Other, "Radio refused to send second block"));
        }
    }

    let mut read_buf = vec![0; 4];
    port.read_exact(&mut read_buf).unwrap();

    if read_buf[0] != 0x58 {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid response to read block"));
    }

    let r_addr = ((read_buf[1] as u16) << 8) | (read_buf[2] as u16);
    if r_addr != address {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid address in read block response"));
    }

    let r_size = read_buf[3];
    if r_size != size {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid size in read block response"));
    }

    let mut read_buf = vec![0; usize::from(r_size)];
    port.read_exact(&mut read_buf).unwrap();

    let buf = [0x06];
    port.write_all(&buf)?;

    Ok(read_buf)
}

fn send_block<T: SerialPort>(port: &mut T, address: u16, data: &[u8]) -> io::Result<bool> {
    let buf = vec![0x58, (address >> 8) as u8, (address & 0xff) as u8, data.len() as u8];

    // combine buf and data into one block
    let buf = buf.into_iter().chain(data.iter().cloned()).collect::<Vec<_>>();
    port.write_all(&buf)?;

    let mut ack = [0; 1];
    port.read_exact(&mut ack)?;
    if ack[0] != 0x06 {
        Err(io::Error::new(io::ErrorKind::Other, "Radio refused to accept block"))
    } else {
        Ok(true)
    }
}

fn init<T: SerialPort>(port: &mut T) -> io::Result<Vec<u8>> {
    let magic = [0x50, 0xbb, 0xff, 0x20, 0x12, 0x07, 0x25];
    port.write_all(&magic)?;
    std::thread::sleep(std::time::Duration::from_millis(50));

    let mut ack = [0; 1];
    port.read_exact(&mut ack).unwrap();
    
    if ack[0] != 0x06 {
        return Err(io::Error::new(io::ErrorKind::Other, "ACK not received"));
    }

    let mut buf = [0x02];
    port.write_all(&buf)?;

    let mut read_buf = vec![0; 12];

    for i in 0..12 {
        port.read_exact(&mut buf)?;
        read_buf[i] = buf[0];
        if buf[0] == 0xdd {
            break;
        }
    }

    if read_buf.len() >= 8 && read_buf.len() <= 12 {
        let mut i = 0;
        for b in &read_buf {
            i += 1;
            if *b == 0xdd {
                break;
            }
        }
        read_buf.truncate(i);
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid length of ident"));
    }

    buf[0] = 0x06;
    port.write_all(&buf)?;

    port.read_exact(&mut buf)?;
    if buf[0] != 0x06 {
        return Err(io::Error::new(io::ErrorKind::Other, "Radio refused clone"));
    }

    Ok(read_buf)
}

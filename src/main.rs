use sysinfo::{System, SystemExt};

fn main() {
    get_basic_info_using_cupid();
    get_sysinfo();
}

fn get_sysinfo() {
    let system = System::new();
    let ram = system.free_memory();
    println!("System has {ram}KB total memory");
}

fn get_basic_info_using_cupid() {
    let information = cupid::master();
    println!("CPU information available: {:#?}", information);
    if let Some(information) = information {
        if information.sse4_2() {
            println!("SSE 4.2 Available");
        }
        if information.intel_64_bit_architecture() {
            println!("Intel 64 bit architecture");
        }

        if information.aesni() {
            println!("AES new instructions available");
        }
    }
}

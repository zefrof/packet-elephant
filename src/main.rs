use std::env;
use pcap::Device;

fn parse_args(args: &Vec<String>) {
	if args.contains(&"-l".to_string()) || args.contains(&"--list".to_string()) { 
		list_devices();
	} else {
		println!("Default device selected.");
	}
}

fn list_devices() {
	let device_list = Device::list().unwrap();
	for i in device_list.iter() {
		println!("{:?}", i);
	}
}

fn main() {
   // let mut cap = Device::lookup().unwrap().open().unwrap();

	let args: Vec<String> = env::args().collect();
	parse_args(&args);	

   //let mut cap = Device::open(look).unwrap();
	/* 
	while let Ok(packet) = cap.next() {
        println!("received packet! {:?}", packet);
    } */
}
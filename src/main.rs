fn main() {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id()
        );

        if device_desc.vendor_id() == 11720 && device_desc.product_id() == 36885 {
            for config_desc_index in 0..device_desc.num_configurations() {
                let config_desc = match device.config_descriptor(config_desc_index) {
                    Ok(c) => c,
                    Err(_) => continue
                };

                for interface in config_desc.interfaces() {
                    for interface_desc in interface.descriptors() {
                        for endpoint_desc in interface_desc.endpoint_descriptors() {
                            println!("Address: {}", endpoint_desc.address());
                            println!("Direction: {:?}", endpoint_desc.direction());
                            println!("Transfer Type: {:?}", endpoint_desc.transfer_type());
                            println!("Sync Type: {:?}", endpoint_desc.sync_type());
                            println!("Usage Type: {:?}", endpoint_desc.usage_type());

                            if endpoint_desc.direction() == rusb::Direction::In {
                                let mut device_handler = match device.open() {
                                    Ok(h) => h,
                                    Err(_) => continue
                                };

                                match device_handler.kernel_driver_active(interface_desc.interface_number()) {
                                    Ok(r) => {
                                        if r {
                                            match device_handler.detach_kernel_driver(interface_desc.interface_number()) {
                                                Ok(_) => println!("Detached kernel driver"),
                                                Err(err) => println!("Failed to detach kernel driver: {:?}", err)
                                            }
                                        } else {
                                            println!("No kernel driver attached");
                                        }
                                    },
                                    Err(err) => println!("Failed to determine if a kerne driver is active: {}", err)
                                }

                                match device_handler.claim_interface(interface_desc.interface_number()) {
                                    Ok(_) => println!("Claimed interface"),
                                    Err(err) => println!("Failed to claim interface: {:?}", err)
                                };

                                let mut buf: [u8; 8] = [
                                    0, 0, 0, 0, 0, 0, 0, 0
                                ];

                                match device_handler.read_interrupt(
                                    endpoint_desc.address(),
                                    &mut buf,
                                    std::time::Duration::from_millis(5000)
                                ) {
                                    Ok(s) => println!("Size: {}", s),
                                    Err(err) => println!("Failed to read interrupt: {:?}", err)
                                };

                                println!("Buffer: {:?}", buf);
                            }
                        }
                    }
                }
            }
        } else {
            println!("Device does not match search.");
        }
    }
}

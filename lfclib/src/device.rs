use consts;
use std::time::Duration;

pub struct LuxaforDeviceDescriptor {
    pub vendor_id  : u16,
    pub product_id : u16
}

pub struct LuxaforContext {
    usb_context : ::libusb::Context
}

pub struct LuxaforDevice<'a> {
    usb_device : ::libusb::Device<'a>
}

impl<'a> LuxaforDevice<'a> {
    pub fn new(device : ::libusb::Device) -> ::libusb::Result<LuxaforDevice> {
        Ok(LuxaforDevice {
            usb_device: device
        })
    }

    fn prepare_device(&self) -> ::libusb::Result<(::libusb::DeviceHandle, u8)>
    {
        let mut device = self.usb_device.open()?;

        device.reset()?;

        let had_kern_driver = device.kernel_driver_active(0)?;
        if had_kern_driver {
            println!("kernel driver");
            device.detach_kernel_driver(0)?;
        }

        for n in 0..self.usb_device.device_descriptor()?.num_configurations() {
            let config = self.usb_device.config_descriptor(n)?;

            for interface in config.interfaces() {
                for desc in interface.descriptors() {
                    for endpoint in desc.endpoint_descriptors() {
                        if endpoint.direction() == ::libusb::Direction::In {
                            device.set_active_configuration(config.number())?;
                            device.claim_interface(desc.interface_number())?;
                            device.set_alternate_setting(desc.interface_number(), desc.setting_number())?;
                            return Ok((device, endpoint.number()));
                        }
                    }
                }
            }
        }
        Err(::libusb::Error::Io)
    }

    pub fn solid(&self, r : u8, g : u8, b : u8) -> ::libusb::Result<()> {
        let (device, address) = self.prepare_device()?;

        device.write_interrupt(address, &[consts::mode::STATIC, consts::led::ALL, r, g, b], Duration::from_secs(5))?;

        Ok(())
    }
}

impl LuxaforContext {
    pub fn new() -> ::libusb::Result<LuxaforContext> {
        Ok(LuxaforContext {
            usb_context: ::libusb::Context::new()?
        })
    }

    pub fn devices(&self, expected : LuxaforDeviceDescriptor) -> ::libusb::Result<Vec<LuxaforDevice>> {
        self.usb_context
            .devices()?
            .iter()
            .filter(|d| {
                let descriptor = d.device_descriptor();

                if ! descriptor.is_ok() {
                    return false;
                }

                let descriptor = descriptor.unwrap();

                descriptor.vendor_id() == expected.vendor_id &&
                    descriptor.product_id() == expected.product_id
            })
            .map(|d| LuxaforDevice::new(d))
            .collect()
    }
}
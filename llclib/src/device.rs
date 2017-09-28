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
    fn new(device : ::libusb::Device) -> ::libusb::Result<LuxaforDevice> {
        Ok(LuxaforDevice {
            usb_device: device
        })
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
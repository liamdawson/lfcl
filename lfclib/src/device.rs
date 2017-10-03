use consts;
use hidapi::{HidResult, HidApi, HidDevice};

pub struct LuxaforDeviceDescriptor {
    pub vendor_id  : u16,
    pub product_id : u16
}

pub struct LuxaforContext {
    hid_api : HidApi
}

pub struct LuxaforDevice<'a> {
    hid_device : HidDevice<'a>
}

impl LuxaforContext {
    pub fn new() -> HidResult<LuxaforContext> {
        Ok(LuxaforContext {
            hid_api: HidApi::new()?
        })
    }

    pub fn open_device(&self, device_descriptor: LuxaforDeviceDescriptor) -> HidResult<LuxaforDevice> {
        LuxaforDevice::new(self.hid_api.open(device_descriptor.vendor_id, device_descriptor.product_id)?)
    }
}

impl<'a> LuxaforDevice<'a> {
    pub fn new(device: HidDevice) -> HidResult<LuxaforDevice> {
        Ok(LuxaforDevice {
            hid_device: device
        })
    }

    pub fn solid(self, r: u8, g: u8, b: u8) -> HidResult<usize> {
        self.hid_device.write(&[consts::mode::STATIC, consts::led::ALL, r, g, b])
    }
}

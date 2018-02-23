use bindings::*;
use result::*;
use frame;
use contact;

pub struct DeviceList(SenselDeviceList);

pub struct Device {
    handle: *mut ::std::os::raw::c_void,
    frame_data: *mut SenselFrameData,
    pub id: SenselDeviceID,
    pub sensor_info: SenselSensorInfo,
    pub fw_info: SenselFirmwareInfo,
    pub num_leds: u8,
    pub max_led_brightness: u16,
    pub supported_frame_content: frame::Mask
}

pub fn get_device_list() -> Result<DeviceList, SenselError> {
    unsafe {
        let mut list = ::std::mem::zeroed();
        sensel_result(senselGetDeviceList(&mut list))
            .and(Ok(list.into()))
    }
}

impl SenselSensorInfo {
    pub fn get_num_sensors(&self) -> usize {
        self.num_rows as usize * self.num_cols as usize
    }
}

impl DeviceList {
    pub fn as_slice(&self) -> &[SenselDeviceID] {
        &self.0.devices[..self.0.num_devices as usize]
    }
}

impl Into<DeviceList> for SenselDeviceList {
    fn into(self) -> DeviceList {
        DeviceList(self)
    }
}

impl SenselDeviceID {
    pub fn get_serial_num(&self) -> &str {
        ::std::str::from_utf8(&self.serial_num).unwrap()
    }
    pub fn get_com_port(&self) -> &str {
        ::std::str::from_utf8(&self.com_port).unwrap()
    }
    pub fn open(self) -> Result<Device, SenselError> {
        unsafe {
            let mut handle = ::std::mem::zeroed();
            let mut fw_info = ::std::mem::zeroed();
            let mut sensor_info = ::std::mem::zeroed();
            let mut frame_data = ::std::mem::zeroed();
            let mut num_leds = ::std::mem::zeroed();
            let mut max_led_brightness = ::std::mem::zeroed();
            let mut supported_frame_content = ::std::mem::zeroed();

            sensel_result(senselOpenDeviceByID(&mut handle, self.idx))
                .and_then(|_| sensel_result(senselGetFirmwareInfo(handle, &mut fw_info)))
                .and_then(|_| sensel_result(senselGetSensorInfo(handle, &mut sensor_info)))
                .and_then(|_| sensel_result(senselGetNumAvailableLEDs(handle, &mut num_leds)))
                .and_then(|_| sensel_result(senselGetMaxLEDBrightness(handle, &mut max_led_brightness)))
                .and_then(|_| sensel_result(senselGetSupportedFrameContent(handle, &mut supported_frame_content)))
                .and_then(|_| sensel_result(senselAllocateFrameData(handle, &mut frame_data)))
                .and(Ok(Device {
                    handle,
                    frame_data,
                    id: self,
                    sensor_info,
                    fw_info,
                    num_leds,
                    max_led_brightness,
                    supported_frame_content: frame::Mask::from_bits_truncate(supported_frame_content)
                }))
        }
    }
}

impl Device {
    pub fn soft_reset(&self) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSoftReset(self.handle))
        }
    }

    pub fn get_power_button_pressed(&self) -> Result<bool, SenselError> {
        let mut pressed = 0;
        unsafe {
            sensel_result(senselGetPowerButtonPressed(self.handle, &mut pressed))
                .and(Ok(pressed != 0))
        }
    }

    pub fn set_led_brightness(&self, led_id: u8, brightness: u16) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSetLEDBrightness(self.handle, led_id, brightness))
        }
    }
    pub fn get_led_brightness(&self, led_id: u8) -> Result<u16, SenselError> {
        let mut brightness = 0;
        unsafe {
            sensel_result(senselGetLEDBrightness(self.handle, led_id, &mut brightness))
                .and(Ok(brightness))
        }
    }

    pub fn set_scan_mode(&self, mode: SenselScanMode) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSetScanMode(self.handle, mode))
        }
    }
    pub fn get_scan_mode(&self) -> Result<SenselScanMode, SenselError> {
        let mut mode = SenselScanMode::SCAN_MODE_DISABLE;
        unsafe {
            sensel_result(senselGetScanMode(self.handle, &mut mode))
                .and(Ok(mode))
        }
    }

    pub fn set_scan_detail(&self, detail: SenselScanDetail) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSetScanDetail(self.handle, detail))
        }
    }
    pub fn get_scan_detail(&self) -> Result<SenselScanDetail, SenselError> {
        let mut detail = SenselScanDetail::SCAN_DETAIL_UNKNOWN;
        unsafe {
            sensel_result(senselGetScanDetail(self.handle, &mut detail))
                .and(Ok(detail))
        }
    }

    pub fn set_buffer_control(&self, num: u8) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSetBufferControl(self.handle, num))
        }
    }
    pub fn get_buffer_control(&self) -> Result<u8, SenselError> {
        let mut num = 0;
        unsafe {
            sensel_result(senselGetBufferControl(self.handle, &mut num))
                .and(Ok(num))
        }
    }

    pub fn set_max_frame_rate(&self, val: u16) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSetMaxFrameRate(self.handle, val))
        }
    }
    pub fn get_max_frame_rate(&self) -> Result<u16, SenselError> {
        let mut val = 0;
        unsafe {
            sensel_result(senselGetMaxFrameRate(self.handle, &mut val))
                .and(Ok(val))
        }
    }

    pub fn set_frame_content(&self, mask: frame::Mask) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSetFrameContent(self.handle, mask.bits()))
        }
    }
    pub fn get_frame_content(&self) -> Result<frame::Mask, SenselError> {
        let mut mask = 0;
        unsafe {
            sensel_result(senselGetFrameContent(self.handle, &mut mask))
                .and(Ok(frame::Mask::from_bits_truncate(mask)))
        }
    }

    pub fn set_contacts_mask(&self, mask: contact::Mask) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSetContactsMask(self.handle, mask.bits()))
        }
    }
    pub fn get_contacts_mask(&self) -> Result<contact::Mask, SenselError> {
        let mut mask = 0;
        unsafe {
            sensel_result(senselGetContactsMask(self.handle, &mut mask))
                .and(Ok(contact::Mask::from_bits_truncate(mask)))
        }
    }

    pub fn set_contacts_min_force(&self, val: u16) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSetContactsMinForce(self.handle, val))
        }
    }
    pub fn get_contacts_min_force(&self) -> Result<u16, SenselError> {
        let mut val = 0;
        unsafe {
            sensel_result(senselGetContactsMinForce(self.handle, &mut val))
                .and(Ok(val))
        }
    }

    pub fn set_contacts_enable_blob_merge(&self, val: bool) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSetContactsEnableBlobMerge(self.handle, val as u8))
        }
    }
    pub fn get_contacts_enable_blob_merge(&self) -> Result<bool, SenselError> {
        let mut val = 0;
        unsafe {
            sensel_result(senselGetContactsEnableBlobMerge(self.handle, &mut val))
                .and(Ok(val != 0))
        }
    }

    pub fn set_dynamic_baseline_enabled(&self, val: bool) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSetDynamicBaselineEnabled(self.handle, val as u8))
        }
    }
    pub fn get_dynamic_baseline_enabled(&self) -> Result<bool, SenselError> {
        let mut val = 0;
        unsafe {
            sensel_result(senselGetDynamicBaselineEnabled(self.handle, &mut val))
                .and(Ok(val != 0))
        }
    }

    pub fn start_scanning(&self) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselStartScanning(self.handle))
        }
    }
    pub fn stop_scanning(&self) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselStopScanning(self.handle))
        }
    }

    pub fn read_sensor(&self) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselReadSensor(self.handle))
        }
    }
    pub fn get_num_available_frames(&self) -> Result<usize, SenselError> {
        let mut num_frames = 0;
        unsafe {
            sensel_result(senselGetNumAvailableFrames(self.handle, &mut num_frames))
                .and(Ok(num_frames as usize))
        }
    }
    pub fn get_frame(&self) -> Result<frame::Frame, SenselError> {
        unsafe {
            sensel_result(senselGetFrame(self.handle, self.frame_data))
                .and(Ok(frame::from_frame_data(*self.frame_data, self.sensor_info)))
        }
    }
    pub fn close(self) -> () {
        // move and drop the device
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            // always close on drop to prevent memory leaks
            // never throws error
            senselClose(self.handle);
        }
    }
}
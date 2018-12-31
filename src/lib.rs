#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//TODO: Research implementation of creating a client for kinesis
fn newCreateKinesisVideoClient() {
    unsafe {
//        let device_info = __DeviceInfo {
//            version: 1,
//            name: "new stream",
//            tagCount: 2,
//            streamCount: 1
//        };
//
//        let p_device_info = PDeviceInfo = device_info as *mut __DeviceInfo;
//        createKinesisVideoClient();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

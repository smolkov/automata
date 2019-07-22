use serde_derive::{Deserialize, Serialize};



pub async fn get_local() -> Result<Status,WqaError> {
    let device = Device::load_no_fallback(store::root_directory().join("device.ron"))?;
    Ok(device)
}
pub async fn set_local(device: Device) -> Result<(), WqaError> {
    device.write(store::root_directory().join("device.ron"))?;
    Ok(())
}
// pub async fn from_git() -> Result<Device,WqaError> {
//     Ok(Device::default())
// }

pub async fn set_serial( serial: String ) -> Result<(),WqaError> {
    let mut device = get_local().await?;
    device.set_serial(serial);
    device.write(store::root_directory().join("device.ron"))?;
    Ok(())
}
//

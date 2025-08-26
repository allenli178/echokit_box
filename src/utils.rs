use esp_idf_svc::nvs::{EspNvs, NvsDefault};
pub fn get_nvs_string(nvs: &EspNvs<NvsDefault>, key: &str, buffer: &mut [u8]) -> Option<String> {
    nvs.get_str(key, buffer)
        .map_err(|e| log::error!("Failed to get {}: {:?}", key, e))
        .ok()
        .flatten()
        .map(|s| s.to_string())
}

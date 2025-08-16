// Define the data model for IoT devices
#[derive(Debug, Serialize, Deserialize)]
struct IoTDevice {
    id: String,
    name: String,
    device_type: String,
    location: String,
    last_seen: DateTime<Utc>,
    status: DeviceStatus,
    sensors: Vec<Sensor>,
}

// Define the data model for sensors
#[derive(Debug, Serialize, Deserialize)]
struct Sensor {
    id: String,
    name: String,
    sensor_type: String,
    value: f64,
    unit: String,
    last_updated: DateTime<Utc>,
}

// Define the data model for device status
#[derive(Debug, Serialize, Deserialize)]
enum DeviceStatus {
    Online,
    Offline,
    Unknown,
}

// Define the data model for the decentralized monitor
#[derive(Debug, Serialize, Deserialize)]
struct DecentralizedMonitor {
    devices: Vec<IoTDevice>,
    network_id: String,
    network_name: String,
}

// Implement the DecentralizedMonitor struct
impl DecentralizedMonitor {
    fn new(network_id: String, network_name: String) -> Self {
        DecentralizedMonitor {
            devices: vec![],
            network_id,
            network_name,
        }
    }

    fn add_device(&mut self, device: IoTDevice) {
        self.devices.push(device);
    }

    fn get_device(&self, id: &str) -> Option<&IoTDevice> {
        self.devices.iter().find(|device| device.id == id)
    }

    fn update_device_status(&mut self, id: &str, status: DeviceStatus) {
        if let Some(device) = self.devices.iter_mut().find(|device| device.id == id) {
            device.status = status;
        }
    }
}
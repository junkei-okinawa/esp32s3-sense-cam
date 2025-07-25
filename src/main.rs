use log::info;
use anyhow::Result;

use esp_idf_svc::hal::peripherals::Peripherals;
use esp32s3_sense_cam::espcam::Camera;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    info!("Starting ESP32-S3 Sense Camera Example");

    let peripherals = Peripherals::take().unwrap();

    let camera = Camera::new(
        peripherals.pins.gpio10, // XCLK
        peripherals.pins.gpio48, // Y9
        peripherals.pins.gpio11, // Y8
        peripherals.pins.gpio12, // Y7
        peripherals.pins.gpio14, // Y6
        peripherals.pins.gpio16, // Y5
        peripherals.pins.gpio18, // Y4
        peripherals.pins.gpio17, // Y3
        peripherals.pins.gpio15, // Y2
        peripherals.pins.gpio38, // VSYNC
        peripherals.pins.gpio47, // HREF
        peripherals.pins.gpio13, // PCLK
        peripherals.pins.gpio40, // SIOD
        peripherals.pins.gpio39, // SIOC
        esp_idf_sys::camera::pixformat_t_PIXFORMAT_JPEG,
        // esp_idf_sys::camera::pixformat_t_PIXFORMAT_GRAYSCALE,
        // esp_idf_sys::camera::framesize_t_FRAMESIZE_UXGA,
        esp_idf_sys::camera::framesize_t_FRAMESIZE_QQVGA, // 160x120
    )
    .unwrap();

    info!("Camera initialized successfully");

    loop {
        info!("Waiting for camera frame...");
        let framebuffer = camera.get_framebuffer();

        if let Some(framebuffer) = framebuffer {
            info!("Got framebuffer!");
            info!("width: {}", framebuffer.width());
            info!("height: {}", framebuffer.height());
            info!("len: {}", framebuffer.data().len());
            info!("format: {}", framebuffer.format());

            std::thread::sleep(std::time::Duration::from_millis(1000));
        } else {
            info!("no framebuffer");
        }
    }
}

use std::time::Duration;
use std::sync::{Arc, Mutex};

use anyhow::{bail, Result};

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::io::{EspIOError, Write},
    hal::peripherals::Peripherals,
    http::{server::EspHttpServer, Method},
};

use esp32s3_sense_cam::{config::get_config, espcam::Camera, wifi_handler::my_wifi};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let sysloop = EspSystemEventLoop::take()?;

    let peripherals = Peripherals::take().unwrap();

    let config = get_config();

    let _wifi = match my_wifi(
        config.wifi_ssid,
        config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => inner,
        Err(err) => {
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    let camera = Arc::new(Mutex::new(Camera::new(
        peripherals.pins.gpio10, // XCLK
        peripherals.pins.gpio15, // d0 (Y2)
        peripherals.pins.gpio17, // d1 (Y3)
        peripherals.pins.gpio18, // d2 (Y4)
        peripherals.pins.gpio16, // d3 (Y5)
        peripherals.pins.gpio14, // d4 (Y6)
        peripherals.pins.gpio12, // d5 (Y7)
        peripherals.pins.gpio11, // d6 (Y8)
        peripherals.pins.gpio48, // d7 (Y9)
        peripherals.pins.gpio38, // VSYNC
        peripherals.pins.gpio47, // HREF
        peripherals.pins.gpio13, // PCLK
        peripherals.pins.gpio40, // SIOD
        peripherals.pins.gpio39, // SIOC
        esp_idf_sys::camera::pixformat_t_PIXFORMAT_JPEG,
        esp_idf_sys::camera::framesize_t_FRAMESIZE_UXGA, // 1600x1200
    ).unwrap()));

    let mut server = EspHttpServer::new(&esp_idf_svc::http::server::Configuration::default())?;

    server.fn_handler("/", Method::Get, |request| {
        let mut response = request.into_ok_response()?;
        response.write_all("ok".as_bytes())?;
        Ok::<(), EspIOError>(())
    })?;

    let camera_jpg = camera.clone();
    server.fn_handler("/camera.jpg", Method::Get, move |request| {
        log::info!("camera.jpg requested");
        let camera = camera_jpg.lock().unwrap();
        camera.get_framebuffer();
        let framebuffer = camera.get_framebuffer();

        if let Some(framebuffer) = framebuffer {
            log::info!("Got framebuffer! len={}", framebuffer.data().len());
            let data = framebuffer.data();

            let headers = [
                ("Content-Type", "image/jpeg"),
                ("Content-Length", &data.len().to_string()),
            ];
            let mut response = request.into_response(200, Some("OK"), &headers).unwrap();
            response.write_all(data)?;
        } else {
            log::warn!("No framebuffer!");
            let mut response = request.into_ok_response()?;
            response.write_all("no framebuffer".as_bytes())?;
        }

        Ok::<(), EspIOError>(())
    })?;

    let camera_mjpeg = camera.clone();
    server.fn_handler("/camera.mjpeg", Method::Get, move |request| {
        log::info!("camera.mjpeg requested");

        let headers = [
            ("Content-Type", "multipart/x-mixed-replace; boundary=frame"),
        ];
        let mut response = request.into_response(200, Some("OK"), &headers).unwrap();

        loop {
            let camera = camera_mjpeg.lock().unwrap();
            camera.get_framebuffer();
            if let Some(framebuffer) = camera.get_framebuffer() {
                let data = framebuffer.data();
                let frame_header = format!(
                    "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                    data.len()
                );
                response.write_all(frame_header.as_bytes())?;
                response.write_all(data)?;
                response.write_all(b"\r\n")?;
            } else {
                log::warn!("No framebuffer!");
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        response.write_all(b"--frame--\r\n")?;

        Ok::<(), EspIOError>(())
    })?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

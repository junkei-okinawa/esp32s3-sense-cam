
# xiao esp32s3 sense Rust Webカメラ

xiao esp32s3 senseで動作するRust実装（esp-rs）のWebカメラです。
Webサーバ経由でカメラ映像（Motion JPEG動画）を配信します。

## 前提条件

- rustc 1.86.0-nightly
- espflash 3.3.0
- esp-idf 5.1.3

## セットアップ手順

1. リポジトリをクローン
    ```bash
    git clone <your-repo-url>
    ```
2. esp32-cameraコンポーネントをダウンロード
    ```bash
    git submodule update --init
    ```
3. WiFi設定ファイルをコピーし、あなたのWiFi情報に書き換えてください
    ```bash
    cp src/wifi_config.rs.example src/wifi_config.rs
    # src/wifi_config.rs を編集し、SSIDとパスワードを設定
    ```
4. webserverをビルド＆書き込み
    ```bash
    cargo espflash flash --port <device port> --partition-table partitions.csv --monitor --example webserver
    ```

## 動画の視聴方法

1. 書き込み後、シリアルモニタやログに
    ```
    esp32s3_sense_cam::wifi_handler: Wifi DHCP info: IpInfo { ip: <camera device ip address> ...}
    ```
    のようにカメラデバイスのIPアドレスが表示されます。
2. ブラウザで `http://<ip_address>/camera.mjpeg` にアクセスすると、Webカメラの動画（Motion JPEG）が表示されます。

## Reference
esp32-camera のピン指定や PSRAM 有効化のために sdkconfig.defaults で指定する key value がわかりづらく苦労しました。
- [Seeed Studio XIAO ESP32S3 Senseでのカメラ使用法](https://wiki.seeedstudio.com/ja/xiao_esp32s3_camera_usage/)
- [esp-rs/esp-idf-sys issue#177](https://github.com/esp-rs/esp-idf-sys/issues/177)
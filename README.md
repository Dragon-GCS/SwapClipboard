# Switch Clip Board

An simple server to switch clip board between windows and ios.

## Usage

1. Build the project by `cargo build --release`
2. Run the server by `./target/release/scb.exe`
3. Set the [shortcut](!https://www.icloud.com/shortcuts/b1a42623d97c4b85ad46628c2699abc2) on your ios device.

>Optional:
>
> - run the server in background:
> `Start-Process -FilePath D:\Projects\Rust\target_dir\release\air-drop.exe -WindowStyle Hidden`
> - Add automation to the shortcut when touch NFC.

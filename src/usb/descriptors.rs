pub const DEV_DESC: [u8; 18] = [
    0x12,
    0x01,
    0x00,
    0x02,
    0x00,
    0x00,
    0x00,
    0x40,
    0xff,
    0xff,
    0xff,
    0xff,
    0x01,
    0x00,
    0x01,
    0x02,
    0x03,
    0x01,
];

pub const CONF_DESC: [u8; 34] = [
    0x09,
    0x02,
    0x22,
    0x00,
    0x01,
    0x01,
    0x04,
    0x80,
    0xfa,
    0x09,
    0x04,
    0x00,
    0x00,
    0x01,
    0x03,
    0x01,
    0x01,
    0x05,
    0x09,
    0x21,
    0x11,
    0x01,
    0x00,
    0x01,
    0x22,
    0x29,
    0x00,
    0x07,
    0x05,
    0x81,
    0x03,
    0x40,
    0x00,
    0x01,
];

pub const HID_REPORT_DESC: [u8; 41] = [
    0x05, 0x01, // Usage Page: Generic Desktop Controls
    0x09, 0x06, // Usage: Keyboard
    0xa1, 0x01, // Collection: Application
    0x85, 0x01, //   Report ID: 1
    0x05, 0x07, //   Usage Page: Keybaord
    0x75, 0x01, //   Report Size: 1
    0x95, 0x08, //   Report Count: 8
    0x19, 0xe0, //   Usage Minimum: Keyboard LeftControl
    0x29, 0xe7, //   Usage Maximum: Keyboard Right GUI
    0x15, 0x00, //   Logical Minimum: 0
    0x25, 0x01, //   Logical Maximum: 1
    0x81, 0x02, //   Input: Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position
    0x95, 0x03, //   Report Count (3)
    0x75, 0x08, //   Report Size (8)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x64, //   Logical Maximum (100)
    0x05, 0x07, //   Usage Page (Kbrd/Keypad)
    0x19, 0x00, //   Usage Minimum (0x00)
    0x29, 0x65, //   Usage Maximum (0x65)
    0x81, 0x00, //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0xC0,       // End Collection
];

pub const DEVICE_QUALIFIER: [u8; 10] = [0x0a, 0x06, 0x00, 0x02, 0x00, 0x00, 0x40, 0x01, 0x00, 0x00];

pub const LANG_STR: [u8; 4] = [0x04, 0x03, 0x09, 0x04];

// String Descriptor 1, "Rusty Manufacturer"
pub const MANUFACTURER_STR: [u8; 38] = [
    0x26, 0x03, //
    0x52, 0x00, // R
    0x75, 0x00, // u
    0x73, 0x00, // s
    0x74, 0x00, // t
    0x79, 0x00, // y
    0x20, 0x00, //  
    0x4d, 0x00, // M
    0x61, 0x00, // a
    0x6e, 0x00, // n
    0x75, 0x00, // u
    0x66, 0x00, // f
    0x61, 0x00, // a
    0x63, 0x00, // c
    0x74, 0x00, // t
    0x75, 0x00, // u
    0x72, 0x00, // r
    0x65, 0x00, // e
    0x72, 0x00, // r
];

// String Descriptor 2, "Rusty Product"
pub const PRODUCT_STR: [u8; 28] = [
    0x1c, 0x03, //
    0x52, 0x00, // R
    0x75, 0x00, // u
    0x73, 0x00, // s
    0x74, 0x00, // t
    0x79, 0x00, // y
    0x20, 0x00, //  
    0x50, 0x00, // P
    0x72, 0x00, // r
    0x6f, 0x00, // o
    0x64, 0x00, // d
    0x75, 0x00, // u
    0x63, 0x00, // c
    0x74, 0x00, // t
];

pub const SERIAL_NUMBER_STR: [u8; 14] = [
    0x0e, 0x03, //
    0x31, 0x00, // 1
    0x32, 0x00, // 2
    0x33, 0x00, // 3
    0x41, 0x00, // A
    0x42, 0x00, // B
    0x43, 0x00, // C
];

pub const CONF_STR: [u8; 40] = [
    0x28,
    0x03,
    0x52,
    0x00,
    0x75,
    0x00,
    0x73,
    0x00,
    0x74,
    0x00,
    0x79,
    0x00,
    0x20,
    0x00,
    0x43,
    0x00,
    0x6f,
    0x00,
    0x6e,
    0x00,
    0x66,
    0x00,
    0x69,
    0x00,
    0x67,
    0x00,
    0x75,
    0x00,
    0x72,
    0x00,
    0x61,
    0x00,
    0x74,
    0x00,
    0x69,
    0x00,
    0x6f,
    0x00,
    0x6e,
    0x00,
];

use rand::Rng;
use qrcodegen::{QrCode, QrCodeEcc};
use std::fs::write;

//Generates uuid v4, lowercase without dashes
pub fn uuid() -> Result<String, UUIDError> {
    Ok(rnd_uuid()?.join("")) 
}

//Generates uuid v4, CAPITAL case without dashes
pub fn uuid_uppercase() -> Result<String, UUIDError> {
    Ok(to_capital(rnd_uuid()?.join(""))) 
}

//Generates uuid v4 in dashed form low case
pub fn uuid_dashed() -> Result<String, UUIDError> {
    let mut uuid_vec = rnd_uuid()?; 
    uuid_vec.insert(4, "-".to_string());
    uuid_vec.insert(7, "-".to_string());
    uuid_vec.insert(10, "-".to_string());
    uuid_vec.insert(13, "-".to_string()); 
    Ok(uuid_vec.join(""))
}

//Generates uuid v4 in dashed form, CAPITAL case
pub fn uuid_dashed_uppercase() -> Result<String, UUIDError> {
    let mut uuid_vec = rnd_uuid()?; 
    uuid_vec.insert(4, "-".to_string());
    uuid_vec.insert(7, "-".to_string());
    uuid_vec.insert(10, "-".to_string());
    uuid_vec.insert(13, "-".to_string()); 
    Ok(to_capital(uuid_vec.join("")))
}

//Generates qr-code of the uuid v4 and writes it as a svg file "uuid".svg 
pub fn to_svg(uuid: String) -> Result<(), UUIDError> {
    match QrCode::encode_text(&uuid[..], QrCodeEcc::Medium) {
        Ok(uuid_qrcode) => {
            let uuid_qrcode_svg = to_svg_string(&uuid_qrcode, 4);
            write(format!("{}.svg", &uuid), &uuid_qrcode_svg)?;
        }
        Err(e) => eprintln!("{}", e.to_string()),
    };
    Ok(())
}

//Generates random RFC4122 version 4 UUID as Vec<String>
fn rnd_uuid() -> Result<Vec<String>, UUIDError> {
    let uuid_vec = vec![
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        to_four(u8_hex_rnd().as_str())?,
        u8_hex_rnd(),
        to_two(u8_hex_rnd().as_str())?,
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
    ];
    Ok(uuid_vec)
}

//Returns a pseudo random hex integer over the u8 range 0..=255 as a double character String
fn u8_hex_rnd() -> String {
    let mut rng = rand::thread_rng();

    //Pseudo random integer over the u8 range 0..=255
    let u: u8 = rng.gen();
    let mut h = format!("{:x}", &u);

    //Adds 0 to numbers generated within the range 0..=15
    if u < 16 {
        h.push('0');
        h = h.chars().rev().collect::<String>();
    }
    h
}

//*****************************************************************************
//Helper functions to convert pseudo random uuid-like string to a valid uuid v4 
fn to_four(s: &str) -> Result<String, UUIDError> {
    let n: u32 = u32::from_str_radix(s, 16)?;   
    Ok(format!("{:x}", 64 + n - ((n >> 4) << 4))) 
}

fn to_two(s: &str) -> Result<String, UUIDError> {
    let n: u32 = u32::from_str_radix(s, 16)?;
    Ok(format!("{:x}", 128 + n - ((n >> 6) << 6)))
}
//*****************************************************************************

//Converts lowercase string to CAPITAL case string
fn to_capital(s: String) -> String {
    s.to_ascii_uppercase()
}

//*****************************************************************************
//Custom error
#[derive(Debug)]
pub enum UUIDError { 
    ParseIntError(std::num::ParseIntError),
    IOError(std::io::Error),   
}

impl From<std::num::ParseIntError> for UUIDError {
    fn from(error: std::num::ParseIntError) -> Self {
        UUIDError::ParseIntError(error)
    }
}

impl From<std::io::Error> for UUIDError {
    fn from(error: std::io::Error) -> Self {
        UUIDError::IOError(error)
    }
}
//*****************************************************************************

//*****************************************************************************
// From: https://github.com/nayuki/QR-Code-generator/blob/master/rust/examples/qrcodegen-demo.rs
// Returns a string of SVG code for an image depicting
// the given QR Code, with the given number of border modules.
// The string always uses Unix newlines (\n), regardless of the platform. 

fn to_svg_string(qr: &QrCode, border: i32) -> String {
    assert!(border >= 0, "Border must be non-negative");
    let mut result = String::new();
    result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
    let dimension = qr
        .size()
        .checked_add(border.checked_mul(2).unwrap())
        .unwrap()
        ;
    result += &format!(
		"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
    result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
    result += "\t<path d=\"";
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    result += " ";
                }
                result += &format!("M{},{}h1v1h-1z", x + border, y + border);
            }
        }
    }
    result += "\" fill=\"#000000\"/>\n";
    result += "</svg>\n";
    result
}
//*****************************************************************************
//End of code
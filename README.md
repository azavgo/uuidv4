## Library to generate version 4 (random based) UUID strings 

### Features: 
1. Generates psuedo random RFC4122 version 4 UUID string;  
1. UUID v4 string supported formats: 
    * Low case, *e.g.* 2ac166ea469342a1b5e6dc6fc7af4985. Function signature: `pub fn uuid() -> Result<String, UUIDError>`
    * Upper case, *e.g.* D529FFFC30A64873816260D77944B4FF. Function signature: `pub fn uuid_uppercase() -> Result<String, UUIDError>`
    * Low case with dashes, *e.g.* cf57a620-4042-4e6a-a93b-6f44ae48935f. Function signature: `pub fn uuid_dashed() -> Result<String, UUIDError>`
    * Upper case with dashes, *e.g.* 3C1EE587-3D25-4095-807C-918848243E32. Function signature: `pub fn uuid_dashed_uppercase() -> Result<String, UUIDError>`
1. UUID QR code svg file is generated using **qrcodegen** crate (https://crates.io/crates/qrcodegen) and **svg_to_string** function from https://github.com/nayuki/QR-Code-generator/blob/master/rust/examples/qrcodegen-demo.rs. Function signature: `pub fn to_svg(uuid: String) -> Result<(), UUIDError>`
1. A basic user interface function to run the program in the terminal with the options "--low", "--upper", "--dashed", "--svg", "?", "--help". No option defaults to "--low". Function signature: `pub fn uuid_ui -> Result<(), UUIDError>`

### How to use this library: 
1. Add to Cargo.toml: 
```Toml
    [dependencies]
    uuidv4 = {git = "https://github.com/azavgo/uuidv4", branch = "main"}
```
2. Generate version 4 UUID string:  
```Rust
    use uuidv4::*;

    fn main() -> Result<(), UUIDError>{
        let uuid = uuid_dashed()?;
        println!("Generated UUID v4: {}", uuid); 
        Ok(())
    }
  
```
3. Generate QR code from the UUID string and write it as a "uuid".svg file: 
```Rust
    use uuidv4::*;

    fn main() -> Result<(), UUIDError>{
        let uuid = uuid_dashed()?;
        to_svg(uuid)?;
        Ok(())
    } 
``` 
4. User interface functionality to run the program from the terminal: 
```Rust
    use uuidv4::*;

    fn main() -> Result<(), UUIDError>{
        uuid_ui()?;
        Ok(())
    } 
``` 
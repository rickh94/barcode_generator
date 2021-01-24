extern crate barcoders;
extern crate pyo3;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions;


use barcoders::sym::code39::*;
use barcoders::generators::image::*;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

#[pyfunction]
fn generate_instrument_barcode(_py: Python, instrument_number: &str, output_path_prefix: &str, xdim: u32, height: u32) -> PyResult<()> {
    let barcode = Code39::new(instrument_number);
    let barcode = match barcode {
        Ok(barcode) => barcode,
        Err(_) => {
            return Err(exceptions::Exception::py_err("Failed to generate"));
        }
    };

    let png = Image::PNG {
        height,
        xdim,
        rotation: Rotation::Zero,
        foreground: Color::new([0, 0, 0, 255]),
        background: Color::new([255, 255, 255, 255]),
    };
    let encoded = barcode.encode();
    let bytes = match png.generate(&encoded[..]) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Err(exceptions::Exception::py_err("Failed to create image"));
        }
    };
    let file_name = format!("{prefix}/{num}.png", num = instrument_number, prefix = output_path_prefix);
    let new_file = match File::create(&Path::new(&file_name)) {
        Ok(new_file) => new_file,
        Err(_) => {
            return Err(exceptions::Exception::py_err("Could not create file"));
        }
    };

    let mut writer = BufWriter::new(new_file);
    match writer.write(&bytes[..]) {
        Ok(_) => Ok(()),
        Err(_) => Err(exceptions::Exception::py_err("Failed to write file."))
    }
}


#[pymodule]
fn barcode(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(generate_instrument_barcode))?;

    Ok(())
}

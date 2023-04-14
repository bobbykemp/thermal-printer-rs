use super::epson_printer_constants::printer_hardware::BEEP;
use std::error::Error;
use std::fmt;

pub struct EpsonPrinter {
    buffer: Vec<u8>,
}

#[derive(Debug)]
struct EpsonPrinterError {
    message: String,
}

impl EpsonPrinterError {
    fn new(message: String) -> EpsonPrinterError {
        EpsonPrinterError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for EpsonPrinterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for EpsonPrinterError {}

// the functions here should return a buffer of bytes that correspond
// to a series of actions that do something useful on the printer
// i.e. printing a document then beeping

// NOTE: To be useful for my current purposes, just need to implement the following from node-thermal-printer:
// newLine()
// println()
// cut()
// openCashDrawer()
// execute()
// clear()
// isPrinterConnected()
// printImageBuffer()

impl EpsonPrinter {
    fn append(&mut self, buf: Vec<u8>) {
        self.buffer.extend(buf);
    }

    fn beep(&self) -> Vec<u8> {
        BEEP.to_vec()
    }

    fn clear_buffer(&mut self) {
        self.buffer.clear();
        self.buffer.shrink_to_fit();
    }

    fn set_text_size(&mut self, height: u8, width: u8) -> Result<Vec<u8>, EpsonPrinterError> {
        self.clear_buffer();
        if height > 7 {
            return Err(EpsonPrinterError {
                message: "Height must be between 0 and 7".to_string(),
            });
        }
        if width > 7 {
            return Err(EpsonPrinterError {
                message: "Width must be between 0 and 7".to_string(),
            });
        }
        let concatened_width_height: u8 = ((height as u8) << 4) | width;
        self.buffer.extend([0x1D, 0x21]);
        self.buffer.extend([concatened_width_height]);
        Ok(self.buffer.clone())
    }
}

#[test]
fn append_test() {
    // can append one value to the buffer
    let mut printer: EpsonPrinter = EpsonPrinter { buffer: Vec::new() };
    printer.append(vec![0x00]);
    assert_eq!(printer.buffer, vec![0x00]);

    // can append empty vector to the buffer
    printer = EpsonPrinter { buffer: vec![] };
    printer.append(vec![]);
    assert_eq!(printer.buffer, vec![]);

    // can append multiple values to the buffer
    printer = EpsonPrinter {
        buffer: vec![0x08, 0x009],
    };
    printer.append(vec![0x001, 0x033]);
    assert_eq!(printer.buffer, vec![0x08, 0x009, 0x001, 0x033]);
}

#[test]
fn beep_test() {
    let printer: EpsonPrinter = EpsonPrinter { buffer: Vec::new() };
    assert_eq!(printer.beep(), BEEP);
}

#[test]
fn clear_test() {
    let mut printer: EpsonPrinter = EpsonPrinter {
        buffer: vec![0x00, 0x01],
    };
    assert_eq!(printer.buffer, vec![0x00, 0x01]);
    printer.clear_buffer();
    assert_eq!(printer.buffer, vec![]);
}

#[test]
fn set_text_size_test() {
    let mut printer: EpsonPrinter = EpsonPrinter {
        buffer: vec![0x00, 0x01],
    };

    assert!(printer.set_text_size(u8::MAX, u8::MAX).is_err());

    // min would be 0
    printer.set_text_size(u8::MIN, u8::MIN).unwrap();
    assert_eq!(printer.buffer, vec![0x1D, 0x21, 0x00]);

    for i in 0 as u8..=7 as u8 {
        printer.set_text_size(0, i).unwrap();
        assert_eq!(printer.buffer, vec![0x1D, 0x21, i]);
    }

    printer.set_text_size(1, 0).unwrap();
    assert_eq!(printer.buffer, vec![0x1D, 0x21, 0x10]);

    printer.set_text_size(1, 1).unwrap();
    assert_eq!(printer.buffer, vec![0x1D, 0x21, 0x11]);

    printer.set_text_size(1, 7).unwrap();
    assert_eq!(printer.buffer, vec![0x1D, 0x21, 0x17]);

    printer.set_text_size(4, 4).unwrap();
    assert_eq!(printer.buffer, vec![0x1D, 0x21, 0x44]);

    printer.set_text_size(4, 7).unwrap();
    assert_eq!(printer.buffer, vec![0x1D, 0x21, 0x47]);

    printer.set_text_size(7, 7).unwrap();
    assert_eq!(printer.buffer, vec![0x1D, 0x21, 0x77]);

    assert!(printer.set_text_size(0, 8).is_err());
    assert!(printer.set_text_size(8, 0).is_err());
    assert!(printer.set_text_size(8, 8).is_err());
}

pub struct EpsonPrinter {
    buffer: Vec<u8>,
}

impl EpsonPrinter {
    fn append(&mut self, buf: Vec<u8>) {
        self.buffer.extend(buf);
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

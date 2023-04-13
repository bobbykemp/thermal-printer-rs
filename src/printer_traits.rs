pub trait Beepable {
    fn beep(&self);
}

pub trait Printable {
    fn print_qr(&self);
    fn print_barcode(&self);
    fn print_image(&self);
    fn print_image_buffer(&self);
}

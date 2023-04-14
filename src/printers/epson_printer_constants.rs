pub mod control_sequences {
    pub const CTL_LF: [u8; 1] = [0x0a]; // Print and line feed
    pub const CTL_FF: [u8; 1] = [0x0c]; // Form feed
    pub const CTL_CR: [u8; 1] = [0x0d]; // Carriage return
    pub const CTL_HT: [u8; 1] = [0x09]; // Horizontal tab
    pub const CTL_SET_HT: [u8; 2] = [0x1b, 0x44]; // Set horizontal tab positions
    pub const CTL_VT: [u8; 3] = [0x1b, 0x64, 0x04]; // Vertical tab
}

pub mod printer_hardware {
    pub const HW_INIT: [u8; 2] = [0x1b, 0x40];
    pub const HW_SELECT: [u8; 3] = [0x1b, 0x3d, 0x01];
    pub const HW_RESET: [u8; 4] = [0x1b, 0x3f, 0x0a, 0x00];
    pub const BEEP: [u8; 2] = [0x1b, 0x1e];
    pub const UPSIDE_DOWN_ON: [u8; 3] = [0x1b, 0x7b, 0x01];
    pub const UPSIDE_DOWN_OFF: [u8; 3] = [0x1b, 0x7b, 0x00];
    pub const CD_KICK_2: [u8; 3] = [0x1b, 0x70, 0x00];
    pub const CD_KICK_5: [u8; 3] = [0x1b, 0x70, 0x01];
    pub const PAPER_FULL_CUT: [u8; 3] = [0x1d, 0x56, 0x00];
    pub const PAPER_PART_CUT: [u8; 3] = [0x1d, 0x56, 0x01];
}

pub mod text_format {
    pub const TXT_NORMAL: [u8; 3] = [0x1b, 0x21, 0x00];
    pub const TXT_2HEIGHT: [u8; 3] = [0x1b, 0x21, 0x10];
    pub const TXT_2WIDTH: [u8; 3] = [0x1b, 0x21, 0x20];
    pub const TXT_4SQUARE: [u8; 3] = [0x1b, 0x21, 0x30];
    pub const TXT_UNDERL_OFF: [u8; 3] = [0x1b, 0x2d, 0x00];
    pub const TXT_UNDERL_ON: [u8; 3] = [0x1b, 0x2d, 0x01];
    pub const TXT_UNDERL2_ON: [u8; 3] = [0x1b, 0x2d, 0x02];
    pub const TXT_BOLD_OFF: [u8; 3] = [0x1b, 0x45, 0x00];
    pub const TXT_BOLD_ON: [u8; 3] = [0x1b, 0x45, 0x01];
    pub const TXT_INVERT_OFF: [u8; 3] = [0x1d, 0x42, 0x00];
    pub const TXT_INVERT_ON: [u8; 3] = [0x1d, 0x42, 0x01];
    pub const TXT_FONT_A: [u8; 3] = [0x1b, 0x4d, 0x00];
    pub const TXT_FONT_B: [u8; 3] = [0x1b, 0x4d, 0x01];
    pub const TXT_ALIGN_LT: [u8; 3] = [0x1b, 0x61, 0x00];
    pub const TXT_ALIGN_CT: [u8; 3] = [0x1b, 0x61, 0x01];
    pub const TXT_ALIGN_RT: [u8; 3] = [0x1b, 0x61, 0x02];
}

pub mod code_pages {
    pub const CODE_PAGE_PC437_USA: [u8; 3] = [0x1b, 0x74, 0];
    pub const CODE_PAGE_KATAKANA: [u8; 3] = [0x1b, 0x74, 1];
    pub const CODE_PAGE_PC850_MULTILINGUAL: [u8; 3] = [0x1b, 0x74, 2];
    pub const CODE_PAGE_PC860_PORTUGUESE: [u8; 3] = [0x1b, 0x74, 3];
    pub const CODE_PAGE_PC863_CANADIAN_FRENCH: [u8; 3] = [0x1b, 0x74, 4];
    pub const CODE_PAGE_PC865_NORDIC: [u8; 3] = [0x1b, 0x74, 5];
    pub const CODE_PAGE_PC851_GREEK: [u8; 3] = [0x1b, 0x74, 11];
    pub const CODE_PAGE_PC853_TURKISH: [u8; 3] = [0x1b, 0x74, 12];
    pub const CODE_PAGE_PC857_TURKISH: [u8; 3] = [0x1b, 0x74, 13];
    pub const CODE_PAGE_PC737_GREEK: [u8; 3] = [0x1b, 0x74, 14];
    pub const CODE_PAGE_ISO8859_7_GREEK: [u8; 3] = [0x1b, 0x74, 15];
    pub const CODE_PAGE_WPC1252: [u8; 3] = [0x1b, 0x74, 16];
    pub const CODE_PAGE_PC866_CYRILLIC2: [u8; 3] = [0x1b, 0x74, 17];
    pub const CODE_PAGE_PC852_LATIN2: [u8; 3] = [0x1b, 0x74, 18];
    pub const CODE_PAGE_SLOVENIA: [u8; 3] = [0x1b, 0x74, 18];
    pub const CODE_PAGE_PC858_EURO: [u8; 3] = [0x1b, 0x74, 19];
    pub const CODE_PAGE_KU42_THAI: [u8; 3] = [0x1b, 0x74, 20];
    pub const CODE_PAGE_TIS11_THAI: [u8; 3] = [0x1b, 0x74, 21];
    pub const CODE_PAGE_TIS18_THAI: [u8; 3] = [0x1b, 0x74, 26];
    pub const CODE_PAGE_TCVN3_VIETNAMESE_L: [u8; 3] = [0x1b, 0x74, 30];
    pub const CODE_PAGE_TCVN3_VIETNAMESE_U: [u8; 3] = [0x1b, 0x74, 31];
    pub const CODE_PAGE_PC720_ARABIC: [u8; 3] = [0x1b, 0x74, 32];
    pub const CODE_PAGE_WPC775_BALTIC_RIM: [u8; 3] = [0x1b, 0x74, 33];
    pub const CODE_PAGE_PC855_CYRILLIC: [u8; 3] = [0x1b, 0x74, 34];
    pub const CODE_PAGE_PC861_ICELANDIC: [u8; 3] = [0x1b, 0x74, 35];
    pub const CODE_PAGE_PC862_HEBREW: [u8; 3] = [0x1b, 0x74, 36];
    pub const CODE_PAGE_PC864_ARABIC: [u8; 3] = [0x1b, 0x74, 37];
    pub const CODE_PAGE_PC869_GREEK: [u8; 3] = [0x1b, 0x74, 38];
    pub const CODE_PAGE_ISO8859_2_LATIN2: [u8; 3] = [0x1b, 0x74, 39];
    pub const CODE_PAGE_ISO8859_15_LATIN9: [u8; 3] = [0x1b, 0x74, 40];
    pub const CODE_PAGE_PC1098_FARCI: [u8; 3] = [0x1b, 0x74, 41];
    pub const CODE_PAGE_PC1118_LITHUANIAN: [u8; 3] = [0x1b, 0x74, 42];
    pub const CODE_PAGE_PC1119_LITHUANIAN: [u8; 3] = [0x1b, 0x74, 43];
    pub const CODE_PAGE_PC1125_UKRANIAN: [u8; 3] = [0x1b, 0x74, 44];
    pub const CODE_PAGE_WPC1250_LATIN2: [u8; 3] = [0x1b, 0x74, 45];
    pub const CODE_PAGE_WPC1251_CYRILLIC: [u8; 3] = [0x1b, 0x74, 46];
    pub const CODE_PAGE_WPC1253_GREEK: [u8; 3] = [0x1b, 0x74, 47];
    pub const CODE_PAGE_WPC1254_TURKISH: [u8; 3] = [0x1b, 0x74, 48];
    pub const CODE_PAGE_WPC1255_HEBREW: [u8; 3] = [0x1b, 0x74, 49];
    pub const CODE_PAGE_WPC1256_ARABIC: [u8; 3] = [0x1b, 0x74, 50];
    pub const CODE_PAGE_WPC1257_BALTIC_RIM: [u8; 3] = [0x1b, 0x74, 51];
    pub const CODE_PAGE_WPC1258_VIETNAMESE: [u8; 3] = [0x1b, 0x74, 52];
    pub const CODE_PAGE_KZ1048_KAZAKHSTAN: [u8; 3] = [0x1b, 0x74, 53];
    pub const CODE_PAGE_JAPAN: [u8; 3] = [0x1b, 0x52, 0x08];
    pub const CODE_PAGE_KOREA: [u8; 3] = [0x1b, 0x52, 0x0D];
    pub const CODE_PAGE_CHINA: [u8; 3] = [0x1b, 0x52, 0x0F];
    pub const CODE_PAGE_HK_TW: [u8; 3] = [0x1b, 0x52, 0x00];
    pub const CODE_PAGE_TCVN_VIETNAMESE: [u8; 3] = [0x1b, 0x74, 52];
}

pub mod character_code_pages {
    pub const PC437_USA: &str = "CP437";
    pub const PC850_MULTILINGUAL: &str = "CP850";
    pub const PC860_PORTUGUESE: &str = "CP860";
    pub const PC863_CANADIAN_FRENCH: &str = "CP863";
    pub const PC865_NORDIC: &str = "CP865";
    pub const PC851_GREEK: &str = "CP860";
    pub const PC857_TURKISH: &str = "CP857";
    pub const PC737_GREEK: &str = "CP737";
    pub const ISO8859_7_GREEK: &str = "ISO-8859-7";
    pub const WPC1252: &str = "CP1252";
    pub const PC866_CYRILLIC2: &str = "CP866";
    pub const PC852_LATIN2: &str = "CP852";
    pub const SLOVENIA: &str = "CP852";
    pub const PC858_EURO: &str = "CP858";
    pub const WPC775_BALTIC_RIM: &str = "CP775";
    pub const PC855_CYRILLIC: &str = "CP855";
    pub const PC861_ICELANDIC: &str = "CP861";
    pub const PC862_HEBREW: &str = "CP862";
    pub const PC864_ARABIC: &str = "CP864";
    pub const PC869_GREEK: &str = "CP869";
    pub const ISO8859_2_LATIN2: &str = "ISO-8859-2";
    pub const ISO8859_15_LATIN9: &str = "ISO-8859-15";
    pub const PC1125_UKRANIAN: &str = "CP1125";
    pub const WPC1250_LATIN2: &str = "WIN1250";
    pub const WPC1251_CYRILLIC: &str = "WIN1251";
    pub const WPC1253_GREEK: &str = "WIN1253";
    pub const WPC1254_TURKISH: &str = "WIN1254";
    pub const WPC1255_HEBREW: &str = "WIN1255";
    pub const WPC1256_ARABIC: &str = "WIN1256";
    pub const WPC1257_BALTIC_RIM: &str = "WIN1257";
    pub const WPC1258_VIETNAMESE: &str = "WIN1258";
    pub const KZ1048_KAZAKHSTAN: &str = "RK1048";
    pub const JAPAN: &str = "EUC-JP";
    pub const KOREA: &str = "EUC-KR";
    pub const CHINA: &str = "EUC-CN";
    pub const HK_TW: &str = "Big5-HKSCS";
    pub const TCVN_VIETNAMESE: &str = "tcvn";
}

pub mod barcode_formats {
    pub const BARCODE_TXT_OFF: [u8; 3] = [0x1d, 0x48, 0x00]; // HRI barcode chars OFF
    pub const BARCODE_TXT_ABV: [u8; 3] = [0x1d, 0x48, 0x01]; // HRI barcode chars above
    pub const BARCODE_TXT_BLW: [u8; 3] = [0x1d, 0x48, 0x02]; // HRI barcode chars below
    pub const BARCODE_TXT_BTH: [u8; 3] = [0x1d, 0x48, 0x03]; // HRI barcode chars both above and below
    pub const BARCODE_FONT_A: [u8; 3] = [0x1d, 0x66, 0x00]; // Font type A for HRI barcode chars
    pub const BARCODE_FONT_B: [u8; 3] = [0x1d, 0x66, 0x01]; // Font type B for HRI barcode chars
    pub const BARCODE_HEIGHT: [u8; 3] = [0x1d, 0x68, 0x64]; // Barcode Height [1-255]
    pub const BARCODE_WIDTH: [u8; 3] = [0x1d, 0x77, 0x03]; // Barcode Width  [2-6]
    pub const BARCODE_UPC_A: [u8; 3] = [0x1d, 0x6b, 0x00]; // Barcode type UPC-A
    pub const BARCODE_UPC_E: [u8; 3] = [0x1d, 0x6b, 0x01]; // Barcode type UPC-E
    pub const BARCODE_EAN13: [u8; 3] = [0x1d, 0x6b, 0x02]; // Barcode type EAN13
    pub const BARCODE_EAN8: [u8; 3] = [0x1d, 0x6b, 0x03]; // Barcode type EAN8
    pub const BARCODE_CODE39: [u8; 3] = [0x1d, 0x6b, 0x04]; // Barcode type CODE39
    pub const BARCODE_ITF: [u8; 3] = [0x1d, 0x6b, 0x05]; // Barcode type ITF
    pub const BARCODE_NW7: [u8; 3] = [0x1d, 0x6b, 0x06]; // Barcode type NW7
}

pub mod qr_codes {
    pub const QRCODE_MODEL1: [u8; 9] = [0x1d, 0x28, 0x6b, 0x04, 0x00, 0x31, 0x41, 0x31, 0x00];
    pub const QRCODE_MODEL2: [u8; 9] = [0x1d, 0x28, 0x6b, 0x04, 0x00, 0x31, 0x41, 0x32, 0x00];
    pub const QRCODE_MODEL3: [u8; 9] = [0x1d, 0x28, 0x6b, 0x04, 0x00, 0x31, 0x41, 0x33, 0x00];

    pub const QRCODE_CORRECTION_L: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x45, 0x30]; // Correction level: L - 7%
    pub const QRCODE_CORRECTION_M: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x45, 0x31]; // Correction level: M - 15%
    pub const QRCODE_CORRECTION_Q: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x45, 0x32]; // Correction level: Q - 25%
    pub const QRCODE_CORRECTION_H: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x45, 0x33]; // Correction level: H - 30%

    pub const QRCODE_CELLSIZE_1: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x43, 0x01]; // Cell size 1
    pub const QRCODE_CELLSIZE_2: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x43, 0x02]; // Cell size 2
    pub const QRCODE_CELLSIZE_3: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x43, 0x03]; // Cell size 3
    pub const QRCODE_CELLSIZE_4: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x43, 0x04]; // Cell size 4
    pub const QRCODE_CELLSIZE_5: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x43, 0x05]; // Cell size 5
    pub const QRCODE_CELLSIZE_6: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x43, 0x06]; // Cell size 6
    pub const QRCODE_CELLSIZE_7: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x43, 0x07]; // Cell size 7
    pub const QRCODE_CELLSIZE_8: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x43, 0x08]; // Cell size 8

    pub const QRCODE_PRINT: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x31, 0x51, 0x30];
}

pub mod pdf417 {
    pub const PDF417_CORRECTION: [u8; 8] = [0x1D, 0x28, 0x6B, 0x04, 0x00, 0x30, 0x45, 0x31]; // Append 1-40 for ratio
    pub const PDF417_ROW_HEIGHT: [u8; 7] = [0x1D, 0x28, 0x6B, 0x03, 0x00, 0x30, 0x44]; // Append 2-8 for height
    pub const PDF417_WIDTH: [u8; 7] = [0x1D, 0x28, 0x6B, 0x03, 0x00, 0x30, 0x43]; // Append 2-8 for width
    pub const PDF417_COLUMNS: [u8; 7] = [0x1D, 0x28, 0x6B, 0x03, 0x00, 0x30, 0x41];
    pub const PDF417_OPTION_STANDARD: [u8; 8] = [0x1D, 0x28, 0x6B, 0x03, 0x00, 0x30, 0x46, 0x00]; // Standard barcode
    pub const PDF417_OPTION_TRUNCATED: [u8; 8] = [0x1D, 0x28, 0x6B, 0x03, 0x00, 0x30, 0x46, 0x01]; // Truncated barcode
    pub const PDF417_PRINT: [u8; 8] = [0x1D, 0x28, 0x6B, 0x03, 0x00, 0x30, 0x51, 0x30];
}

pub mod maxi_modes {
    // MaxiCode
    // Formatted data containing a structured Carrier Message with a numeric postal code. (US)
    pub const MAXI_MODE2: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x32, 0x41, 0x32];
    // Formatted data containing a structured Carrier Message with an alphanumeric postal code. (International)
    pub const MAXI_MODE3: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x32, 0x41, 0x33];
    pub const MAXI_MODE4: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x32, 0x41, 0x34]; // Unformatted data with Standard Error Correction.
    pub const MAXI_MODE5: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x32, 0x41, 0x35]; // Unformatted data with Enhanced Error Correction.
    pub const MAXI_MODE6: [u8; 8] = [0x1d, 0x28, 0x6b, 0x03, 0x00, 0x32, 0x41, 0x36]; // For programming hardware devices.

    pub const MAXI_PRINT: [u8; 8] = [0x1d, 0x28, 0x6B, 0x03, 0x00, 0x32, 0x51, 0x30];
}

pub mod raster {
    // Image format
    pub const S_RASTER_N: [u8; 4] = [0x1d, 0x76, 0x30, 0x00]; // Set raster image normal size
    pub const S_RASTER_2W: [u8; 4] = [0x1d, 0x76, 0x30, 0x01]; // Set raster image double width
    pub const S_RASTER_2H: [u8; 4] = [0x1d, 0x76, 0x30, 0x02]; // Set raster image double height
    pub const S_RASTER_Q: [u8; 4] = [0x1d, 0x76, 0x30, 0x03]; // Set raster image quadruple
}

pub mod printing_density {
    pub const PD_N50: [u8; 3] = [0x1d, 0x7c, 0x00]; // Printing Density -50%
    pub const PD_N37: [u8; 3] = [0x1d, 0x7c, 0x01]; // Printing Density -37.5%
    pub const PD_N25: [u8; 3] = [0x1d, 0x7c, 0x02]; // Printing Density -25%
    pub const PD_N12: [u8; 3] = [0x1d, 0x7c, 0x03]; // Printing Density -12.5%
    pub const PD_0: [u8; 3] = [0x1d, 0x7c, 0x04]; // Printing Density  0%
    pub const PD_P50: [u8; 3] = [0x1d, 0x7c, 0x08]; // Printing Density +50%
    pub const PD_P37: [u8; 3] = [0x1d, 0x7c, 0x07]; // Printing Density +37.5%
    pub const PD_P25: [u8; 3] = [0x1d, 0x7c, 0x06]; // Printing Density +25%
}

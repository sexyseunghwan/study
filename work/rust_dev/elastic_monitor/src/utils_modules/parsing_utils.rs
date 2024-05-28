use crate::common::*;

/*
    Function that URL-encodes the target string
*/
pub fn get_url_encoding(input_str: &str) -> String {

    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`')
                                    .add(b'#').add(b'?').add(b'{').add(b'}')
                                    .add(b'/').add(b':').add(b';').add(b'=')
                                    .add(b'@').add(b'[').add(b']').add(b'\\')
                                    .add(b'^').add(b'|');

    utf8_percent_encode(input_str, FRAGMENT).to_string()

}
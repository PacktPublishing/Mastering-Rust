use std::string::FromUtf8Error;

fn bytestring_to_string_with_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = match String::from_utf8(str) {
        Ok(str) => str.to_uppercase(),
        Err(err) => return Err(err)
    };

    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}

fn bytestring_to_string_with_try(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = try!(String::from_utf8(str));
    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}

fn bytestring_to_string_with_qmark(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = String::from_utf8(str)?;
    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}

fn main() {
    let faulty_bytestring = vec!(130, 131, 132, 133);
    let ok_bytestring = vec!(80, 82, 84, 85, 86);

    let s1_faulty = bytestring_to_string_with_match(faulty_bytestring.clone());
    let s1_ok = bytestring_to_string_with_match(ok_bytestring.clone());

    let s2_faulty = bytestring_to_string_with_qmark(faulty_bytestring.clone());
    let s2_ok = bytestring_to_string_with_qmark(ok_bytestring.clone());

    println!("Read the string: {:?}", s1_faulty);
    println!("Read the string: {:?}", s1_ok);
    println!("Read the string: {:?}", s2_faulty);
    println!("Read the string: {:?}", s2_ok);

}

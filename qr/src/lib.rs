pub fn gen(link: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let qr_code = qr_code::QrCode::new(link.as_bytes())?;
    let bmp = qr_code.to_bmp();
    let mut bmp_vec: Vec<u8> = Vec::new();
    bmp.write(&mut bmp_vec)?;
    Ok(bmp_vec)
}

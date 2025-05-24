#[cfg(test)]
mod tests {
    // 测试二维码导入
    use cdrsdk::ffi::import_qr::import_qrcode;
    #[test]
    fn test_import_qrcode() {
        // C:\Users\dowell\Desktop\2222.cdr
        // let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.cdr".to_string();
        // let src ="C:\\Users\\dowell\\Desktop\\2222.cdr".to_string();
        let res = import_qrcode("21");
        println!("{:?}",res);
    }
    // 测试模板导入
    use cdrsdk::ffi::import_tpl::import_produce_tpl;
    #[test]
    fn test_import_tpl() {
        // C:\Users\dowell\Desktop\2222.cdr
        // let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.cdr".to_string();
        // let src ="C:\\Users\\dowell\\Desktop\\2222.cdr".to_string();
        let res = import_produce_tpl("21");
        println!("{:?}",res);
    }
}
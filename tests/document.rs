// 获取coreldraw的状态
#[cfg(test)]
mod tests {
    use cdrsdk::ffi::document;
    #[test]
    fn test_do_cdr_app_document() {
        // C:\Users\dowell\Desktop\2222.cdr
        // let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.cdr".to_string();
        let name = document::get_document_name("25".to_string());
        println!("{:?}",name);
        let path = document::get_document_path("25".to_string());
        println!("{:?}",path);
        let fullfilename = document::get_document_fullfilename("25".to_string());
        println!("{:?}",fullfilename);
    }
}
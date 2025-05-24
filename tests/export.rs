// 获取coreldraw的状态
#[cfg(test)]
mod tests {
    use cdrsdk::ffi::cdrexport;
    use cdrsdk::ffi::export_cover;
    #[test]
    fn test_export_dxf_file() {
        // C:\Users\dowell\Desktop\2222.cdr
        let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.dxf";
        // let src ="C:\\Users\\dowell\\Desktop\\2222.cdr".to_string();
        let res = cdrexport::export_dxf_file("25",src);
        println!("{:?}",res);
    }
    #[test]
    fn test_export_cover_file() {
        // C:\Users\dowell\Desktop\2222.cdr
        let src = "D:\\codes\\dytes\\cdrsdk\\cache\\123123.jpg";
        let res = export_cover::export_cover(src.to_string(),"25");
        println!("{:?}",res);
    }
}
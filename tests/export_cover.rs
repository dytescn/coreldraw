// 获取coreldraw的状态
#[cfg(test)]
mod tests {
    use cdrsdk::ffi::export_cover;
    #[test]
    fn test_export_cdrexport_cover() {
        // C:\Users\dowell\Desktop\2222.cdr
        let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.png";
        // let src ="C:\\Users\\dowell\\Desktop\\2222.cdr".to_string();
        let res = export_cover::export_selection_cover(src, "25");
        println!("{:?}",res);
    }
}
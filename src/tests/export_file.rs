// 获取coreldraw的状态
#[cfg(test)]
mod tests {
    use cdrsdk::ffi::export_file;
    #[test]
    fn test_export_tpl_file() {
        // C:\Users\dowell\Desktop\2222.cdr
        let src = "C:\\Users\\Lenovo\\Desktop\\imgcache\\3333.cdr";
        // let src ="C:\\Users\\dowell\\Desktop\\2222.cdr".to_string();
        let res = export_file::export_file(src.to_string(),"25");
        println!("{:?}",res);
    }
}
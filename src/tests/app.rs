// 获取coreldraw的状态
#[cfg(test)]
mod tests {
    use cdrsdk::ffi::app;
    #[test]
    fn test_do_cdr_app_open() {
        // C:\Users\dowell\Desktop\2222.cdr
        // let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.cdr".to_string();
        let src ="C:\\Users\\dowell\\Desktop\\2222.cdr".to_string();
        let res = app::do_cdr_app_open("测试".to_string(),src,"25".to_string());
        println!("{:?}",res);
    }
    #[test]
    fn test_do_cdr_app_info() {
        // C:\Users\dowell\Desktop\2222.cdr
        // let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.cdr".to_string();
        let res = app::get_app_execute_path("25".to_string());
        println!("{:?}",res);
    }
}
// 获取coreldraw的状态
#[cfg(test)]
mod tests {
    use cdrsdk::ffi::cdrstatus;
    #[test]
    fn test_check_corel_status() {
        // C:\Users\dowell\Desktop\2222.cdr
        // let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.cdr".to_string();
        // let src ="C:\\Users\\dowell\\Desktop\\2222.cdr".to_string();
        let res = cdrstatus::check_corel_status("25");
        println!("{:?}",res);
    }
}
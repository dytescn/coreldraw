// 获取coreldraw的状态
#[cfg(test)]
mod tests {
    use cdrsdk::ffi::selection;
    #[test]
    fn test_select_tpl_file() {
        // C:\Users\dowell\Desktop\2222.cdr
        // let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.pdf";
        // let src ="C:\\Users\\dowell\\Desktop\\2222.cdr".to_string();
        let res = selection::check_selection("25");
        println!("{:?}",res);
    }
}
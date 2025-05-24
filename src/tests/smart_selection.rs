// 获取coreldraw的状态
#[cfg(test)]
mod tests {
    use cdrsdk::ffi::smart_selection;
    #[test]
    fn test_check_corel_status() {
        // C:\Users\dowell\Desktop\2222.cdr
        // let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.cdr".to_string();
        // let src ="C:\\Users\\dowell\\Desktop\\2222.cdr".to_string();
        let res = smart_selection::search_layer_by_color("25",true,true,true,true);
        println!("{:?}",res);
    }
}

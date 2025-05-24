// 获取coreldraw的状态
#[cfg(test)]
mod tests {
    use cdrsdk::ffi::export_comps;
    #[test]
    fn test_export_svg_file() {
        // C:\Users\dowell\Desktop\2222.cdr
        let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.svg";
        // let src ="C:\\Users\\dowell\\Desktop\\2222.cdr".to_string();
        let res = export_comps::export_select_comps(src,"svg","25");
        println!("{:?}",res);
    }
}
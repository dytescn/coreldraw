#[cfg(test)]
mod tests {
    use cdrsdk::ffi::layer::create_qrcode_layer;
    #[test]
    fn test_layer() {
        // C:\Users\dowell\Desktop\2222.cdr
        // let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.cdr".to_string();
        // let src ="C:\\Users\\dowell\\Desktop\\2222.cdr".to_string();
        let res = create_qrcode_layer("25","./cache/fengxuntpl.svg");
        println!("...zzz{:?}zzz...",res);
        // println!("{:?}",res);
    }
}
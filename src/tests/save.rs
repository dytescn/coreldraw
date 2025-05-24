#[cfg(test)]
mod tests {
    use cdrsdk::ffi::save;
    #[test]
    fn it_works() {
        let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\".to_string();
        let s_uid = "d701bc8f9f0c44988277e02f23cf58e1".to_string();
        let v_uid = "e75377dc96174d2ea4c396fea3fb3240".to_string();
        let res = save::do_cdr_file_save(src,"测试".to_string(),s_uid,v_uid,25,"25");
        println!("{:?}",res);
    }
}
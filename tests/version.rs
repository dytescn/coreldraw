// 获取信息
#[cfg(test)]
mod tests {
    use cdrsdk::ffi::version;
    #[test]
    fn it_works() {
        // let src = "E:\\rustcode\\bindcom\\cache\\123.cdr".to_string();
       let version =  version::get_version_list();
       println!("..........{:?}.........",version);
    }
}
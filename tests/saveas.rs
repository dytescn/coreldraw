#[cfg(test)]
mod tests {
    use cdrsdk::ffi::saveas;
    #[test]
    fn it_works() {
        // let src = "E:\\rustcode\\bindcom\\cache\\123.cdr".to_string();
       let version =  saveas::save_as_file("./cache/3333.cdr","1111".to_string(),25,"25");
       println!("..........{:?}.........",version);
        // println!("{:?}",res);
    }
}
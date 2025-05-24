pub struct IvgActiveView {
    disp:ComObject
}

impl IvgActiveView{
   pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
}
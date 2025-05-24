use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;
use crate::sdk::layer;
use crate::sdk::page;

// 魔棒
pub fn search_layer_by_color(ver:&str,check_fill:bool,check_outline_color:bool,check_outline_width:bool,check_trans:bool){
    // 获取当前选中文件
    unsafe{
        Com::CoInitialize(None).log_error("init com error").unwrap();
        let app = IvgApplication::new(&ver).log_error("init application error").unwrap();
        if let Some(doc) = app.get_active_document().log_error("init doc err"){
            // 获取当前选中图像
            let activeshape_res = doc.get_ActiveShape();
            if activeshape_res.is_none(){
                return;
            }
            let activeshape = activeshape_res.unwrap();
            // 填充颜色
            let active_fill = activeshape.get_Fill().unwrap();
            let active_fill_color = active_fill.get_UniformColor().unwrap();
            // 边框宽度与颜色
            let active_outline = activeshape.get_Outline().unwrap();
            let active_outline_width = active_outline.get_Width();
            let active_outline_color = active_outline.get_Color().unwrap();
            println!("outline:::{:?}",active_outline_width);

            // 透明度
            let active_Transparency = activeshape.get_Transparency().unwrap();
            let active_Transparency_type = active_Transparency.get_Type();
            println!("active_Transparency_type:::{:?}",active_Transparency_type);

            let pages = doc.get_pages();
            let active_page = doc.get_activepage().log_error("got active page err").unwrap();
            if let Some(activelayer) = doc.get_ActiveLayer().log_error("got layer error"){
                let allshapes = activelayer.get_Shapes().unwrap();
                let sum = allshapes.get_Count();
                for cur_i  in 1..sum+1  {
                    let cur_shape = allshapes.get_Item(cur_i);
                    // 设置选中状态
                    let fill_info = cur_shape.get_Fill().unwrap();
                    let cur_color =fill_info.get_UniformColor().unwrap();
                    // 判断填充颜色
                    if check_fill {
                        if active_fill_color.get_RGBValue() !=cur_color.get_RGBValue() {
                            continue;
                        }      
                    }
                    // 判断边框宽度
                    if check_outline_width {
                        let cur_shape_outline =  cur_shape.get_Outline().unwrap();
                        let cur_shape_outline_width = cur_shape_outline.get_Width();
                        if active_outline_width !=cur_shape_outline_width {
                            continue;
                        }      
                    }
                    // 判断边框颜色
                    if check_outline_color {
                        let cur_shape_outline =  cur_shape.get_Outline().unwrap();
                        let cur_shape_outline_color = cur_shape_outline.get_Color().unwrap();
                        if active_outline_color.get_RGBValue() !=cur_shape_outline_color.get_RGBValue() {
                            continue;
                        }      
                    }
                    // // 判断透明度
                    if check_trans {
                        let cur_Transparency = cur_shape.get_Transparency().unwrap();
                        let cur_Transparency_type = cur_Transparency.get_Type();
                        // println!(".......cur_Transparency_value{:?}",cur_Transparency_value);
                        if active_Transparency_type !=cur_Transparency_type {
                            continue;
                        }
                        
                        if active_Transparency_type==1 {
                            let active_Transparency_uniform = active_Transparency.get_Uniform();
                            let cur_Transparency_uniform = cur_Transparency.get_Uniform();
                            if cur_Transparency_uniform != active_Transparency_uniform{
                                continue;
                            }
                        }      
                    }
                    cur_shape.put_Selected(true);
                }
            } 
        }
        Com::CoUninitialize();
    }

}
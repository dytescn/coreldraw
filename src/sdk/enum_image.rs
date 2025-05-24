pub enum cdrImageType {
    cdrBlackAndWhiteImage, //	0	Specifies black-and-white
    cdr16ColorsImage, //	1	Specifies 16-color
    cdrGrayscaleImage, //	2	Specifies grayscale
    cdrPalettedImage, //	3	Specifies paletted
    cdrRGBColorImage, //	4	Specifies RGB
    cdrCMYKColorImage, //	5	Specifies CMYK
    cdrDuotoneImage, //	6	Specifies duotone
    cdrLABImage, //	7	Specifies LAB
    cdrCMYKMultiChannelImage, //	8	Specifies multi-channel CMYK
    cdrRGBMultiChannelImage, //	9	Specifies multi-channel RGB
    cdrSpotMultiChannelImage, //	10	Specifies multi-channel spot    
}

pub fn cdrImageType_to_value(val:cdrImageType)->usize{
    match val {
        cdrImageType::cdrBlackAndWhiteImage => {0},
        cdrImageType::cdr16ColorsImage =>  {1},
        cdrImageType::cdrGrayscaleImage => {2},
        cdrImageType::cdrPalettedImage => {3},
        cdrImageType::cdrRGBColorImage => {4},
        cdrImageType::cdrCMYKColorImage => {5},
        cdrImageType::cdrDuotoneImage => {6},
        cdrImageType::cdrLABImage => {7},
        cdrImageType::cdrCMYKMultiChannelImage => {8},
        cdrImageType::cdrRGBMultiChannelImage => {9},
        cdrImageType::cdrSpotMultiChannelImage => {10},
    }
}

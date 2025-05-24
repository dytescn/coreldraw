pub enum cdrFilter {
    cdrAutoSense, //	0	Detect the proper filter automatically (Import only)
    cdrBMP, //	769	Windows Bitmap
    cdrPCX, //	770	PaintBrush
    cdrTGA, //	771	Targa Bitmap
    cdrTIFF, //	772	TIFF Bitmap
    cdrGIF, //	773	CompuServe Bitmap
    cdrJPEG, //	774	JPEG Bitmaps
    cdrSCT, //	776	Scitex CT Bitmap
    cdrICO, //	784	Windows 3.x/NT Icon Resource (Import only)
    cdrCUR, //	785	Windows 3.x/NT Cursor Resource (Import only)
    cdrEXE, //	786	Windows 3.x/NT Bitmap Resource (Import only)
    cdrIMG, //	787	GEM Paint File
    cdrPSD, //	788	Adobe Photoshop
    cdrPP4, //	789	Picture Publisher 4 (Import only)
    cdrMAC, //	791	MACPaint Bitmap
    cdrOS2BMP, //	792	OS/2 Bitmap
    cdrWI, //	793	Wavelet Compressed Bitmap
    cdrCAL, //	800	CALS Compressed Bitmap
    cdrDCS, //	801	Desktop Color Separations (Export only)
    cdrPNG, //	802	Portable Network Graphics
    cdrEPSPhotoPaint, //	804	Corel PHOTO-PAINT Encapsulated PostScript
    cdrRIFF, //807	Corel Painter 5/6 (Import only)
    cdrXPM, //	809	XPixMap Image
    cdrXCF, //	816	Gimp Image (Import only)
    cdrJPG2000Import, //	818	JPG2000 Bitmaps (Import only)
    cdrJP2, //	820	JPG2000 Standard (Export only)
    cdrJPC, //	821	JPG2000 Codestream (Export only)
    cdrRAW, //	823	Camera RAW (Import only)
    cdrHEIF, //	824	HEIF (Import only)
    cdrWEBP, //	825	WEBP Google Web Picture
    cdrCGM, //	1280	Computer Graphics Metafile
    cdrHPGL, //	1281	HPGL Plotter File
    cdrPLT, //	1281	HPGL Plotter File
    cdrGEM, //	1284	GEM File
    cdrWPG, //	1287	Corel WordPerfect Graphic
    cdrPSEncapsulated, //	1289	Encapsulated PostScript
    cdrEPS, //	1289	Encapsulated PostScript
    cdrPSInterpreted, //	1290	PostScript Interpreted (Import only)
    cdrMET, //	1291	MET MetaFile (Import only)
    cdrNAP, //	1292	NAP MetaFile (Import only)
    cdrPICT, //	1293	Macintosh PICT
    cdrWMF, //	1294	Windows Metafile
    cdrDXF, //	1296	AutoCAD (DXF)
    cdrEMF, //	1300	Enhanced Windows Metafile
    cdrTTF, //	1302	True Type Font (Export only)
    cdrAT1, //	1303	Adobe Type 1 Font (Export only)
    cdrAI, //	1305	Adobe Illustrator
    cdrVSD, //	1315	Visio (Import only)
    cdrDWG, //	1328	AutoCAD (DWG)
    cdrFMV, //	1329	Frame Vector Metafile (Import only)
    cdrPDF, //	1333	Adobe Portable Document Format, Adobe Illustrator 9 (Import only)
    cdrAI9, //	1333	Adobe Illustrator 9, Adobe Portable Document Format (Import only)
    cdrPIC, //	1334	Lotus Pic (Import only)
    cdrDRW, //	1339	Corel/Micrografx Designer (Import only)
    cdrDSF, //	1339	Corel/Micrografx Designer (Import only)
    cdrFH, //	1344	Macromedia Freehand (Import only)
    cdrSVG, //	1345	Scalable Vector Graphics
    cdrSVGZ, //	1347	Compressed SVG
    cdrPUB, //	1349	Microsoft Publisher (Import only)
    cdrAVI, //	1536	Video for Windows
    cdrPPT, //	1548	Microsoft PowerPoint (Import only)
    cdrHTML, //	1557	HyperText Markup Language (Import only)
    cdrGIFAnimation, //	1558	GIF Animation
    cdr3DMF, //	1559	3DMF - 3D Model (Import only)
    cdrCMX6, //	1793	Corel Presentation Exchange
    cdrCMX5, //	1794	Corel Presentation Exchange 5.0
    cdrCDR, //	1795	CorelDRAW
    cdrCDX, //	1796	CorelDRAW Compressed (Import only)
    cdrCPX, //	1797	Corel CMX Compressed (Import only)
    cdrCPT8, //	1799	Corel PHOTO-PAINT 7/8 Image (Export only)
    cdrCDT, //	1800	CorelDRAW Template
    cdrPAT, //	1801	CorelDRAW Pattern File
    cdrCLK, //	1802	Corel R.A.V.E.
    cdrDES, //	1805	CorelDESIGNER
    cdrSHW, //	1806	Corel Presentations (Import only)
    cdrCPT, //	1808	Corel PHOTO-PAINT Image
    cdrCPT10, //	1808	Corel PHOTO-PAINT 9/10 Image
    cdrCPT11, //	1808	Corel PHOTO-PAINT 11 Image
    cdrCPT9, //	1808	Corel PHOTO-PAINT 9/10 Image
    cdrCSL, //	1810	Corel Symbol Library
    cdrCMX64, //	1811	Corel Presentation Exchange 64-bit
    cdrTXT, //	2048	ANSI Text
    cdrWord95, //	2049	MS Word for Windows 6/7
    cdrWord2, //	2050	MS Word for Windows 2.x
    cdrWord55, //	2051	MS Word 3.0, 4.0, 5.0, 5.5
    cdrMacWord5, //	2052	MS Word for Macintosh 4.0, 5.0
    cdrRTF, //	2053	Rich Text Format
    cdrWP9, //	2055	Corel WordPerfect 6+
    cdrWPD, //	2055	Corel WordPerfect 6+
    cdrWP51, //	2056	Corel WordPerfect 5.1
    cdrWP50, //	2057	Corel WordPerfect 5.0
    cdrWP4, //	2058	Corel WordPerfect 4.2
    cdrWSW, //	2059	WordStar for Windows 1.x, 2.0
    cdrWS7, //	2060	WordStar 7.0
    cdrWS2000,  //2061	WordStar 2000
    cdrWSD, //	2061	WordStar 2000
    cdrXY, //	2062	XYWrite for Windows 4.0
    cdrSAM, //	2063	Ami Professional (Import only)
    cdrXLS, //	2064	Microsoft Excel (Import only)
    cdrWKS, //	2066	LOTUS 1-2-3 (Import only)
    cdrQuattroPro, //	2067	Corel Quattro Pro (Import only)
    cdrDOC, //	2068	MS Word 97/2000
    cdrWord2000, //	2068	MS Word 97/2000
    cdrWS6, //	2163	WordStar 5.0, 5.5, 6.0 (Import only)
    cdrWS4, //	2164	WordStar 3.3, 3.31, 3.45, 4.0 (Import only)
    cdrMIF, //	2165	FrameMaker 3.0, 4.0, 5.0 (Import only)
    cdrWPM, //	2166	Corel WordPerfect 1.0, 2.x for Macintosh (Import only)
    cdrCGZ, //	2315	Compressed CGM
}


pub fn cdrFilter_to_value(val:cdrFilter)->i32{
    match val {
        cdrFilter::cdrAutoSense=>{0}, //	0	Detect the proper filter automatically (Import only)
        cdrFilter::cdrBMP=>{769}, //	769	Windows Bitmap
        cdrFilter::cdrPCX=>{770}, //	770	PaintBrush
        cdrFilter::cdrTGA=>{771}, //	771	Targa Bitmap
        cdrFilter::cdrTIFF=>{772}, //	772	TIFF Bitmap
        cdrFilter::cdrGIF=>{773}, //	773	CompuServe Bitmap
        cdrFilter::cdrJPEG=>{774}, //	774	JPEG Bitmaps
        cdrFilter::cdrSCT=>{776}, //	776	Scitex CT Bitmap
        cdrFilter::cdrICO=>{784}, //	784	Windows 3.x/NT Icon Resource (Import only)
        cdrFilter::cdrCUR=>{785}, //	785	Windows 3.x/NT Cursor Resource (Import only)
        cdrFilter::cdrEXE=>{786}, //	786	Windows 3.x/NT Bitmap Resource (Import only)
        cdrFilter::cdrIMG=>{787}, //	787	GEM Paint File
        cdrFilter::cdrPSD=>{788}, //	788	Adobe Photoshop
        cdrFilter::cdrPP4=>{789}, //	789	Picture Publisher 4 (Import only)
        cdrFilter::cdrMAC=>{791}, //	791	MACPaint Bitmap
        cdrFilter::cdrOS2BMP=>{792}, //	792	OS/2 Bitmap
        cdrFilter::cdrWI=>{793}, //	793	Wavelet Compressed Bitmap
        cdrFilter::cdrCAL=>{800}, //	800	CALS Compressed Bitmap
        cdrFilter::cdrDCS=>{801}, //	801	Desktop Color Separations (Export only)
        cdrFilter::cdrPNG=>{802}, //	802	Portable Network Graphics
        cdrFilter::cdrEPSPhotoPaint=>{804}, //	804	Corel PHOTO-PAINT Encapsulated PostScript
        cdrFilter::cdrRIFF=>{807}, //807	Corel Painter 5/6 (Import only)
        cdrFilter::cdrXPM=>{809}, //	809	XPixMap Image
        cdrFilter::cdrXCF=>{816}, //	816	Gimp Image (Import only)
        cdrFilter::cdrJPG2000Import=>{818}, //	818	JPG2000 Bitmaps (Import only)
        cdrFilter::cdrJP2=>{820}, //	820	JPG2000 Standard (Export only)
        cdrFilter::cdrJPC=>{821}, //	821	JPG2000 Codestream (Export only)
        cdrFilter::cdrRAW=>{823}, //	823	Camera RAW (Import only)
        cdrFilter::cdrHEIF=>{824}, //	824	HEIF (Import only)
        cdrFilter::cdrWEBP=>{825}, //	825	WEBP Google Web Picture
        cdrFilter::cdrCGM=>{1280}, //	1280	Computer Graphics Metafile
        cdrFilter::cdrHPGL=>{1281}, //	1281	HPGL Plotter File
        cdrFilter::cdrPLT=>{1281}, //	1281	HPGL Plotter File
        cdrFilter::cdrGEM=>{1284}, //	1284	GEM File
        cdrFilter::cdrWPG=>{1287}, //	1287	Corel WordPerfect Graphic
        cdrFilter::cdrPSEncapsulated=>{1289}, //	1289	Encapsulated PostScript
        cdrFilter::cdrEPS=>{1289}, //	1289	Encapsulated PostScript
        cdrFilter::cdrPSInterpreted=>{1290}, //	1290	PostScript Interpreted (Import only)
        cdrFilter::cdrMET=>{1291}, //	1291	MET MetaFile (Import only)
        cdrFilter::cdrNAP=>{1292}, //	1292	NAP MetaFile (Import only)
        cdrFilter::cdrPICT=>{1293}, //	1293	Macintosh PICT
        cdrFilter::cdrWMF=>{1294}, //	1294	Windows Metafile
        cdrFilter::cdrDXF=>{1296}, //	1296	AutoCAD (DXF)
        cdrFilter::cdrEMF=>{1300}, //	1300	Enhanced Windows Metafile
        cdrFilter::cdrTTF=>{1302}, //	1302	True Type Font (Export only)
        cdrFilter::cdrAT1=>{1303}, //	1303	Adobe Type 1 Font (Export only)
        cdrFilter::cdrAI=>{1305}, //	1305	Adobe Illustrator
        cdrFilter::cdrVSD=>{1315}, //	1315	Visio (Import only)
        cdrFilter::cdrDWG=>{1328}, //	1328	AutoCAD (DWG)
        cdrFilter::cdrFMV=>{1329}, //	1329	Frame Vector Metafile (Import only)
        cdrFilter::cdrPDF=>{1333}, //	1333	Adobe Portable Document Format, Adobe Illustrator 9 (Import only)
        cdrFilter::cdrAI9=>{1333}, //	1333	Adobe Illustrator 9, Adobe Portable Document Format (Import only)
        cdrFilter::cdrPIC=>{1334}, //	1334	Lotus Pic (Import only)
        cdrFilter::cdrDRW=>{1339}, //	1339	Corel/Micrografx Designer (Import only)
        cdrFilter::cdrDSF=>{1339}, //	1339	Corel/Micrografx Designer (Import only)
        cdrFilter::cdrFH=>{1344}, //	1344	Macromedia Freehand (Import only)
        cdrFilter::cdrSVG=>{1345}, //	1345	Scalable Vector Graphics
        cdrFilter::cdrSVGZ=>{1347}, //	1347	Compressed SVG
        cdrFilter::cdrPUB=>{1349}, //	1349	Microsoft Publisher (Import only)
        cdrFilter::cdrAVI=>{1536}, //	1536	Video for Windows
        cdrFilter::cdrPPT=>{1548}, //	1548	Microsoft PowerPoint (Import only)
        cdrFilter::cdrHTML=>{1557}, //	1557	HyperText Markup Language (Import only)
        cdrFilter::cdrGIFAnimation=>{1558}, //	1558	GIF Animation
        cdrFilter::cdr3DMF=>{1559}, //	1559	3DMF - 3D Model (Import only)
        cdrFilter::cdrCMX6=>{1793}, //	1793	Corel Presentation Exchange
        cdrFilter::cdrCMX5=>{1794}, //	1794	Corel Presentation Exchange 5.0
        cdrFilter::cdrCDR=>{1795}, //	1795	CorelDRAW
        cdrFilter::cdrCDX=>{1796}, //	1796	CorelDRAW Compressed (Import only)
        cdrFilter::cdrCPX=>{1797}, //	1797	Corel CMX Compressed (Import only)
        cdrFilter::cdrCPT8=>{1799}, //	1799	Corel PHOTO-PAINT 7/8 Image (Export only)
        cdrFilter::cdrCDT=>{1800}, //	1800	CorelDRAW Template
        cdrFilter::cdrPAT=>{1801}, //	1801	CorelDRAW Pattern File
        cdrFilter::cdrCLK=>{1802}, //	1802	Corel R.A.V.E.
        cdrFilter::cdrDES=>{1805}, //	1805	CorelDESIGNER
        cdrFilter::cdrSHW=>{1806}, //	1806	Corel Presentations (Import only)
        cdrFilter::cdrCPT=>{1808}, //	1808	Corel PHOTO-PAINT Image
        cdrFilter::cdrCPT10=>{1808}, //	1808	Corel PHOTO-PAINT 9/10 Image
        cdrFilter::cdrCPT11=>{1808}, //	1808	Corel PHOTO-PAINT 11 Image
        cdrFilter::cdrCPT9=>{1808}, //	1808	Corel PHOTO-PAINT 9/10 Image
        cdrFilter::cdrCSL=>{1810}, //	1810	Corel Symbol Library
        cdrFilter::cdrCMX64=>{1811}, //	1811	Corel Presentation Exchange 64-bit
        cdrFilter::cdrTXT=>{2048}, //	2048	ANSI Text
        cdrFilter::cdrWord95=>{2049}, //	2049	MS Word for Windows 6/7
        cdrFilter::cdrWord2=>{2050}, //	2050	MS Word for Windows 2.x
        cdrFilter::cdrWord55=>{2051}, //	2051	MS Word 3.0, 4.0, 5.0, 5.5
        cdrFilter::cdrMacWord5=>{2052}, //	2052	MS Word for Macintosh 4.0, 5.0
        cdrFilter::cdrRTF=>{2053}, //	2053	Rich Text Format
        cdrFilter::cdrWP9=>{2055}, //	2055	Corel WordPerfect 6+
        cdrFilter::cdrWPD=>{2055}, //	2055	Corel WordPerfect 6+
        cdrFilter::cdrWP51=>{2056}, //	2056	Corel WordPerfect 5.1
        cdrFilter::cdrWP50=>{2057}, //	2057	Corel WordPerfect 5.0
        cdrFilter::cdrWP4=>{2058}, //	2058	Corel WordPerfect 4.2
        cdrFilter::cdrWSW=>{2059}, //	2059	WordStar for Windows 1.x, 2.0
        cdrFilter::cdrWS7=>{2060}, //	2060	WordStar 7.0
        cdrFilter::cdrWS2000=>{2061},  //2061	WordStar 2000
        cdrFilter::cdrWSD=>{2061}, //	2061	WordStar 2000
        cdrFilter::cdrXY=>{2062}, //	2062	XYWrite for Windows 4.0
        cdrFilter::cdrSAM=>{2063}, //	2063	Ami Professional (Import only)
        cdrFilter::cdrXLS=>{2064}, //	2064	Microsoft Excel (Import only)
        cdrFilter::cdrWKS=>{2066}, //	2066	LOTUS 1-2-3 (Import only)
        cdrFilter::cdrQuattroPro=>{2067}, //	2067	Corel Quattro Pro (Import only)
        cdrFilter::cdrDOC=>{2068}, //	2068	MS Word 97/2000
        cdrFilter::cdrWord2000=>{2068}, //	2068	MS Word 97/2000
        cdrFilter::cdrWS6=>{2163}, //	2163	WordStar 5.0, 5.5, 6.0 (Import only)
        cdrFilter::cdrWS4=>{2164}, //	2164	WordStar 3.3, 3.31, 3.45, 4.0 (Import only)
        cdrFilter::cdrMIF=>{2165}, //	2165	FrameMaker 3.0, 4.0, 5.0 (Import only)
        cdrFilter::cdrWPM=>{2166}, //	2166	Corel WordPerfect 1.0, 2.x for Macintosh (Import only)
        cdrFilter::cdrCGZ=>{2315}, //	2315	Compressed CGM
    }
}
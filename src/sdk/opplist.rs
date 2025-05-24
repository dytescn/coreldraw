// 系统中所有的枚举型

//  enum cdrFilter {
//     cdrAutoSense = 0,
//     cdr3DMF = 1559,
//     cdrAI = 1305,
//     cdrAI9 = 1333,
//     cdrAT1 = 1303,
//     cdrAVI = 1536,
//     cdrBMP = 769,
//     cdrCAL = 800,
//     cdrCDR = 1795,
//     cdrCDT = 1800,
//     cdrCDX = 1796,
//     cdrCGM = 1280,
//     cdrCLK = 1802,
//     cdrCMX5 = 1794,
//     cdrCMX6 = 1793,
//     // cdrCPT = 1808,
//     // cdrCPT10 = 1808,
//     // cdrCPT11 = 1808,
//     cdrCPT8 = 1799,
//     cdrCPT9 = 1808,
//     cdrCPX = 1797,
//     cdrCUR = 785,
//     cdrDCS = 801,
//     cdrDOC = 2068,
//     // cdrDRW = 1339,
//     // cdrDSF = 1339,
//     cdrDWG = 1328,
//     cdrDXF = 1296,
//     cdrEMF = 1300,
//     cdrEPS = 1289,
//     cdrEPSPhotoPaint = 804,
//     cdrEXE = 786,
//     cdrFH = 1344,
//     cdrFMV = 1329,
//     cdrFPX = 806,
//     cdrGEM = 1284,
//     cdrGIF = 773,
//     cdrGIFAnimation = 1558,
//     cdrHPGL = 1281,
//     cdrHTML = 1557,
//     cdrICO = 784,
//     cdrIMG = 787,
//     cdrJP2 = 820,
//     cdrJPC = 821,
//     cdrJPEG = 774,
//     cdrJPG2000Import = 818,
//     cdrMAC = 791,
//     cdrMacWord5 = 2052,
//     cdrMET = 1291,
//     cdrMIF = 2165,
//     cdrNAP = 1292,
//     cdrOS2BMP = 792,
//     cdrPAT = 1801,
//     cdrPCD = 775,
//     cdrPCX = 770,
//     // cdrPDF = 1333,
//     cdrPIC = 1334,
//     cdrPICT = 1293,
//     cdrPLT = 1281,
//     cdrPNG = 802,
//     cdrPP4 = 789,
//     cdrPP5 = 803,
//     cdrPPF = 819,
//     cdrPPT = 1548,
//     cdrPSD = 788,
//     cdrPSEncapsulated = 1289,
//     cdrPSInterpreted = 1290,
//     cdrQTM = 1542,
//     cdrQTVR = 1560,
//     cdrQuattroPro = 2067,
//     cdrRIFF = 807,
//     cdrRTF = 2053,
//     cdrSAM = 2063,
//     cdrSCT = 776,
//     cdrSHW = 1806,
//     cdrSVG = 1345,
//     cdrSVGZ = 1347,
//     cdrSWF = 1343,
//     cdrTGA = 771,
//     cdrTIFF = 772,
//     cdrTTF = 1302,
//     cdrTXT = 2048,
//     cdrVSD = 1315,
//     cdrWI = 793,
//     cdrWKS = 2066,
//     cdrWMF = 1294,
//     cdrWord2 = 2050,
//     cdrWord55 = 2051,
//     cdrWord95 = 2049,
//     cdrWord2000 = 2068,
//     cdrWP4 = 2058,
//     cdrWP50 = 2057,
//     cdrWP51 = 2056,
//     cdrWP9 = 2055,
//     cdrWPD = 2055,
//     cdrWPG = 1287,
//     cdrWPM = 2166,
//     cdrWS4 = 2164,
//     cdrWS6 = 2163,
//     cdrWS7 = 2060,
//     cdrWS2000 = 2061,
//     cdrWSD = 2061,
//     cdrWSW = 2059,
//     cdrWVL = 793,
//     cdrXCF = 816,
//     cdrXLS = 2064,
//     cdrXPM = 809,
//     cdrXY = 2062,
//     cdrCSL = 1810,
//     cdrDES = 1805,
//     cdrPSP = 822,
//     cdrRAW = 823,
//     cdrCGZ = 2315,
//     cdrCMX64 = 1811,
//     cdrPUB = 1349,
// }

 enum cdrFillType {
    cdrNoFill = 0,
    cdrUniformFill = 1,
    cdrFountainFill = 2,
    cdrPostscriptFill = 3,
    cdrTextureFill = 8,
    cdrPatternFill = 9,
    cdrHatchFill = 10,
}

//  enum cdrFountainFillType {
//     cdrLinearFountainFill = 1,
//     cdrRadialFountainFill = 2,
//     cdrConicalFountainFill = 3,
//     cdrSquareFountainFill = 4,
// }

//  enum cdrFountainFillBlendType {
//     cdrDirectFountainFillBlend = 0,
//     cdrRainbowCWFountainFillBlend = 1,
//     cdrRainbowCCWFountainFillBlend = 2,
//     cdrCustomFountainFillBlend = 3,
// }

//  enum cdrPaletteType {
//     cdrFixedPalette = 0,
//     cdrCustomPalette = 1,
//     cdrDocumentPalette = 2,
// }

//  enum cdrPaletteID {
//     cdrCustom = 0,
//     cdrTRUMATCH = 1,
//     cdrPANTONEProcess = 2,
//     cdrPANTONECorel8 = 3,
//     cdrUniform = 7,
//     cdrFOCOLTONE = 8,
//     cdrSpectraMaster = 9,
//     cdrTOYO = 10,
//     cdrDIC = 11,
//     cdrLab = 13,
//     cdrHKS = 23,
//     cdrWebSafe = 25,
//     cdrPANTONEMetallic = 30,
//     cdrPANTONEPastelCoated = 31,
//     cdrPANTONEPastelUncoated = 32,
//     cdrPANTONEHexCoated = 33,
//     cdrPANTONEHexUncoated = 34,
//     cdrPANTONEMatte = 35,
//     cdrPANTONECoated = 36,
//     cdrPANTONEUncoated = 37,
//     cdrPANTONEProcessCoatedEURO = 38,
//     cdrPANTONESolid2ProcessEURO = 39,
//     cdrSVGPalette = 40,
//     cdrUserInks = 16,
// }

//  enum cdrFittedOrientation {
//     cdrRotateOrientation = 0,
//     cdrVerticalSkewOrientation = 1,
//     cdrHorizontalSkewOrientation = 2,
//     cdrUprightOrientation = 3,
//     cdrTransformOrientation = 4,
// }

//  enum cdrFittedVertPlacement {
//     cdrCustomVertPlacement = 0,
//     cdrBaselineVertPlacement = 1,
//     cdrAscenderVertPlacement = 2,
//     cdrDescenderVertPlacement = 3,
//     cdrCenterVertPlacement = 4,
// }

//  enum cdrFittedPlacement {
//     cdrLeftPlacement = 0,
//     cdrRightPlacement = 1,
//     cdrCenterPlacement = 2,
// }

//  enum cdrFittedQuadrant {
//     cdrLeftQuadrant = 0,
//     cdrRightQuadrant = 1,
//     cdrTopQuadrant = 2,
//     cdrBottomQuadrant = 3,
// }

//  enum cdrFontStyle {
//     cdrNormalFontStyle = 0,
//     cdrBoldFontStyle = 1,
//     cdrItalicFontStyle = 2,
//     cdrBoldItalicFontStyle = 3,
//     cdrThinFontStyle = 4,
//     cdrThinItalicFontStyle = 5,
//     cdrExtraLightFontStyle = 6,
//     cdrExtraLightItalicFontStyle = 7,
//     cdrMediumFontStyle = 8,
//     cdrMediumItalicFontStyle = 9,
//     cdrSemiBoldFontStyle = 10,
//     cdrSemiBoldItalicFontStyle = 11,
//     cdrExtraBoldFontStyle = 12,
//     cdrExtraBoldItalicFontStyle = 13,
//     cdrHeavyFontStyle = 14,
//     cdrHeavyItalicFontStyle = 15,
//     cdrMixedFontStyle = 16,
//     cdrLightFontStyle = 17,
//     cdrLightItalicFontStyle = 18,
// }

//  enum cdrFontLine {
//     cdrNoFontLine = 0,
//     cdrSingleThinFontLine = 1,
//     cdrSingleThinWordFontLine = 2,
//     cdrSingleThickFontLine = 3,
//     cdrSingleThickWordFontLine = 4,
//     cdrDoubleThinFontLine = 5,
//     cdrDoubleThinWordFontLine = 6,
//     cdrMixedFontLine = 7,
// }

//  enum cdrFontCase {
//     cdrNormalFontCase = 0,
//     cdrSmallCapsFontCase = 1,
//     cdrAllCapsFontCase = 2,
//     cdrMixedFontCase = 3,
// }

//  enum cdrFontPosition {
//     cdrNormalFontPosition = 0,
//     cdrSubscriptFontPosition = 1,
//     cdrSuperscriptFontPosition = 2,
//     cdrMixedFontPosition = 3,
// }

//  enum cdrAlignment {
//     cdrNoAlignment = 0,
//     cdrLeftAlignment = 1,
//     cdrRightAlignment = 2,
//     cdrCenterAlignment = 3,
//     cdrFullJustifyAlignment = 4,
//     cdrForceJustifyAlignment = 5,
//     cdrMixedAlignment = 6,
// }

//  enum cdrLineSpacingType {
//     cdrPercentOfCharacterHeightLineSpacing = 0,
//     cdrPointLineSpacing = 1,
//     cdrPercentOfPointSizeLineSpacing = 2,
//     cdrMixedLineSpacing = 3,
// }

//  enum cdrColorType {
//     cdrColorPantone = 1,
//     cdrColorCMYK = 2,
//     cdrColorCMY = 4,
//     cdrColorRGB = 5,
//     cdrColorHSB = 6,
//     cdrColorHLS = 7,
//     cdrColorBlackAndWhite = 8,
//     cdrColorGray = 9,
//     cdrColorYIQ = 11,
//     cdrColorLab = 12,
//     cdrColorPantoneHex = 14,
//     cdrColorRegistration = 20,
//     cdrColorSpot = 25,
//     cdrColorMixed = 99,
//     cdrColorUserInk = 22,
//     cdrColorMultiChannel = 26,
// }

//  enum cdrImageType {
//     cdrBlackAndWhiteImage = 0,
//     cdr16ColorsImage = 1,
//     cdrGrayscaleImage = 2,
//     cdrPalettedImage = 3,
//     cdrRGBColorImage = 4,
//     cdrCMYKColorImage = 5,
//     cdrDuotoneImage = 6,
//     cdrLABImage = 7,
//     cdrCMYKMultiChannelImage = 8,
//     cdrRGBMultiChannelImage = 9,
//     cdrSpotMultiChannelImage = 10,
// }

//  enum cdrViewType {
//     cdrSimpleWireframeView = 0,
//     cdrWireframeView = 1,
//     cdrDraftView = 2,
//     cdrNormalView = 3,
//     cdrEnhancedView = 4,
//     cdrEnhancedViewWithOverprints = 5,
//     cdrPixelView = 6,
// }

//  enum cdrWindowState {
//     cdrWindowNormal = 1,
//     cdrWindowMinimized = 2,
//     cdrWindowMaximized = 3,
//     cdrWindowRestore = 9,
// }

//  enum cdrPatternFillType {
//     cdrTwoColorPattern = 0,
//     cdrFullColorPattern = 1,
//     cdrBitmapPattern = 2,
// }

//  enum cdrTileOffsetType {
//     cdrTileOffsetRow = 0,
//     cdrTileOffsetColumn = 1,
// }

//  enum cdrPatternCanvasSize {
//     cdrPatternCanvas16x16 = 0,
//     cdrPatternCanvas32x32 = 1,
//     cdrPatternCanvas64x64 = 2,
//     cdrPatternCanvasCustom = 3,
// }

//  enum cdrFlipAxes {
//     cdrFlipHorizontal = 1,
//     cdrFlipVertical = 2,
//     cdrFlipBoth = 3,
// }

//  enum cdrTexturePropertyType {
//     cdrTexturePropertyNumeric = 0,
//     cdrTexturePropertyColorRGB = 1,
//     cdrTexturePropertyColorHSB = 2,
//     cdrTexturePropertyColorCMYK = 3,
// }

//  enum cdrWindowArrangeStyle {
//     cdrTileHorizontally = 0,
//     cdrTileVertically = 1,
//     cdrCascade = 2,
// }

//  enum cdrTransparencyType {
//     cdrNoTransparency = 0,
//     cdrUniformTransparency = 1,
//     cdrFountainTransparency = 2,
//     cdrPatternTransparency = 3,
//     cdrTextureTransparency = 4,
// }

//  enum cdrTransparencyAppliedTo {
//     cdrApplyToFill = 0,
//     cdrApplyToOutline = 1,
//     cdrApplyToFillAndOutline = 2,
// }

//  enum cdrImageMode {
//     cdrImageBlackWhite = 0,
//     cdrImageGrayscale16 = 1,
//     cdrImageGrayscale = 2,
//     cdrImagePaletted = 3,
//     cdrImageRGB = 4,
//     cdrImageCMYK = 5,
//     cdrImageDuotone = 6,
//     cdrImageLAB = 7,
//     cdrImageRGB48 = 8,
//     cdrImageMultiChannel = 9,
// }

//  enum cdrImagePaletteType {
//     cdrPaletteUniform = 0,
//     cdrPaletteStdVGA = 1,
//     cdrPaletteAdaptive = 2,
//     cdrPaletteOptimized = 3,
//     cdrPaletteBlackBody = 4,
//     cdrPaletteGrayscale = 5,
//     cdrPaletteSystem = 6,
//     cdrPaletteIE = 7,
//     cdrPaletteNetscape = 8,
//     cdrPaletteCustom = 9,
// }

//  enum cdrDitherType {
//     cdrDitherNone = 0,
//     cdrDitherOrdered = 1,
//     cdrDitherJarvis = 2,
//     cdrDitherStucki = 3,
//     cdrDitherFloyd = 4,
// }

//  enum cdrRenderType {
//     cdrRenderLineArt = 0,
//     cdrRenderOrdered = 1,
//     cdrRenderJarvis = 2,
//     cdrRenderStucki = 3,
//     cdrRenderFloyd = 4,
//     cdrRenderHalftone = 5,
//     cdrRenderCardinality = 6,
// }

//  enum cdrHalftoneType {
//     cdrHalftoneSquare = 0,
//     cdrHalftoneRound = 1,
//     cdrHalftoneLine = 2,
//     cdrHalftoneCross = 3,
//     cdrHalftoneFixed4x4 = 4,
//     cdrHalftoneFixed8x8 = 5,
// }

//  enum cdrDuotoneType {
//     cdrMonotone = 0,
//     cdrDuotone = 1,
//     cdrTritone = 2,
//     cdrQuadtone = 3,
// }

//  enum cdrUnit {
//     cdrTenthMicron = 0,
//     cdrInch = 1,
//     cdrFoot = 2,
//     cdrMillimeter = 3,
//     cdrCentimeter = 4,
//     cdrPixel = 5,
//     cdrMile = 6,
//     cdrMeter = 7,
//     cdrKilometer = 8,
//     cdrDidots = 9,
//     cdrAgate = 10,
//     cdrYard = 11,
//     cdrPica = 12,
//     cdrCicero = 13,
//     cdrPoint = 14,
//     cdrUnitQ = 15,
//     cdrUnitH = 16,
// }

// //  enum cdrCompressionType {
// //     cdrCompressionNone = 0,
// //     cdrCompressionLZW = 1,
// //     cdrCompressionPackBits = 2,
// //     cdrCompressionHuffman = 3,
// //     cdrCompressionCCITT3_1d = 4,
// //     cdrCompressionCCITT3_2d = 5,
// //     cdrCompressionCCITT4 = 6,
// //     cdrCompressionRLE_LW = 7,
// //     cdrCompressionZIP = 7,
// //     cdrCompressionJPEG = 8,
// // }

//  enum cdrMergeMode {
//     cdrMergeNormal = 0,
//     cdrMergeAND = 1,
//     cdrMergeOR = 2,
//     cdrMergeXOR = 3,
//     cdrMergeInvert = 6,
//     cdrMergeAdd = 7,
//     cdrMergeSubtract = 8,
//     cdrMergeMultiply = 9,
//     cdrMergeDivide = 10,
//     cdrMergeIfLighter = 11,
//     cdrMergeIfDarker = 12,
//     cdrMergeTexturize = 13,
//     cdrMergeColor = 14,
//     cdrMergeHue = 15,
//     cdrMergeSaturation = 16,
//     cdrMergeLightness = 17,
//     cdrMergeRed = 18,
//     cdrMergeGreen = 19,
//     cdrMergeBlue = 20,
//     cdrMergeDifference = 24,
//     cdrMergeBehind = 27,
//     cdrMergeScreen = 28,
//     cdrMergeOverlay = 29,
//     cdrMergeSoftlight = 30,
//     cdrMergeHardlight = 31,
//     cdrMergeDodge = 33,
//     cdrMergeBurn = 34,
//     cdrMergeExclusion = 36,
// }

//  enum cdrReferencePoint {
//     cdrTopRight = 1,
//     cdrTopMiddle = 2,
//     cdrTopLeft = 3,
//     cdrMiddleLeft = 4,
//     cdrBottomLeft = 5,
//     cdrBottomMiddle = 6,
//     cdrBottomRight = 7,
//     cdrMiddleRight = 8,
//     cdrCenter = 9,
// }

//  enum cdrEllipseType {
//     cdrEllipse = 0,
//     cdrArc = 1,
//     cdrPie = 2,
// }

//  enum cdrPolygonType {
//     cdrPolygon = 0,
//     cdrStar = 1,
//     cdrPolygonAsStar = 2,
// }

//  enum cdrSpiralType {
//     cdrSymmetric = 0,
//     cdrLogarithmic = 1,
// }

//  enum cdrShapeType {
//     cdrNoShape = 0,
//     cdrRectangleShape = 1,
//     cdrEllipseShape = 2,
//     cdrCurveShape = 3,
//     cdrPolygonShape = 4,
//     cdrBitmapShape = 5,
//     cdrTextShape = 6,
//     cdrGroupShape = 7,
//     cdrSelectionShape = 8,
//     cdrGuidelineShape = 9,
//     cdrBlendGroupShape = 10,
//     cdrExtrudeGroupShape = 11,
//     cdrOLEObjectShape = 12,
//     cdrContourGroupShape = 13,
//     cdrLinearDimensionShape = 14,
//     cdrBevelGroupShape = 15,
//     cdrDropShadowGroupShape = 16,
//     cdr3DObjectShape = 17,
//     cdrArtisticMediaGroupShape = 18,
//     cdrConnectorShape = 19,
//     cdrMeshFillShape = 20,
//     cdrCustomShape = 21,
//     cdrCustomEffectGroupShape = 22,
//     cdrSymbolShape = 23,
//     cdrHTMLFormObjectShape = 24,
//     cdrHTMLActiveObjectShape = 25,
//     cdrPerfectShape = 26,
//     cdrEPSShape = 27,
// }

//  enum cdrPageOrientation {
//     cdrPortrait = 0,
//     cdrLandscape = 1,
// }

//  enum cdrAntiAliasingType {
//     cdrNoAntiAliasing = 0,
//     cdrNormalAntiAliasing = 1,
//     cdrSupersampling = 2,
// }

//  enum cdrOutlineType {
//     cdrNoOutline = 0,
//     cdrOutline = 1,
//     cdrEnhancedOutline = 2,
// }

//  enum cdrTextType {
//     cdrArtisticText = 0,
//     cdrParagraphText = 1,
//     cdrArtisticFittedText = 2,
//     cdrParagraphFittedText = 3,
// }

//  enum cdrTextIndexingType {
//     cdrCharacterIndexing = 0,
//     cdrWordIndexing = 1,
//     cdrParagraphIndexing = 2,
// }

//  enum cdrTextFrames {
//     cdrThisFrameOnly = 0,
//     cdrStartAtThisFrame = 1,
//     cdrAllFrames = 2,
// }

//  enum cdrExportRange {
//     cdrAllPages = 0,
//     cdrCurrentPage = 1,
//     cdrSelection = 2,
// }

//  enum cdrThumbnailSize {
//     cdrNoThumbnail = 0,
//     cdr1KMonoThumbnail = 1,
//     cdr5KColorThumbnail = 2,
//     cdr10KColorThumbnail = 3,
// }

//  enum cdrFileVersion {
//     cdrCurrentVersion = 0,
//     cdrVersion5 = 5,
//     cdrVersion6 = 6,
//     cdrVersion7 = 7,
//     cdrVersion8 = 8,
//     cdrVersion9 = 9,
//     cdrVersion10 = 10,
//     cdrVersion11 = 11,
//     cdrVersion12 = 12,
//     cdrVersion13 = 13,
//     cdrVersion1 = 1,
//     cdrVersion2 = 2,
//     cdrVersion3 = 3,
//     cdrVersion4 = 4,
//     cdrVersion105 = 105,
//     cdrVersion14 = 14,
//     cdrVersion15 = 15,
//     cdrVersion16 = 16,
//     cdrVersion17 = 17,
//     cdrVersion18 = 18,
//     cdrVersion19 = 19,
//     cdrVersion20 = 20,
//     cdrVersion21 = 21,
// }

// //  enum cdrPointType {
// //     cdrSnapPoint = 0,
// //     cdrFreePoint = 1,
// //     cdrSnapPointUser = 1,
// //     cdrSnapPointObject = 2,
// //     cdrSnapPointBBox = 4,
// //     cdrSnapPointEdge = 8,
// //     cdrSnapPointFree = 16,
// //     cdrSnapPointAuto = 32,
// // }

//  enum cdrPresetPoint {
//     cdrTopLeftPoint = 0xffffffff,
//     cdrTopPoint = 0xfffffffe,
//     cdrTopRightPoint = 0xfffffffd,
//     cdrRightPoint = 0xfffffffc,
//     cdrBottomRightPoint = 0xfffffffb,
//     cdrBottomPoint = 0xfffffffa,
//     cdrBottomLeftPoint = 0xfffffff9,
//     cdrLeftPoint = 0xfffffff8,
//     cdrCenterPoint = 0xfffffff7,
//     cdrFirstPoint = 0xfffffff6,
//     cdrLastPoint = 0xfffffff5,
// }

//  enum cdrNodeType {
//     cdrCuspNode = 0,
//     cdrSmoothNode = 1,
//     cdrSymmetricalNode = 2,
//     cdrMixedNodes = 3,
// }

//  enum cdrSegmentType {
//     cdrLineSegment = 0,
//     cdrCurveSegment = 1,
//     cdrMixedSegments = 2,
// }

//  enum cdrSegmentOffsetType {
//     cdrAbsoluteSegmentOffset = 0,
//     cdrRelativeSegmentOffset = 1,
//     cdrParamSegmentOffset = 2,
// }

// //  enum cdrCursorShape {
// //     cdrCursorWinAppStarting = 0,
// //     cdrCursorWinArrow = 1,
// //     cdrCursorWinCross = 2,
// //     cdrCursorWinHelp = 3,
// //     cdrCursorWinIbeam = 4,
// //     cdrCursorWinNo = 5,
// //     cdrCursorWinSizeAll = 6,
// //     cdrCursorWinSizeNeSw = 7,
// //     cdrCursorWinSizeNs = 8,
// //     cdrCursorWinSizeNwSe = 9,
// //     cdrCursorWinSizeWe = 10,
// //     cdrCursorWinUpArrow = 11,
// //     cdrCursorWinWait = 12,
// //     cdrCursorSmallcrosshair = 301,
// //     cdrCursorPick = 305,
// //     cdrCursorNodeEdit = 306,
// //     cdrCursorEyeDrop = 326,
// //     cdrCursorExtPick = 351,
// //     cdrCursorPickNone = 305,
// //     cdrCursorPenJoin = 380,
// //     cdrCursorPickOvertarget = 396,
// //     cdrCursorTrimSingle = 428,
// //     cdrCursorWeldSingle = 430,
// //     cdrCursorIntersectSingle = 432,
// // }

//  enum cdrPositionOfPointOverShape {
//     cdrOutsideShape = 0,
//     cdrOnMarginOfShape = 1,
//     cdrInsideShape = 2,
// }

//  enum cdrOutlineLineCaps {
//     cdrOutlineUndefinedLineCaps = 0xffffffff,
//     cdrOutlineButtLineCaps = 0,
//     cdrOutlineRoundLineCaps = 1,
//     cdrOutlineSquareLineCaps = 2,
// }

//  enum cdrOutlineLineJoin {
//     cdrOutlineUndefinedLineJoin = 0xffffffff,
//     cdrOutlineMiterLineJoin = 0,
//     cdrOutlineRoundLineJoin = 1,
//     cdrOutlineBevelLineJoin = 2,
// }

//  enum cdrShapeEnumDirection {
//     cdrShapeEnumTopFirst = 0,
//     cdrShapeEnumBottomFirst = 1,
// }

//  enum cdrPageBackground {
//     cdrPageBackgroundNone = 0,
//     cdrPageBackgroundSolid = 1,
//     cdrPageBackgroundBitmap = 2,
// }

//  enum cdrEffectType {
//     cdrBlend = 0,
//     cdrExtrude = 1,
//     cdrEnvelope = 2,
//     cdrTextOnPath = 3,
//     cdrControlPath = 4,
//     cdrDropShadow = 5,
//     cdrContour = 6,
//     cdrDistortion = 7,
//     cdrPerspective = 8,
//     cdrLens = 9,
//     cdrCustomEffect = 10,
// }

//  enum cdrBlendMode {
//     cdrBlendSteps = 0,
//     cdrBlendSpacing = 1,
// }

//  enum cdrExtrudeType {
//     cdrExtrudeSmallBack = 0,
//     cdrExtrudeSmallFront = 1,
//     cdrExtrudeBigBack = 2,
//     cdrExtrudeBigFront = 3,
//     cdrExtrudeBackParallel = 4,
//     cdrExtrudeFrontParallel = 5,
// }

//  enum cdrExtrudeShading {
//     cdrExtrudeObjectFill = 0,
//     cdrExtrudeSolidFill = 1,
//     cdrExtrudeColorShading = 2,
// }

//  enum cdrFeatherType {
//     cdrFeatherInside = 0,
//     cdrFeatherMiddle = 1,
//     cdrFeatherOutside = 2,
//     cdrFeatherAverage = 3,
//     cdrFeatherGaussianBlur = 4,
// }

//  enum cdrEdgeType {
//     cdrEdgeLinear = 0,
//     cdrEdgeSquared = 1,
//     cdrEdgeFlat = 2,
//     cdrEdgeInverseSquared = 3,
//     cdrEdgeMesa = 4,
//     cdrEdgeGaussian = 5,
// }

//  enum cdrDropShadowType {
//     cdrDropShadowFlat = 0,
//     cdrDropShadowBottom = 1,
//     cdrDropShadowTop = 2,
//     cdrDropShadowLeft = 3,
//     cdrDropShadowRight = 4,
// }

//  enum cdrExtrudeLightPosition {
//     cdrLightFrontTopLeft = 0,
//     cdrLightFrontTop = 1,
//     cdrLightFrontTopRight = 2,
//     cdrLightFrontLeft = 3,
//     cdrLightFrontCenter = 4,
//     cdrLightFrontRight = 5,
//     cdrLightFrontBottomLeft = 6,
//     cdrLightFrontBottom = 7,
//     cdrLightFrontBottomRight = 8,
//     cdrLightBackTopLeft = 9,
//     cdrLightBackTop = 10,
//     cdrLightBackTopRight = 11,
//     cdrLightBackRight = 14,
//     cdrLightBackBottomRight = 17,
// }

//  enum cdrExtrudeVPType {
//     cdrVPLockedToShape = 0,
//     cdrVPLockedToPage = 1,
//     cdrVPShared = 2,
// }

//  enum cdrEnvelopeMode {
//     cdrEnvelopeHorizontal = 0,
//     cdrEnvelopeOriginal = 1,
//     cdrEnvelopePutty = 2,
//     cdrEnvelopeVertical = 3,
// }

//  enum cdrLensType {
//     cdrLensMagnify = 0,
//     cdrLensFishEye = 1,
//     cdrLensWireframe = 2,
//     cdrLensColorLimit = 3,
//     cdrLensColorAdd = 4,
//     cdrLensInvert = 5,
//     cdrLensBrighten = 6,
//     cdrLensTintedGrayscale = 7,
//     cdrLensHeatMap = 8,
//     cdrLensTransparency = 9,
//     cdrLensCustomColorMap = 10,
// }

//  enum cdrContourDirection {
//     cdrContourInside = 0,
//     cdrContourOutside = 1,
//     cdrContourToCenter = 2,
// }

//  enum cdrDistortionType {
//     cdrDistortionPushPull = 0,
//     cdrDistortionZipper = 1,
//     cdrDistortionTwister = 2,
//     cdrDistortionCustom = 3,
// }

//  enum cdrTools {
//     cdrToolNone = 0,
//     cdrToolPick = 1,
//     cdrToolNodeEdit = 2,
//     cdrToolKnife = 64,
//     cdrToolBezierKnife = 81,
//     cdrToolEraser = 68,
//     cdrToolDrawRectangle = 3,
//     cdrToolDrawEllipse = 4,
//     cdrToolDrawText = 7,
//     cdrToolDrawFreehand = 5,
//     cdrToolDrawNaturalPen = 103,
//     cdrToolDrawBezier = 6,
//     cdrToolHorizontalDimension = 54,
//     cdrToolVerticalDimension = 55,
//     cdrToolAutoDimension = 110,
//     cdrToolSlantedDimension = 56,
//     cdrToolLeaderText = 57,
//     cdrToolAngledDimension = 58,
//     cdrToolConnectorLines = 59,
//     cdrToolDrawPolygon = 60,
//     cdrToolDrawSpiral = 62,
//     cdrToolDrawGrid = 63,
//     cdrToolZoom = 66,
//     cdrToolPan = 67,
//     cdrToolFill = 71,
//     cdrToolTransparency = 72,
//     cdrToolInteractiveExtrude = 73,
//     cdrToolBlend = 74,
//     cdrToolRotate = 75,
//     cdrToolReflect = 76,
//     cdrToolScale = 77,
//     cdrToolSkew = 78,
//     cdrToolDistortion = 79,
//     cdrToolContour = 113,
//     cdrToolInsertHTMLFormObject = 111,
//     cdrToolDropShadow = 80,
//     cdrToolDrawConnector = 115,
//     cdrToolEyeDropper = 119,
// }

//  enum cdrGridType {
//     cdrGridDot = 0,
//     cdrGridLine = 1,
// }

//  enum cdrGuideType {
//     cdrAllGuides = 0xffffffff,
//     cdrHorizontalGuide = 0,
//     cdrVerticalGuide = 1,
//     cdrSlantedGuide = 2,
// }

//  enum cdrURLRegion {
//     cdrURLRegionDefault = 0,
//     cdrURLRegionRectangle = 1,
//     cdrURLRegionShape = 2,
// }

//  enum cdrPanoseMatchingType {
//     cdrPanosePrompt = 0,
//     cdrPanoseTemporary = 1,
//     cdrPanosePermanent = 2,
// }

//  enum cdrDataFormatType {
//     cdrFormatGeneral = 0,
//     cdrFormatDateTime = 1,
//     cdrFormatLinearAngular = 2,
//     cdrFormatNumeric = 3,
// }

//  enum cdrShapeLevel {
//     cdrLevelGroup = 0,
//     cdrLevelContainer = 1,
//     cdrLevelLayer = 2,
//     cdrLevelPage = 3,
//     cdrLevelDocument = 4,
// }

//  enum cdrTriState {
//     cdrFalse = 0,
//     cdrTrue = 0xffffffff,
//     cdrUndefined = 0xfffffffe,
// }

//  enum cdrTextChangeCase {
//     cdrTextSentenceCase = 0,
//     cdrTextLowerCase = 1,
//     cdrTextUpperCase = 2,
//     cdrTextTitleCase = 3,
//     cdrTextToggleCase = 4,
// }

// //  enum cdrTextLanguage {
// //     cdrLanguageMixed = 0xffffffff,
// //     cdrLanguageNone = 0,
// //     cdrAfrikaans = 1078,
// //     cdrAlbanian = 1052,
// //     cdrArabicAlgeria = 5121,
// //     cdrArabicBahrain = 15361,
// //     cdrArabicEgypt = 3073,
// //     cdrArabicIraq = 2049,
// //     cdrArabicJordan = 11265,
// //     cdrArabicKuwait = 13313,
// //     cdrArabicLebanon = 12289,
// //     cdrArabicLibya = 4097,
// //     cdrArabicMorocco = 6145,
// //     cdrArabicOman = 8193,
// //     cdrArabicQatar = 16385,
// //     cdrArabic = 1025,
// //     cdrArabicSyria = 10241,
// //     cdrArabicTunisia = 7169,
// //     cdrArabicUAE = 14337,
// //     cdrArabicYemen = 9217,
// //     cdrArmenian = 1067,
// //     cdrAssamese = 1101,
// //     cdrAzeriCyrillic = 2092,
// //     cdrAzeriLatin = 1068,
// //     cdrBasque = 1069,
// //     cdrByelorussian = 1059,
// //     cdrBengali = 1093,
// //     cdrBulgarian = 1026,
// //     cdrBurmese = 1109,
// //     cdrCatalan = 1027,
// //     cdrChineseHongKong = 3076,
// //     cdrChineseMacao = 5124,
// //     cdrSimplifiedChinese = 2052,
// //     cdrChineseSingapore = 4100,
// //     cdrTraditionalChinese = 1028,
// //     cdrCroatian = 1050,
// //     cdrCzech = 1029,
// //     cdrDanish = 1030,
// //     cdrBelgianDutch = 2067,
// //     cdrDutch = 1043,
// //     cdrEnglishAUS = 3081,
// //     cdrEnglishBelize = 10249,
// //     cdrEnglishCanadian = 4105,
// //     cdrEnglishCaribbean = 9225,
// //     cdrEnglishIreland = 6153,
// //     cdrEnglishJamaica = 8201,
// //     cdrEnglishNewZealand = 5129,
// //     cdrEnglishPhilippines = 13321,
// //     cdrEnglishSouthAfrica = 7177,
// //     cdrEnglishTrinidad = 11273,
// //     cdrEnglishUK = 2057,
// //     cdrEnglishUS = 1033,
// //     cdrEnglishZimbabwe = 12297,
// //     cdrEstonian = 1061,
// //     cdrFaeroese = 1080,
// //     cdrFarsi = 1065,
// //     cdrFinnish = 1035,
// //     cdrBelgianFrench = 2060,
// //     cdrFrenchCameroon = 11276,
// //     cdrFrenchCanadian = 3084,
// //     cdrFrenchCotedIvoire = 12300,
// //     cdrFrench = 1036,
// //     cdrFrenchLuxembourg = 5132,
// //     cdrFrenchMali = 13324,
// //     cdrFrenchMonaco = 6156,
// //     cdrFrenchReunion = 8204,
// //     cdrFrenchSenegal = 10252,
// //     cdrSwissFrench = 4108,
// //     cdrFrenchWestIndies = 7180,
// //     cdrFrenchZaire = 9228,
// //     cdrFrisianNetherlands = 1122,
// //     cdrGaelicIreland = 2108,
// //     cdrGaelicScotland = 1084,
// //     cdrGalician = 1110,
// //     cdrGeorgian = 1079,
// //     cdrGermanAustria = 3079,
// //     cdrGerman = 1031,
// //     cdrGermanLiechtenstein = 5127,
// //     cdrGermanLuxembourg = 4103,
// //     cdrSwissGerman = 2055,
// //     cdrGreek = 1032,
// //     cdrGujarati = 1095,
// //     cdrHebrew = 1037,
// //     cdrHindi = 1081,
// //     cdrHungarian = 1038,
// //     cdrIcelandic = 1039,
// //     cdrIndonesian = 1057,
// //     cdrItalian = 1040,
// //     cdrSwissItalian = 2064,
// //     cdrJapanese = 1041,
// //     cdrKannada = 1099,
// //     cdrKashmiri = 1120,
// //     cdrKazakh = 1087,
// //     cdrKhmer = 1107,
// //     cdrKirghiz = 1088,
// //     cdrKonkani = 1111,
// //     cdrKorean = 1042,
// //     cdrLao = 1108,
// //     cdrLatvian = 1062,
// //     cdrLithuanian = 1063,
// //     cdrMacedonian = 1071,
// //     cdrMalaysian = 1086,
// //     cdrMalayBruneiDarussalam = 2110,
// //     cdrMalayalam = 1100,
// //     cdrMaltese = 1082,
// //     cdrManipuri = 1112,
// //     cdrMarathi = 1102,
// //     cdrMongolian = 1104,
// //     cdrNepali = 1121,
// //     cdrNorwegianBokmol = 1044,
// //     cdrNorwegianNynorsk = 2068,
// //     cdrOriya = 1096,
// //     cdrPolish = 1045,
// //     cdrBrazilianPortuguese = 1046,
// //     cdrPortuguese = 2070,
// //     cdrPunjabi = 1094,
// //     cdrRhaetoRomanic = 1047,
// //     cdrRomanianMoldova = 2072,
// //     cdrRomanian = 1048,
// //     cdrRussianMoldova = 2073,
// //     cdrRussian = 1049,
// //     cdrSamiLappish = 1083,
// //     cdrSanskrit = 1103,
// //     cdrSerbianCyrillic = 3098,
// //     cdrSerbianLatin = 2074,
// //     cdrSindhi = 1113,
// //     cdrSlovak = 1051,
// //     cdrSlovenian = 1060,
// //     cdrSorbian = 1070,
// //     cdrSpanishArgentina = 11274,
// //     cdrSpanishBolivia = 16394,
// //     cdrSpanishChile = 13322,
// //     cdrSpanishColombia = 9226,
// //     cdrSpanishCostaRica = 5130,
// //     cdrSpanishDominicanRepublic = 7178,
// //     cdrSpanishEcuador = 12298,
// //     cdrSpanishElSalvador = 17418,
// //     cdrSpanishGuatemala = 4106,
// //     cdrSpanishHonduras = 18442,
// //     cdrMexicanSpanish = 2058,
// //     cdrSpanishNicaragua = 19466,
// //     cdrSpanishPanama = 6154,
// //     cdrSpanishParaguay = 15370,
// //     cdrSpanishPeru = 10250,
// //     cdrSpanishPuertoRico = 20490,
// //     cdrSpanishModernSort = 3082,
// //     cdrSpanish = 1034,
// //     cdrSpanishUruguay = 14346,
// //     cdrSpanishVenezuela = 8202,
// //     cdrSesotho = 1072,
// //     cdrSutu = 1072,
// //     cdrSwahili = 1089,
// //     cdrSwedishFinland = 2077,
// //     cdrSwedish = 1053,
// //     cdrTajik = 1064,
// //     cdrTamil = 1097,
// //     cdrTatar = 1092,
// //     cdrTelugu = 1098,
// //     cdrThai = 1054,
// //     cdrTibetan = 1105,
// //     cdrTsonga = 1073,
// //     cdrTswana = 1074,
// //     cdrTurkish = 1055,
// //     cdrTurkmen = 1090,
// //     cdrUkrainian = 1058,
// //     cdrUrdu = 1056,
// //     cdrUzbekCyrillic = 2115,
// //     cdrUzbekLatin = 1091,
// //     cdrVenda = 1075,
// //     cdrVietnamese = 1066,
// //     cdrWelsh = 1106,
// //     cdrXhosa = 1076,
// //     cdrZulu = 1077,
// // }

//  enum cdrTextCharSet {
//     cdrCharSetMixed = 0xffffffff,
//     cdrCharSetANSI = 0,
//     cdrCharSetDefault = 1,
//     cdrCharSetSymbol = 2,
//     cdrCharSetShiftJIS = 128,
//     cdrCharSetHangul = 129,
//     cdrCharSetChineseBig5 = 136,
//     cdrCharSetOEM = 255,
//     cdrCharSetJohab = 130,
//     cdrCharSetHebrew = 177,
//     cdrCharSetArabic = 178,
//     cdrCharSetGreek = 161,
//     cdrCharSetTurkish = 162,
//     cdrCharSetVietnamese = 163,
//     cdrCharSetThai = 222,
//     cdrCharSetEastEurope = 238,
//     cdrCharSetRussian = 204,
//     cdrCharSetMac = 77,
//     cdrCharSetBaltic = 186,
// }

//  enum cdrVerticalAlignment {
//     cdrTopJustify = 0,
//     cdrCenterJustify = 1,
//     cdrBottomJustify = 2,
//     cdrFullJustify = 3,
// }

//  enum cdrCloneLinkType {
//     cdrCloneFill = 1,
//     cdrCloneOutline = 2,
//     cdrCloneShape = 4,
//     cdrCloneTransform = 8,
//     cdrCloneBmpColorMask = 16,
//     cdrCloneAll = 31,
// }

//  enum cdrWrapStyle {
//     cdrWrapNone = 0xffffffff,
//     cdrWrapContourLeft = 0,
//     cdrWrapContourRight = 1,
//     cdrWrapContourStraddle = 2,
//     cdrWrapSquareLeft = 3,
//     cdrWrapSquareRight = 4,
//     cdrWrapSquareStraddle = 5,
//     cdrWrapSquareAboveBelow = 6,
// }

//  enum cdrCurveElementType {
//     cdrElementStart = 0,
//     cdrElementLine = 1,
//     cdrElementCurve = 2,
//     cdrElementControl = 3,
// }

//  enum cdrAddinFilter {
//     cdrAddinFilterNone = 0,
//     cdrAddinFilterShapeCreated = 1,
//     cdrAddinFilterNew = 2,
//     cdrAddinFilterExecute = 4,
// }

//  enum cdrAlignType {
//     cdrAlignLeft = 1,
//     cdrAlignRight = 2,
//     cdrAlignHCenter = 3,
//     cdrAlignTop = 4,
//     cdrAlignBottom = 8,
//     cdrAlignVCenter = 12,
// }

//  enum cdrDistributeType {
//     cdrDistributeLeft = 1,
//     cdrDistributeRight = 2,
//     cdrDistributeHCenter = 3,
//     cdrDistributeHSpacing = 4,
//     cdrDistributeTop = 8,
//     cdrDistributeBottom = 16,
//     cdrDistributeVCenter = 24,
//     cdrDistributeVSpacing = 32,
// }

//  enum cdrDimensionType {
//     cdrDimensionLinear = 0,
//     cdrDimensionAngular = 1,
// }

//  enum cdrLinearDimensionType {
//     cdrDimensionHorizontal = 0,
//     cdrDimensionVertical = 1,
//     cdrDimensionSlanted = 2,
// }

//  enum cdrDimensionStyle {
//     cdrDimensionStyleDecimal = 0,
//     cdrDimensionStyleFractional = 1,
//     cdrDimensionStyleEngineering = 2,
//     cdrDimensionStyleArchitectural = 3,
// }

//  enum cdrDimensionLinearUnits {
//     cdrDimensionUnitInX = 0,
//     cdrDimensionUnitIN = 1,
//     cdrDimensionUnitInches = 2,
//     cdrDimensionUnitFtX = 3,
//     cdrDimensionUnitFT = 4,
//     cdrDimensionUnitMI = 5,
//     cdrDimensionUnitMiles = 6,
//     cdrDimensionUnitYDS = 7,
//     cdrDimensionUnitYards = 8,
//     cdrDimensionUnitM = 9,
//     cdrDimensionUnitMeters = 10,
//     cdrDimensionUnitKM = 11,
//     cdrDimensionUnitKilometers = 12,
//     cdrDimensionUnitCM = 13,
//     cdrDimensionUnitCentimeters = 14,
//     cdrDimensionUnitMM = 15,
//     cdrDimensionUnitMillimeters = 16,
//     cdrDimensionUnitPicas = 17,
//     cdrDimensionUnitPoints = 18,
//     cdrDimensionUnitCiceros = 19,
//     cdrDimensionUnitDidots = 20,
// }

//  enum cdrDimensionPlacement {
//     cdrDimensionAboveLine = 0,
//     cdrDimensionWithinLine = 1,
//     cdrDimensionBelowLine = 2,
// }

//  enum cdrDimensionAngularUnits {
//     cdrDimensionUnitDegrees = 0,
//     cdrDimensionUnitRadians = 1,
//     cdrDimensionUnitGradians = 2,
//     cdrDimensionUnitDegreesSymbol = 3,
// }

//  enum cdrDimensionSymbol {
//     cdrDimensionSymbolNone = 0,
//     cdrDimensionSymbol1After = 1,
//     cdrDimensionSymbol1Before = 2,
//     cdrDimensionSymbol3After = 3,
//     cdrDimensionSymbol3Before = 4,
//     cdrDimensionSymbolDAfter = 5,
//     cdrDimensionSymbolDBefore = 6,
// }

//  enum cdrSymbolType {
//     cdrSymbolTypeSymbol = 1,
//     cdrSymbolTypeSprite = 2,
//     cdrSymbolTypeButton = 4,
// }

//  enum cdrCurveElementFlags {
//     cdrFlagSelected = 1,
//     cdrFlagUser = 4,
//     cdrFlagClosed = 8,
//     cdrFlagValid = 128,
// }

//  enum cdrPaletteSortMethod {
//     cdrSortReverse = 0,
//     cdrSortHue = 1,
//     cdrSortBrightness = 2,
//     cdrSortSaturation = 3,
//     cdrSortRGB = 4,
//     cdrSortHSB = 5,
//     cdrSortName = 6,
// }

//  enum cdrImportMode {
//     cdrImportFull = 0,
//     cdrImportCrop = 1,
//     cdrImportResample = 2,
// }

//  enum cdrImportTextFormatting {
//     cdrImportTextFontAndFormatting = 0,
//     cdrImportNoFormatting = 1,
//     cdrImportFormattingOnly = 2,
// }

//  enum cdrImportTableOutline {
//     cdrImportNoOutline = 0,
//     cdrImportTableOnly = 1,
//     cdrImportCellsOnly = 2,
//     cdrImportTableAndCells = 3,
// }

//  enum cdrDataType {
//     cdrDataTypeString = 0,
//     cdrDataTypeNumber = 1,
//     cdrDataTypeEvent = 2,
//     cdrDataTypeAction = 3,
// }

//  enum cdrPaletteVersion {
//     cdrPaletteVersion9 = 0,
//     cdrPaletteVersion12 = 1,
// }

//  enum cdrOLEType {
//     cdrOLEUnknown = 0,
//     cdrOLELinked = 1,
//     cdrOLEEmbedded = 2,
//     cdrOLEStatic = 3,
// }

//  enum cdrTextAlignOrigin {
//     cdrTextAlignFirstBaseline = 0,
//     cdrTextAlignLastBaseline = 1,
//     cdrTextAlignBoundingBox = 2,
// }

//  enum cdrCompareType {
//     cdrCompareShapeType = 1,
//     cdrCompareFillType = 2,
//     cdrCompareFill = 4,
//     cdrCompareOutlineWidth = 8,
//     cdrCompareOutlineColor = 16,
//     cdrCompareOutline = 32,
//     cdrCompareShapeWidth = 64,
//     cdrCompareShapeHeight = 128,
// }

//  enum cdrCompareCondition {
//     cdrCompareEquals = 0,
//     cdrCompareGreater = 1,
//     cdrCompareLess = 2,
//     cdrCompareAtLeast = 3,
//     cdrCompareAtMost = 4,
// }

//  enum cdrApplicationID {
//     cdrCorelDRAW = 0,
//     cdrCorelDESIGNER = 1,
// }

//  enum cdrApplicationClass {
//     cdrCreativeGraphics = 0,
//     cdrTechnicalGraphics = 1,
// }

//  enum cdrShapeChangeScope {
//     cdrShapeChangeFill = 1,
//     cdrShapeChangeOutline = 2,
//     cdrShapeChangeTransparency = 4,
//     cdrShapeChangeContent = 8,
//     cdrShapeChangeProperties = 16,
//     cdrShapeChangeLocking = 32,
//     cdrShapeChangeStyle = 64,
//     cdrShapeChangeTextAttr = 128,
// }

//  enum cdrTreeNodeType {
//     cdrShapeNode = 1,
//     cdrGroupNode = 2,
//     cdrLinkGroupNode = 3,
//     cdrSymbolNode = 4,
//     cdrRootNode = 17,
//     cdrPageNode = 18,
//     cdrLayerNode = 19,
// }

//  enum cdrTextTabAlignment {
//     cdrTextTabLeft = 0,
//     cdrTextTabRight = 1,
//     cdrTextTabCenter = 2,
//     cdrTextTabDecimal = 3,
// }

//  enum cdrTextEffect {
//     cdrTextEffectMixed = 0xffffffff,
//     cdrTextEffectNone = 0,
//     cdrTextEffectBullet = 1,
//     cdrTextEffectDropCap = 2,
// }

//  enum clrMonitorCalibration {
//     clrMonitorCalibrationOff = 0,
//     clrMonitorCalibrationOn = 1,
//     clrMonitorSimulateComposite = 2,
//     clrMonitorSimulateSeparation = 3,
// }

//  enum clrRenderingIntent {
//     clrRenderAutomatic = 0,
//     clrRenderSaturation = 1,
//     clrRenderPerceptual = 2,
//     clrRenderRelative = 3,
//     clrRenderAbsolute = 4,
// }

// //  enum clrColorEngine {
// //     clrEngineKodak = 0,
// //     clrEngineICM2 = 1,
// //     clrEngineICM = 1,
// //     clrEngineWCS = 2,
// //     clrEngineAdobe = 3,
// //     clrEngineNone = 4,
// //     clrEngineLcms = 5,
// // }

// //  enum clrDeviceType {
// //     clrMonitor = 0,
// //     clrCompositePrinter = 1,
// //     clrSeparationPrinter = 2,
// //     clrScanner = 3,
// //     clrInternalRGB = 4,
// //     clrDeviceAll = 0xffffffff,
// //     clrDeviceDisplay = 0,
// //     clrDeviceOutput = 1,
// //     clrDeviceInput = 3,
// //     clrDeviceColorSpace = 4,
// // }

//  enum clrCompPrnCalibration {
//     clrCompPrnCalibrationOff = 0,
//     clrCompPrnCalibrationOn = 1,
//     clrCompPrnSimulateSeparation = 2,
// }

//  enum clrImportColorCorrection {
//     clrNoImportCorrection = 0,
//     clrApplyEmbeddedOrDefaultProfile = 1,
//     clrApplyCustomImportProfile = 2,
// }

//  enum clrExportColorCorrection {
//     clrNoExportCorrection = 0,
//     clrEmbedInternalRGBProfile = 1,
//     clrEmbedCustomExportProfile = 2,
// }

//  enum cdrFillMode {
//     cdrFillAlternate = 0,
//     cdrFillWinding = 1,
// }

//  enum cdrCommandCheckState {
//     cdrCommandUnchecked = 0,
//     cdrCommandChecked = 1,
// }

//  enum cdrAppStartupMode {
//     cdrStartupWelcomeScreen = 0,
//     cdrStartupNewDocument = 1,
//     cdrStartupOpenDocument = 2,
//     cdrStartupLastEditedDocument = 3,
//     cdrStartupNewFromTemplate = 4,
//     cdrStartupTutorial = 5,
//     cdrStartupDoNothing = 6,
// }

//  enum cdrDistanceMode {
//     cdrModeNoOffset = 0,
//     cdrModeOffset = 1,
//     cdrModeSpacing = 2,
// }

//  enum cdrDirection {
//     cdrLeft = 0,
//     cdrRight = 1,
//     cdrUp = 2,
//     cdrDown = 3,
// }

//  enum cdrShapeLinkType {
//     cdrLinkDirectDependents = 0,
//     cdrLinkDirectDependentsButClones = 1,
//     cdrLinkDirectDependentClones = 2,
//     cdrLinkAllDependents = 3,
//     cdrLinkAllDependentsButClones = 4,
//     cdrLinkAllDependentClones = 5,
//     cdrLinkDirectControls = 6,
//     cdrLinkDirectControlsButClones = 7,
//     cdrLinkAllConnections = 8,
// }

//  enum cdrEnvelopeCopyMode {
//     cdrEnvelopeUseOriginal = 0,
//     cdrEnvelopeCenter = 1,
//     cdrEnvelopeStretch = 2,
//     cdrEnvelopeFit = 3,
// }

//  enum cdrContourEndCapType {
//     cdrContourButtCap = 0,
//     cdrContourRoundCap = 1,
//     cdrContourSquareCap = 2,
// }

//  enum cdrContourCornerType {
//     cdrContourCornerBevel = 0,
//     cdrContourCornerOffsetBevel = 1,
//     cdrContourCornerRound = 2,
//     cdrContourCornerMiteredBevel = 3,
//     cdrContourCornerMiteredOffsetBevel = 4,
//     cdrContourCornerMiteredRound = 5,
// }

//  enum cdrTraceType {
//     cdrTraceLineArt = 1,
//     cdrTraceLogo = 2,
//     cdrTraceDetailedLogo = 3,
//     cdrTraceClipart = 4,
//     cdrTraceLowQualityImage = 5,
//     cdrTraceHighQualityImage = 6,
//     cdrTraceTechnical = 7,
//     cdrTraceLineDrawing = 8,
// }

//  enum cdrTraceBackgroundMode {
//     cdrTraceBackgroundAutomatic = 1,
//     cdrTraceBackgroundManual = 2,
// }

//  enum cdrTextPropertySet {
//     cdrTextPropertyAll = 0xffffffff,
//     cdrTextPropertyFill = 1,
//     cdrTextPropertyOutline = 2,
//     cdrTextPropertyFont = 4,
//     cdrTextPropertySize = 8,
//     cdrTextPropertyStyle = 16,
// }

//  enum cdrDocLayout {
//     cdrDocFullPage = 0,
//     cdrDocBook = 1,
//     cdrDocBooklet = 2,
//     cdrDocTentCard = 3,
//     cdrDocSideFoldCard = 4,
//     cdrDocTopFoldCard = 5,
//     cdrDocTriFoldBrochure = 6,
// }

//  enum cdrCopyProperties {
//     cdrCopyOutlineColor = 1,
//     cdrCopyOutlinePen = 2,
//     cdrCopyFill = 4,
//     cdrCopyTextAttrs = 8,
// }

//  enum cdrOverprintState {
//     cdrOverprintInvalid = 1,
//     cdrOverprintOn = 2,
//     cdrOverprintOff = 4,
// }

//  enum cdrObjectSnapPointType {
//     cdrObjectPointTop = 1,
//     cdrObjectPointBottom = 2,
//     cdrObjectPointLeft = 3,
//     cdrObjectPointRight = 4,
// }

//  enum cdrConnectorType {
//     cdrConnectorStraight = 0,
//     cdrConnectorRightAngle = 1,
//     cdrConnectorBSpline = 2,
//     cdrConnectorBezier = 3,
//     cdrConnectorObject = 4,
// }

//  enum cdrCornerType {
//     cdrCornerTypeRound = 0,
//     cdrCornerTypeScallop = 1,
//     cdrCornerTypeChamfer = 2,
// }

//  enum clrColorModel {
//     clrColorModelRGB = 0,
//     clrColorModelCMYK = 1,
//     clrColorModelGrayscale = 2,
//     clrColorModelLab = 3,
//     clrColorModelHexachrome = 4,
// }

//  enum clrColorPolicyAction {
//     clrAssignBaselineProfile = 0,
//     clrConvertToBaselineProfile = 1,
//     clrConvertToEmbeddedProfile = 2,
//     clrAssignEmbeddedProfile = 3,
// }

//  enum cdrAlignDistributeH {
//     cdrAlignDistributeHNone = 0,
//     cdrAlignDistributeHAlignRight = 1,
//     cdrAlignDistributeHAlignLeft = 2,
//     cdrAlignDistributeHAlignCenter = 3,
//     cdrAlignDistributeHDistributeRight = 4,
//     cdrAlignDistributeHDistributeLeft = 5,
//     cdrAlignDistributeHDistributeCenter = 6,
//     cdrAlignDistributeHDistributeSpacing = 7,
// }

//  enum cdrAlignDistributeV {
//     cdrAlignDistributeVNone = 0,
//     cdrAlignDistributeVAlignTop = 1,
//     cdrAlignDistributeVAlignBottom = 2,
//     cdrAlignDistributeVAlignCenter = 3,
//     cdrAlignDistributeVDistributeTop = 4,
//     cdrAlignDistributeVDistributeBottom = 5,
//     cdrAlignDistributeVDistributeCenter = 6,
//     cdrAlignDistributeVDistributeSpacing = 7,
// }

//  enum cdrDistributeArea {
//     cdrDistributeToSelection = 0,
//     cdrDistributeToPage = 1,
//     cdrDistributeToRect = 2,
// }

//  enum cdrAlignShapesTo {
//     cdrAlignShapesToLastSelected = 0,
//     cdrAlignShapesToEdgeOfPage = 1,
//     cdrAlignShapesToCenterOfPage = 2,
//     cdrAlignShapesToGrid = 3,
//     cdrAlignShapesToPoint = 4,
// }

//  enum cdrOutlineJustification {
//     cdrOutlineJustificationUndefined = 0xffffffff,
//     cdrOutlineJustificationMiddle = 0,
//     cdrOutlineJustificationInside = 1,
//     cdrOutlineJustificationOutside = 2,
// }

//  enum cdrFillStyleType {
//     cdrNoFillStyle = 0,
//     cdrUniformFillStyle = 1,
//     cdrFountainFillStyle = 2,
//     cdrPostscriptFillStyle = 3,
//     cdrTwoColorPatternFillStyle = 4,
//     cdrBitmapPatternFillStyle = 6,
//     cdrTextureFillStyle = 8,
//     cdrFullColorPatternFillStyle = 9,
//     cdrHatchFillStyle = 10,
// }

//  enum cdrFountainFillSpreadMethod {
//     cdrFountainFillSpreadMethodPad = 0,
//     cdrFountainFillSpreadMethodReflect = 1,
//     cdrFountainFillSpreadMethodRepeat = 2,
// }

//  enum cdrProjectPlane {
//     cdrProjectFront = 0,
//     cdrProjectRight = 1,
//     cdrProjectTop = 2,
// }

//  enum cdrOnScreenCurvePenStyle {
//     cdrOnScreenCurvePenSolid = 0,
//     cdrOnScreenCurvePenDash = 1,
//     cdrOnScreenCurvePenDot = 2,
//     cdrOnScreenCurvePenDashDot = 3,
//     cdrOnScreenCurvePenDashDotDot = 4,
//     cdrOnScreenCurvePenAlternatingDots = 5,
// }

//  enum cdrOnScreenTextAlign {
//     cdrLeftOnScreenText = 0,
//     cdrRightOnScreenText = 1,
//     cdrCenterOnScreenText = 2,
// }

//  enum cdrWeldFlags {
//     cdrWeldFlagNone = 0,
//     cdrWeldFlagFollowOriginalCurve = 1,
//     cdrWeldFlagUseTightTolerance = 2,
//     cdrWeldFlagAutoCloseOnFailure = 4,
//     cdrWeldFlagRemoveKnifeEdges = 8,
//     cdrWeldFlagAutoClose = 16,
// }

//  enum cdrWeldMethod {
//     cdrWeldMethodUnion = 0,
//     cdrWeldMethodIntersect = 1,
//     cdrWeldMethodSubtract = 2,
//     cdrWeldMethodXor = 3,
// }

//  enum cdrOutlineDashAdjust {
//     cdrOutlineDashAdjustUndefined = 0xffffffff,
//     cdrOutlineDashAdjustNone = 0,
//     cdrOutlineDashAdjustFullLineScale = 1,
//     cdrOutlineDashAdjustEvenCorner = 2,
// }

//  enum PrnFileMode {
//     prnSingleFile = 0,
//     prnSeparatePages = 1,
//     prnSeparatePlates = 2,
// }

//  enum PrnPrintRange {
//     prnWholeDocument = 0,
//     prnCurrentPage = 1,
//     prnSelection = 2,
//     prnPageRange = 3,
// }

//  enum PrnPlateType {
//     prnCyan = 0,
//     prnMagenta = 1,
//     prnYellow = 2,
//     prnBlack = 3,
//     prnOrange = 4,
//     prnGreen = 5,
//     prnSpot = 6,
// }

//  enum PrnRegistrationStyle {
//     prnStandard = 0,
//     prnLong = 1,
//     prnSquare = 2,
//     prnHalfInverted = 3,
//     prnCorel = 4,
// }

//  enum PrnPostScriptLevel {
//     prnPSLevel1 = 1,
//     prnPSLevel2 = 2,
//     prnPSLevel3 = 3,
// }

//  enum PrnPDFStartup {
//     prnPDFFullScreen = 0,
//     prnPDFPageOnly = 1,
//     prnPDFThumbnails = 2,
//     prnPDFOutlines = 3,
// }

//  enum PrnImageTrap {
//     prnTrapNormal = 0,
//     prnTrapSpread = 1,
//     prnTrapChoke = 2,
//     prnTrapCenter = 3,
// }

//  enum PrnTrapType {
//     prnLayerNormal = 0,
//     prnLayerTransparent = 1,
//     prnLayerOpaque = 2,
//     prnLayerOpaqueIgnore = 3,
// }

//  enum PrnColorMode {
//     prnModeFullColor = 0,
//     prnModeGrayscale = 1,
//     prnModeBlack = 2,
// }

//  enum PrnBitmapColorMode {
//     prnBitmapCMYK = 0,
//     prnBitmapRGB = 1,
//     prnBitmapGrayscale = 2,
//     prnBitmapNative = 3,
// }

//  enum PrnPageSet {
//     prnPageSetAll = 0,
//     prnPageSetOdd = 1,
//     prnPageSetEven = 2,
//     prnPageSetLeft = 3,
//     prnPageSetRight = 4,
// }

//  enum PrnPaperSize {
//     prnPaperSizeLetter = 1,
//     prnPaperSizeLetterSmall = 2,
//     prnPaperSizeTabloid = 3,
//     prnPaperSizeLedger = 4,
//     prnPaperSizeLegal = 5,
//     prnPaperSizeStatement = 6,
//     prnPaperSizeExecutive = 7,
//     prnPaperSizeA3 = 8,
//     prnPaperSizeA4 = 9,
//     prnPaperSizeA4Small = 10,
//     prnPaperSizeA5 = 11,
//     prnPaperSizeB4 = 12,
//     prnPaperSizeB5 = 13,
//     prnPaperSizeFolio = 14,
//     prnPaperSizeQuarto = 15,
//     prnPaperSize10x14 = 16,
//     prnPaperSize11x17 = 17,
//     prnPaperSizeNote = 18,
//     prnPaperSizeEnv9 = 19,
//     prnPaperSizeEnv10 = 20,
//     prnPaperSizeEnv11 = 21,
//     prnPaperSizeEnv12 = 22,
//     prnPaperSizeEnv14 = 23,
//     prnPaperSizeCSheet = 24,
//     prnPaperSizeDSheet = 25,
//     prnPaperSizeESheet = 26,
//     prnPaperSizeEnvDL = 27,
//     prnPaperSizeEnvC5 = 28,
//     prnPaperSizeEnvC3 = 29,
//     prnPaperSizeEnvC4 = 30,
//     prnPaperSizeEnvC6 = 31,
//     prnPaperSizeEnvC65 = 32,
//     prnPaperSizeISOB4 = 33,
//     prnPaperSizeISOB5 = 34,
//     prnPaperSizeISOB6 = 35,
//     prnPaperSizeEnvItaly = 36,
//     prnPaperSizeEnvMonarch = 37,
//     prnPaperSizeEnvPersonal = 38,
//     prnPaperSizeFanfoldUS = 39,
//     prnPaperSizeFanfoldGerman = 40,
//     prnPaperSizeFanfoldLglGerman = 41,
//     prnPaperSizeJapanesePostcard = 43,
//     prnPaperSize9x11 = 44,
//     prnPaperSize10x11 = 45,
//     prnPaperSize15x11 = 46,
//     prnPaperSizeEnvInvite = 47,
//     prnPaperSizeLetterExtra = 50,
//     prnPaperSizeLegalExtra = 51,
//     prnPaperSizeTabloidExtra = 52,
//     prnPaperSizeA4Extra = 53,
//     prnPaperSizeLetterTransverse = 54,
//     prnPaperSizeA4Transverse = 55,
//     prnPaperSizeLetterExtraTransverse = 56,
//     prnPaperSizeAPlus = 57,
//     prnPaperSizeBPlus = 58,
//     prnPaperSizeLetterPlus = 59,
//     prnPaperSizeA4Plus = 60,
//     prnPaperSizeA5Transverse = 61,
//     prnPaperSizeB5Transverse = 62,
//     prnPaperSizeA3Extra = 63,
//     prnPaperSizeA5Extra = 64,
//     prnPaperSizeB5Extra = 65,
//     prnPaperSizeA2 = 66,
//     prnPaperSizeA3Transverse = 67,
//     prnPaperSizeA3ExtraTransverse = 68,
//     prnPaperSizeJapaneseDblPostcard = 69,
//     prnPaperSizeA6 = 70,
//     prnPaperSizeLetterRotated = 75,
//     prnPaperSizeA3Rotated = 76,
//     prnPaperSizeA4Rotated = 77,
//     prnPaperSizeA5Rotated = 78,
//     prnPaperSizeB4JISRotated = 79,
//     prnPaperSizeB5JISRotated = 80,
//     prnPaperSizeJapanesePostcardRotated = 81,
//     prnPaperSizeJapaneseDlbPostcardRotated = 82,
//     prnPaperSizeA6Rotated = 83,
//     prnPaperSizeB6JIS = 88,
//     prnPaperSizeB6JISRotated = 89,
//     prnPaperSize12x11 = 90,
//     prnPaperSizeP16K = 93,
//     prnPaperSizeP32K = 94,
//     prnPaperSizeP32KBig = 95,
//     prnPaperSizePEnv1 = 96,
//     prnPaperSizePEnv2 = 97,
//     prnPaperSizePEnv3 = 98,
//     prnPaperSizePEnv4 = 99,
//     prnPaperSizePEnv5 = 100,
//     prnPaperSizePEnv6 = 101,
//     prnPaperSizePEnv7 = 102,
//     prnPaperSizePEnv8 = 103,
//     prnPaperSizePEnv9 = 104,
//     prnPaperSizePEnv10 = 105,
//     prnPaperSizeP16KRotated = 106,
//     prnPaperSizeP32KRotated = 107,
//     prnPaperSizeP32KBigRotated = 108,
//     prnPaperSizePEnv1Rotated = 109,
//     prnPaperSizePEnv2Rotated = 110,
//     prnPaperSizePEnv3Rotated = 111,
//     prnPaperSizePEnv4Rotated = 112,
//     prnPaperSizePEnv5Rotated = 113,
//     prnPaperSizePEnv6Rotated = 114,
//     prnPaperSizePEnv7Rotated = 115,
//     prnPaperSizePEnv8Rotated = 116,
//     prnPaperSizePEnv9Rotated = 117,
//     prnPaperSizePEnv10Rotated = 118,
// }

//  enum PrnPaperOrientation {
//     prnPaperPortrait = 1,
//     prnPaperLandscape = 2,
//     prnPaperRotatedLandscape = 3,
// }

//  enum PrnPlaceType {
//     prnPlaceAsInDocument = 0,
//     prnPlaceFitToPage = 1,
//     prnPlaceCenterPage = 2,
//     prnPlaceCenterTop = 3,
//     prnPlaceCenterLeft = 4,
//     prnPlaceCenterRight = 5,
//     prnPlaceCenterBottom = 6,
//     prnPlaceLeftTop = 7,
//     prnPlaceRightTop = 8,
//     prnPlaceLeftBottom = 9,
//     prnPlaceRightBottom = 10,
//     prnPlaceCustom = 11,
// }

//  enum PrnObjectsColorMode {
//     prnObjectsCMYK = 0,
//     prnObjectsRGB = 1,
//     prnObjectsGrayscale = 2,
//     prnObjectsNative = 3,
// }

//  enum PrnPageMatchingMode {
//     prnPageMatchPrinterDefault = 0,
//     prnPageMatchOrientation = 1,
//     prnPageMatchSizeAndOrientation = 2,
// }

//  enum pdfExportRange {
//     pdfWholeDocument = 0,
//     pdfCurrentPage = 1,
//     pdfSelection = 2,
//     pdfPageRange = 3,
// }

//  enum pdfBitmapCompressionType {
//     pdfNone = 0,
//     pdfLZW = 1,
//     pdfJPEG = 2,
//     pdfZIP = 3,
//     pdfJP2 = 4,
// }

//  enum pdfEncodingType {
//     pdfASCII85 = 0,
//     pdfBinary = 1,
// }

//  enum pdfColorMode {
//     pdfRGB = 0,
//     pdfCMYK = 1,
//     pdfGrayscale = 2,
//     pdfNative = 3,
// }

//  enum pdfColorProfile {
//     pdfCompositeProfile = 0,
//     pdfSeparationProfile = 1,
// }

//  enum pdfEPSAs {
//     pdfPostscript = 0,
//     pdfPreview = 1,
// }

//  enum pdfDisplayOnStart {
//     pdfPageOnly = 0,
//     pdfFullScreen = 1,
//     pdfBookmarks = 2,
//     pdfThumbnails = 3,
// }

//  enum pdfVersion {
//     pdfVersion12 = 0,
//     pdfVersion13 = 1,
//     pdfVersionPDFX1 = 2,
//     pdfVersion14 = 3,
//     pdfVersionPDFX1a = 4,
//     pdfVersionPDFX3 = 5,
//     pdfVersion15 = 6,
//     pdfVersionPDFA1b = 7,
//     pdfVersion17 = 8,
//     pdfVersion17_Acrobat9 = 9,
//     pdfVersionPDFX4 = 10,
// }

//  enum pdfTextExportMode {
//     pdfTextAsUnicode = 0,
//     pdfTextAsAscii = 1,
// }

//  enum pdfPrintPermissions {
//     pdfPrintPermissionNone = 0,
//     pdfPrintPermissionLowResolution = 1,
//     pdfPrintPermissionHighResolution = 2,
// }

//  enum pdfEditPermissions {
//     pdfEditPermissionNone = 0,
//     pdfEditPermissionInsertDeleteRotatePage = 1,
//     pdfEditPermissionAny = 2,
// }

//  enum pdfEncryptionType {
//     pdfEncryptTypeNone = 0,
//     pdfEncryptTypeStandard = 1,
//     pdfEncryptTypeAES = 2,
//     pdfEncryptTypeAES256 = 3,
// }

//  enum pdfSpotType {
//     pdfSpotAsSpot = 0,
//     pdfSpotAsRGB = 1,
//     pdfSpotAsCMYK = 2,
//     pdfSpotAsGray = 3,
// }

//  enum cuiBarType {
//     cuiBarTypeNormal = 0,
//     cuiBarTypeMenuBar = 1,
//     cuiBarTypePopup = 2,
//     cuiBarTypeStatusBar = 3,
//     cuiBarTypePropertyBar = 4,
// }

//  enum cuiBarPosition {
//     cuiBarLeft = 0,
//     cuiBarTop = 1,
//     cuiBarRight = 2,
//     cuiBarBottom = 3,
//     cuiBarFloating = 4,
// }

//  enum cuiBarProtection {
//     cuiBarNoProtection = 0,
//     cuiBarNoCustomize = 1,
//     cuiBarNoMove = 4,
//     cuiBarNoChangeVisible = 8,
//     cuiBarNoChangeDock = 16,
//     cuiBarNoVerticalDock = 32,
//     cuiBarNoHorizontalDock = 64,
// }

//  enum cuiWindowState {
//     cuiWindowStateRestored = 0,
//     cuiWindowStateMaximized = 1,
//     cuiWindowStateMinimized = 2,
// }

//  enum cuiDockHostOrientation {
//     cuiDockHostHorizontal = 0,
//     cuiDockHostVertical = 1,
// }

//  enum cuiDockItemType {
//     cuiDockItemDockHost = 0,
//     cuiDockItemViewHost = 1,
// }

//  enum cuiDockOperation {
//     cuiDockOperationInsert = 0,
//     cuiDockOperationSplitTopLeft = 1,
//     cuiDockOperationSplitBottomRight = 2,
// }

//  enum cuiMessageBoxFlags {
//     Default = 0,
//     NoAutoWrap = 1,
// }

//  enum cuiTaskPriority {
//     kMaybeLater = 0,
//     kLater = 1,
//     kSoon = 4,
//     kASAP = 3,
// }

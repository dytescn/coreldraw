src/
└── sdk/
    ├── mod.rs
    ├── prelude.rs
    ├── types.rs                      # CLSID / IID / 常量（从 RIDL 尾部 class 段抄）
    │
    ├── enums/                        # 所有 cdr*/clr*/pdf*/Prn*/cui* 枚举
    │   ├── mod.rs
    │   ├── app.rs                    # cdrApplicationID/Class/StartupMode, cdrTriState ...
    │   ├── shape.rs                  # cdrShapeType / cdrNodeType / cdrSegmentType ...
    │   ├── curve.rs                  # cdrSegmentOffsetType / cdrNodeType / cdrWeldMethod ...
    │   ├── color.rs                  # cdrColorType / cdrPaletteID / clrColorModel / clrRenderingIntent
    │   ├── fill.rs                   # cdrFillType / cdrFountainFillType / cdrPatternFillType ...
    │   ├── outline.rs                # cdrOutlineType / cdrOutlineLineCaps / cdrOutlineLineJoin ...
    │   ├── text.rs                   # cdrTextType / cdrAlignment / cdrFontStyle / cdrFontLine ...
    │   ├── effect.rs                 # cdrEffectType / cdrDropShadowType / cdrDistortionType ...
    │   ├── style.rs                  # cdrFillStyleType / cdrOutlineJustification ...
    │   ├── page.rs                   # cdrPageOrientation / cdrPageBackground
    │   ├── layer.rs                  # (预留)
    │   ├── view.rs                   # cdrViewType / cdrWindowState / cdrWindowArrangeStyle
    │   ├── filter.rs                 # cdrFilter（大表） / cdrFileVersion / cdrExportRange
    │   ├── image.rs                  # cdrImageType / cdrCompressionType / cdrAntiAliasingType
    │   ├── print.rs                  # Prn* / pdf*
    │   └── ui.rs                     # cui*
    │
    ├── app/                          # IVGApplication 及其附属
    │   ├── mod.rs
    │   ├── application.rs            # IVGApplication（现有文件重构）
    │   ├── application_event.rs      # IVGApplicationEvents
    │   ├── status.rs                 # IVGAppStatus
    │   └── window.rs                 # IVGAppWindow
    │
    ├── document/                     # IVGDocument / IVGDocuments / Metadata / DataFields
    │   ├── mod.rs
    │   ├── document.rs
    │   ├── documents.rs
    │   ├── document_event.rs         # IVGDocumentEvents
    │   ├── metadata.rs               # IVGMetadata
    │   └── data_fields.rs            # IVGDataField(s) / IVGDataItem(s)
    │
    ├── page/                         # IVGPage / IVGPages / PageSize / Spread
    │   ├── mod.rs
    │   ├── page.rs
    │   ├── pages.rs
    │   ├── page_size.rs              # IVGPageSize / IVGPageSizes
    │   └── spread.rs                 # IVGSpread / IVGSpreads
    │
    ├── layer/                        # IVGLayer / IVGLayers
    │   ├── mod.rs
    │   ├── layer.rs
    │   └── layers.rs
    │
    ├── shape/                        # IVGShape 及其子类型
    │   ├── mod.rs
    │   ├── shape.rs
    │   ├── shapes.rs
    │   ├── shape_range.rs
    │   ├── rectangle.rs              # IVGRectangle
    │   ├── ellipse.rs                # IVGEllipse
    │   ├── polygon.rs                # IVGPolygon
    │   ├── bitmap.rs                 # IVGBitmap
    │   ├── image.rs                  # IVGImage / IVGImageTile(s)
    │   ├── eps.rs                    # IVGEPS
    │   ├── connector.rs              # IVGConnector
    │   ├── guide.rs                  # IVGGuide
    │   ├── power_clip.rs             # IVGPowerClip
    │   ├── custom_shape.rs           # IVGCustomShape
    │   └── selection_info.rs         # IVGSelectionInformation
    │
    ├── curve/                        # IVGCurve 一族
    │   ├── mod.rs
    │   ├── curve.rs
    │   ├── sub_path.rs               # IVGSubPath / IVGSubPaths
    │   ├── node.rs                   # IVGNode / IVGNodes
    │   ├── node_range.rs             # IVGNodeRange
    │   ├── segment.rs                # IVGSegment / IVGSegments
    │   ├── segment_range.rs          # IVGSegmentRange
    │   ├── cross_point.rs            # IVGCrossPoint / IVGCrossPoints
    │   └── bspline.rs                # IVGBSpline / ControlPoint(s)
    │
    ├── geometry/                     # IVGPoint / IVGVector / IVGRect / TransformMatrix / SnapPoint
    │   ├── mod.rs
    │   ├── point.rs                  # IVGPoint / IVGPointRange
    │   ├── vector.rs                 # IVGVector
    │   ├── rect.rs                   # IVGRect
    │   ├── transform.rs              # IVGTransformMatrix
    │   ├── math_utils.rs             # IVGMathUtils
    │   └── snap_point.rs             # IVGSnapPoint(s) / Range / User / Object / BBox / Edge
    │
    ├── color/                        # IVGColor 一族
    │   ├── mod.rs
    │   ├── color.rs
    │   ├── colors.rs
    │   ├── context.rs                # IVGColorContext
    │   ├── profile.rs                # IVGColorProfile / IVGColorProfiles
    │   ├── manager.rs                # IVGColorManager / IVGColorManagementPolicy
    │   └── duotone.rs                # IVGDuotone / Ink / Overprint
    │
    ├── fill/                         # IVGFill 一族
    │   ├── mod.rs
    │   ├── fill.rs
    │   ├── fountain.rs               # IVGFountainFill / Color(s)
    │   ├── pattern.rs                # IVGPatternFill / PatternCanvas(es)
    │   ├── texture.rs                # IVGTextureFill / Properties / Property
    │   ├── postscript.rs             # IVGPostScriptFill / IVGPSScreenOptions
    │   ├── hatch.rs                  # IVGHatchFill / Library / Libraries / Pattern(s) / Fills
    │   └── metadata.rs               # IVGFillMetadata / IVGLocalizableString
    │
    ├── outline/                      # IVGOutline 一族
    │   ├── mod.rs
    │   ├── outline.rs
    │   ├── outline_style.rs          # IVGOutlineStyle / IVGOutlineStyles
    │   └── arrow_head.rs             # IVGArrowHead / IVGArrowHeads / Options
    │
    ├── text/                         # IVGText 一族
    │   ├── mod.rs
    │   ├── text.rs
    │   ├── range.rs                  # IVGTextRange / IVGTextRanges
    │   ├── frame.rs                  # IVGTextFrame / IVGTextFrames
    │   ├── characters.rs
    │   ├── words.rs
    │   ├── lines.rs
    │   ├── paragraphs.rs
    │   ├── columns.rs
    │   └── tab_positions.rs
    │
    ├── effect/                       # IVGEffect 一族
    │   ├── mod.rs
    │   ├── effect.rs
    │   ├── effects.rs
    │   ├── blend.rs                  # IVGEffectBlend
    │   ├── contour.rs
    │   ├── control_path.rs
    │   ├── distortion.rs             # + Custom / PushPull / Zipper / Twister
    │   ├── drop_shadow.rs
    │   ├── envelope.rs
    │   ├── extrude.rs                # + IVGExtrudeVanishingPoint
    │   ├── lens.rs
    │   ├── perspective.rs
    │   └── text_on_path.rs
    │
    ├── style/                        # IVGStyle 一族
    │   ├── mod.rs
    │   ├── style.rs
    │   ├── styles.rs
    │   ├── style_sheet.rs
    │   ├── style_fill.rs
    │   ├── style_outline.rs
    │   ├── style_character.rs
    │   ├── style_paragraph.rs
    │   ├── style_frame.rs
    │   └── style_transparency.rs
    │
    ├── view/                         # IVGView / IVGActiveView / IVGWindow
    │   ├── mod.rs
    │   ├── view.rs
    │   ├── views.rs
    │   ├── active_view.rs
    │   ├── window.rs
    │   └── windows.rs
    │
    ├── tree/                         # IVGTreeNode / IVGTreeNodes / IVGTreeManager
    │   ├── mod.rs
    │   ├── node.rs
    │   ├── nodes.rs
    │   └── manager.rs
    │
    ├── symbol/                       # IVGSymbol / IVGSymbolDefinition / IVGSymbolLibrary
    │   ├── mod.rs
    │   ├── symbol.rs
    │   ├── definition.rs             # IVGSymbolDefinition / IVGSymbolDefinitions
    │   └── library.rs                # IVGSymbolLibrary / IVGSymbolLibraries
    │
    ├── structs/                      # IVGStruct* 全部集中
    │   ├── mod.rs
    │   ├── save_as_options.rs
    │   ├── export_options.rs
    │   ├── import_options.rs         # + IStructImportCropOptions / ResampleOptions
    │   ├── open_options.rs
    │   ├── create_options.rs
    │   ├── paste_options.rs
    │   ├── palette_options.rs
    │   ├── font_properties.rs
    │   ├── align_properties.rs
    │   ├── space_properties.rs
    │   ├── hyphenation_settings.rs
    │   └── color_conversion_options.rs
    │
    ├── print/                        # IPrn* + IPDFVBASettings
    │   ├── mod.rs
    │   ├── settings.rs
    │   ├── job.rs
    │   ├── document.rs               # IPrnVBAPrintDocument(s) / PrintPage(s)
    │   ├── printer.rs                # IPrnVBAPrinter / IPrnVBAPrinters
    │   ├── separations.rs            # SeparationPlates
    │   ├── prepress.rs
    │   ├── postscript.rs
    │   ├── trapping.rs
    │   ├── options.rs
    │   ├── layout.rs
    │   └── pdf_settings.rs
    │
    ├── import_export/                # filter 相关
    │   ├── mod.rs
    │   ├── import_filter.rs          # ICorelImportFilter
    │   ├── export_filter.rs          # ICorelExportFilter
    │   └── handlers.rs               # IImportCropHandler / IImportResampleHandler
    │
    ├── ui/                           # ICUI* 系列
    │   ├── mod.rs
    │   ├── application.rs            # ICUIApplication
    │   ├── framework.rs              # ICUIFrameWork
    │   ├── command_bar.rs            # ICUICommandBars / Bar / Mode(s)
    │   ├── control.rs                # ICUIControls / ICUIControl
    │   ├── frame_window.rs           # ICUIFrameWindows / ICUIFrameWindow
    │   ├── view_host.rs              # ICUIViewHosts / ICUIViewHost
    │   ├── view_window.rs            # ICUIViewWindows / ICUIViewWindow
    │   ├── dock_host.rs              # ICUIDockHosts / ICUIDockHost
    │   ├── dock_item.rs              # ICUIDockItems / ICUIDockItem
    │   ├── screen_rect.rs            # ICUIScreenRect
    │   ├── data_context.rs           # ICUIDataContext / Source(Proxy/Factory)
    │   ├── image_list.rs             # ICUIImageList / ICUIBitmapImage
    │   ├── status_text.rs            # ICUIStatusText
    │   ├── warning.rs                # ICUIWarning
    │   ├── task_manager.rs           # ICUITaskManager / Task / BackgroundTask ...
    │   └── automation.rs             # ICUIAutomation / ICUIControlData
    │
    └── misc/
        ├── mod.rs
        ├── properties.rs             # IVGProperties
        ├── transparency.rs           # IVGTransparency
        ├── url.rs                    # IVGURL
        ├── component.rs              # IVGComponent(s)
        ├── clone_link.rs             # IVGCloneLink
        ├── clipboard.rs              # IVGClipboard
        ├── ruler.rs                  # IVGRulers
        ├── grid.rs                   # IVGGrid
        ├── recent_files.rs           # IVGRecentFile(s)
        ├── font_list.rs              # IVGFontList
        ├── workspace.rs              # IVGWorkspace(s)
        ├── palette.rs                # IVGPalette(s) / IVGPaletteManager
        ├── script_tools.rs           # ICorelScriptTools
        ├── gms.rs                    # IVGGMSManager / Project(s) / Macro(s)
        ├── on_screen.rs              # IVGOnScreenCurve / Handle / Text
        ├── tool_state.rs             # IVGToolState / Attributes
        ├── tool_shape.rs             # IVGToolShape / Attributes
        ├── trace_settings.rs         # IVGTraceSettings
        ├── filetypes.rs              # （现有）
        └── opplist.rs                # （现有）
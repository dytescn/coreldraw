#![allow(non_camel_case_types, non_snake_case, unused)]

use winapi::{ENUM, RIDL, STRUCT};
use winapi::ctypes::c_void;
use winapi::shared::guiddef::GUID;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL, DATE};
use winapi::um::oaidl::{SAFEARRAY, VARIANT, IDispatch, IDispatchVtbl, LPDISPATCH};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl, LPUNKNOWN};
use winapi::um::winnt::{LPCWSTR, INT};





RIDL!{#[uuid(0xf5200003, 0x8d23, 0x0001, 0x89, 0xe7, 0x00, 0x00, 0x86, 0x1e, 0xbb, 0xd6)]
interface ICorelImportFilter(ICorelImportFilterVtbl): IDispatch(IDispatchVtbl) {
    fn Reset(
    ) -> HRESULT,
    fn Finish(
    ) -> HRESULT,
    fn get_HasDialog(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ShowDialog(
        hWnd: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xf5200000, 0x8d23, 0x0002, 0x89, 0xe7, 0x00, 0x00, 0x86, 0x1e, 0xbb, 0xd6)]
interface ICorelExportFilter(ICorelExportFilterVtbl): IDispatch(IDispatchVtbl) {
    fn Reset(
    ) -> HRESULT,
    fn Finish(
    ) -> HRESULT,
    fn get_HasDialog(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ShowDialog(
        hWnd: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0002, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIAutomation(ICUIAutomationVtbl): IDispatch(IDispatchVtbl) {
    fn GetNumItemsOnBar(
        GuidBar: BSTR,
        NumItems: *mut i32,
    ) -> HRESULT,
    fn GetItem(
        GuidBar: BSTR,
        Index: i32,
        HasSubBar: *mut VARIANT_BOOL,
        GuidItem: *mut BSTR,
    ) -> HRESULT,
    fn GetItemInstanceHwnd(
        GuidParent: BSTR,
        GuidItem: BSTR,
        hWnd: *mut i32,
    ) -> HRESULT,
    fn GetSubBar(
        GuidBar: BSTR,
        GuidSubBar: *mut BSTR,
        HasSubBar: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ShowBar(
        GuidBar: BSTR,
        Show: VARIANT_BOOL,
        WasShown: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetCaptionText(
        GuidItem: BSTR,
        Caption: *mut BSTR,
    ) -> HRESULT,
    fn Invoke(
        GuidItem: BSTR,
    ) -> HRESULT,
    fn IsEnabled(
        GuidItem: BSTR,
        IsEnabled: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetItemScreenRect(
        GuidParent: BSTR,
        GuidItem: BSTR,
        TopLeftX: *mut i32,
        TopLeftY: *mut i32,
        Width: *mut i32,
        Height: *mut i32,
        IsOnScreen: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn InvokeItem(
        GuidItem: BSTR,
    ) -> HRESULT,
    fn InvokeDialogItem(
        GuidDialog: BSTR,
        GuidItem: BSTR,
    ) -> HRESULT,
    fn GetControlData(
        Guid: BSTR,
        Result: *mut *mut ICUIControlData,
    ) -> HRESULT,
    fn GetControlDataEx(
        GuidParent: BSTR,
        Guid: BSTR,
        Result: *mut *mut ICUIControlData,
    ) -> HRESULT,
    fn GetActiveMenuItemScreenRect(
        itemIndex: INT,
        TopLeftX: *mut i32,
        TopLeftY: *mut i32,
        Width: *mut i32,
        Height: *mut i32,
        IsOnScreen: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetActiveMenuItemGuid(
        itemIndex: INT,
        GuidItem: *mut BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0001, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIControlData(ICUIControlDataVtbl): IDispatch(IDispatchVtbl) {
    fn GetValue(
        PropertyName: BSTR,
        prop: *mut VARIANT,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0003, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIFrameWork(ICUIFrameWorkVtbl): IDispatch(IDispatchVtbl) {
    fn get_CommandBars(
        pVal: *mut *mut ICUICommandBars,
    ) -> HRESULT,
    fn get_MainMenu(
        pVal: *mut *mut ICUICommandBar,
    ) -> HRESULT,
    fn get_StatusBar(
        pVal: *mut *mut ICUICommandBar,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn ImportWorkspace(
        FileName: BSTR,
    ) -> HRESULT,
    fn get_Automation(
        pVal: *mut *mut ICUIAutomation,
    ) -> HRESULT,
    fn ShowDocker(
        Guid: BSTR,
    ) -> HRESULT,
    fn HideDocker(
        Guid: BSTR,
    ) -> HRESULT,
    fn IsDockerVisible(
        Guid: BSTR,
        IsVisible: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn AddDocker(
        Guid: BSTR,
        ClassName: BSTR,
        AssemblyPath: BSTR,
    ) -> HRESULT,
    fn RemoveDocker(
        Guid: BSTR,
    ) -> HRESULT,
    fn get_FrameWindows(
        ppVal: *mut *mut ICUIFrameWindows,
    ) -> HRESULT,
    fn get_MainFrameWindow(
        ppVal: *mut *mut ICUIFrameWindow,
    ) -> HRESULT,
    fn get_Application(
        ppVal: *mut *mut ICUIApplication,
    ) -> HRESULT,
    fn CreateFrameWindowForViewHost(
        ViewHostToInsert: *const ICUIViewHost,
        ppVal: *mut *mut ICUIFrameWindow,
    ) -> HRESULT,
    fn CreateFrameWindowForView(
        ViewToInsert: *const ICUIViewWindow,
        ppVal: *mut *mut ICUIFrameWindow,
    ) -> HRESULT,
    fn ShowDialog(
        Guid: BSTR,
    ) -> HRESULT,
    fn HideDialog(
        Guid: BSTR,
    ) -> HRESULT,
    fn ShowMessageBox(
        szMessage: BSTR,
        szMainInstruction: BSTR,
        unFlags: i32,
        pImage: *const ICUIBitmapImage,
        szHelpGuid: BSTR,
        szWarningName: BSTR,
        eFlags: cuiMessageBoxFlags,
        pDataContext: *const ICUIDataContext,
        pVal: *mut i32,
    ) -> HRESULT,
    fn GetWarning(
        szWarningID: BSTR,
        bHidden: VARIANT_BOOL,
        __MIDL__ICUIFrameWork0000: *mut *mut ICUIWarning,
    ) -> HRESULT,
    fn get_TaskManager(
        ppVal: *mut *mut ICUITaskManager,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0004, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUICommandBars(ICUICommandBarsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        IndexOrName: VARIANT,
        pVal: *mut *mut ICUICommandBar,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Add(
        Name: BSTR,
        Position: cuiBarPosition,
        Temporary: VARIANT_BOOL,
        pVal: *mut *mut ICUICommandBar,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0009, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUICommandBar(ICUICommandBarVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cuiBarType,
    ) -> HRESULT,
    fn get_Visible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Visible(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Controls(
        pVal: *mut *mut ICUIControls,
    ) -> HRESULT,
    fn get_Modes(
        pVal: *mut *mut ICUICommandBarModes,
    ) -> HRESULT,
    fn get_BuiltIn(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Enabled(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Enabled(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Left(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Left(
        pVal: i32,
    ) -> HRESULT,
    fn get_Top(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Top(
        pVal: i32,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_NameLocal(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_NameLocal(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Position(
        pVal: *mut cuiBarPosition,
    ) -> HRESULT,
    fn put_Position(
        pVal: cuiBarPosition,
    ) -> HRESULT,
    fn get_Protection(
        pVal: *mut cuiBarProtection,
    ) -> HRESULT,
    fn put_Protection(
        pVal: cuiBarProtection,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn ShowPopup(
        x: VARIANT,
        y: VARIANT,
    ) -> HRESULT,
    fn SetWidth(
        Width: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0007, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIControls(ICUIControlsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut ICUIControl,
    ) -> HRESULT,
    fn get__NewEnum(
        ppVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Add(
        ControlID: BSTR,
        Index: i32,
        Temporary: VARIANT_BOOL,
        ppVal: *mut *mut ICUIControl,
    ) -> HRESULT,
    fn AddCustomButton(
        CategoryID: BSTR,
        Command: BSTR,
        Index: i32,
        Temporary: VARIANT_BOOL,
        ppVal: *mut *mut ICUIControl,
    ) -> HRESULT,
    fn AddCustomControl(
        ClassName: BSTR,
        AssemblyPath: BSTR,
        Index: i32,
        Temporary: VARIANT_BOOL,
        ppVal: *mut *mut ICUIControl,
    ) -> HRESULT,
    fn AddToggleButton(
        Guid: BSTR,
        Index: i32,
        Temporary: VARIANT_BOOL,
        ppVal: *mut *mut ICUIControl,
    ) -> HRESULT,
    fn Remove(
        Index: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0008, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIControl(ICUIControlVtbl): IDispatch(IDispatchVtbl) {
    fn get_Caption(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Caption(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_DescriptionText(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_DescriptionText(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Height(
        pVal: i32,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Width(
        pVal: i32,
    ) -> HRESULT,
    fn get_ID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Parameter(
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn put_Parameter(
        pVal: VARIANT,
    ) -> HRESULT,
    fn get_Tag(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Tag(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_ToolTipText(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_ToolTipText(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Visible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Visible(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetIcon(
        RowIndex: i32,
        ColumnIndex: i32,
    ) -> HRESULT,
    fn SetCustomIcon(
        ImageFile: BSTR,
    ) -> HRESULT,
    fn SetIcon2(
        Icon: BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0005, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUICommandBarModes(ICUICommandBarModesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        IndexOrName: VARIANT,
        pVal: *mut *mut ICUICommandBarMode,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0006, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUICommandBarMode(ICUICommandBarModeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_NameLocal(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Controls(
        ppVal: *mut *mut ICUIControls,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee000f, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIFrameWindows(ICUIFrameWindowsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut ICUIFrameWindow,
    ) -> HRESULT,
    fn get__NewEnum(
        ppVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Find(
        ID: BSTR,
        ppVal: *mut *mut ICUIFrameWindow,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut ICUIFrameWindow,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut ICUIFrameWindow,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0010, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIFrameWindow(ICUIFrameWindowVtbl): IDispatch(IDispatchVtbl) {
    fn get_ID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Caption(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn Minimize(
    ) -> HRESULT,
    fn Maximize(
    ) -> HRESULT,
    fn Restore(
    ) -> HRESULT,
    fn get_State(
        pVal: *mut cuiWindowState,
    ) -> HRESULT,
    fn get_IsMainFrame(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Close(
    ) -> HRESULT,
    fn Activate(
    ) -> HRESULT,
    fn get_IsActive(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Handle(
        pVal: *mut i32,
    ) -> HRESULT,
    fn TileViews(
        TileHorizontally: VARIANT_BOOL,
    ) -> HRESULT,
    fn CombineViews(
    ) -> HRESULT,
    fn get_ViewHosts(
        ppVal: *mut *mut ICUIViewHosts,
    ) -> HRESULT,
    fn get_DockHosts(
        ppVal: *mut *mut ICUIDockHosts,
    ) -> HRESULT,
    fn get_RootDockHost(
        ppVal: *mut *mut ICUIDockHost,
    ) -> HRESULT,
    fn get_Position(
        ppVal: *mut *mut ICUIScreenRect,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0011, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIViewHosts(ICUIViewHostsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut ICUIViewHost,
    ) -> HRESULT,
    fn get__NewEnum(
        ppVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Find(
        ID: BSTR,
        ppVal: *mut *mut ICUIViewHost,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut ICUIViewHost,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut ICUIViewHost,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0012, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIViewHost(ICUIViewHostVtbl): IDispatch(IDispatchVtbl) {
    fn get_ID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_DockHost(
        ppVal: *mut *mut ICUIDockHost,
    ) -> HRESULT,
    fn get_Views(
        ppVal: *mut *mut ICUIViewWindows,
    ) -> HRESULT,
    fn get_DockItem(
        ppVal: *mut *mut ICUIDockItem,
    ) -> HRESULT,
    fn get_Position(
        ppVal: *mut *mut ICUIScreenRect,
    ) -> HRESULT,
    fn InsertView(
        ViewToInsert: *const ICUIViewWindow,
        Index: i32,
    ) -> HRESULT,
    fn InsertViewHost(
        ViewHostToInsert: *const ICUIViewHost,
        Index: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0014, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIDockHost(ICUIDockHostVtbl): IDispatch(IDispatchVtbl) {
    fn get_ID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Orientation(
        pVal: *mut cuiDockHostOrientation,
    ) -> HRESULT,
    fn get_ParentDockHost(
        ppVal: *mut *mut ICUIDockHost,
    ) -> HRESULT,
    fn get_DockItem(
        ppVal: *mut *mut ICUIDockItem,
    ) -> HRESULT,
    fn get_Children(
        ppVal: *mut *mut ICUIDockItems,
    ) -> HRESULT,
    fn InsertViewHost(
        ViewHostToInsert: *const ICUIViewHost,
        Index: i32,
        Operation: cuiDockOperation,
    ) -> HRESULT,
    fn InsertView(
        ViewToInsert: *const ICUIViewWindow,
        Index: i32,
        Operation: cuiDockOperation,
        ppVal: *mut *mut ICUIViewHost,
    ) -> HRESULT,
    fn get_Position(
        ppVal: *mut *mut ICUIScreenRect,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0018, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIDockItem(ICUIDockItemVtbl): IDispatch(IDispatchVtbl) {
    fn get_ID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cuiDockItemType,
    ) -> HRESULT,
    fn get_ViewHost(
        ppVal: *mut *mut ICUIViewHost,
    ) -> HRESULT,
    fn get_DockHost(
        ppVal: *mut *mut ICUIDockHost,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_RelativeSize(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_RelativeSize(
        pVal: i32,
    ) -> HRESULT,
    fn get_Position(
        ppVal: *mut *mut ICUIScreenRect,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0019, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIScreenRect(ICUIScreenRectVtbl): IDispatch(IDispatchVtbl) {
    fn get_Left(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Left(
        pVal: i32,
    ) -> HRESULT,
    fn get_Right(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Right(
        pVal: i32,
    ) -> HRESULT,
    fn get_Top(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Top(
        pVal: i32,
    ) -> HRESULT,
    fn get_Bottom(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Bottom(
        pVal: i32,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Width(
        pVal: i32,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Height(
        pVal: i32,
    ) -> HRESULT,
    fn get_CenterX(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CenterX(
        pVal: i32,
    ) -> HRESULT,
    fn get_CenterY(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CenterY(
        pVal: i32,
    ) -> HRESULT,
    fn SetPosition(
        Left: i32,
        Top: i32,
        Width: i32,
        Height: i32,
    ) -> HRESULT,
    fn Resize(
        Width: i32,
        Height: i32,
    ) -> HRESULT,
    fn Move(
        Left: i32,
        Top: i32,
    ) -> HRESULT,
    fn get_ReadOnly(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut ICUIScreenRect,
    ) -> HRESULT,
    fn CopyAssign(
        Source: *const ICUIScreenRect,
    ) -> HRESULT,
    fn Offset(
        OffsetX: i32,
        OffsetY: i32,
    ) -> HRESULT,
    fn Inflate(
        Left: i32,
        Top: i32,
        Right: i32,
        Bottom: i32,
    ) -> HRESULT,
    fn IsPointInside(
        x: i32,
        y: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Union(
        Source: *const ICUIScreenRect,
        ppVal: *mut *mut ICUIScreenRect,
    ) -> HRESULT,
    fn Intersect(
        Source: *const ICUIScreenRect,
        ppVal: *mut *mut ICUIScreenRect,
    ) -> HRESULT,
    fn IsEmpty(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0017, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIDockItems(ICUIDockItemsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut ICUIDockItem,
    ) -> HRESULT,
    fn get__NewEnum(
        ppVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Find(
        ID: BSTR,
        ppVal: *mut *mut ICUIDockItem,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut ICUIDockItem,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut ICUIDockItem,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0016, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIViewWindow(ICUIViewWindowVtbl): IDispatch(IDispatchVtbl) {
    fn get_ID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_ViewHost(
        ppVal: *mut *mut ICUIViewHost,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Kind(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Title(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Description(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Position(
        ppVal: *mut *mut ICUIScreenRect,
    ) -> HRESULT,
    fn Activate(
    ) -> HRESULT,
    fn Close(
    ) -> HRESULT,
    fn get_AppView(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0015, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIViewWindows(ICUIViewWindowsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut ICUIViewWindow,
    ) -> HRESULT,
    fn get__NewEnum(
        ppVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Find(
        ID: BSTR,
        ppVal: *mut *mut ICUIViewWindow,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut ICUIViewWindow,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut ICUIViewWindow,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0013, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIDockHosts(ICUIDockHostsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut ICUIDockHost,
    ) -> HRESULT,
    fn get__NewEnum(
        ppVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Find(
        ID: BSTR,
        ppVal: *mut *mut ICUIDockHost,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut ICUIDockHost,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut ICUIDockHost,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee000a, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIApplication(ICUIApplicationVtbl): IDispatch(IDispatchVtbl) {
    fn get_DataContext(
        ppVal: *mut *mut ICUIDataContext,
    ) -> HRESULT,
    fn RegisterDataSource(
        DataSourceName: BSTR,
        Factory: *const ICUIDataSourceFactory,
        CategoryList: BSTR,
        AutoCreateInstance: VARIANT_BOOL,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn UnregisterDataSource(
        DataSourceName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn CreateImageList(
        ppVal: *mut *mut ICUIImageList,
    ) -> HRESULT,
    fn get_FrameWork(
        ppVal: *mut *mut ICUIFrameWork,
    ) -> HRESULT,
    fn CreateScreenRect(
        Left: i32,
        Top: i32,
        Width: i32,
        Height: i32,
        ppVal: *mut *mut ICUIScreenRect,
    ) -> HRESULT,
    fn CreateBitmapImage(
        ImageData: VARIANT,
        MaxSize: i32,
        ppVal: *mut *mut ICUIBitmapImage,
    ) -> HRESULT,
    fn CreateStatusText(
        ppVal: *mut *mut ICUIStatusText,
    ) -> HRESULT,
    fn LoadLocalizedString(
        Guid: BSTR,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn CreateDataSourceFactory(
        DataSourceFactoryObject: LPDISPATCH,
        ppVal: *mut *mut ICUIDataSourceFactory,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee000b, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIDataContext(ICUIDataContextVtbl): IDispatch(IDispatchVtbl) {
    fn CreateChildDataContext(
        CategoryList: BSTR,
        ppVal: *mut *mut ICUIDataContext,
    ) -> HRESULT,
    fn get_Categories(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn HasCategory(
        Category: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn AddDataSource(
        DataSourceName: BSTR,
        DataSourceObject: LPDISPATCH,
        ppVal: *mut *mut ICUIDataSourceProxy,
    ) -> HRESULT,
    fn ShowDialog(
        dialogID: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetDataSource(
        DataSourceName: BSTR,
        ppVal: *mut *mut ICUIDataSourceProxy,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee000c, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIDataSourceProxy(ICUIDataSourceProxyVtbl): IDispatch(IDispatchVtbl) {
    fn UpdateListeners(
        ListenerNames: BSTR,
    ) -> HRESULT,
    fn get_Application(
        ppVal: *mut *mut ICUIApplication,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn InvokeMethod(
        MethodName: BSTR,
    ) -> HRESULT,
    fn GetProperty(
        PropertyName: BSTR,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn SetProperty(
        PropertyName: BSTR,
        Value: VARIANT,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee000d, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIDataSourceFactory(ICUIDataSourceFactoryVtbl): IDispatch(IDispatchVtbl) {
    fn CreateDataSource(
        DataSourceName: BSTR,
        Proxy: *const ICUIDataSourceProxy,
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee000e, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIImageList(ICUIImageListVtbl): IDispatch(IDispatchVtbl) {
    fn get_ImageCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn ImageExists(
        Key: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn RemoveAll(
    ) -> HRESULT,
    fn AddImage(
        Key: BSTR,
        ImageData: VARIANT,
        MaxSize: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ImageKeys(
        pVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn RemoveImage(
        Key: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn AddBitmap(
        Key: BSTR,
        Bitmap: *const ICUIBitmapImage,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee001a, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIBitmapImage(ICUIBitmapImageVtbl): IDispatch(IDispatchVtbl) {
    fn get_Valid(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee001b, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIStatusText(ICUIStatusTextVtbl): IDispatch(IDispatchVtbl) {
    fn SetCaptionText(
        Text: BSTR,
    ) -> HRESULT,
    fn SetBitmap(
        Bitmap: *const ICUIBitmapImage,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee001c, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIWarning(ICUIWarningVtbl): IDispatch(IDispatchVtbl) {
    fn get_Enabled(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Enabled(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Description(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Text(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Title(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn DoWarningDialog(
        unFlags: i32,
        Text: BSTR,
        pVal: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee001f, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUITaskManager(ICUITaskManagerVtbl): IDispatch(IDispatchVtbl) {
    fn RunOnUIThread(
        pTask: *const ICUITask,
    ) -> HRESULT,
    fn RunInBackground(
        priority: cuiTaskPriority,
        pTask: *const ICUIBackgroundTask,
        ppVal: *mut *mut ICUIRunningBackgroundTask,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee001d, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUITask(ICUITaskVtbl): IDispatch(IDispatchVtbl) {
    fn RunTask(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee001e, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIBackgroundTask(ICUIBackgroundTaskVtbl): ICUITask(ICUITaskVtbl) {
    fn FinalizeTask(
    ) -> HRESULT,
    fn FreeTask(
    ) -> HRESULT,
    fn QuitTask(
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0021, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIRunningBackgroundTask(ICUIRunningBackgroundTaskVtbl): ICUIRunningTask(ICUIRunningTaskVtbl) {
    fn WaitUntilDone(
    ) -> HRESULT,
    fn Reprioritize(
        __MIDL__ICUIRunningBackgroundTask0000: cuiTaskPriority,
    ) -> HRESULT,
    fn FinalizeIfDone(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9cee0020, 0x42a0, 0x5980, 0x43, 0xa3, 0x7a, 0xa7, 0x14, 0x61, 0x48, 0x2c)]
interface ICUIRunningTask(ICUIRunningTaskVtbl): IDispatch(IDispatchVtbl) {
    fn TryAbort(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa25250a9, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrintLayout(IPrnVBAPrintLayoutVtbl): IDispatch(IDispatchVtbl) {
    fn get_UseBleedLimit(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseBleedLimit(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BleedLimit(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_BleedLimit(
        pVal: f64,
    ) -> HRESULT,
    fn get_PrintTiledPages(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PrintTiledPages(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PrintTilingMarks(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PrintTilingMarks(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TileOverlap(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TileOverlap(
        pVal: f64,
    ) -> HRESULT,
    fn get_Placement(
        pVal: *mut PrnPlaceType,
    ) -> HRESULT,
    fn put_Placement(
        pVal: PrnPlaceType,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa2525098, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrinter(IPrnVBAPrinterVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Default(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Ready(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Port(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Description(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_PostScriptEnabled(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColorEnabled(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ShowDialog(
    ) -> HRESULT,
    fn get_PageSizeMatchingSupported(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PageSizeMatchingSupported(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa2525099, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrinters(IPrnVBAPrintersVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        nIndex: i32,
        pVal: *mut *mut IPrnVBAPrinter,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Default(
        pVal: *mut *mut IPrnVBAPrinter,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa252509a, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBASeparationPlate(IPrnVBASeparationPlateVtbl): IDispatch(IDispatchVtbl) {
    fn get_Enabled(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Enabled(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut PrnPlateType,
    ) -> HRESULT,
    fn get_Color(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Frequency(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Frequency(
        pVal: f64,
    ) -> HRESULT,
    fn get_Angle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Angle(
        pVal: f64,
    ) -> HRESULT,
    fn get_OverprintText(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_OverprintText(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_OverprintGraphic(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_OverprintGraphic(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa252509b, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBASeparationPlates(IPrnVBASeparationPlatesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        Index: i32,
        pVal: *mut *mut IPrnVBASeparationPlate,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa252509c, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrintSeparations(IPrnVBAPrintSeparationsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Enabled(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Enabled(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_InColor(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_InColor(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Hexachrome(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Hexachrome(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_SpotToCMYK(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_SpotToCMYK(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_EmptyPlates(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_EmptyPlates(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PreserveOverprints(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PreserveOverprints(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AlwaysOverprintBlack(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AlwaysOverprintBlack(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AutoSpreading(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AutoSpreading(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AutoSpreadAmount(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_AutoSpreadAmount(
        pVal: f64,
    ) -> HRESULT,
    fn get_AutoSpreadFixed(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AutoSpreadFixed(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AutoSpreadTextAbove(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_AutoSpreadTextAbove(
        pVal: f64,
    ) -> HRESULT,
    fn get_AdvancedSettings(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AdvancedSettings(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Plates(
        pVal: *mut *mut IPrnVBASeparationPlates,
    ) -> HRESULT,
    fn get_Resolution(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Resolution(
        pVal: i32,
    ) -> HRESULT,
    fn get_BasicScreen(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_BasicScreen(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_HalftoneType(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_HalftoneType(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_ScreenTechnology(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_ScreenTechnology(
        pVal: BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa252509d, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrintPrepress(IPrnVBAPrintPrepressVtbl): IDispatch(IDispatchVtbl) {
    fn get_Invert(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Invert(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Mirror(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Mirror(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FileInfo(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FileInfo(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_JobName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_JobName(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_PageNumbers(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PageNumbers(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_InfoWithinPage(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_InfoWithinPage(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CropMarks(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_CropMarks(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ExteriorCropMarks(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ExteriorCropMarks(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RegistrationMarks(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RegistrationMarks(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RegistrationStyle(
        pVal: *mut PrnRegistrationStyle,
    ) -> HRESULT,
    fn put_RegistrationStyle(
        pVal: PrnRegistrationStyle,
    ) -> HRESULT,
    fn get_ColorCalibrationBar(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ColorCalibrationBar(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DensitometerScale(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DensitometerScale(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Densities(
        Index: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Densities(
        Index: i32,
        pVal: i32,
    ) -> HRESULT,
    fn get_MarksToObjects(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MarksToObjects(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa252509e, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrintPostScript(IPrnVBAPrintPostScriptVtbl): IDispatch(IDispatchVtbl) {
    fn get_Level(
        pVal: *mut PrnPostScriptLevel,
    ) -> HRESULT,
    fn put_Level(
        pVal: PrnPostScriptLevel,
    ) -> HRESULT,
    fn get_ConformToDSC(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ConformToDSC(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_JPEGCompression(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_JPEGCompression(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_JPEGQuality(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_JPEGQuality(
        pVal: i32,
    ) -> HRESULT,
    fn get_MaintainOPILinks(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MaintainOPILinks(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ResolveDCSLinks(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ResolveDCSLinks(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DownloadType1(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DownloadType1(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TrueTypeToType1(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TrueTypeToType1(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PDFStartup(
        pVal: *mut PrnPDFStartup,
    ) -> HRESULT,
    fn put_PDFStartup(
        pVal: PrnPDFStartup,
    ) -> HRESULT,
    fn get_PDFHyperlinks(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PDFHyperlinks(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_pdfBookmarks(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_pdfBookmarks(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MaxPointsPerCurve(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_MaxPointsPerCurve(
        pVal: i32,
    ) -> HRESULT,
    fn get_Flatness(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Flatness(
        pVal: i32,
    ) -> HRESULT,
    fn get_AutoIncreaseFlatness(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AutoIncreaseFlatness(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AutoIncreaseFountainSteps(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AutoIncreaseFountainSteps(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_OptimizeFountainFills(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_OptimizeFountainFills(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ScreenFrequency(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_ScreenFrequency(
        pVal: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa252509f, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBATrapLayer(IPrnVBATrapLayerVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut PrnPlateType,
    ) -> HRESULT,
    fn get_Color(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Density(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Density(
        pVal: f64,
    ) -> HRESULT,
    fn get_TrapType(
        pVal: *mut PrnTrapType,
    ) -> HRESULT,
    fn put_TrapType(
        pVal: PrnTrapType,
    ) -> HRESULT,
    fn get_Order(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Order(
        pVal: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa25250a0, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBATrapLayers(IPrnVBATrapLayersVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        pVal: *mut *mut IPrnVBATrapLayer,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa25250a1, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrintTrapping(IPrnVBAPrintTrappingVtbl): IDispatch(IDispatchVtbl) {
    fn get_Enabled(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Enabled(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Layers(
        pVal: *mut *mut IPrnVBATrapLayers,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Width(
        pVal: f64,
    ) -> HRESULT,
    fn get_BlackWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_BlackWidth(
        pVal: f64,
    ) -> HRESULT,
    fn get_ColorScaling(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_ColorScaling(
        pVal: i32,
    ) -> HRESULT,
    fn get_StepLimit(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_StepLimit(
        pVal: i32,
    ) -> HRESULT,
    fn get_BlackColorLimit(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_BlackColorLimit(
        pVal: i32,
    ) -> HRESULT,
    fn get_BlackDensityLimit(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_BlackDensityLimit(
        pVal: f64,
    ) -> HRESULT,
    fn get_SlidingTrapLimit(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_SlidingTrapLimit(
        pVal: i32,
    ) -> HRESULT,
    fn get_ImageTrap(
        pVal: *mut PrnImageTrap,
    ) -> HRESULT,
    fn put_ImageTrap(
        pVal: PrnImageTrap,
    ) -> HRESULT,
    fn get_ObjectsToImage(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ObjectsToImage(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_InternalImageTrapping(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_InternalImageTrapping(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TrapMonoBitmaps(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TrapMonoBitmaps(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa25250a2, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrintOptions(IPrnVBAPrintOptionsVtbl): IDispatch(IDispatchVtbl) {
    fn get_UseColorProfile(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseColorProfile(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PrintVectors(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PrintVectors(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PrintBitmaps(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PrintBitmaps(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PrintText(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PrintText(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TextInBlack(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TextInBlack(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColorMode(
        pVal: *mut PrnColorMode,
    ) -> HRESULT,
    fn put_ColorMode(
        pVal: PrnColorMode,
    ) -> HRESULT,
    fn get_MarksToPage(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MarksToPage(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BitmapColorMode(
        pVal: *mut PrnBitmapColorMode,
    ) -> HRESULT,
    fn put_BitmapColorMode(
        pVal: PrnBitmapColorMode,
    ) -> HRESULT,
    fn get_FountainSteps(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_FountainSteps(
        pVal: i32,
    ) -> HRESULT,
    fn get_RasterizePage(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RasterizePage(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RasterizeResolution(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_RasterizeResolution(
        pVal: i32,
    ) -> HRESULT,
    fn get_DownsampleColor(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DownsampleColor(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DownsampleGray(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DownsampleGray(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DownsampleMono(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DownsampleMono(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColorResolution(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_ColorResolution(
        pVal: i32,
    ) -> HRESULT,
    fn get_GrayResolution(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_GrayResolution(
        pVal: i32,
    ) -> HRESULT,
    fn get_MonoResolution(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_MonoResolution(
        pVal: i32,
    ) -> HRESULT,
    fn get_JobInformation(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_JobInformation(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AppInfo(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AppInfo(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DriverInfo(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DriverInfo(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PrintJobInfo(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PrintJobInfo(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_SepsInfo(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_SepsInfo(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FontInfo(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FontInfo(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_LinkInfo(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_LinkInfo(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_InRIPTrapInfo(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_InRIPTrapInfo(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ObjectsColorMode(
        pVal: *mut PrnObjectsColorMode,
    ) -> HRESULT,
    fn put_ObjectsColorMode(
        pVal: PrnObjectsColorMode,
    ) -> HRESULT,
    fn get_PreservePureBlack(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PreservePureBlack(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa25250a3, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrintSettings(IPrnVBAPrintSettingsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Printer(
        pVal: *mut *mut IPrnVBAPrinter,
    ) -> HRESULT,
    fn putref_Printer(
        pVal: *const IPrnVBAPrinter,
    ) -> HRESULT,
    fn get_UsePPD(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UsePPD(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PPDFile(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_PPDFile(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_PrintToFile(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PrintToFile(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FileName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_FileName(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_ForMac(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ForMac(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FileMode(
        pVal: *mut PrnFileMode,
    ) -> HRESULT,
    fn put_FileMode(
        pVal: PrnFileMode,
    ) -> HRESULT,
    fn get_PrintRange(
        pVal: *mut PrnPrintRange,
    ) -> HRESULT,
    fn put_PrintRange(
        pVal: PrnPrintRange,
    ) -> HRESULT,
    fn get_PageRange(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_PageRange(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Copies(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Copies(
        pVal: i32,
    ) -> HRESULT,
    fn get_Collate(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Collate(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Separations(
        pVal: *mut *mut IPrnVBAPrintSeparations,
    ) -> HRESULT,
    fn get_Prepress(
        pVal: *mut *mut IPrnVBAPrintPrepress,
    ) -> HRESULT,
    fn get_PostScript(
        pVal: *mut *mut IPrnVBAPrintPostScript,
    ) -> HRESULT,
    fn get_Trapping(
        pVal: *mut *mut IPrnVBAPrintTrapping,
    ) -> HRESULT,
    fn get_Options(
        pVal: *mut *mut IPrnVBAPrintOptions,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn Load(
        FileName: BSTR,
        pbSuccess: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Save(
        FileName: BSTR,
        pbSuccess: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ShowDialog(
        pbPrint: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn SelectPrinter(
        Name: BSTR,
    ) -> HRESULT,
    fn get_PageSet(
        pVal: *mut PrnPageSet,
    ) -> HRESULT,
    fn put_PageSet(
        pVal: PrnPageSet,
    ) -> HRESULT,
    fn get_PaperOrientation(
        pVal: *mut PrnPaperOrientation,
    ) -> HRESULT,
    fn put_PaperOrientation(
        pVal: PrnPaperOrientation,
    ) -> HRESULT,
    fn get_PaperSize(
        pVal: *mut PrnPaperSize,
    ) -> HRESULT,
    fn put_PaperSize(
        pVal: PrnPaperSize,
    ) -> HRESULT,
    fn SetPaperSize(
        PaperSize: PrnPaperSize,
        Orientation: PrnPaperOrientation,
    ) -> HRESULT,
    fn get_PaperWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_PaperHeight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn SetCustomPaperSize(
        Width: f64,
        Height: f64,
        Orientation: PrnPaperOrientation,
    ) -> HRESULT,
    fn get_Layout(
        pVal: *mut *mut IPrnVBAPrintLayout,
    ) -> HRESULT,
    fn PrintOut(
    ) -> HRESULT,
    fn PrintColorProof(
        ProofSettings: LPDISPATCH,
    ) -> HRESULT,
    fn get_PageMatchingMode(
        pVal: *mut PrnPageMatchingMode,
    ) -> HRESULT,
    fn put_PageMatchingMode(
        pVal: PrnPageMatchingMode,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa25250a4, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrintDocument(IPrnVBAPrintDocumentVtbl): IDispatch(IDispatchVtbl) {
    fn _GetPrintDocument(
        pDoc: *mut INT_PTR,
    ) -> HRESULT,
}}

pub type INT_PTR = i64;

RIDL!{#[uuid(0xa25250a5, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrintDocuments(IPrnVBAPrintDocumentsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        Index: i32,
        pVal: *mut *mut IPrnVBAPrintDocument,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa25250a6, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrintPage(IPrnVBAPrintPageVtbl): IDispatch(IDispatchVtbl) {
    fn _GetPrintDocument(
        pDoc: *mut INT_PTR,
    ) -> HRESULT,
    fn _GetPrintPage(
        pPage: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa25250a7, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrintPages(IPrnVBAPrintPagesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        Index: i32,
        pVal: *mut *mut IPrnVBAPrintPage,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa25250a8, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPrnVBAPrintJob(IPrnVBAPrintJobVtbl): IDispatch(IDispatchVtbl) {
    fn get_Settings(
        pVal: *mut *mut IPrnVBAPrintSettings,
    ) -> HRESULT,
    fn get_Documents(
        pVal: *mut *mut IPrnVBAPrintDocuments,
    ) -> HRESULT,
    fn get_Pages(
        pVal: *mut *mut IPrnVBAPrintPages,
    ) -> HRESULT,
    fn Clear(
    ) -> HRESULT,
    fn PrintOut(
    ) -> HRESULT,
    fn AddDocument(
        Document: *const IPrnVBAPrintDocument,
        PageRange: BSTR,
    ) -> HRESULT,
    fn AddPage(
        Page: *const IPrnVBAPrintPage,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa2524ffb, 0x50c1, 0x11d3, 0x8e, 0xa3, 0x00, 0x90, 0x27, 0x1b, 0xec, 0xdd)]
interface IPDFVBASettings(IPDFVBASettingsVtbl): IDispatch(IDispatchVtbl) {
    fn Reset(
    ) -> HRESULT,
    fn Load(
        SettingName: BSTR,
        Result: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Save(
        SettingName: BSTR,
        Result: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ShowDialog(
        Result: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn PublishToPDF(
        FileName: BSTR,
    ) -> HRESULT,
    fn put_PublishRange(
        pExportRange: pdfExportRange,
    ) -> HRESULT,
    fn get_PublishRange(
        pExportRange: *mut pdfExportRange,
    ) -> HRESULT,
    fn put_PageRange(
        pszExportPagesRange: BSTR,
    ) -> HRESULT,
    fn get_PageRange(
        pszExportPagesRange: *mut BSTR,
    ) -> HRESULT,
    fn put_Author(
        pszAuthor: BSTR,
    ) -> HRESULT,
    fn get_Author(
        pszAuthor: *mut BSTR,
    ) -> HRESULT,
    fn put_Subject(
        pszSubject: BSTR,
    ) -> HRESULT,
    fn get_Subject(
        pszSubject: *mut BSTR,
    ) -> HRESULT,
    fn put_Keywords(
        pszKeywords: BSTR,
    ) -> HRESULT,
    fn get_Keywords(
        pszKeywords: *mut BSTR,
    ) -> HRESULT,
    fn put_BitmapCompression(
        pBitmapCompressionType: pdfBitmapCompressionType,
    ) -> HRESULT,
    fn get_BitmapCompression(
        pBitmapCompressionType: *mut pdfBitmapCompressionType,
    ) -> HRESULT,
    fn put_JPEGQualityFactor(
        pnQuality: INT,
    ) -> HRESULT,
    fn get_JPEGQualityFactor(
        pnQuality: *mut INT,
    ) -> HRESULT,
    fn put_TextAsCurves(
        pbTextAsCurves: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TextAsCurves(
        pbTextAsCurves: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_EmbedFonts(
        pbEmbedFonts: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_EmbedFonts(
        pbEmbedFonts: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_EmbedBaseFonts(
        pbEmbedBaseFonts: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_EmbedBaseFonts(
        pbEmbedBaseFonts: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TrueTypeToType1(
        pbTrueTypeToType1: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TrueTypeToType1(
        pbTrueTypeToType1: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_SubsetFonts(
        pbSubsetType1Fonts: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_SubsetFonts(
        pbSubsetType1Fonts: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_SubsetPct(
        pnLevel: i32,
    ) -> HRESULT,
    fn get_SubsetPct(
        pnLevel: *mut i32,
    ) -> HRESULT,
    fn put_CompressText(
        pbCompressText: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CompressText(
        pbCompressText: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Encoding(
        pEncoding: pdfEncodingType,
    ) -> HRESULT,
    fn get_Encoding(
        pEncoding: *mut pdfEncodingType,
    ) -> HRESULT,
    fn put_DownsampleColor(
        pbDownsample: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DownsampleColor(
        pbDownsample: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DownsampleGray(
        pbDownsample: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DownsampleGray(
        pbDownsample: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DownsampleMono(
        pbDownsample: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DownsampleMono(
        pbDownsample: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ColorResolution(
        pnDownsampleResolution: i32,
    ) -> HRESULT,
    fn get_ColorResolution(
        pnDownsampleResolution: *mut i32,
    ) -> HRESULT,
    fn put_MonoResolution(
        pnDownsampleResolution: i32,
    ) -> HRESULT,
    fn get_MonoResolution(
        pnDownsampleResolution: *mut i32,
    ) -> HRESULT,
    fn put_GrayResolution(
        pnDownsampleResolution: i32,
    ) -> HRESULT,
    fn get_GrayResolution(
        pnDownsampleResolution: *mut i32,
    ) -> HRESULT,
    fn put_Hyperlinks(
        pbIncludeHyperlinks: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Hyperlinks(
        pbIncludeHyperlinks: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Bookmarks(
        pbGenerateBookmarks: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Bookmarks(
        pbGenerateBookmarks: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Thumbnails(
        pbGenerateThumbnails: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Thumbnails(
        pbGenerateThumbnails: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Startup(
        pDisplayOnStart: pdfDisplayOnStart,
    ) -> HRESULT,
    fn get_Startup(
        pDisplayOnStart: *mut pdfDisplayOnStart,
    ) -> HRESULT,
    fn put_ComplexFillsAsBitmaps(
        pbComplexFillsAsBitmaps: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ComplexFillsAsBitmaps(
        pbComplexFillsAsBitmaps: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Overprints(
        pbPreserveOverprints: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Overprints(
        pbPreserveOverprints: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Halftones(
        pbPreserveHalftones: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Halftones(
        pbPreserveHalftones: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_SpotColors(
        pbPreserveSpotColors: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_SpotColors(
        pbPreserveSpotColors: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MaintainOPILinks(
        pbMaintainOPILinks: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MaintainOPILinks(
        pbMaintainOPILinks: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FountainSteps(
        pnFountainSteps: i32,
    ) -> HRESULT,
    fn get_FountainSteps(
        pnFountainSteps: *mut i32,
    ) -> HRESULT,
    fn put_EPSAs(
        peEPSAs: pdfEPSAs,
    ) -> HRESULT,
    fn get_EPSAs(
        peEPSAs: *mut pdfEPSAs,
    ) -> HRESULT,
    fn put_pdfVersion(
        pePDFVersion: pdfVersion,
    ) -> HRESULT,
    fn get_pdfVersion(
        pePDFVersion: *mut pdfVersion,
    ) -> HRESULT,
    fn put_IncludeBleed(
        pbIncludeBleed: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IncludeBleed(
        pbIncludeBleed: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Bleed(
        pnBleed: INT,
    ) -> HRESULT,
    fn get_Bleed(
        pnBleed: *mut INT,
    ) -> HRESULT,
    fn put_Linearize(
        pbLinearize: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Linearize(
        pbLinearize: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_CropMarks(
        pbCropMarks: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CropMarks(
        pbCropMarks: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RegistrationMarks(
        pbRegistrationMarks: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RegistrationMarks(
        pbRegistrationMarks: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DensitometerScales(
        pbDensitometerScales: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DensitometerScales(
        pbDensitometerScales: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FileInformation(
        pbFileInformation: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FileInformation(
        pbFileInformation: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ColorMode(
        peColorSet: pdfColorMode,
    ) -> HRESULT,
    fn get_ColorMode(
        peColorSet: *mut pdfColorMode,
    ) -> HRESULT,
    fn put_UseColorProfile(
        pbUseColorProfile: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_UseColorProfile(
        pbUseColorProfile: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ColorProfile(
        peColorProfile: pdfColorProfile,
    ) -> HRESULT,
    fn get_ColorProfile(
        peColorProfile: *mut pdfColorProfile,
    ) -> HRESULT,
    fn put_EmbedFilename(
        pszEmbedFilename: BSTR,
    ) -> HRESULT,
    fn get_EmbedFilename(
        pszEmbedFilename: *mut BSTR,
    ) -> HRESULT,
    fn put_EmbedFile(
        pbEmbedFile: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_EmbedFile(
        pbEmbedFile: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_JP2QualityFactor(
        pnQuality: INT,
    ) -> HRESULT,
    fn get_JP2QualityFactor(
        pnQuality: *mut INT,
    ) -> HRESULT,
    fn put_TextExportMode(
        pExportMode: pdfTextExportMode,
    ) -> HRESULT,
    fn get_TextExportMode(
        pExportMode: *mut pdfTextExportMode,
    ) -> HRESULT,
    fn put_PrintPermissions(
        pPrintPermission: pdfPrintPermissions,
    ) -> HRESULT,
    fn get_PrintPermissions(
        pPrintPermission: *mut pdfPrintPermissions,
    ) -> HRESULT,
    fn put_EditPermissions(
        pEditPermission: pdfEditPermissions,
    ) -> HRESULT,
    fn get_EditPermissions(
        pEditPermission: *mut pdfEditPermissions,
    ) -> HRESULT,
    fn put_ContentCopyingAllowed(
        pbEnable: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ContentCopyingAllowed(
        pbEnable: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_OpenPassword(
        pszOpenPassword: BSTR,
    ) -> HRESULT,
    fn get_OpenPassword(
        pszOpenPassword: *mut BSTR,
    ) -> HRESULT,
    fn put_PermissionPassword(
        pszPermissionPassword: BSTR,
    ) -> HRESULT,
    fn get_PermissionPassword(
        pszPermissionPassword: *mut BSTR,
    ) -> HRESULT,
    fn put_ConvertSpotColors(
        pbConvertSpotColors: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ConvertSpotColors(
        pbConvertSpotColors: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_EncryptType(
        peEncryptType: pdfEncryptionType,
    ) -> HRESULT,
    fn get_EncryptType(
        peEncryptType: *mut pdfEncryptionType,
    ) -> HRESULT,
    fn put_OutputSpotColorsAs(
        pnConvertSpotColorsTo: pdfSpotType,
    ) -> HRESULT,
    fn get_OutputSpotColorsAs(
        pnConvertSpotColorsTo: *mut pdfSpotType,
    ) -> HRESULT,
    fn put_OverprintBlackLimit(
        pnOverprintBlackLimit: INT,
    ) -> HRESULT,
    fn get_OverprintBlackLimit(
        pnOverprintBlackLimit: *mut INT,
    ) -> HRESULT,
    fn put_ProtectedTextAsCurves(
        pbProtectedTextAsCurves: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ProtectedTextAsCurves(
        pbProtectedTextAsCurves: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580024, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDocument(IVGDocumentVtbl): IDispatch(IDispatchVtbl) {
    
}}

RIDL!{#[uuid(0xb058000b, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGApplication(IVGApplicationVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Visible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Visible(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Documents(
        Documents: *mut *mut IVGDocuments,
    ) -> HRESULT,
    fn get_ActiveDocument(
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get_ActivePage(
        ppVal: *mut *mut IVGPage,
    ) -> HRESULT,
    fn get_ActiveWindow(
        ppVal: *mut *mut IVGWindow,
    ) -> HRESULT,
    fn get_Windows(
        ppVal: *mut *mut IVGWindows,
    ) -> HRESULT,
    fn CorelScriptTools(
        ppVal: *mut *mut ICorelScriptTools,
    ) -> HRESULT,
    fn get_ActiveWorkspace(
        ppVal: *mut *mut IVGWorkspace,
    ) -> HRESULT,
    fn get_Workspaces(
        ppVal: *mut *mut IVGWorkspaces,
    ) -> HRESULT,
    fn get_ActivePalette(
        ppVal: *mut *mut IVGPalette,
    ) -> HRESULT,
    fn get_Palettes(
        ppVal: *mut *mut IVGPalettes,
    ) -> HRESULT,
    fn Quit(
    ) -> HRESULT,
    fn CreateColor(
        ColorString: BSTR,
        ColorVariable: *mut *mut IVGColor,
    ) -> HRESULT,
    fn get_FontList(
        ppVal: *mut *mut IVGFontList,
    ) -> HRESULT,
    fn get_AppWindow(
        ppVal: *mut *mut IVGAppWindow,
    ) -> HRESULT,
    fn get_RecentFiles(
        ppVal: *mut *mut IVGRecentFiles,
    ) -> HRESULT,
    fn get_VBE(
        ppReturn: *mut LPDISPATCH,
    ) -> HRESULT,
    fn cdrMixedDouble(
        pVal: *mut f64,
    ) -> HRESULT,
    fn cdrMixedSingle(
        pVal: *mut f32,
    ) -> HRESULT,
    fn cdrMixedLong(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_EventsEnabled(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_EventsEnabled(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn OpenDocument(
        FileName: BSTR,
        CodePage: i32,
        ppDocument: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn CreateDocument(
        ppDocument: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn CreateColorEx(
        ColorModel: i32,
        V1: i32,
        V2: i32,
        V3: i32,
        V4: i32,
        V5: i32,
        V6: i32,
        V7: i32,
        ColorVariable: *mut *mut IVGColor,
    ) -> HRESULT,
    fn get_ArrowHeads(
        ppVal: *mut *mut IVGArrowHeads,
    ) -> HRESULT,
    fn get_OutlineStyles(
        ppVal: *mut *mut IVGOutlineStyles,
    ) -> HRESULT,
    fn get_Version(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_VersionMajor(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_VersionMinor(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_VersionBuild(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Path(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_ConfigPath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_SetupPath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_ActiveLayer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_ActiveSelection(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_PatternCanvases(
        ppVal: *mut *mut IVGPatternCanvases,
    ) -> HRESULT,
    fn get_Clipboard(
        ppVal: *mut *mut IVGClipboard,
    ) -> HRESULT,
    fn get_ActiveSelectionRange(
        pVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn get_ActiveTool(
        pTool: *mut cdrTools,
    ) -> HRESULT,
    fn put_ActiveTool(
        pTool: cdrTools,
    ) -> HRESULT,
    fn get_ActiveShape(
        pVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Optimization(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Optimization(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PanoseMatching(
        pVal: *mut cdrPanoseMatchingType,
    ) -> HRESULT,
    fn put_PanoseMatching(
        pVal: cdrPanoseMatchingType,
    ) -> HRESULT,
    fn get_AddIns(
        ppReturn: *mut LPDISPATCH,
    ) -> HRESULT,
    fn CreateRGBColor(
        Red: i32,
        Green: i32,
        Blue: i32,
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreateCMYColor(
        Cyan: i32,
        Magenta: i32,
        Yellow: i32,
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreateCMYKColor(
        Cyan: i32,
        Magenta: i32,
        Yellow: i32,
        Black: i32,
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreateGrayColor(
        GrayValue: i32,
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreateHLSColor(
        Hue: i32,
        Lightness: i32,
        Saturation: i32,
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreateHSBColor(
        Hue: i32,
        Saturation: i32,
        Brightness: i32,
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreateBWColor(
        White: VARIANT_BOOL,
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreateYIQColor(
        y: i32,
        I: i32,
        Q: i32,
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreateLabColor(
        L: i32,
        A: i32,
        B: i32,
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreateFixedColor(
        PaletteID: cdrPaletteID,
        PaletteIndex: i32,
        Tint: i32,
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreateRegistrationColor(
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreateSnapPoint(
        PositionX: f64,
        PositionY: f64,
        pVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn CreateDocumentFromTemplate(
        Template: BSTR,
        IncludeGraphics: VARIANT_BOOL,
        ppDocument: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get_Printers(
        ppReturn: *mut *mut IPrnVBAPrinters,
    ) -> HRESULT,
    fn get_PrintJob(
        ppReturn: *mut *mut IPrnVBAPrintJob,
    ) -> HRESULT,
    fn get_CommandBars(
        ppReturn: *mut *mut ICUICommandBars,
    ) -> HRESULT,
    fn get_StatusBar(
        ppReturn: *mut *mut ICUICommandBar,
    ) -> HRESULT,
    fn get_MainMenu(
        ppReturn: *mut *mut ICUICommandBar,
    ) -> HRESULT,
    fn get_GMSManager(
        ppVal: *mut *mut IVGGMSManager,
    ) -> HRESULT,
    fn ImportWorkspace(
        FileName: BSTR,
    ) -> HRESULT,
    fn Refresh(
    ) -> HRESULT,
    fn CreateStructSaveAsOptions(
        ppVal: *mut *mut IVGStructSaveAsOptions,
    ) -> HRESULT,
    fn CreateStructExportOptions(
        ppVal: *mut *mut IVGStructExportOptions,
    ) -> HRESULT,
    fn CreateStructImportOptions(
        ppVal: *mut *mut IVGStructImportOptions,
    ) -> HRESULT,
    fn CreateStructPaletteOptions(
        ppVal: *mut *mut IVGStructPaletteOptions,
    ) -> HRESULT,
    fn CreateNodeRange(
        ppVal: *mut *mut IVGNodeRange,
    ) -> HRESULT,
    fn CreateSegmentRange(
        ppVal: *mut *mut IVGSegmentRange,
    ) -> HRESULT,
    fn CreateShapeRange(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn CreatePatternCanvas(
        ppVal: *mut *mut IVGPatternCanvas,
    ) -> HRESULT,
    fn CreateCurve(
        Document: *const IVGDocument,
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_UserDataPath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn InitializeVBA(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_HelpFile(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_FrameWork(
        ppReturn: *mut *mut ICUIFrameWork,
    ) -> HRESULT,
    fn CreateStructFontProperties(
        ppVal: *mut *mut IVGStructFontProperties,
    ) -> HRESULT,
    fn CreateStructAlignProperties(
        ppVal: *mut *mut IVGStructAlignProperties,
    ) -> HRESULT,
    fn CreateStructSpaceProperties(
        ppVal: *mut *mut IVGStructSpaceProperties,
    ) -> HRESULT,
    fn CreateStructHyphenationSettings(
        ppVal: *mut *mut IVGStructHyphenationSettings,
    ) -> HRESULT,
    fn get_Components(
        ppVal: *mut *mut IVGComponents,
    ) -> HRESULT,
    fn get_SymbolLibraries(
        ppVal: *mut *mut IVGSymbolLibraries,
    ) -> HRESULT,
    fn AdviseEvents(
        EventSink: LPDISPATCH,
        pCookie: *mut i32,
    ) -> HRESULT,
    fn UnadviseEvents(
        Cookie: i32,
    ) -> HRESULT,
    fn get_ID(
        pVal: *mut cdrApplicationID,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Class(
        pVal: *mut cdrApplicationClass,
    ) -> HRESULT,
    fn get_PlatformVersionMajor(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_PlatformVersionMinor(
        pVal: *mut i32,
    ) -> HRESULT,
    fn CheckPlatformVersion(
        VersionMajor: i32,
        VersionMinor: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Status(
        ppVal: *mut *mut IVGAppStatus,
    ) -> HRESULT,
    fn ConvertUnits(
        Value: f64,
        FromUnit: cdrUnit,
        ToUnit: cdrUnit,
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_UILanguage(
        pVal: *mut cdrTextLanguage,
    ) -> HRESULT,
    fn IsUILanguageAvailable(
        Language: cdrTextLanguage,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PageSizes(
        ppVal: *mut *mut IVGPageSizes,
    ) -> HRESULT,
    fn get_Unit(
        pVal: *mut cdrUnit,
    ) -> HRESULT,
    fn put_Unit(
        pVal: cdrUnit,
    ) -> HRESULT,
    fn ConvertToUnicode(
        String: BSTR,
        CodePage: i32,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn ConvertFromUnicode(
        String: BSTR,
        CodePage: i32,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_UserWorkspacePath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_LanguagePath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_ActiveTreeManager(
        ppVal: *mut *mut IVGTreeManager,
    ) -> HRESULT,
    fn get_ActiveVirtualLayer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn CreateDuotone(
        ppVal: *mut *mut IVGDuotone,
    ) -> HRESULT,
    fn get_ColorManager(
        ppVal: *mut *mut IVGColorManager,
    ) -> HRESULT,
    fn get_EnhancedOutlines(
        ppVal: *mut *mut IVGOutlineStyles,
    ) -> HRESULT,
    fn AddPluginCommand(
        CommandID: BSTR,
        Caption: BSTR,
        Tooltip: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn RemovePluginCommand(
        CommandID: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn CreateOutlineStyle(
        DashDotCount: i32,
        DashDotLengths: *const SAFEARRAY,
        ppVal: *mut *mut IVGOutlineStyle,
    ) -> HRESULT,
    fn get_StartupMode(
        pVal: *mut cdrAppStartupMode,
    ) -> HRESULT,
    fn put_StartupMode(
        pVal: cdrAppStartupMode,
    ) -> HRESULT,
    fn get_GlobalUserData(
        ppVal: *mut *mut IVGProperties,
    ) -> HRESULT,
    fn get_SessionUserData(
        ppVal: *mut *mut IVGProperties,
    ) -> HRESULT,
    fn Evaluate(
        Expression: BSTR,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn CreateRect(
        x: f64,
        y: f64,
        Width: f64,
        Height: f64,
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn ForceUpdateFontTable(
    ) -> HRESULT,
    fn get_ActiveSpread(
        ppVal: *mut *mut IVGSpread,
    ) -> HRESULT,
    fn OpenDocumentAsCopy(
        FileName: BSTR,
        Options: *const IVGStructOpenOptions,
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get_DefaultColorContext(
        ppVal: *mut *mut IVGColorContext,
    ) -> HRESULT,
    fn CreateColorContext(
        RGBProfile: *const IVGColorProfile,
        CMYKProfile: *const IVGColorProfile,
        GrayscaleProfile: *const IVGColorProfile,
        RenderingIntent: clrRenderingIntent,
        BlendingColorModel: clrColorModel,
        ppVal: *mut *mut IVGColorContext,
    ) -> HRESULT,
    fn CreateColorContext2(
        ColorProfileList: BSTR,
        RenderingIntent: clrRenderingIntent,
        BlendingColorModel: clrColorModel,
        ppVal: *mut *mut IVGColorContext,
    ) -> HRESULT,
    fn CreateDocumentEx(
        Options: *const IVGStructCreateOptions,
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn OpenDocumentEx(
        FileName: BSTR,
        Options: *const IVGStructOpenOptions,
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn CreateStructOpenOptions(
        ppVal: *mut *mut IVGStructOpenOptions,
    ) -> HRESULT,
    fn CreateStructCreateOptions(
        ppVal: *mut *mut IVGStructCreateOptions,
    ) -> HRESULT,
    fn CreateStructPasteOptions(
        ppVal: *mut *mut IVGStructPasteOptions,
    ) -> HRESULT,
    fn CreateProofColorSettings(
        ProfileName: BSTR,
        RenderingIntent: clrRenderingIntent,
        PreserveColorValues: VARIANT_BOOL,
        ppVal: *mut *mut IVGProofColorSettings,
    ) -> HRESULT,
    fn get_PaletteManager(
        ppVal: *mut *mut IVGPaletteManager,
    ) -> HRESULT,
    fn CreateSpotColor(
        PaletteIdentifier: BSTR,
        SpotColorID: i32,
        Tint: i32,
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreateSpotColorByName(
        PaletteIdentifier: BSTR,
        SpotColorName: BSTR,
        Tint: i32,
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn CreatePaletteColor(
        PaletteIdentifier: BSTR,
        ColorIndex: i32,
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn GetSupportedOpenTypeFeatures(
        pVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn CreateFillMetadata(
        ppVal: *mut *mut IVGFillMetadata,
    ) -> HRESULT,
    fn get_AddonPath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_ProgramPath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_ActiveToolStateGuid(
        pTool: *mut BSTR,
    ) -> HRESULT,
    fn put_ActiveToolStateGuid(
        pTool: BSTR,
    ) -> HRESULT,
    fn RegisterToolState(
        ToolStateGuid: BSTR,
        ToolStateName: BSTR,
        ToolState: *const IVGToolState,
    ) -> HRESULT,
    fn UnregisterToolState(
        ToolStateGuid: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn CreateOnScreenCurve(
        ppVal: *mut *mut IVGOnScreenCurve,
    ) -> HRESULT,
    fn CreateOnScreenHandle(
        ppVal: *mut *mut IVGOnScreenHandle,
    ) -> HRESULT,
    fn CreateOnScreenText(
        ppVal: *mut *mut IVGOnScreenText,
    ) -> HRESULT,
    fn get_Math(
        ppVal: *mut *mut IVGMathUtils,
    ) -> HRESULT,
    fn RegisterUserApplicationPreference(
        GroupName: BSTR,
        KeyName: BSTR,
        DefaultVal: VARIANT,
    ) -> HRESULT,
    fn GetApplicationPreferenceValue(
        GroupName: BSTR,
        KeyName: BSTR,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn SetApplicationPreferenceValue(
        GroupName: BSTR,
        KeyName: BSTR,
        newVal: VARIANT,
    ) -> HRESULT,
    fn CreateProperties(
        ppVal: *mut *mut IVGProperties,
    ) -> HRESULT,
    fn RegisterToolShape(
        ToolShapeGuid: BSTR,
        ToolShapeAttributes: *const IVGToolShapeAttributes,
        ToolShape: *const IVGToolShape,
    ) -> HRESULT,
    fn CreateToolShapeAttributes(
        ppVal: *mut *mut IVGToolShapeAttributes,
    ) -> HRESULT,
    fn get_UILanguageCode(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn StartTemporaryToolState(
        StateGuid: BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580025, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDocuments(IVGDocumentsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580048, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPage(IVGPageVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGPages,
    ) -> HRESULT,
    fn get_Layers(
        ppVal: *mut *mut IVGLayers,
    ) -> HRESULT,
    fn get_Shapes(
        ppVal: *mut *mut IVGShapes,
    ) -> HRESULT,
    fn get_ActiveLayer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_Paper(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_SizeWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_SizeWidth(
        pVal: f64,
    ) -> HRESULT,
    fn get_SizeHeight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_SizeHeight(
        pVal: f64,
    ) -> HRESULT,
    fn get_Resolution(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Resolution(
        pVal: i32,
    ) -> HRESULT,
    fn get_Bleed(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Bleed(
        pVal: f64,
    ) -> HRESULT,
    fn get_Orientation(
        pVal: *mut cdrPageOrientation,
    ) -> HRESULT,
    fn put_Orientation(
        pVal: cdrPageOrientation,
    ) -> HRESULT,
    fn get_Index(
        pIndex: *mut i32,
    ) -> HRESULT,
    fn Activate(
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn CreateLayer(
        LayerName: BSTR,
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn TextFind(
        Text: BSTR,
        CaseSensitive: VARIANT_BOOL,
        ppFound: *mut *mut IVGShape,
    ) -> HRESULT,
    fn TextReplace(
        OldText: BSTR,
        NewText: BSTR,
        CaseSensitive: VARIANT_BOOL,
        ReplaceSelectedOnly: VARIANT_BOOL,
    ) -> HRESULT,
    fn SelectShapesAtPoint(
        x: f64,
        y: f64,
        SelectUnfilled: VARIANT_BOOL,
        HotArea: f64,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn SelectShapesFromRectangle(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        Touch: VARIANT_BOOL,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Background(
        pVal: *mut cdrPageBackground,
    ) -> HRESULT,
    fn put_Background(
        pVal: cdrPageBackground,
    ) -> HRESULT,
    fn get_Color(
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_Color(
        pVal: *const IVGColor,
    ) -> HRESULT,
    fn get_PrintExportBackground(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PrintExportBackground(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Guides(
        Type: cdrGuideType,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn FindShape(
        Name: BSTR,
        Type: cdrShapeType,
        StaticID: i32,
        Recursive: VARIANT_BOOL,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn FindShapes(
        Name: BSTR,
        Type: cdrShapeType,
        Recursive: VARIANT_BOOL,
        ppRet: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn MoveTo(
        Index: i32,
    ) -> HRESULT,
    fn UnlockAllShapes(
    ) -> HRESULT,
    fn get_Properties(
        ppVal: *mut *mut IVGProperties,
    ) -> HRESULT,
    fn GetSize(
        Width: *mut f64,
        Height: *mut f64,
    ) -> HRESULT,
    fn SetSize(
        Width: f64,
        Height: f64,
    ) -> HRESULT,
    fn get_CenterX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_CenterY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn CustomCommand(
        ComponentID: BSTR,
        CommandID: BSTR,
        Parameters: *const SAFEARRAY,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_Previous(
        ppVal: *mut *mut IVGPage,
    ) -> HRESULT,
    fn get_Next(
        ppVal: *mut *mut IVGPage,
    ) -> HRESULT,
    fn get_SelectableShapes(
        ppVal: *mut *mut IVGShapes,
    ) -> HRESULT,
    fn get_TreeNode(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn GetCenterPosition(
        CenterX: *mut f64,
        CenterY: *mut f64,
    ) -> HRESULT,
    fn SelectSize(
        PageSizeName: BSTR,
        Landscape: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_GuidesLayer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_DesktopLayer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_GridLayer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn GetBoundingBox(
        x: *mut f64,
        y: *mut f64,
        Width: *mut f64,
        Height: *mut f64,
    ) -> HRESULT,
    fn get_LeftX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_RightX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_BottomY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_TopY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_AllLayers(
        ppVal: *mut *mut IVGLayers,
    ) -> HRESULT,
    fn get_BoundingBox(
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn get_Spread(
        ppVal: *mut *mut IVGSpread,
    ) -> HRESULT,
    fn FindClosestSnapPoint(
        TypeSet: cdrPointType,
        PositionX: f64,
        PositionY: f64,
        HotArea: f64,
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn GetObjectsBoundingBox(
        SelectedOnly: VARIANT_BOOL,
        IncludeNonPrintable: VARIANT_BOOL,
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn FindShapeAtPoint(
        x: f64,
        y: f64,
        TreatAsFilled: VARIANT_BOOL,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580049, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPages(IVGPagesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGPage,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGPage,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGPage,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580041, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGLayers(IVGLayersVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGPage,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Find(
        LayerName: BSTR,
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_Top(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_Bottom(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580040, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGLayer(IVGLayerVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGLayers,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Shapes(
        ppVal: *mut *mut IVGShapes,
    ) -> HRESULT,
    fn Activate(
    ) -> HRESULT,
    fn get_Visible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Visible(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Printable(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Printable(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Editable(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Editable(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Master(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Master(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_OverrideColor(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_OverrideColor(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Color(
        pVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_Color(
        pVal: *const IVGColor,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn MoveAbove(
        Layer: *const IVGLayer,
    ) -> HRESULT,
    fn MoveBelow(
        Layer: *const IVGLayer,
    ) -> HRESULT,
    fn Import(
        FileName: BSTR,
        Filter: cdrFilter,
        Options: *const IVGStructImportOptions,
    ) -> HRESULT,
    fn CreateRectangle(
        Left: f64,
        Top: f64,
        Right: f64,
        Bottom: f64,
        CornerUL: i32,
        CornerUR: i32,
        CornerLR: i32,
        CornerLL: i32,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateEllipse(
        Left: f64,
        Top: f64,
        Right: f64,
        Bottom: f64,
        StartAngle: f64,
        EndAngle: f64,
        Pie: VARIANT_BOOL,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreatePolygon(
        Left: f64,
        Top: f64,
        Right: f64,
        Bottom: f64,
        Sides: i32,
        SubPaths: i32,
        Complexity: i32,
        Star: VARIANT_BOOL,
        StarComplexity: i32,
        MaxComplexity: i32,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateGridBoxes(
        Left: f64,
        Top: f64,
        Right: f64,
        Bottom: f64,
        Wide: i32,
        High: i32,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateSpiral(
        Left: f64,
        Top: f64,
        Right: f64,
        Bottom: f64,
        NumRevolutions: i32,
        SpiralType: cdrSpiralType,
        GrowthRate: i32,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateArtisticText(
        Left: f64,
        Bottom: f64,
        Text: BSTR,
        LanguageID: cdrTextLanguage,
        CharSet: cdrTextCharSet,
        Font: BSTR,
        Size: f32,
        Bold: cdrTriState,
        Italic: cdrTriState,
        Underline: cdrFontLine,
        Alignment: cdrAlignment,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateParagraphText(
        Left: f64,
        Top: f64,
        Right: f64,
        Bottom: f64,
        Text: BSTR,
        LanguageID: cdrTextLanguage,
        CharSet: cdrTextCharSet,
        Font: BSTR,
        Size: f32,
        Bold: cdrTriState,
        Italic: cdrTriState,
        Underline: cdrFontLine,
        Alignment: cdrAlignment,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateCurveSegment(
        StartX: f64,
        StartY: f64,
        EndX: f64,
        EndY: f64,
        StartingControlPointLength: f64,
        StartingControlPointAngle: f64,
        EndingControlPointLength: f64,
        EndingControlPointAngle: f64,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateLineSegment(
        StartX: f64,
        StartY: f64,
        EndX: f64,
        EndY: f64,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateConnector(
        Start: *const IVGSnapPoint,
        End: *const IVGSnapPoint,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateCurve(
        Source: *const IVGCurve,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Paste(
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateGuideAngle(
        x: f64,
        y: f64,
        Angle: f64,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateGuide(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateEllipse2(
        CenterX: f64,
        CenterY: f64,
        Radius1: f64,
        Radius2: f64,
        StartAngle: f64,
        EndAngle: f64,
        Pie: VARIANT_BOOL,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn FindShape(
        Name: BSTR,
        Type: cdrShapeType,
        StaticID: i32,
        Recursive: VARIANT_BOOL,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn FindShapes(
        Name: BSTR,
        Type: cdrShapeType,
        Recursive: VARIANT_BOOL,
        ppRet: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn CreateRectangle2(
        x: f64,
        y: f64,
        Width: f64,
        Height: f64,
        RadiusUL: f64,
        RadiusUR: f64,
        RadiusLR: f64,
        RadiusLL: f64,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateFreeConnector(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Properties(
        ppVal: *mut *mut IVGProperties,
    ) -> HRESULT,
    fn get_MasterProperties(
        ppVal: *mut *mut IVGProperties,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn CreateCurveSegment2(
        x1: f64,
        y1: f64,
        StartingControlPointX: f64,
        StartingControlPointY: f64,
        EndingControlPointX: f64,
        EndingControlPointY: f64,
        x2: f64,
        y2: f64,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn ImportEx(
        FileName: BSTR,
        Filter: cdrFilter,
        Options: *const IVGStructImportOptions,
        ppRet: *mut *mut ICorelImportFilter,
    ) -> HRESULT,
    fn CreateArtisticTextWide(
        Left: f64,
        Bottom: f64,
        Text: BSTR,
        LanguageID: cdrTextLanguage,
        CharSet: cdrTextCharSet,
        Font: BSTR,
        Size: f32,
        Bold: cdrTriState,
        Italic: cdrTriState,
        Underline: cdrFontLine,
        Alignment: cdrAlignment,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateParagraphTextWide(
        Left: f64,
        Top: f64,
        Right: f64,
        Bottom: f64,
        Text: BSTR,
        LanguageID: cdrTextLanguage,
        CharSet: cdrTextCharSet,
        Font: BSTR,
        Size: f32,
        Bold: cdrTriState,
        Italic: cdrTriState,
        Underline: cdrFontLine,
        Alignment: cdrAlignment,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CustomCommand(
        ComponentID: BSTR,
        CommandID: BSTR,
        Parameters: *const SAFEARRAY,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn CreateCustomShape(
        TypeID: BSTR,
        Parameters: *const SAFEARRAY,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateLinearDimension(
        Type: cdrLinearDimensionType,
        Point1: *const IVGSnapPoint,
        Point2: *const IVGSnapPoint,
        TextCentered: VARIANT_BOOL,
        TextX: f64,
        TextY: f64,
        Style: cdrDimensionStyle,
        Precision: i32,
        ShowUnits: VARIANT_BOOL,
        Units: cdrDimensionLinearUnits,
        Placement: cdrDimensionPlacement,
        HorizontalText: VARIANT_BOOL,
        BoxedText: VARIANT_BOOL,
        LeadingZero: VARIANT_BOOL,
        Prefix: BSTR,
        Suffix: BSTR,
        OutlineWidth: f64,
        Arrows: *const IVGArrowHead,
        OutlineColor: *const IVGColor,
        TextFont: BSTR,
        TextSize: f64,
        TextColor: *const IVGColor,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateAngularDimension(
        Center: *const IVGSnapPoint,
        Point1: *const IVGSnapPoint,
        Point2: *const IVGSnapPoint,
        TextX: f64,
        TextY: f64,
        Precision: i32,
        ShowUnits: VARIANT_BOOL,
        Units: cdrDimensionAngularUnits,
        BoxedText: VARIANT_BOOL,
        LeadingZero: VARIANT_BOOL,
        Prefix: BSTR,
        Suffix: BSTR,
        OutlineWidth: f64,
        Arrows: *const IVGArrowHead,
        OutlineColor: *const IVGColor,
        TextFont: BSTR,
        TextSize: f64,
        TextColor: *const IVGColor,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateSymbol(
        x: f64,
        y: f64,
        SymbolName: BSTR,
        Library: *const IVGSymbolLibrary,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreatePolygon2(
        CenterX: f64,
        CenterY: f64,
        Radius: f64,
        Sides: i32,
        Angle: f64,
        InnerRadius: f64,
        Star: VARIANT_BOOL,
        Sharpness: i32,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn PasteSpecial(
        FormatName: BSTR,
        PasteLink: VARIANT_BOOL,
        DisplayAsIcon: VARIANT_BOOL,
        Caption: BSTR,
        Icon: VARIANT,
        pRet: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn CreateOLEObject(
        ObjectID: BSTR,
        DisplayAsIcon: VARIANT_BOOL,
        Caption: BSTR,
        Icon: VARIANT,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateOLEObjectFromFile(
        FileName: BSTR,
        Link: VARIANT_BOOL,
        DisplayAsIcon: VARIANT_BOOL,
        Caption: BSTR,
        Icon: VARIANT,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_SelectableShapes(
        ppVal: *mut *mut IVGShapes,
    ) -> HRESULT,
    fn get_TreeNode(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn get_IsGuidesLayer(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsDesktopLayer(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsGridLayer(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSpecialLayer(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MasterLayer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_AbsoluteIndex(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Page(
        ppVal: *mut *mut IVGPage,
    ) -> HRESULT,
    fn get_Above(
        IgnoreMasters: VARIANT_BOOL,
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_Below(
        IgnoreMasters: VARIANT_BOOL,
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn CreateRectangleRect(
        Rect: *const IVGRect,
        RadiusUL: f64,
        RadiusUR: f64,
        RadiusLR: f64,
        RadiusLL: f64,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateEllipseRect(
        Rect: *const IVGRect,
        StartAngle: f64,
        EndAngle: f64,
        Pie: VARIANT_BOOL,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateConnectorEx(
        Type: cdrConnectorType,
        Start: *const IVGSnapPoint,
        End: *const IVGSnapPoint,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateRightAngleConnector(
        Start: *const IVGSnapPoint,
        End: *const IVGSnapPoint,
        CornerRadius: f64,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateBSpline(
        Source: *const IVGBSpline,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn PasteEx(
        Options: *const IVGStructPasteOptions,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn CreateToolShape(
        ToolShapeGuid: BSTR,
        ShapeProperties: *const IVGProperties,
        pVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateBitmap(
        Left: f64,
        Top: f64,
        Right: f64,
        Bottom: f64,
        Image: *const IVGImage,
        ImageAlpha: *const IVGImage,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateBitmap2(
        x: f64,
        y: f64,
        Width: f64,
        Height: f64,
        Image: *const IVGImage,
        ImageAlpha: *const IVGImage,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CreateBitmapRect(
        Rect: *const IVGRect,
        Image: *const IVGImage,
        ImageAlpha: *const IVGImage,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058005f, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGShapes(IVGShapesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Range(
        IndexArray: *const SAFEARRAY,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn All(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn AllExcluding(
        IndexArray: *const SAFEARRAY,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn FindShape(
        Name: BSTR,
        Type: cdrShapeType,
        StaticID: i32,
        Recursive: VARIANT_BOOL,
        Query: BSTR,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn FindShapes(
        Name: BSTR,
        Type: cdrShapeType,
        Recursive: VARIANT_BOOL,
        Query: BSTR,
        ppRet: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058005d, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGShape(IVGShapeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_StaticID(
        pVal: *mut i32,
    ) -> HRESULT,
    fn ConvertToCurves(
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Shapes(
        ppVal: *mut *mut IVGShapes,
    ) -> HRESULT,
    fn get_Rectangle(
        ppVal: *mut *mut IVGRectangle,
    ) -> HRESULT,
    fn get_PositionX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PositionX(
        pVal: f64,
    ) -> HRESULT,
    fn get_PositionY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PositionY(
        pVal: f64,
    ) -> HRESULT,
    fn get_SizeWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_SizeWidth(
        pVal: f64,
    ) -> HRESULT,
    fn get_SizeHeight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_SizeHeight(
        pVal: f64,
    ) -> HRESULT,
    fn get_Ellipse(
        ppVal: *mut *mut IVGEllipse,
    ) -> HRESULT,
    fn get_Polygon(
        ppVal: *mut *mut IVGPolygon,
    ) -> HRESULT,
    fn get_Curve(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_Bitmap(
        ppVal: *mut *mut IVGBitmap,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrShapeType,
    ) -> HRESULT,
    fn get_Outline(
        ppVal: *mut *mut IVGOutline,
    ) -> HRESULT,
    fn get_Fill(
        ppVal: *mut *mut IVGFill,
    ) -> HRESULT,
    fn get_Text(
        ppVal: *mut *mut IVGText,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn Duplicate(
        OffsetX: f64,
        OffsetY: f64,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Skew(
        AngleX: f64,
        AngleY: f64,
    ) -> HRESULT,
    fn Move(
        DeltaX: f64,
        DeltaY: f64,
    ) -> HRESULT,
    fn get_RotationAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RotationAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_RotationCenterX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RotationCenterX(
        pVal: f64,
    ) -> HRESULT,
    fn get_RotationCenterY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RotationCenterY(
        pVal: f64,
    ) -> HRESULT,
    fn Rotate(
        Angle: f64,
    ) -> HRESULT,
    fn ConvertToBitmap(
        BitDepth: i32,
        Grayscale: VARIANT_BOOL,
        Dithered: VARIANT_BOOL,
        TransparentBG: VARIANT_BOOL,
        Resolution: i32,
        AntiAliasing: cdrAntiAliasingType,
        UseColorProfile: VARIANT_BOOL,
        MultiChannel: VARIANT_BOOL,
        AlwaysOverprintBlack: VARIANT_BOOL,
        OverprintBlackLimit: i32,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Group(
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Ungroup(
    ) -> HRESULT,
    fn UngroupAll(
    ) -> HRESULT,
    fn OrderToFront(
    ) -> HRESULT,
    fn OrderToBack(
    ) -> HRESULT,
    fn OrderForwardOne(
    ) -> HRESULT,
    fn OrderBackOne(
    ) -> HRESULT,
    fn OrderFrontOf(
        Shape: *const IVGShape,
    ) -> HRESULT,
    fn OrderBackOf(
        Shape: *const IVGShape,
    ) -> HRESULT,
    fn OrderIsInFrontOf(
        Shape: *const IVGShape,
        pbResult: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn AddToSelection(
    ) -> HRESULT,
    fn RemoveFromSelection(
    ) -> HRESULT,
    fn Separate(
    ) -> HRESULT,
    fn get_Layer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn put_Layer(
        ppVal: *const IVGLayer,
    ) -> HRESULT,
    fn get_SnapPoints(
        ppVal: *mut *mut IVGSnapPoints,
    ) -> HRESULT,
    fn get_Connector(
        ppVal: *mut *mut IVGConnector,
    ) -> HRESULT,
    fn IsOnShape(
        x: f64,
        y: f64,
        HotArea: f64,
        pVal: *mut cdrPositionOfPointOverShape,
    ) -> HRESULT,
    fn CreateArrowHead(
        ppVal: *mut *mut IVGArrowHead,
    ) -> HRESULT,
    fn Copy(
    ) -> HRESULT,
    fn Cut(
    ) -> HRESULT,
    fn Clone(
        OffsetX: f64,
        OffsetY: f64,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Stretch(
        StretchX: f64,
        StretchY: f64,
        StretchCharactersSize: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetPosition(
        PositionX: f64,
        PositionY: f64,
    ) -> HRESULT,
    fn SetSize(
        Width: f64,
        Height: f64,
    ) -> HRESULT,
    fn GetPosition(
        PositionX: *mut f64,
        PositionY: *mut f64,
    ) -> HRESULT,
    fn GetSize(
        Width: *mut f64,
        Height: *mut f64,
    ) -> HRESULT,
    fn get_Properties(
        ppVal: *mut *mut IVGProperties,
    ) -> HRESULT,
    fn OrderReverse(
    ) -> HRESULT,
    fn Combine(
        ppRetVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn BreakApart(
    ) -> HRESULT,
    fn put_Fill(
        ppVal: *const IVGFill,
    ) -> HRESULT,
    fn Weld(
        TargetShape: *const IVGShape,
        LeaveSource: VARIANT_BOOL,
        LeaveTarget: VARIANT_BOOL,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Trim(
        TargetShape: *const IVGShape,
        LeaveSource: VARIANT_BOOL,
        LeaveTarget: VARIANT_BOOL,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Intersect(
        TargetShape: *const IVGShape,
        LeaveSource: VARIANT_BOOL,
        LeaveTarget: VARIANT_BOOL,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Effects(
        ppVal: *mut *mut IVGEffects,
    ) -> HRESULT,
    fn get_Effect(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn CreateDropShadow(
        Type: cdrDropShadowType,
        Opacity: i32,
        Feather: i32,
        OffsetX: f64,
        OffsetY: f64,
        Color: *const IVGColor,
        FeatherType: cdrFeatherType,
        FeatherEdge: cdrEdgeType,
        PerspectiveAngle: f64,
        PerspectiveStretch: f64,
        Fade: i32,
        MergeMode: cdrMergeMode,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn CreateBlend(
        Shape: *const IVGShape,
        Steps: INT,
        ColorBlendType: cdrFountainFillBlendType,
        Mode: cdrBlendMode,
        Spacing: f64,
        Angle: f64,
        Loop: VARIANT_BOOL,
        Path: *const IVGShape,
        RotateShapes: VARIANT_BOOL,
        SpacingAccel: i32,
        ColorAccel: i32,
        AccelSize: VARIANT_BOOL,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn CreateExtrude(
        Type: cdrExtrudeType,
        VPType: cdrExtrudeVPType,
        VPX: f64,
        VPY: f64,
        Depth: f64,
        Shading: cdrExtrudeShading,
        BaseColor: *const IVGColor,
        ShadingColor: *const IVGColor,
        BevelDepth: f64,
        BevelAngle: f64,
        BevelColor: *const IVGColor,
        BevelOnly: VARIANT_BOOL,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn CreateEnvelope(
        PresetIndex: i32,
        Mode: cdrEnvelopeMode,
        KeepLines: VARIANT_BOOL,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn Flip(
        Axes: cdrFlipAxes,
    ) -> HRESULT,
    fn get_Locked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Locked(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_OriginalWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_OriginalHeight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Selected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Selected(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn CreateLens(
        Type: cdrLensType,
        RateOrMagnification: f64,
        Color1: *const IVGColor,
        Color2: *const IVGColor,
        ColorMapPalette: cdrFountainFillBlendType,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn CreatePerspective(
        HorizVanishPointX: VARIANT,
        HorizVanishPointY: VARIANT,
        VertVanishPointX: VARIANT,
        VertVanishPointY: VARIANT,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn CreateContour(
        Direction: cdrContourDirection,
        Offset: f64,
        Steps: i32,
        BlendType: cdrFountainFillBlendType,
        OutlineColor: *const IVGColor,
        FillColor: *const IVGColor,
        FillColor2: *const IVGColor,
        SpacingAccel: i32,
        ColorAccel: i32,
        EndCapType: cdrContourEndCapType,
        CornerType: cdrContourCornerType,
        MiterLimit: f64,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn CreatePushPullDistortion(
        OriginX: f64,
        OriginY: f64,
        Amplitude: i32,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn CreateZipperDistortion(
        OriginX: f64,
        OriginY: f64,
        Amplitude: i32,
        Frequency: i32,
        Random: VARIANT_BOOL,
        Smooth: VARIANT_BOOL,
        Local: VARIANT_BOOL,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn CreateTwisterDistortion(
        OriginX: f64,
        OriginY: f64,
        Angle: f64,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_Guide(
        ppVal: *mut *mut IVGGuide,
    ) -> HRESULT,
    fn AddToPowerClip(
        Shape: *const IVGShape,
        CenterInContainer: cdrTriState,
    ) -> HRESULT,
    fn RemoveFromContainer(
        Level: i32,
    ) -> HRESULT,
    fn get_PowerClip(
        ppPowerClip: *mut *mut IVGPowerClip,
    ) -> HRESULT,
    fn get_PowerClipParent(
        ppShape: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_DrapeFill(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DrapeFill(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_OverprintFill(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_OverprintFill(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_OverprintOutline(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_OverprintOutline(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_URL(
        ppVal: *mut *mut IVGURL,
    ) -> HRESULT,
    fn get_ObjectData(
        ppDataItems: *mut *mut IVGDataItems,
    ) -> HRESULT,
    fn get_CloneLink(
        ppCloneLink: *mut *mut IVGCloneLink,
    ) -> HRESULT,
    fn get_Clones(
        ppShapeRange: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn get_AbsoluteHScale(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_AbsoluteVScale(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_AbsoluteSkew(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Transparency(
        ppTransparency: *mut *mut IVGTransparency,
    ) -> HRESULT,
    fn GetMatrix(
        d11: *mut f64,
        d12: *mut f64,
        d21: *mut f64,
        d22: *mut f64,
        tx: *mut f64,
        ty: *mut f64,
    ) -> HRESULT,
    fn SetMatrix(
        d11: f64,
        d12: f64,
        d21: f64,
        d22: f64,
        tx: f64,
        ty: f64,
    ) -> HRESULT,
    fn ConvertToBitmapEx(
        Mode: cdrImageType,
        Dithered: VARIANT_BOOL,
        Transparent: VARIANT_BOOL,
        Resolution: i32,
        AntiAliasing: cdrAntiAliasingType,
        UseColorProfile: VARIANT_BOOL,
        AlwaysOverprintBlack: VARIANT_BOOL,
        OverprintBlackLimit: i32,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn SkewEx(
        AngleX: f64,
        AngleY: f64,
        CenterX: f64,
        CenterY: f64,
    ) -> HRESULT,
    fn RotateEx(
        Angle: f64,
        CenterX: f64,
        CenterY: f64,
    ) -> HRESULT,
    fn get_ParentGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn SetBoundingBox(
        x: f64,
        y: f64,
        Width: f64,
        Height: f64,
        KeepAspect: VARIANT_BOOL,
        ReferencePoint: cdrReferencePoint,
    ) -> HRESULT,
    fn CreateSelection(
    ) -> HRESULT,
    fn SetRotationCenter(
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn ClearEffect(
        Type: cdrEffectType,
    ) -> HRESULT,
    fn get_Next(
        Level: cdrShapeLevel,
        EnterGroups: VARIANT_BOOL,
        Loop: VARIANT_BOOL,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Previous(
        Level: cdrShapeLevel,
        EnterGroups: VARIANT_BOOL,
        Loop: VARIANT_BOOL,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn StretchEx(
        CenterX: f64,
        CenterY: f64,
        StretchX: f64,
        StretchY: f64,
        StretchCharactersSize: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetSizeEx(
        CenterX: f64,
        CenterY: f64,
        Width: f64,
        Height: f64,
    ) -> HRESULT,
    fn GetBoundingBox(
        x: *mut f64,
        y: *mut f64,
        Width: *mut f64,
        Height: *mut f64,
        UseOutline: VARIANT_BOOL,
    ) -> HRESULT,
    fn UngroupEx(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn UngroupAllEx(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn BreakApartEx(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn ApplyStyle(
        StyleName: BSTR,
    ) -> HRESULT,
    fn get_WrapText(
        pVal: *mut cdrWrapStyle,
    ) -> HRESULT,
    fn put_WrapText(
        pVal: cdrWrapStyle,
    ) -> HRESULT,
    fn get_TextWrapOffset(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TextWrapOffset(
        pVal: f64,
    ) -> HRESULT,
    fn PlaceTextInside(
        TextShape: *const IVGShape,
        pVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_DisplayCurve(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn CustomCommand(
        ComponentID: BSTR,
        CommandID: BSTR,
        Parameters: *const SAFEARRAY,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_Custom(
        ppVal: *mut *mut IVGCustomShape,
    ) -> HRESULT,
    fn CreateCustomEffect(
        EffectID: BSTR,
        Parameters: *const SAFEARRAY,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn CreateCustomDistortion(
        DistortionID: BSTR,
        Parameters: *const SAFEARRAY,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn AlignToShape(
        Type: cdrAlignType,
        Shape: *const IVGShape,
        TextAlignOrigin: cdrTextAlignOrigin,
    ) -> HRESULT,
    fn AlignToShapeRange(
        Type: cdrAlignType,
        ShapeRange: *const IVGShapeRange,
        TextAlignOrigin: cdrTextAlignOrigin,
    ) -> HRESULT,
    fn AlignToPage(
        Type: cdrAlignType,
        TextAlignOrigin: cdrTextAlignOrigin,
    ) -> HRESULT,
    fn AlignToPageCenter(
        Type: cdrAlignType,
        TextAlignOrigin: cdrTextAlignOrigin,
    ) -> HRESULT,
    fn AlignToGrid(
        Type: cdrAlignType,
        TextAlignOrigin: cdrTextAlignOrigin,
    ) -> HRESULT,
    fn AlignToPoint(
        Type: cdrAlignType,
        x: f64,
        y: f64,
        TextAlignOrigin: cdrTextAlignOrigin,
    ) -> HRESULT,
    fn get_Dimension(
        ppVal: *mut *mut IVGDimension,
    ) -> HRESULT,
    fn get_Symbol(
        ppVal: *mut *mut IVGSymbol,
    ) -> HRESULT,
    fn ConvertToSymbol(
        Name: BSTR,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_OLE(
        ppVal: *mut *mut IVGOLE,
    ) -> HRESULT,
    fn DuplicateAsRange(
        OffsetX: f64,
        OffsetY: f64,
        ppRet: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn CloneAsRange(
        OffsetX: f64,
        OffsetY: f64,
        ppRet: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn MoveToLayer(
        Layer: *const IVGLayer,
    ) -> HRESULT,
    fn CopyToLayer(
        Layer: *const IVGLayer,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn CopyToLayerAsRange(
        Layer: *const IVGLayer,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn ClearTransformations(
    ) -> HRESULT,
    fn Distribute(
        Type: cdrDistributeType,
        PageExtent: VARIANT_BOOL,
    ) -> HRESULT,
    fn CompareTo(
        Shape: *const IVGShape,
        CompareType: cdrCompareType,
        Condition: cdrCompareCondition,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Selectable(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ApplyEffectInvert(
    ) -> HRESULT,
    fn ApplyEffectPosterize(
        Level: i32,
    ) -> HRESULT,
    fn ApplyEffectBCI(
        Brighness: i32,
        Contrast: i32,
        Intensity: i32,
    ) -> HRESULT,
    fn ApplyEffectColorBalance(
        CyanRed: i32,
        MagentaGreen: i32,
        YellowBlue: i32,
        ApplyToShadows: VARIANT_BOOL,
        ApplyToMidtones: VARIANT_BOOL,
        ApplyToHighlights: VARIANT_BOOL,
        PreserveLuminance: VARIANT_BOOL,
    ) -> HRESULT,
    fn ApplyEffectGamma(
        Gamma: f64,
    ) -> HRESULT,
    fn ApplyEffectHSL(
        Hue: VARIANT,
        Saturation: VARIANT,
        Lightness: VARIANT,
    ) -> HRESULT,
    fn TransformMatrix(
        d11: f64,
        d12: f64,
        d21: f64,
        d22: f64,
        tx: f64,
        ty: f64,
    ) -> HRESULT,
    fn AffineTransform(
        d11: f64,
        d12: f64,
        d21: f64,
        d22: f64,
        CenterX: f64,
        CenterY: f64,
    ) -> HRESULT,
    fn get_TreeNode(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn ReplaceWith(
        VirtualShape: *const IVGShape,
    ) -> HRESULT,
    fn get_Virtual(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanHaveFill(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanHaveOutline(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSimpleShape(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Fillet(
        Radius: f64,
        CombineSmoothSegments: VARIANT_BOOL,
    ) -> HRESULT,
    fn Chamfer(
        DistanceA: f64,
        DistanceB: f64,
        CombineSmoothSegments: VARIANT_BOOL,
    ) -> HRESULT,
    fn Scallop(
        Radius: f64,
        CombineSmoothSegments: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FillMode(
        pVal: *mut cdrFillMode,
    ) -> HRESULT,
    fn put_FillMode(
        pVal: cdrFillMode,
    ) -> HRESULT,
    fn get_LeftX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_RightX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_TopY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_BottomY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn StepAndRepeat(
        NumCopies: i32,
        DistanceX: f64,
        DistanceY: f64,
        ModeX: cdrDistanceMode,
        DirectionX: cdrDirection,
        ModeY: cdrDistanceMode,
        DirectionY: cdrDirection,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn get_OverprintBitmap(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_OverprintBitmap(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn IsTypeAnyOf(
        TypeList: *const SAFEARRAY,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetLinkedShapes(
        LinkType: cdrShapeLinkType,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn CreateEnvelopeFromShape(
        Source: *const IVGShape,
        Mode: cdrEnvelopeMode,
        KeepLines: VARIANT_BOOL,
        CopyMode: cdrEnvelopeCopyMode,
        CornerIndices: VARIANT,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn CreateEnvelopeFromCurve(
        Source: *const IVGCurve,
        Mode: cdrEnvelopeMode,
        KeepLines: VARIANT_BOOL,
        CopyMode: cdrEnvelopeCopyMode,
        CornerIndices: VARIANT,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_EPS(
        ppVal: *mut *mut IVGEPS,
    ) -> HRESULT,
    fn Evaluate(
        Expression: BSTR,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_BoundingBox(
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn GetPositionEx(
        ReferencePoint: cdrReferencePoint,
        x: *mut f64,
        y: *mut f64,
    ) -> HRESULT,
    fn SetPositionEx(
        ReferencePoint: cdrReferencePoint,
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn get_CenterX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_CenterX(
        pVal: f64,
    ) -> HRESULT,
    fn get_CenterY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_CenterY(
        pVal: f64,
    ) -> HRESULT,
    fn put_LeftX(
        pVal: f64,
    ) -> HRESULT,
    fn put_RightX(
        pVal: f64,
    ) -> HRESULT,
    fn put_TopY(
        pVal: f64,
    ) -> HRESULT,
    fn put_BottomY(
        pVal: f64,
    ) -> HRESULT,
    fn get_ZOrder(
        pVal: *mut i32,
    ) -> HRESULT,
    fn CompareToEx(
        Shape2: *const IVGShape,
        Condition: BSTR,
        Data: VARIANT,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn CopyPropertiesFrom(
        Source: *const IVGShape,
        Properties: cdrCopyProperties,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetOverprintFillState(
        pVal: *mut cdrOverprintState,
    ) -> HRESULT,
    fn GetOverprintOutlineState(
        pVal: *mut cdrOverprintState,
    ) -> HRESULT,
    fn get_Page(
        ppVal: *mut *mut IVGPage,
    ) -> HRESULT,
    fn SnapPointsOfType(
        TypeSet: cdrPointType,
        ppVal: *mut *mut IVGSnapPoints,
    ) -> HRESULT,
    fn FindSnapPoint(
        ReferenceData: BSTR,
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn get_Spread(
        ppVal: *mut *mut IVGSpread,
    ) -> HRESULT,
    fn get_PixelAlignedRendering(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PixelAlignedRendering(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BSpline(
        ppVal: *mut *mut IVGBSpline,
    ) -> HRESULT,
    fn CreateDocumentFrom(
        TemporaryDocument: VARIANT_BOOL,
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn AlignAndDistribute(
        MethodH: cdrAlignDistributeH,
        MethodV: cdrAlignDistributeV,
        AlignTo: cdrAlignShapesTo,
        DistributeArea: cdrDistributeArea,
        UseOutline: VARIANT_BOOL,
        TextAlignOrigin: cdrTextAlignOrigin,
        PointX: f64,
        PointY: f64,
        DistributeRect: *const IVGRect,
    ) -> HRESULT,
    fn get_Style(
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
    fn CreateBoundary(
        x: f64,
        y: f64,
        PlaceOnTop: VARIANT_BOOL,
        DeleteSource: VARIANT_BOOL,
        pVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn EqualDivide(
        Divisions: i32,
        Gap: f64,
        Group: VARIANT_BOOL,
        Combine: VARIANT_BOOL,
        DeleteSource: VARIANT_BOOL,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn Project(
        Plane: cdrProjectPlane,
        ReferencePoint: cdrReferencePoint,
        ApplyToDuplicate: VARIANT_BOOL,
        ppRetVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Unproject(
        Plane: cdrProjectPlane,
        ReferencePoint: cdrReferencePoint,
        ApplyToDuplicate: VARIANT_BOOL,
        ppRetVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_TransformationMatrix(
        TransformMatrix: *mut *mut IVGTransformMatrix,
    ) -> HRESULT,
    fn put_TransformationMatrix(
        TransformMatrix: *const IVGTransformMatrix,
    ) -> HRESULT,
    fn ApplyTransformMatrix(
        TransformMatrix: *const IVGTransformMatrix,
    ) -> HRESULT,
    fn get_Visible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Visible(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn ModifyToolShapeProperties(
        ShapePropertiesToModify: *const IVGProperties,
    ) -> HRESULT,
    fn GetToolShapeGuid(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn CreateParallelCurves(
        Count: i32,
        distanceBetweenCurves: f64,
        ppParallels: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn FindShapeAtPoint(
        x: f64,
        y: f64,
        TreatAsFilled: VARIANT_BOOL,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn GetColorTypes(
        ppVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetColors(
        MaxBitmapColors: i32,
        ppVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn FlattenEffects(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580057, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGRectangle(IVGRectangleVtbl): IDispatch(IDispatchVtbl) {
    fn get_CornerUpperLeft(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CornerUpperLeft(
        pVal: i32,
    ) -> HRESULT,
    fn get_CornerUpperRight(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CornerUpperRight(
        pVal: i32,
    ) -> HRESULT,
    fn get_CornerLowerLeft(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CornerLowerLeft(
        pVal: i32,
    ) -> HRESULT,
    fn get_CornerLowerRight(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CornerLowerRight(
        pVal: i32,
    ) -> HRESULT,
    fn get_EqualCorners(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MaxRadius(
        pVal: *mut f64,
    ) -> HRESULT,
    fn SetRoundness(
        Roundness: i32,
    ) -> HRESULT,
    fn SetRadius(
        Radius: f64,
    ) -> HRESULT,
    fn get_RadiusUpperLeft(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RadiusUpperLeft(
        pVal: f64,
    ) -> HRESULT,
    fn get_RadiusUpperRight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RadiusUpperRight(
        pVal: f64,
    ) -> HRESULT,
    fn get_RadiusLowerLeft(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RadiusLowerLeft(
        pVal: f64,
    ) -> HRESULT,
    fn get_RadiusLowerRight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RadiusLowerRight(
        pVal: f64,
    ) -> HRESULT,
    fn get_CornerType(
        pVal: *mut cdrCornerType,
    ) -> HRESULT,
    fn put_CornerType(
        pVal: cdrCornerType,
    ) -> HRESULT,
    fn get_RelativeCornerScaling(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RelativeCornerScaling(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580037, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEllipse(IVGEllipseVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrEllipseType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrEllipseType,
    ) -> HRESULT,
    fn get_StartAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_StartAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_EndAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_EndAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_Clockwise(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Clockwise(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CenterX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_CenterX(
        pVal: f64,
    ) -> HRESULT,
    fn get_CenterY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_CenterY(
        pVal: f64,
    ) -> HRESULT,
    fn get_HRadius(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_HRadius(
        pVal: f64,
    ) -> HRESULT,
    fn get_VRadius(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_VRadius(
        pVal: f64,
    ) -> HRESULT,
    fn SetCenterPosition(
        PositionX: f64,
        PositionY: f64,
    ) -> HRESULT,
    fn GetCenterPosition(
        PositionX: *mut f64,
        PositionY: *mut f64,
    ) -> HRESULT,
    fn SetRadius(
        HRadius: f64,
        VRadius: f64,
    ) -> HRESULT,
    fn GetRadius(
        HRadius: *mut f64,
        VRadius: *mut f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580051, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPolygon(IVGPolygonVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrPolygonType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrPolygonType,
    ) -> HRESULT,
    fn get_Sides(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Sides(
        pVal: i32,
    ) -> HRESULT,
    fn get_Sharpness(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Sharpness(
        pVal: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580019, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGCurve(IVGCurveVtbl): IDispatch(IDispatchVtbl) {
    fn get_Length(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_SubPaths(
        ppVal: *mut *mut IVGSubPaths,
    ) -> HRESULT,
    fn get_Nodes(
        ppVal: *mut *mut IVGNodes,
    ) -> HRESULT,
    fn get_Segments(
        ppVal: *mut *mut IVGSegments,
    ) -> HRESULT,
    fn get_Closed(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Closed(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn Selection(
        ppVal: *mut *mut IVGNodeRange,
    ) -> HRESULT,
    fn CreateSubPath(
        x: f64,
        y: f64,
        ppVal: *mut *mut IVGSubPath,
    ) -> HRESULT,
    fn ReverseDirection(
    ) -> HRESULT,
    fn IsOnCurve(
        x: f64,
        y: f64,
        HotArea: f64,
        pVal: *mut cdrPositionOfPointOverShape,
    ) -> HRESULT,
    fn BindToDocument(
        Document: *const IVGDocument,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn CopyAssign(
        Source: *const IVGCurve,
    ) -> HRESULT,
    fn CreateSubPathFromArray(
        Source: *const SAFEARRAY,
        Closed: VARIANT_BOOL,
        NumElements: i32,
        ppVal: *mut *mut IVGSubPath,
    ) -> HRESULT,
    fn AppendCurve(
        Source: *const IVGCurve,
    ) -> HRESULT,
    fn GetCurveInfo(
        ppVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn PutCurveInfo(
        Source: *const SAFEARRAY,
        NumElements: i32,
        pRet: *mut i32,
    ) -> HRESULT,
    fn ClearSelection(
    ) -> HRESULT,
    fn GetPolyline(
        CurvePrecision: i32,
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn RemoveOverlaps(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn Contour(
        Offset: f64,
        Direction: cdrContourDirection,
        EndCapType: cdrContourEndCapType,
        CornerType: cdrContourCornerType,
        MiterLimit: f64,
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn IsPointInside(
        x: f64,
        y: f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn IsRectOnEdge(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn FindClosestSegment(
        x: f64,
        y: f64,
        ParamOffset: *mut f64,
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn FindNodeAtPoint(
        x: f64,
        y: f64,
        HotArea: f64,
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn FindSegmentAtPoint(
        x: f64,
        y: f64,
        ParamOffset: *mut f64,
        HotArea: f64,
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn WeldWith(
        Curve: *const IVGCurve,
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_IsClockwise(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Area(
        pVal: *mut f64,
    ) -> HRESULT,
    fn GetBoundingBox(
        x: *mut f64,
        y: *mut f64,
        Width: *mut f64,
        Height: *mut f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BoundingBox(
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn IntersectsWith(
        Curve: *const IVGCurve,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn AppendSubpathFitToPoints(
        Points: *const IVGPointRange,
        UseCurrentViewForTolerance: VARIANT_BOOL,
        tolerance: f64,
    ) -> HRESULT,
    fn AppendSubpathFitToPointsAndCusps(
        Points: *const IVGPointRange,
        CuspIndexArray: *const SAFEARRAY,
        UseCurrentViewForTolerance: VARIANT_BOOL,
        tolerance: f64,
    ) -> HRESULT,
    fn ApplyTransformMatrix(
        TransformMatrix: *const IVGTransformMatrix,
    ) -> HRESULT,
    fn AppendSubpathFromPoints(
        Points: *const IVGPointRange,
        Close: VARIANT_BOOL,
    ) -> HRESULT,
    fn CreateCurveMappedToStroke(
        Stroke: *const IVGSubPath,
        ScaleY: f64,
        SelfWeld: VARIANT_BOOL,
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn CreateCurveMappedToStrokeAndReferenceLine(
        Stroke: *const IVGSubPath,
        Start: *const IVGPoint,
        End: *const IVGPoint,
        ScaleY: f64,
        SelfWeld: VARIANT_BOOL,
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn AutoReduceNodes(
        AmountToReduce0To100: f64,
        SelectedNodesOnly: VARIANT_BOOL,
    ) -> HRESULT,
    fn JoinTouchingSubpaths(
        AllowSubpathReversals: VARIANT_BOOL,
        tolerance: f64,
    ) -> HRESULT,
    fn AppendSubpathCircle(
        CenterX: f64,
        CenterY: f64,
        Radius: f64,
    ) -> HRESULT,
    fn AppendSubpathRectangle(
        Left: f64,
        Top: f64,
        Right: f64,
        Bottom: f64,
    ) -> HRESULT,
    fn AppendSubpathThreePointArc(
        StartX: f64,
        StartY: f64,
        EndX: f64,
        EndY: f64,
        ThirdX: f64,
        ThirdY: f64,
    ) -> HRESULT,
    fn SelfWeldClosedSubpaths(
    ) -> HRESULT,
    fn AppendSubpathEllipse(
        CenterX: f64,
        CenterY: f64,
        RadiusH: f64,
        RadiusV: f64,
    ) -> HRESULT,
    fn WeldEx(
        TargetCurve: *const IVGCurve,
        Method: cdrWeldMethod,
        WindingSource: VARIANT_BOOL,
        WindingTarget: VARIANT_BOOL,
        Flags: i32,
        pVal: *mut *mut IVGCurve,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058006b, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSubPaths(IVGSubPathsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGSubPath,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGSubPath,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGSubPath,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058006a, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSubPath(IVGSubPathVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_Nodes(
        ppVal: *mut *mut IVGNodes,
    ) -> HRESULT,
    fn get_Segments(
        ppVal: *mut *mut IVGSegments,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Length(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Closed(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Closed(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PositionX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PositionX(
        pVal: f64,
    ) -> HRESULT,
    fn get_PositionY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PositionY(
        pVal: f64,
    ) -> HRESULT,
    fn get_SizeWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_SizeHeight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn Selection(
        ppVal: *mut *mut IVGNodeRange,
    ) -> HRESULT,
    fn ReverseDirection(
    ) -> HRESULT,
    fn Extract(
        OldCurve: *mut *mut IVGShape,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn GetPosition(
        PositionX: *mut f64,
        PositionY: *mut f64,
    ) -> HRESULT,
    fn SetPosition(
        PositionX: f64,
        PositionY: f64,
    ) -> HRESULT,
    fn Move(
        DeltaX: f64,
        DeltaY: f64,
    ) -> HRESULT,
    fn IsOnSubPath(
        x: f64,
        y: f64,
        HotArea: f64,
        pVal: *mut cdrPositionOfPointOverShape,
    ) -> HRESULT,
    fn AppendLineSegment(
        x: f64,
        y: f64,
        AppendAtBeginning: VARIANT_BOOL,
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn AppendCurveSegment(
        x: f64,
        y: f64,
        StartingControlPointLength: f64,
        StartingControlPointAngle: f64,
        EndingControlPointLength: f64,
        EndingControlPointAngle: f64,
        AppendAtBeginning: VARIANT_BOOL,
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn GetPointPositionAt(
        x: *mut f64,
        y: *mut f64,
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
    ) -> HRESULT,
    fn BreakApartAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn AddNodeAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn GetPerpendicularAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        pVal: *mut f64,
    ) -> HRESULT,
    fn GetTangentAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        pVal: *mut f64,
    ) -> HRESULT,
    fn GetIntersections(
        Target: *const IVGSubPath,
        OffsetType: cdrSegmentOffsetType,
        ppVal: *mut *mut IVGCrossPoints,
    ) -> HRESULT,
    fn GetSegmentAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        SegmentOffset: *mut f64,
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn Next(
        ppVal: *mut *mut IVGSubPath,
    ) -> HRESULT,
    fn Previous(
        ppVal: *mut *mut IVGSubPath,
    ) -> HRESULT,
    fn GetCurvatureAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        pdCurvature: *mut f64,
    ) -> HRESULT,
    fn GetCurveSpeedAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        pdCurveSpeed: *mut f64,
    ) -> HRESULT,
    fn get_StartNode(
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn get_EndNode(
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn FindSegmentOffset(
        AbsoluteOffset: f64,
        Segment: *mut *mut IVGSegment,
        ParamOffset: *mut f64,
        Remainder: *mut f64,
        pbRes: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FirstSegment(
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn get_LastSegment(
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn AppendCurveSegment2(
        x: f64,
        y: f64,
        StartingControlPointX: f64,
        StartingControlPointY: f64,
        EndingControlPointX: f64,
        EndingControlPointY: f64,
        AppendAtBeginning: VARIANT_BOOL,
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn GetCurveInfo(
        ppVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn PutCurveInfo(
        Source: *const SAFEARRAY,
        NumElements: i32,
        pRet: *mut i32,
    ) -> HRESULT,
    fn GetPolyline(
        CurvePrecision: i32,
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn IsPointInside(
        x: f64,
        y: f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn IsRectOnEdge(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn FindClosestSegment(
        x: f64,
        y: f64,
        ParamOffset: *mut f64,
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn FindNodeAtPoint(
        x: f64,
        y: f64,
        HotArea: f64,
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn FindSegmentAtPoint(
        x: f64,
        y: f64,
        ParamOffset: *mut f64,
        HotArea: f64,
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn get_IsClockwise(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Area(
        pVal: *mut f64,
    ) -> HRESULT,
    fn GetBoundingBox(
        x: *mut f64,
        y: *mut f64,
        Width: *mut f64,
        Height: *mut f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BoundingBox(
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn EqualDivide(
        Divisions: i32,
        Gap: f64,
    ) -> HRESULT,
    fn GetEvenlySpacedPoints(
        DistanceBetweenPointsAlongCurve: f64,
        ScaleDistanceToFit: VARIANT_BOOL,
        ppVal: *mut *mut IVGPointRange,
    ) -> HRESULT,
    fn GetPerpendicularVectorAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        Normalize: VARIANT_BOOL,
        ppVal: *mut *mut IVGVector,
    ) -> HRESULT,
    fn GetTangentVectorAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        Normalize: VARIANT_BOOL,
        ppVal: *mut *mut IVGVector,
    ) -> HRESULT,
    fn GetPointAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580044, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGNodes(IVGNodesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Range(
        IndexArray: *const SAFEARRAY,
        ppVal: *mut *mut IVGNodeRange,
    ) -> HRESULT,
    fn All(
        ppVal: *mut *mut IVGNodeRange,
    ) -> HRESULT,
    fn AllExcluding(
        IndexArray: *const SAFEARRAY,
        ppVal: *mut *mut IVGNodeRange,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580042, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGNode(IVGNodeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_PositionX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PositionX(
        pVal: f64,
    ) -> HRESULT,
    fn get_PositionY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PositionY(
        pVal: f64,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrNodeType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrNodeType,
    ) -> HRESULT,
    fn get_SubPath(
        ppVal: *mut *mut IVGSubPath,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_SubPathIndex(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_AbsoluteIndex(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_IsEnding(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn JoinWith(
        Target: *const IVGNode,
    ) -> HRESULT,
    fn ConnectWith(
        Target: *const IVGNode,
    ) -> HRESULT,
    fn BreakApart(
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn GetPosition(
        PositionX: *mut f64,
        PositionY: *mut f64,
    ) -> HRESULT,
    fn SetPosition(
        PositionX: f64,
        PositionY: f64,
    ) -> HRESULT,
    fn Move(
        DeltaX: f64,
        DeltaY: f64,
    ) -> HRESULT,
    fn Next(
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn Previous(
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn GetDistanceFrom(
        Node: *const IVGNode,
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Segment(
        pVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn get_PrevSegment(
        pVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn get_NextSegment(
        pVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn Fillet(
        Radius: f64,
        CombineSmoothSegments: VARIANT_BOOL,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Chamfer(
        DistanceA: f64,
        DistanceB: f64,
        CombineSmoothSegments: VARIANT_BOOL,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Scallop(
        Radius: f64,
        CombineSmoothSegments: VARIANT_BOOL,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Selected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Selected(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn CreateSelection(
    ) -> HRESULT,
    fn ExtendSubPaths(
        Node2: *const IVGNode,
        JoinPaths: VARIANT_BOOL,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn AveragePositionWith(
        Node2: *const IVGNode,
        JoinPaths: VARIANT_BOOL,
    ) -> HRESULT,
    fn GetPoint(
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
    fn SetPoint(
        ppVal: *const IVGPoint,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580059, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSegment(IVGSegmentVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrSegmentType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrSegmentType,
    ) -> HRESULT,
    fn get_SubPath(
        ppVal: *mut *mut IVGSubPath,
    ) -> HRESULT,
    fn get_Length(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_SubPathIndex(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_AbsoluteIndex(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_StartingControlPointLength(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_StartingControlPointLength(
        pVal: f64,
    ) -> HRESULT,
    fn get_StartingControlPointAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_StartingControlPointAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_EndingControlPointLength(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_EndingControlPointLength(
        pVal: f64,
    ) -> HRESULT,
    fn get_EndingControlPointAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_EndingControlPointAngle(
        pVal: f64,
    ) -> HRESULT,
    fn GetPointPositionAt(
        x: *mut f64,
        y: *mut f64,
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
    ) -> HRESULT,
    fn BreakApartAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn AddNodeAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn GetPerpendicularAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        pVal: *mut f64,
    ) -> HRESULT,
    fn GetTangentAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        pVal: *mut f64,
    ) -> HRESULT,
    fn GetIntersections(
        Target: *const IVGSegment,
        OffsetType: cdrSegmentOffsetType,
        ppVal: *mut *mut IVGCrossPoints,
    ) -> HRESULT,
    fn Next(
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn Previous(
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn get_StartNode(
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn get_EndNode(
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn get_StartingControlPointX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_StartingControlPointX(
        pVal: f64,
    ) -> HRESULT,
    fn get_StartingControlPointY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_StartingControlPointY(
        pVal: f64,
    ) -> HRESULT,
    fn get_EndingControlPointX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_EndingControlPointX(
        pVal: f64,
    ) -> HRESULT,
    fn get_EndingControlPointY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_EndingControlPointY(
        pVal: f64,
    ) -> HRESULT,
    fn GetPeaks(
        Angle: f64,
        Offset1: *mut f64,
        Offset2: *mut f64,
        OffsetType: cdrSegmentOffsetType,
        pVal: *mut i32,
    ) -> HRESULT,
    fn GetBendPoints(
        Offset1: *mut f64,
        Offset2: *mut f64,
        OffsetType: cdrSegmentOffsetType,
        pVal: *mut i32,
    ) -> HRESULT,
    fn GetCurvatureAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        pVal: *mut f64,
    ) -> HRESULT,
    fn GetCurveSpeedAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        pVal: *mut f64,
    ) -> HRESULT,
    fn FindParamOffset(
        AbsoluteOffset: f64,
        ParamOffset: *mut f64,
        Remainder: *mut f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetAbsoluteOffset(
        ParamOffset: f64,
        AbsoluteOffset: *mut f64,
    ) -> HRESULT,
    fn GetStartingControlPointPosition(
        PositionX: *mut f64,
        PositionY: *mut f64,
    ) -> HRESULT,
    fn SetStartingControlPointPosition(
        PositionX: f64,
        PositionY: f64,
    ) -> HRESULT,
    fn GetEndingControlPointPosition(
        PositionX: *mut f64,
        PositionY: *mut f64,
    ) -> HRESULT,
    fn SetEndingControlPointPosition(
        PositionX: f64,
        PositionY: f64,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_Selected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Selected(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn CreateSelection(
    ) -> HRESULT,
    fn GetPolyline(
        CurvePrecision: i32,
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn IsRectOnEdge(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn FindClosestPoint(
        x: f64,
        y: f64,
        ParamOffset: *mut f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn FindParamOffsetAtPoint(
        x: f64,
        y: f64,
        ParamOffset: *mut f64,
        HotArea: f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetBoundingBox(
        x: *mut f64,
        y: *mut f64,
        Width: *mut f64,
        Height: *mut f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BoundingBox(
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn GetPerpendicularVectorAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        Normalize: VARIANT_BOOL,
        ppVal: *mut *mut IVGVector,
    ) -> HRESULT,
    fn GetTangentVectorAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        Normalize: VARIANT_BOOL,
        ppVal: *mut *mut IVGVector,
    ) -> HRESULT,
    fn GetPointAt(
        Offset: f64,
        OffsetType: cdrSegmentOffsetType,
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580018, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGCrossPoints(IVGCrossPointsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGCrossPoint,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580017, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGCrossPoint(IVGCrossPointVtbl): IDispatch(IDispatchVtbl) {
    fn get_PositionX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_PositionY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Offset(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Offset2(
        pVal: *mut f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800a3, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGRect(IVGRectVtbl): IDispatch(IDispatchVtbl) {
    fn get_x(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_x(
        pVal: f64,
    ) -> HRESULT,
    fn get_y(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_y(
        pVal: f64,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Width(
        pVal: f64,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Height(
        pVal: f64,
    ) -> HRESULT,
    fn SetRect(
        x: f64,
        y: f64,
        Width: f64,
        Height: f64,
    ) -> HRESULT,
    fn GetRect(
        x: *mut f64,
        y: *mut f64,
        Width: *mut f64,
        Height: *mut f64,
    ) -> HRESULT,
    fn CopyAssign(
        Source: *const IVGRect,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn Intersect(
        Rect: *const IVGRect,
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn get_IsEmpty(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Union(
        Rect: *const IVGRect,
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn Offset(
        OffsetX: f64,
        OffsetY: f64,
    ) -> HRESULT,
    fn Inflate(
        Left: f64,
        Top: f64,
        Right: f64,
        Bottom: f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn IsPointInside(
        x: f64,
        y: f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Clear(
    ) -> HRESULT,
    fn ChangeContext(
        SrcDoc: *const IVGDocument,
        DestDoc: *const IVGDocument,
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn get_Left(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Right(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Top(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Bottom(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_CenterX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_CenterY(
        pVal: *mut f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800d2, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGVector(IVGVectorVtbl): IDispatch(IDispatchVtbl) {
    fn get_x(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_x(
        pVal: f64,
    ) -> HRESULT,
    fn get_y(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_y(
        pVal: f64,
    ) -> HRESULT,
    fn get_Length(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Length(
        pVal: f64,
    ) -> HRESULT,
    fn get_Angle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Angle(
        pVal: f64,
    ) -> HRESULT,
    fn GetOffsettedPoint(
        Origin: *const IVGPoint,
        Distance: f64,
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
    fn Add(
        Vector: *const IVGVector,
    ) -> HRESULT,
    fn Subtract(
        Vector: *const IVGVector,
    ) -> HRESULT,
    fn MultiplyBy(
        Multiplier: f64,
    ) -> HRESULT,
    fn Negate(
    ) -> HRESULT,
    fn Normalize(
    ) -> HRESULT,
    fn AngleBetween(
        Vector: *const IVGVector,
        pVal: *mut f64,
    ) -> HRESULT,
    fn SmallAngleBetween(
        Vector: *const IVGVector,
        pVal: *mut f64,
    ) -> HRESULT,
    fn DotProduct(
        Vector: *const IVGVector,
        pVal: *mut f64,
    ) -> HRESULT,
    fn CrossProduct(
        Vector: *const IVGVector,
        pVal: *mut f64,
    ) -> HRESULT,
    fn SetFromPoints(
        Start: *const IVGPoint,
        End: *const IVGPoint,
    ) -> HRESULT,
    fn ProjectOnto(
        Vector: *const IVGVector,
        ppVal: *mut *mut IVGVector,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGVector,
    ) -> HRESULT,
    fn BindToDocument(
        Document: *const IVGDocument,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800d0, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPoint(IVGPointVtbl): IDispatch(IDispatchVtbl) {
    fn get_x(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_x(
        pVal: f64,
    ) -> HRESULT,
    fn get_y(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_y(
        pVal: f64,
    ) -> HRESULT,
    fn Add(
        Vector: *const IVGVector,
    ) -> HRESULT,
    fn Subtract(
        Vector: *const IVGVector,
    ) -> HRESULT,
    fn DistanceTo(
        Point: *const IVGPoint,
        pVal: *mut f64,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
    fn BindToDocument(
        Document: *const IVGDocument,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580043, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGNodeRange(IVGNodeRangeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrNodeType,
    ) -> HRESULT,
    fn get_PositionX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_PositionY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_SizeWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_SizeHeight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn Add(
        Node: *const IVGNode,
    ) -> HRESULT,
    fn Remove(
        Index: i32,
    ) -> HRESULT,
    fn Move(
        DeltaX: f64,
        DeltaY: f64,
        AnchorIndex: i32,
        ElasticMode: VARIANT_BOOL,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn Stretch(
        RatioX: f32,
        RatioY: f32,
        UseAnchorPoint: VARIANT_BOOL,
        StretchAnchorX: f64,
        StretchAnchorY: f64,
    ) -> HRESULT,
    fn Rotate(
        Angle: f64,
        UseCenterPoint: VARIANT_BOOL,
        RotationCenterX: f64,
        RotationCenterY: f64,
    ) -> HRESULT,
    fn Skew(
        AngleX: f64,
        AngleY: f64,
        UseAnchorPoint: VARIANT_BOOL,
        SkewAnchorX: f64,
        SkewAnchorY: f64,
    ) -> HRESULT,
    fn AutoReduce(
        PrecisionMargin: f64,
    ) -> HRESULT,
    fn RemoveAll(
    ) -> HRESULT,
    fn SetType(
        Type: cdrNodeType,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn AddRange(
        NodeRange: *const IVGNodeRange,
    ) -> HRESULT,
    fn get_SegmentRange(
        ppVal: *mut *mut IVGSegmentRange,
    ) -> HRESULT,
    fn BreakApart(
    ) -> HRESULT,
    fn Smoothen(
        Smoothness: i32,
    ) -> HRESULT,
    fn RemoveRange(
        NodeRange: *const IVGNodeRange,
    ) -> HRESULT,
    fn Fillet(
        Radius: f64,
        CombineSmoothSegments: VARIANT_BOOL,
    ) -> HRESULT,
    fn Chamfer(
        DistanceA: f64,
        DistanceB: f64,
        CombineSmoothSegments: VARIANT_BOOL,
    ) -> HRESULT,
    fn Scallop(
        Radius: f64,
        CombineSmoothSegments: VARIANT_BOOL,
    ) -> HRESULT,
    fn CreateSelection(
    ) -> HRESULT,
    fn AddToSelection(
    ) -> HRESULT,
    fn RemoveFromSelection(
    ) -> HRESULT,
    fn get_FirstNode(
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn get_LastNode(
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn GetBoundingBox(
        x: *mut f64,
        y: *mut f64,
        Width: *mut f64,
        Height: *mut f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BoundingBox(
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058005a, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSegmentRange(IVGSegmentRangeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrSegmentType,
    ) -> HRESULT,
    fn get_Length(
        pVal: *mut f64,
    ) -> HRESULT,
    fn Add(
        Segment: *const IVGSegment,
    ) -> HRESULT,
    fn Remove(
        Index: i32,
    ) -> HRESULT,
    fn AddNode(
    ) -> HRESULT,
    fn RemoveAll(
    ) -> HRESULT,
    fn SetType(
        Type: cdrSegmentType,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn AddRange(
        SegmentRange: *const IVGSegmentRange,
    ) -> HRESULT,
    fn get_NodeRange(
        ppVal: *mut *mut IVGNodeRange,
    ) -> HRESULT,
    fn RemoveRange(
        SegmentRange: *const IVGSegmentRange,
    ) -> HRESULT,
    fn get_FirstSegment(
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn get_LastSegment(
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn CreateSelection(
    ) -> HRESULT,
    fn AddToSelection(
    ) -> HRESULT,
    fn RemoveFromSelection(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058005b, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSegments(IVGSegmentsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Range(
        IndexArray: *const SAFEARRAY,
        ppVal: *mut *mut IVGSegmentRange,
    ) -> HRESULT,
    fn All(
        ppVal: *mut *mut IVGSegmentRange,
    ) -> HRESULT,
    fn AllExcluding(
        IndexArray: *const SAFEARRAY,
        ppVal: *mut *mut IVGSegmentRange,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGSegment,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800d1, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPointRange(IVGPointRangeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
    fn put_Item(
        Index: i32,
        ppVal: *const IVGPoint,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn AddPoint(
        Point: *const IVGPoint,
    ) -> HRESULT,
    fn AddPointXY(
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn InsertPoint(
        Index: i32,
        Point: *const IVGPoint,
    ) -> HRESULT,
    fn AddPoints(
        Points: *const IVGPointRange,
    ) -> HRESULT,
    fn InsertPoints(
        Index: i32,
        Points: *const IVGPointRange,
    ) -> HRESULT,
    fn Remove(
        Index: i32,
    ) -> HRESULT,
    fn RemoveRange(
        StartIndex: i32,
        EndIndex: i32,
    ) -> HRESULT,
    fn RemoveAll(
    ) -> HRESULT,
    fn RemoveAdjacentDuplicates(
    ) -> HRESULT,
    fn Reverse(
    ) -> HRESULT,
    fn Smoothen(
        NumberOfPointsToSmoothAcross: f64,
        Closed: VARIANT_BOOL,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGPointRange,
    ) -> HRESULT,
    fn BindToDocument(
        Document: *const IVGDocument,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800d3, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTransformMatrix(IVGTransformMatrixVtbl): IDispatch(IDispatchVtbl) {
    fn get_d11(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_d11(
        pVal: f64,
    ) -> HRESULT,
    fn get_d12(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_d12(
        pVal: f64,
    ) -> HRESULT,
    fn get_d21(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_d21(
        pVal: f64,
    ) -> HRESULT,
    fn get_d22(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_d22(
        pVal: f64,
    ) -> HRESULT,
    fn get_TranslationX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TranslationX(
        pVal: f64,
    ) -> HRESULT,
    fn get_TranslationY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TranslationY(
        pVal: f64,
    ) -> HRESULT,
    fn get_Translation(
        ppVal: *mut *mut IVGVector,
    ) -> HRESULT,
    fn put_Translation(
        ppVal: *const IVGVector,
    ) -> HRESULT,
    fn SetToIdentity(
    ) -> HRESULT,
    fn Invert(
    ) -> HRESULT,
    fn TranslateBy(
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn TranslateByVector(
        Vector: *mut IVGVector,
    ) -> HRESULT,
    fn SetTranslation(
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn Rotate(
        Angle: f64,
    ) -> HRESULT,
    fn RotateAround(
        Angle: f64,
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn Scale(
        ScaleX: f64,
        ScaleY: f64,
    ) -> HRESULT,
    fn ScaleAround(
        ScaleX: f64,
        ScaleY: f64,
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn Transform(
        TransformMatrix: *const IVGTransformMatrix,
    ) -> HRESULT,
    fn TransformAround(
        TransformMatrix: *const IVGTransformMatrix,
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn TransformPoint(
        Point: *const IVGPoint,
    ) -> HRESULT,
    fn TransformPoints(
        Points: *const IVGPointRange,
    ) -> HRESULT,
    fn TransformVector(
        Vector: *const IVGVector,
    ) -> HRESULT,
    fn UntransformPoint(
        Point: *const IVGPoint,
    ) -> HRESULT,
    fn UntransformPoints(
        Points: *const IVGPointRange,
    ) -> HRESULT,
    fn UntransformVector(
        Vector: *const IVGVector,
    ) -> HRESULT,
    fn get_IsIdentity(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSkewedOrRotatedOrMirrored(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ContainsOnlyTranslation(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSkewedOrRotated(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsScaledOrSkewedOrRotated(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsOrthogonal(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsOrthonormalAxisAligned(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsOrthonormal(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsMirrored(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsScaled(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsTranslated(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGTransformMatrix,
    ) -> HRESULT,
    fn BindToDocument(
        Document: *const IVGDocument,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058000f, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGBitmap(IVGBitmapVtbl): IDispatch(IDispatchVtbl) {
    fn get_SizeWidth(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_SizeHeight(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_ResolutionX(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_ResolutionY(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_ExternallyLinked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ResolveLink(
    ) -> HRESULT,
    fn UpdateLink(
    ) -> HRESULT,
    fn Inflate(
        Width: i32,
        Height: i32,
    ) -> HRESULT,
    fn get_LinkFileName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Mode(
        pVal: *mut cdrImageType,
    ) -> HRESULT,
    fn SaveAs(
        FileName: BSTR,
        Filter: cdrFilter,
        Compression: cdrCompressionType,
        ppVal: *mut *mut ICorelExportFilter,
    ) -> HRESULT,
    fn Resample(
        Width: i32,
        Height: i32,
        AntiAlias: VARIANT_BOOL,
        ResolutionX: f64,
        ResolutionY: f64,
    ) -> HRESULT,
    fn ConvertTo(
        Mode: cdrImageType,
    ) -> HRESULT,
    fn ApplyBitmapEffect(
        UndoString: BSTR,
        Command: BSTR,
    ) -> HRESULT,
    fn Crop(
    ) -> HRESULT,
    fn get_Transparent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Watermarked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_OPILinked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsEPS(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ExternalLinkTime(
        pVal: *mut DATE,
    ) -> HRESULT,
    fn ConvertToPaletted(
        PaletteType: cdrImagePaletteType,
        DitherType: cdrDitherType,
        DitherIntensity: i32,
        Smoothing: i32,
        NumColors: i32,
        ColorSensitive: VARIANT_BOOL,
        TargetColor: i32,
        Importance: i32,
        Lightness: i32,
        ToleranceA: i32,
        ToleranceB: i32,
        Palette: *const VARIANT,
    ) -> HRESULT,
    fn ConvertToPaletted2(
        Options: *const IVGStructPaletteOptions,
    ) -> HRESULT,
    fn ConvertToBW(
        RenderType: cdrRenderType,
        Intensity: i32,
        Threshold: i32,
        Halftone: cdrHalftoneType,
        HalftoneAngle: i32,
        HalftoneSize: i32,
    ) -> HRESULT,
    fn ResetCropEnvelope(
    ) -> HRESULT,
    fn get_Embedded(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Cropped(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CropEnvelopeModified(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CropEnvelope(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_BoundingBoxPath(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn ConvertToDuotone(
        Duotone: *const IVGDuotone,
    ) -> HRESULT,
    fn get_Duotone(
        ppVal: *mut *mut IVGDuotone,
    ) -> HRESULT,
    fn put_LinkFileName(
        pVal: BSTR,
    ) -> HRESULT,
    fn Trace(
        TraceType: cdrTraceType,
        Smoothing: i16,
        DetailLevelPercent: i16,
        ColorMode: cdrColorType,
        PaletteID: cdrPaletteID,
        ColorCount: i32,
        DeleteOriginalObject: VARIANT_BOOL,
        RemoveBackground: VARIANT_BOOL,
        RemoveEntireBackColor: VARIANT_BOOL,
        ppVal: *mut *mut IVGTraceSettings,
    ) -> HRESULT,
    fn get_Image(
        ppVal: *mut *mut IVGImage,
    ) -> HRESULT,
    fn get_ImageAlpha(
        ppVal: *mut *mut IVGImage,
    ) -> HRESULT,
    fn SetImageData(
        Image: *const IVGImage,
        Alpha: *const IVGImage,
        OffsetX: i32,
        OffsetY: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580067, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStructPaletteOptions(IVGStructPaletteOptionsVtbl): IDispatch(IDispatchVtbl) {
    fn put_NumColors(
        pVal: i32,
    ) -> HRESULT,
    fn get_NumColors(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_DitherIntensity(
        pVal: i32,
    ) -> HRESULT,
    fn get_DitherIntensity(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Smoothing(
        pVal: i32,
    ) -> HRESULT,
    fn get_Smoothing(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_DitherType(
        pVal: cdrDitherType,
    ) -> HRESULT,
    fn get_DitherType(
        pVal: *mut cdrDitherType,
    ) -> HRESULT,
    fn put_PaletteType(
        pVal: cdrImagePaletteType,
    ) -> HRESULT,
    fn get_PaletteType(
        pVal: *mut cdrImagePaletteType,
    ) -> HRESULT,
    fn put_Importance(
        pVal: i32,
    ) -> HRESULT,
    fn get_Importance(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Lightness(
        pVal: i32,
    ) -> HRESULT,
    fn get_Lightness(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_ToleranceA(
        pVal: i32,
    ) -> HRESULT,
    fn get_ToleranceA(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_ToleranceB(
        pVal: i32,
    ) -> HRESULT,
    fn get_ToleranceB(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_ColorSensitive(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColorSensitive(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TargetColor(
        pVal: i32,
    ) -> HRESULT,
    fn get_TargetColor(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Palette(
        pVal: VARIANT,
    ) -> HRESULT,
    fn get_Palette(
        pVal: *mut VARIANT,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580091, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDuotone(IVGDuotoneVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrDuotoneType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrDuotoneType,
    ) -> HRESULT,
    fn get_UseOverprints(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseOverprints(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_OverprintCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Overprints(
        Index: i32,
        ppVal: *mut *mut IVGDuotoneOverprint,
    ) -> HRESULT,
    fn get_InkCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Inks(
        Index: i32,
        ppVal: *mut *mut IVGDuotoneInk,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGDuotone,
    ) -> HRESULT,
    fn CopyAssign(
        Duotone: *const IVGDuotone,
    ) -> HRESULT,
    fn ResetOverprints(
    ) -> HRESULT,
    fn Load(
        FileName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Save(
        FileName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580092, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDuotoneOverprint(IVGDuotoneOverprintVtbl): IDispatch(IDispatchVtbl) {
    fn get_Cyan(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Cyan(
        pVal: i32,
    ) -> HRESULT,
    fn get_Magenta(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Magenta(
        pVal: i32,
    ) -> HRESULT,
    fn get_Yellow(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Yellow(
        pVal: i32,
    ) -> HRESULT,
    fn get_Black(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Black(
        pVal: i32,
    ) -> HRESULT,
    fn get_Color(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_Color(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn SetValues(
        Cyan: i32,
        Magenta: i32,
        Yellow: i32,
        Black: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580012, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGColor(IVGColorVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Type(
        Type: *mut cdrColorType,
    ) -> HRESULT,
    fn RGBAssign(
        Red: i32,
        Green: i32,
        Blue: i32,
    ) -> HRESULT,
    fn get_RGBRed(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_RGBRed(
        pVal: i32,
    ) -> HRESULT,
    fn get_RGBGreen(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_RGBGreen(
        pVal: i32,
    ) -> HRESULT,
    fn get_RGBBlue(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_RGBBlue(
        pVal: i32,
    ) -> HRESULT,
    fn ConvertToRGB(
    ) -> HRESULT,
    fn CMYKAssign(
        Cyan: i32,
        Magenta: i32,
        Yellow: i32,
        Black: i32,
    ) -> HRESULT,
    fn get_CMYKCyan(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CMYKCyan(
        pVal: i32,
    ) -> HRESULT,
    fn get_CMYKYellow(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CMYKYellow(
        pVal: i32,
    ) -> HRESULT,
    fn get_CMYKMagenta(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CMYKMagenta(
        pVal: i32,
    ) -> HRESULT,
    fn get_CMYKBlack(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CMYKBlack(
        pVal: i32,
    ) -> HRESULT,
    fn ConvertToCMYK(
    ) -> HRESULT,
    fn CMYAssign(
        Cyan: i32,
        Magenta: i32,
        Yellow: i32,
    ) -> HRESULT,
    fn get_CMYCyan(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CMYCyan(
        pVal: i32,
    ) -> HRESULT,
    fn get_CMYMagenta(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CMYMagenta(
        pVal: i32,
    ) -> HRESULT,
    fn get_CMYYellow(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CMYYellow(
        pVal: i32,
    ) -> HRESULT,
    fn ConvertToCMY(
    ) -> HRESULT,
    fn HSBAssign(
        Hue: i32,
        Saturation: i32,
        Brightness: i32,
    ) -> HRESULT,
    fn get_HSBHue(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_HSBHue(
        pVal: i32,
    ) -> HRESULT,
    fn get_HSBSaturation(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_HSBSaturation(
        pVal: i32,
    ) -> HRESULT,
    fn get_HSBBrightness(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_HSBBrightness(
        pVal: i32,
    ) -> HRESULT,
    fn ConvertToHSB(
    ) -> HRESULT,
    fn HLSAssign(
        Hue: i32,
        Lightness: i32,
        Saturation: i32,
    ) -> HRESULT,
    fn get_HLSHue(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_HLSHue(
        pVal: i32,
    ) -> HRESULT,
    fn get_HLSLightness(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_HLSLightness(
        pVal: i32,
    ) -> HRESULT,
    fn get_HLSSaturation(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_HLSSaturation(
        pVal: i32,
    ) -> HRESULT,
    fn ConvertToHLS(
    ) -> HRESULT,
    fn BWAssign(
        White: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BW(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_BW(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn ConvertToBW(
    ) -> HRESULT,
    fn GrayAssign(
        GrayValue: i32,
    ) -> HRESULT,
    fn get_Gray(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Gray(
        pVal: i32,
    ) -> HRESULT,
    fn ConvertToGray(
    ) -> HRESULT,
    fn CorelScriptAssign(
        ColorModel: i32,
        V1: i32,
        V2: i32,
        V3: i32,
        V4: i32,
        V5: i32,
        V6: i32,
        V7: i32,
    ) -> HRESULT,
    fn CorelScriptGetComponent(
        ColorModel: *mut i32,
        V1: *mut i32,
        V2: *mut i32,
        V3: *mut i32,
        V4: *mut i32,
        V5: *mut i32,
        V6: *mut i32,
        V7: *mut i32,
    ) -> HRESULT,
    fn UserAssign(
        ParentWindowHandle: i32,
    ) -> HRESULT,
    fn CopyAssign(
        Color: *const IVGColor,
    ) -> HRESULT,
    fn get_Name(
        Components: VARIANT_BOOL,
        Name: *mut BSTR,
    ) -> HRESULT,
    fn YIQAssign(
        y: i32,
        I: i32,
        Q: i32,
    ) -> HRESULT,
    fn get_YIQLuminanceY(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_YIQLuminanceY(
        pVal: i32,
    ) -> HRESULT,
    fn get_YIQChromaI(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_YIQChromaI(
        pVal: i32,
    ) -> HRESULT,
    fn get_YIQChromaQ(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_YIQChromaQ(
        pVal: i32,
    ) -> HRESULT,
    fn ConvertToYIQ(
    ) -> HRESULT,
    fn LabAssign(
        L: i32,
        A: i32,
        B: i32,
    ) -> HRESULT,
    fn get_LabLuminance(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_LabLuminance(
        pVal: i32,
    ) -> HRESULT,
    fn get_LabComponentA(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_LabComponentA(
        pVal: i32,
    ) -> HRESULT,
    fn get_LabComponentB(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_LabComponentB(
        pVal: i32,
    ) -> HRESULT,
    fn ConvertToLab(
    ) -> HRESULT,
    fn RegistrationAssign(
    ) -> HRESULT,
    fn FixedAssign(
        PaletteID: cdrPaletteID,
        PaletteIndex: i32,
        Tint: i32,
    ) -> HRESULT,
    fn get_PaletteID(
        pVal: *mut cdrPaletteID,
    ) -> HRESULT,
    fn get_PaletteIndex(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_PaletteIndex(
        pVal: i32,
    ) -> HRESULT,
    fn get_Tint(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Tint(
        pVal: i32,
    ) -> HRESULT,
    fn ConvertToFixed(
        PaletteID: cdrPaletteID,
    ) -> HRESULT,
    fn UserAssignEx(
        ParentWindowHandle: i32,
        pbOK: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn SetName(
        Name: BSTR,
    ) -> HRESULT,
    fn BlendWith(
        Color: *const IVGColor,
        MixRatio: i32,
    ) -> HRESULT,
    fn IsSame(
        Color: *const IVGColor,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsInGamut(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_InGamutColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn get_IsCMYK(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsGray(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsWhite(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSpot(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsTintable(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsValidDuotoneColor(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ValidDuotoneColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn GetColorDistanceFrom(
        Color: *const IVGColor,
        pVal: *mut i32,
    ) -> HRESULT,
    fn IsSimilar(
        Color: *const IVGColor,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ToString(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn StringAssign(
        ColorString: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Invalidate(
    ) -> HRESULT,
    fn get_HexValue(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_HexValue(
        pVal: BSTR,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn get_RGBValue(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_RGBValue(
        pVal: i32,
    ) -> HRESULT,
    fn CopyAppearance(
        Color: *const IVGColor,
        SourceColorContext: *const IVGColorContext,
    ) -> HRESULT,
    fn get_ColorContext(
        ppVal: *mut *mut IVGColorContext,
    ) -> HRESULT,
    fn ConvertTo(
        ColorType: cdrColorType,
        DestinationColorContext: *const IVGColorContext,
        SourceColorContext: *const IVGColorContext,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Palette(
        ppVal: *mut *mut IVGPalette,
    ) -> HRESULT,
    fn SpotAssign(
        PaletteIdentifier: BSTR,
        SpotColorID: i32,
        Tint: i32,
    ) -> HRESULT,
    fn SpotAssignByName(
        PaletteIdentifier: BSTR,
        SpotColorName: BSTR,
        Tint: i32,
    ) -> HRESULT,
    fn ConvertToPalette(
        PaletteIdentifier: BSTR,
    ) -> HRESULT,
    fn get_SpotColorID(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_SpotColorID(
        pVal: i32,
    ) -> HRESULT,
    fn get_SpotColorName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn PaletteAssign(
        PaletteIdentifier: BSTR,
        ColorIndex: i32,
    ) -> HRESULT,
    fn get_PaletteIdentifier(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_IsColorStyle(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColorStyleName(
        Name: *mut BSTR,
    ) -> HRESULT,
    fn ModifyColorStyleColor(
        Color: *const IVGColor,
        ChangeWholeHarmony: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800b0, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGColorContext(IVGColorContextVtbl): IDispatch(IDispatchVtbl) {
    fn get_RGBColorProfile(
        ppVal: *mut *mut IVGColorProfile,
    ) -> HRESULT,
    fn put_RGBColorProfile(
        ppVal: *const IVGColorProfile,
    ) -> HRESULT,
    fn get_CMYKColorProfile(
        ppVal: *mut *mut IVGColorProfile,
    ) -> HRESULT,
    fn put_CMYKColorProfile(
        ppVal: *const IVGColorProfile,
    ) -> HRESULT,
    fn get_GrayscaleColorProfile(
        ppVal: *mut *mut IVGColorProfile,
    ) -> HRESULT,
    fn put_GrayscaleColorProfile(
        ppVal: *const IVGColorProfile,
    ) -> HRESULT,
    fn get_RenderingIntent(
        pVal: *mut clrRenderingIntent,
    ) -> HRESULT,
    fn put_RenderingIntent(
        pVal: clrRenderingIntent,
    ) -> HRESULT,
    fn get_BlendingColorModel(
        pVal: *mut clrColorModel,
    ) -> HRESULT,
    fn put_BlendingColorModel(
        pVal: clrColorModel,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGColorContext,
    ) -> HRESULT,
    fn CopyAssign(
        ColorContext: *const IVGColorContext,
    ) -> HRESULT,
    fn get_ColorProfile(
        ColorModel: clrColorModel,
        ppVal: *mut *mut IVGColorProfile,
    ) -> HRESULT,
    fn put_ColorProfile(
        ColorModel: clrColorModel,
        ppVal: *const IVGColorProfile,
    ) -> HRESULT,
    fn Merge(
        ColorContext: *const IVGColorContext,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn IsSame(
        ColorContext: *const IVGColorContext,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColorProfiles(
        ppVal: *mut *mut IVGColorProfiles,
    ) -> HRESULT,
    fn get_ReadOnly(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColorProfileNameList(
        pVal: *mut BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580098, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGColorProfile(IVGColorProfileVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_FileName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Manufacturer(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_DeviceModel(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_DeviceType(
        pVal: *mut clrDeviceType,
    ) -> HRESULT,
    fn get_Selected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Select(
    ) -> HRESULT,
    fn get_Generic(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Installed(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_ColorModel(
        pVal: *mut clrColorModel,
    ) -> HRESULT,
    fn CreateColorContext(
        RenderingIntent: clrRenderingIntent,
        ppVal: *mut *mut IVGColorContext,
    ) -> HRESULT,
    fn IsSame(
        ColorProfile: *const IVGColorProfile,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580099, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGColorProfiles(IVGColorProfilesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGColorProfile,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_DeviceType(
        pVal: *mut clrDeviceType,
    ) -> HRESULT,
    fn SelectByName(
        Name: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Install(
        FileName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_GenericProfile(
        ppVal: *mut *mut IVGColorProfile,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058004c, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPalette(IVGPaletteVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Name(
        Name: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        Name: BSTR,
    ) -> HRESULT,
    fn Close(
    ) -> HRESULT,
    fn get_Type(
        Type: *mut cdrPaletteType,
    ) -> HRESULT,
    fn Colors(
        Colors: *mut *mut IVGColors,
    ) -> HRESULT,
    fn get_Color(
        Index: i32,
        Color: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_Color(
        Index: i32,
        Color: *const IVGColor,
    ) -> HRESULT,
    fn AddColor(
        Color: *const IVGColor,
    ) -> HRESULT,
    fn InsertColor(
        Index: i32,
        Color: *const IVGColor,
    ) -> HRESULT,
    fn RemoveColor(
        Index: i32,
    ) -> HRESULT,
    fn GetIndexOfColor(
        Color: *const IVGColor,
        Index: *mut i32,
    ) -> HRESULT,
    fn get_DuplicatePresent(
        Duplicate: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColorCount(
        Count: *mut i32,
    ) -> HRESULT,
    fn Save(
    ) -> HRESULT,
    fn get_PaletteID(
        pVal: *mut cdrPaletteID,
    ) -> HRESULT,
    fn get_FileName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn MatchColor(
        Color: *const IVGColor,
        pVal: *mut i32,
    ) -> HRESULT,
    fn FindColor(
        Name: BSTR,
        pVal: *mut i32,
    ) -> HRESULT,
    fn SaveAs(
        FileName: BSTR,
        Name: BSTR,
        Version: cdrPaletteVersion,
    ) -> HRESULT,
    fn get_Document(
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get_Identifier(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_IsEmpty(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Locked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Default(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsOpen(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Open(
    ) -> HRESULT,
    fn MakeDefault(
    ) -> HRESULT,
    fn Delete(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580013, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGColors(IVGColorsVtbl): IDispatch(IDispatchVtbl) {

}}

RIDL!{#[uuid(0xb0580093, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDuotoneInk(IVGDuotoneInkVtbl): IDispatch(IDispatchVtbl) {
    fn get_Color(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_Color(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_HandleCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_HandleX(
        Index: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_HandleX(
        Index: i32,
        pVal: i32,
    ) -> HRESULT,
    fn get_HandleY(
        Index: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_HandleY(
        Index: i32,
        pVal: i32,
    ) -> HRESULT,
    fn AddHandle(
        x: i32,
        y: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn RemoveHandle(
        Index: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn FindHandle(
        x: i32,
        y: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn Load(
        FileName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Save(
        FileName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CurveLevel(
        Index: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn GetCurveLevels(
        ppVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetHandles(
        ppVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn PutHandles(
        HandleArray: VARIANT,
        NumElements: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800a1, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTraceSettings(IVGTraceSettingsVtbl): IDispatch(IDispatchVtbl) {
    fn get_TraceType(
        pVal: *mut cdrTraceType,
    ) -> HRESULT,
    fn put_TraceType(
        pVal: cdrTraceType,
    ) -> HRESULT,
    fn get_Smoothing(
        pVal: *mut i16,
    ) -> HRESULT,
    fn put_Smoothing(
        pVal: i16,
    ) -> HRESULT,
    fn get_DetailLevel(
        pVal: *mut i16,
    ) -> HRESULT,
    fn put_DetailLevel(
        pVal: i16,
    ) -> HRESULT,
    fn get_ColorMode(
        pVal: *mut cdrColorType,
    ) -> HRESULT,
    fn get_PaletteID(
        pVal: *mut cdrPaletteID,
    ) -> HRESULT,
    fn get_ColorCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Color(
        Index: i32,
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn get_DeleteOriginalObject(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DeleteOriginalObject(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RemoveBackground(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RemoveBackground(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RemoveEntireBackColor(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RemoveEntireBackColor(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BackgroundRemovalMode(
        pVal: *mut cdrTraceBackgroundMode,
    ) -> HRESULT,
    fn put_BackgroundRemovalMode(
        pVal: cdrTraceBackgroundMode,
    ) -> HRESULT,
    fn get_BackgroundColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn get_CurveCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_NodeCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_BitmapWidth(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_BitmapHeight(
        pVal: *mut i32,
    ) -> HRESULT,
    fn SetColorCount(
        ColorCount: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn Finish(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn ShowDialog(
        ParentWindowHandle: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ApplyChanges(
    ) -> HRESULT,
    fn SetColorMode(
        ColorMode: cdrColorType,
        PaletteID: cdrPaletteID,
    ) -> HRESULT,
    fn get_DetailLevelPercent(
        pVal: *mut i16,
    ) -> HRESULT,
    fn put_DetailLevelPercent(
        pVal: i16,
    ) -> HRESULT,
    fn get_MaxDetailLevel(
        pVal: *mut i16,
    ) -> HRESULT,
    fn get_MinDetailLevel(
        pVal: *mut i16,
    ) -> HRESULT,
    fn get_CornerSmoothness(
        pVal: *mut i16,
    ) -> HRESULT,
    fn put_CornerSmoothness(
        pVal: i16,
    ) -> HRESULT,
    fn get_MergeAdjacentObjects(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MergeAdjacentObjects(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RemoveOverlap(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RemoveOverlap(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_GroupObjectsByColor(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_GroupObjectsByColor(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058005e, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGShapeRange(IVGShapeRangeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Add(
        Shape: *const IVGShape,
    ) -> HRESULT,
    fn Remove(
        Index: i32,
    ) -> HRESULT,
    fn IndexOf(
        Shape: *const IVGShape,
        pVal: *mut i32,
    ) -> HRESULT,
    fn AddToSelection(
    ) -> HRESULT,
    fn ConvertToCurves(
    ) -> HRESULT,
    fn ConvertToBitmap(
        BitDepth: i32,
        Grayscale: VARIANT_BOOL,
        Dithered: VARIANT_BOOL,
        TransparentBG: VARIANT_BOOL,
        Resolution: i32,
        AntiAliasing: cdrAntiAliasingType,
        UseColorProfile: VARIANT_BOOL,
        MultiChannel: VARIANT_BOOL,
        AlwaysOverprintBlack: VARIANT_BOOL,
        OverprintBlackLimit: i32,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Copy(
    ) -> HRESULT,
    fn Cut(
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn GetPosition(
        PositionX: *mut f64,
        PositionY: *mut f64,
    ) -> HRESULT,
    fn GetSize(
        Width: *mut f64,
        Height: *mut f64,
    ) -> HRESULT,
    fn Move(
        DeltaX: f64,
        DeltaY: f64,
    ) -> HRESULT,
    fn Duplicate(
        OffsetX: f64,
        OffsetY: f64,
        ppRet: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn Clone(
        OffsetX: f64,
        OffsetY: f64,
        ppRet: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn Group(
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn RemoveAll(
    ) -> HRESULT,
    fn OrderToFront(
    ) -> HRESULT,
    fn OrderToBack(
    ) -> HRESULT,
    fn OrderForwardOne(
    ) -> HRESULT,
    fn OrderBackOne(
    ) -> HRESULT,
    fn OrderFrontOf(
        Shape: *const IVGShape,
    ) -> HRESULT,
    fn OrderBackOf(
        Shape: *const IVGShape,
    ) -> HRESULT,
    fn OrderReverse(
    ) -> HRESULT,
    fn Rotate(
        Angle: f64,
    ) -> HRESULT,
    fn RotateEx(
        Angle: f64,
        CenterX: f64,
        CenterY: f64,
    ) -> HRESULT,
    fn Skew(
        AngleX: f64,
        AngleY: f64,
    ) -> HRESULT,
    fn SkewEx(
        AngleX: f64,
        AngleY: f64,
        CenterX: f64,
        CenterY: f64,
    ) -> HRESULT,
    fn UngroupAll(
    ) -> HRESULT,
    fn Flip(
        Axes: cdrFlipAxes,
    ) -> HRESULT,
    fn get_PositionX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PositionX(
        pVal: f64,
    ) -> HRESULT,
    fn get_PositionY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PositionY(
        pVal: f64,
    ) -> HRESULT,
    fn get_SizeWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_SizeWidth(
        pVal: f64,
    ) -> HRESULT,
    fn get_SizeHeight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_SizeHeight(
        pVal: f64,
    ) -> HRESULT,
    fn get_RotationCenterX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RotationCenterX(
        pVal: f64,
    ) -> HRESULT,
    fn get_RotationCenterY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RotationCenterY(
        pVal: f64,
    ) -> HRESULT,
    fn AddToPowerClip(
        Shape: *const IVGShape,
        CenterInContainer: cdrTriState,
    ) -> HRESULT,
    fn RemoveFromContainer(
        Level: i32,
    ) -> HRESULT,
    fn AddRange(
        ShapeRange: *const IVGShapeRange,
    ) -> HRESULT,
    fn SetPosition(
        PositionX: f64,
        PositionY: f64,
    ) -> HRESULT,
    fn SetSize(
        Width: f64,
        Height: f64,
    ) -> HRESULT,
    fn ConvertToBitmapEx(
        Mode: cdrImageType,
        Dithered: VARIANT_BOOL,
        Transparent: VARIANT_BOOL,
        Resolution: i32,
        AntiAliasing: cdrAntiAliasingType,
        UseColorProfile: VARIANT_BOOL,
        AlwaysOverprintBlack: VARIANT_BOOL,
        OverprintBlackLimit: i32,
        ppRet: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Combine(
        ppRetVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn SetBoundingBox(
        x: f64,
        y: f64,
        Width: f64,
        Height: f64,
        KeepAspect: VARIANT_BOOL,
        ReferencePoint: cdrReferencePoint,
    ) -> HRESULT,
    fn ApplyNoFill(
    ) -> HRESULT,
    fn ApplyUniformFill(
        Color: *const IVGColor,
    ) -> HRESULT,
    fn ApplyFountainFill(
        StartColor: *const IVGColor,
        EndColor: *const IVGColor,
        Type: cdrFountainFillType,
        Angle: f64,
        Steps: i32,
        EdgePad: i32,
        MidPoint: i32,
        BlendType: cdrFountainFillBlendType,
        CenterOffsetX: f64,
        CenterOffsetY: f64,
    ) -> HRESULT,
    fn ApplyPatternFill(
        Type: cdrPatternFillType,
        FileName: BSTR,
        PatternCanvasIndex: i32,
        FrontColor: *const IVGColor,
        EndColor: *const IVGColor,
        TransformWithShape: VARIANT_BOOL,
    ) -> HRESULT,
    fn ApplyTextureFill(
        TextureName: BSTR,
        LibraryName: BSTR,
    ) -> HRESULT,
    fn ApplyPostscriptFill(
        IndexOrName: VARIANT,
    ) -> HRESULT,
    fn ConvertOutlineToObject(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn SetOutlineProperties(
        Width: f64,
        Style: *const IVGOutlineStyle,
        Color: *const IVGColor,
        StartArrow: *const IVGArrowHead,
        EndArrow: *const IVGArrowHead,
        BehindFill: cdrTriState,
        ScaleWithShape: cdrTriState,
        LineCaps: cdrOutlineLineCaps,
        LineJoin: cdrOutlineLineJoin,
        NibAngle: f64,
        NibStretch: i32,
        DashDotLength: f64,
        PenWidth: f64,
        MiterLimit: f64,
    ) -> HRESULT,
    fn CreateSelection(
    ) -> HRESULT,
    fn RemoveFromSelection(
    ) -> HRESULT,
    fn SetRotationCenter(
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn Stretch(
        StretchX: f64,
        StretchY: f64,
        StretchCharactersSize: VARIANT_BOOL,
    ) -> HRESULT,
    fn StretchEx(
        CenterX: f64,
        CenterY: f64,
        StretchX: f64,
        StretchY: f64,
        StretchCharactersSize: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetSizeEx(
        CenterX: f64,
        CenterY: f64,
        Width: f64,
        Height: f64,
    ) -> HRESULT,
    fn GetBoundingBox(
        x: *mut f64,
        y: *mut f64,
        Width: *mut f64,
        Height: *mut f64,
        UseOutline: VARIANT_BOOL,
    ) -> HRESULT,
    fn RestoreCloneLink(
        LinkToRestore: cdrCloneLinkType,
    ) -> HRESULT,
    fn ClearEffect(
        Type: cdrEffectType,
    ) -> HRESULT,
    fn RemoveRange(
        Range: *const IVGShapeRange,
    ) -> HRESULT,
    fn DeleteItem(
        Index: i32,
    ) -> HRESULT,
    fn CustomCommand(
        ComponentID: BSTR,
        CommandID: BSTR,
        Parameters: *const SAFEARRAY,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn AlignToShape(
        Type: cdrAlignType,
        Shape: *const IVGShape,
        TextAlignOrigin: cdrTextAlignOrigin,
    ) -> HRESULT,
    fn AlignToShapeRange(
        Type: cdrAlignType,
        ShapeRange: *const IVGShapeRange,
        TextAlignOrigin: cdrTextAlignOrigin,
    ) -> HRESULT,
    fn AlignToPage(
        Type: cdrAlignType,
        TextAlignOrigin: cdrTextAlignOrigin,
    ) -> HRESULT,
    fn AlignToPageCenter(
        Type: cdrAlignType,
        TextAlignOrigin: cdrTextAlignOrigin,
    ) -> HRESULT,
    fn AlignToGrid(
        Type: cdrAlignType,
        TextAlignOrigin: cdrTextAlignOrigin,
    ) -> HRESULT,
    fn AlignToPoint(
        Type: cdrAlignType,
        x: f64,
        y: f64,
        TextAlignOrigin: cdrTextAlignOrigin,
    ) -> HRESULT,
    fn Distribute(
        Type: cdrDistributeType,
        PageExtent: VARIANT_BOOL,
    ) -> HRESULT,
    fn ConvertToSymbol(
        Name: BSTR,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Ungroup(
    ) -> HRESULT,
    fn UngroupEx(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn UngroupAllEx(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn Range(
        IndexArray: *const SAFEARRAY,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn All(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn AllExcluding(
        IndexArray: *const SAFEARRAY,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn BreakApart(
    ) -> HRESULT,
    fn BreakApartEx(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn MoveToLayer(
        Layer: *const IVGLayer,
    ) -> HRESULT,
    fn CopyToLayer(
        Layer: *const IVGLayer,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn ClearTransformations(
    ) -> HRESULT,
    fn Lock(
    ) -> HRESULT,
    fn Unlock(
    ) -> HRESULT,
    fn AlignRangeToShape(
        Type: cdrAlignType,
        Shape: *const IVGShape,
    ) -> HRESULT,
    fn AlignRangeToShapeRange(
        Type: cdrAlignType,
        ShapeRange: *const IVGShapeRange,
    ) -> HRESULT,
    fn AlignRangeToPage(
        Type: cdrAlignType,
    ) -> HRESULT,
    fn AlignRangeToPageCenter(
        Type: cdrAlignType,
    ) -> HRESULT,
    fn AlignRangeToGrid(
        Type: cdrAlignType,
    ) -> HRESULT,
    fn AlignRangeToPoint(
        Type: cdrAlignType,
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn ApplyEffectInvert(
    ) -> HRESULT,
    fn ApplyEffectPosterize(
        Level: i32,
    ) -> HRESULT,
    fn ApplyEffectBCI(
        Brighness: i32,
        Contrast: i32,
        Intensity: i32,
    ) -> HRESULT,
    fn ApplyEffectColorBalance(
        CyanRed: i32,
        MagentaGreen: i32,
        YellowBlue: i32,
        ApplyToShadows: VARIANT_BOOL,
        ApplyToMidtones: VARIANT_BOOL,
        ApplyToHighlights: VARIANT_BOOL,
        PreserveLuminance: VARIANT_BOOL,
    ) -> HRESULT,
    fn ApplyEffectGamma(
        Gamma: f64,
    ) -> HRESULT,
    fn ApplyEffectHSL(
        Hue: VARIANT,
        Saturation: VARIANT,
        Lightness: VARIANT,
    ) -> HRESULT,
    fn AffineTransform(
        d11: f64,
        d12: f64,
        d21: f64,
        d22: f64,
        CenterX: f64,
        CenterY: f64,
    ) -> HRESULT,
    fn ApplyFill(
        Fill: *const IVGFill,
    ) -> HRESULT,
    fn ApplyOutline(
        Outline: *const IVGOutline,
    ) -> HRESULT,
    fn get_ReverseRange(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn Fillet(
        Radius: f64,
        CombineSmoothSegments: VARIANT_BOOL,
    ) -> HRESULT,
    fn Chamfer(
        DistanceA: f64,
        DistanceB: f64,
        CombineSmoothSegments: VARIANT_BOOL,
    ) -> HRESULT,
    fn Scallop(
        Radius: f64,
        CombineSmoothSegments: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetFillMode(
        Mode: cdrFillMode,
    ) -> HRESULT,
    fn ApplyCustomHatchFill(
        Angle: f64,
        Spacing: f64,
        Shift: f64,
        OriginX: f64,
        OriginY: f64,
        Width: f64,
        Color: *const IVGColor,
        Style: *const IVGOutlineStyle,
        DashDotLength: f64,
        PenWidth: f64,
        BackColor: *const IVGColor,
        TransformWithShape: VARIANT_BOOL,
        ScaleLinesWithShape: VARIANT_BOOL,
        UseWorldCoordinates: VARIANT_BOOL,
        FillScale: f64,
        LineScale: f64,
        FillAngle: f64,
        FillSkew: f64,
    ) -> HRESULT,
    fn ApplyHatchFill(
        LibraryName: BSTR,
        HatchNameOrIndex: VARIANT,
        BackColor: *const IVGColor,
        TransformWithShape: VARIANT_BOOL,
        ScaleLinesWithShape: VARIANT_BOOL,
        UseWorldCoordinates: VARIANT_BOOL,
        FillScale: f64,
        LineScale: f64,
        FillAngle: f64,
        FillSkew: f64,
    ) -> HRESULT,
    fn get_LeftX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_RightX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_TopY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_BottomY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Shapes(
        ppVal: *mut *mut IVGShapes,
    ) -> HRESULT,
    fn get_FirstShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_LastShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn StepAndRepeat(
        NumCopies: i32,
        DistanceX: f64,
        DistanceY: f64,
        ModeX: cdrDistanceMode,
        DirectionX: cdrDirection,
        ModeY: cdrDistanceMode,
        DirectionY: cdrDirection,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn Exists(
        Shape: *const IVGShape,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ExistsAnyOfType(
        TypeList: *const SAFEARRAY,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn CountAnyOfType(
        TypeList: *const SAFEARRAY,
        pVal: *mut i32,
    ) -> HRESULT,
    fn FindAnyOfType(
        TypeList: *const SAFEARRAY,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn GetLinkedShapes(
        LinkType: cdrShapeLinkType,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn get_BoundingBox(
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn GetPositionEx(
        ReferencePoint: cdrReferencePoint,
        x: *mut f64,
        y: *mut f64,
    ) -> HRESULT,
    fn SetPositionEx(
        ReferencePoint: cdrReferencePoint,
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn get_CenterX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_CenterX(
        pVal: f64,
    ) -> HRESULT,
    fn get_CenterY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_CenterY(
        pVal: f64,
    ) -> HRESULT,
    fn put_LeftX(
        pVal: f64,
    ) -> HRESULT,
    fn put_RightX(
        pVal: f64,
    ) -> HRESULT,
    fn put_TopY(
        pVal: f64,
    ) -> HRESULT,
    fn put_BottomY(
        pVal: f64,
    ) -> HRESULT,
    fn CopyPropertiesFrom(
        Source: *const IVGShape,
        Properties: cdrCopyProperties,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetOverprintFillState(
        pVal: *mut cdrOverprintState,
    ) -> HRESULT,
    fn GetOverprintOutlineState(
        pVal: *mut cdrOverprintState,
    ) -> HRESULT,
    fn Sort(
        CompareExpression: BSTR,
        StartIndex: i32,
        EndIndex: i32,
        Data: VARIANT,
    ) -> HRESULT,
    fn SetPixelAlignedRendering(
        PixelAligned: VARIANT_BOOL,
    ) -> HRESULT,
    fn CreateDocumentFrom(
        TemporaryDocument: VARIANT_BOOL,
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn AlignAndDistribute(
        MethodH: cdrAlignDistributeH,
        MethodV: cdrAlignDistributeV,
        AlignTo: cdrAlignShapesTo,
        DistributeArea: cdrDistributeArea,
        UseOutline: VARIANT_BOOL,
        TextAlignOrigin: cdrTextAlignOrigin,
        PointX: f64,
        PointY: f64,
        DistributeRect: *const IVGRect,
    ) -> HRESULT,
    fn SetOutlinePropertiesEx(
        Width: f64,
        Style: *const IVGOutlineStyle,
        Color: *const IVGColor,
        StartArrow: *const IVGArrowHead,
        EndArrow: *const IVGArrowHead,
        BehindFill: cdrTriState,
        ScaleWithShape: cdrTriState,
        LineCaps: cdrOutlineLineCaps,
        LineJoin: cdrOutlineLineJoin,
        NibAngle: f64,
        NibStretch: i32,
        DashDotLength: f64,
        PenWidth: f64,
        MiterLimit: f64,
        Justification: cdrOutlineJustification,
    ) -> HRESULT,
    fn CreateBoundary(
        x: f64,
        y: f64,
        PlaceOnTop: VARIANT_BOOL,
        DeleteSource: VARIANT_BOOL,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn EqualDivide(
        Divisions: i32,
        Gap: f64,
        Group: VARIANT_BOOL,
        Combine: VARIANT_BOOL,
        DeleteSource: VARIANT_BOOL,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn Project(
        Plane: cdrProjectPlane,
        ReferencePoint: cdrReferencePoint,
        ApplyToDuplicate: VARIANT_BOOL,
        ppRetVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn Unproject(
        Plane: cdrProjectPlane,
        ReferencePoint: cdrReferencePoint,
        ApplyToDuplicate: VARIANT_BOOL,
        ppRetVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn Show(
    ) -> HRESULT,
    fn Hide(
    ) -> HRESULT,
    fn GetToolShapes(
        ShapeGuid: BSTR,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn ModifyToolShapeProperties(
        ShapePropertiesToModify: *const IVGProperties,
    ) -> HRESULT,
    fn CreateParallelCurves(
        Count: i32,
        distanceBetweenCurves: f64,
        ppParallels: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn GetColorTypes(
        ppVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetColors(
        MaxBitmapColors: i32,
        ppVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn FlattenEffects(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580046, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGOutlineStyle(IVGOutlineStyleVtbl): IDispatch(IDispatchVtbl) {
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_DashCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_DashCount(
        pVal: i32,
    ) -> HRESULT,
    fn get_DashLength(
        Index: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_DashLength(
        Index: i32,
        pVal: i32,
    ) -> HRESULT,
    fn get_GapLength(
        Index: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_GapLength(
        Index: i32,
        pVal: i32,
    ) -> HRESULT,
    fn get_Enhanced(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058000d, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGArrowHead(IVGArrowHeadVtbl): IDispatch(IDispatchVtbl) {
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Curve(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_BaseOutlineScale(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_CenterX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_CenterY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_LineOffset(
        pVal: *mut f64,
    ) -> HRESULT,
    fn BindToDocument(
        Document: *const IVGDocument,
        ppVal: *mut *mut IVGArrowHead,
    ) -> HRESULT,
    fn CompareWith(
        ArrowHead: *const IVGArrowHead,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_DisplayName(
        pVal: *mut BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580038, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGFill(IVGFillVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        Type: *mut cdrFillType,
    ) -> HRESULT,
    fn get_UniformColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_UniformColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_Fountain(
        ppVal: *mut *mut IVGFountainFill,
    ) -> HRESULT,
    fn put_Fountain(
        ppVal: *const IVGFountainFill,
    ) -> HRESULT,
    fn get_Pattern(
        ppVal: *mut *mut IVGPatternFill,
    ) -> HRESULT,
    fn put_Pattern(
        ppVal: *const IVGPatternFill,
    ) -> HRESULT,
    fn get_Texture(
        ppVal: *mut *mut IVGTextureFill,
    ) -> HRESULT,
    fn put_Texture(
        ppVal: *const IVGTextureFill,
    ) -> HRESULT,
    fn get_PostScript(
        ppVal: *mut *mut IVGPostScriptFill,
    ) -> HRESULT,
    fn put_PostScript(
        ppVal: *const IVGPostScriptFill,
    ) -> HRESULT,
    fn ApplyNoFill(
    ) -> HRESULT,
    fn ApplyUniformFill(
        Color: *const IVGColor,
    ) -> HRESULT,
    fn ApplyFountainFill(
        StartColor: *const IVGColor,
        EndColor: *const IVGColor,
        Type: cdrFountainFillType,
        Angle: f64,
        Steps: i32,
        EdgePad: i32,
        MidPoint: i32,
        BlendType: cdrFountainFillBlendType,
        CenterOffsetX: f64,
        CenterOffsetY: f64,
        ppVal: *mut *mut IVGFountainFill,
    ) -> HRESULT,
    fn ApplyPatternFill(
        Type: cdrPatternFillType,
        FileName: BSTR,
        PatternCanvasIndex: i32,
        FrontColor: *const IVGColor,
        EndColor: *const IVGColor,
        TransformWithShape: VARIANT_BOOL,
        ppVal: *mut *mut IVGPatternFill,
    ) -> HRESULT,
    fn ApplyTextureFill(
        TextureName: BSTR,
        LibraryName: BSTR,
        ppVal: *mut *mut IVGTextureFill,
    ) -> HRESULT,
    fn ApplyPostscriptFill(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGPostScriptFill,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGFill,
    ) -> HRESULT,
    fn CopyAssign(
        SourceFill: *const IVGFill,
    ) -> HRESULT,
    fn UserAssign(
        FillType: cdrFillType,
        PatternType: cdrPatternFillType,
        ParentWindowHandle: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PSScreen(
        ppVal: *mut *mut IVGPSScreenOptions,
    ) -> HRESULT,
    fn get_Hatch(
        ppVal: *mut *mut IVGHatchFill,
    ) -> HRESULT,
    fn put_Hatch(
        ppVal: *const IVGHatchFill,
    ) -> HRESULT,
    fn CompareWith(
        Fill: *const IVGFill,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ApplyCustomHatchFill(
        Angle: f64,
        Spacing: f64,
        Shift: f64,
        OriginX: f64,
        OriginY: f64,
        Width: f64,
        Color: *const IVGColor,
        Style: *const IVGOutlineStyle,
        DashDotLength: f64,
        PenWidth: f64,
        BackColor: *const IVGColor,
        TransformWithShape: VARIANT_BOOL,
        ScaleLinesWithShape: VARIANT_BOOL,
        UseWorldCoordinates: VARIANT_BOOL,
        FillScale: f64,
        LineScale: f64,
        FillAngle: f64,
        FillSkew: f64,
        ppVal: *mut *mut IVGHatchFill,
    ) -> HRESULT,
    fn ApplyHatchFill(
        LibraryName: BSTR,
        HatchNameOrIndex: VARIANT,
        BackColor: *const IVGColor,
        TransformWithShape: VARIANT_BOOL,
        ScaleLinesWithShape: VARIANT_BOOL,
        UseWorldCoordinates: VARIANT_BOOL,
        FillScale: f64,
        LineScale: f64,
        FillAngle: f64,
        FillSkew: f64,
        ppVal: *mut *mut IVGHatchFill,
    ) -> HRESULT,
    fn ToString(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn StringAssign(
        FillString: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058003c, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGFountainFill(IVGFountainFillVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrFountainFillType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrFountainFillType,
    ) -> HRESULT,
    fn get_StartX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_StartX(
        pVal: f64,
    ) -> HRESULT,
    fn get_StartY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_StartY(
        pVal: f64,
    ) -> HRESULT,
    fn get_EndX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_EndX(
        pVal: f64,
    ) -> HRESULT,
    fn get_EndY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_EndY(
        pVal: f64,
    ) -> HRESULT,
    fn get_Angle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn SetAngle(
        Angle: f64,
    ) -> HRESULT,
    fn Translate(
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn get_EdgePad(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Steps(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Steps(
        pVal: i32,
    ) -> HRESULT,
    fn get_BlendType(
        pVal: *mut cdrFountainFillBlendType,
    ) -> HRESULT,
    fn put_BlendType(
        pVal: cdrFountainFillBlendType,
    ) -> HRESULT,
    fn get_MidPoint(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_MidPoint(
        pVal: i32,
    ) -> HRESULT,
    fn get_Colors(
        ppVal: *mut *mut IVGFountainColors,
    ) -> HRESULT,
    fn get_StartColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_StartColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_EndColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_EndColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn put_Colors(
        ppVal: *const IVGFountainColors,
    ) -> HRESULT,
    fn SetEdgePad(
        EdgePad: i32,
    ) -> HRESULT,
    fn get_CenterOffsetX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_CenterOffsetX(
        pVal: f64,
    ) -> HRESULT,
    fn get_CenterOffsetY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_CenterOffsetY(
        pVal: f64,
    ) -> HRESULT,
    fn get_SmoothBlend(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_SmoothBlend(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_SpreadMethod(
        pVal: *mut cdrFountainFillSpreadMethod,
    ) -> HRESULT,
    fn put_SpreadMethod(
        pVal: cdrFountainFillSpreadMethod,
    ) -> HRESULT,
    fn get_Anisotropic(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Anisotropic(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsTransparent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MergeMode(
        pVal: *mut cdrMergeMode,
    ) -> HRESULT,
    fn put_MergeMode(
        pVal: cdrMergeMode,
    ) -> HRESULT,
    fn get_ScaleX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_ScaleX(
        pVal: f64,
    ) -> HRESULT,
    fn get_ScaleY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_ScaleY(
        pVal: f64,
    ) -> HRESULT,
    fn get_Skew(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Skew(
        pVal: f64,
    ) -> HRESULT,
    fn get_BlendAcceleration(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_BlendAcceleration(
        pVal: f64,
    ) -> HRESULT,
    fn MakeOpaque(
    ) -> HRESULT,
    fn GetTransformations(
        d11: *mut f64,
        d12: *mut f64,
        d21: *mut f64,
        d22: *mut f64,
    ) -> HRESULT,
    fn SetTransformations(
        d11: f64,
        d12: f64,
        d21: f64,
        d22: f64,
    ) -> HRESULT,
    fn get_HasHSBBlends(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_HasNonLinearBlends(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_End2X(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_End2X(
        pVal: f64,
    ) -> HRESULT,
    fn get_End2Y(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_End2Y(
        pVal: f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058003b, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGFountainColors(IVGFountainColorsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGFountainColor,
    ) -> HRESULT,
    fn put_Item(
        Index: i32,
        ppVal: *const IVGFountainColor,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Add(
        Color: *const IVGColor,
        Position: i32,
    ) -> HRESULT,
    fn get_GrayLevel(
        Index: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_GrayLevel(
        Index: i32,
        pVal: i32,
    ) -> HRESULT,
    fn AddGrayLevel(
        GrayLevel: i32,
        Position: i32,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGFountainColor,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGFountainColor,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058003a, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGFountainColor(IVGFountainColorVtbl): IDispatch(IDispatchVtbl) {
    fn get_Position(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Color(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_Color(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn Move(
        NewPosition: i32,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn get_MidPoint(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_MidPoint(
        pVal: i32,
    ) -> HRESULT,
    fn get_BlendType(
        pVal: *mut cdrFountainFillBlendType,
    ) -> HRESULT,
    fn put_BlendType(
        pVal: cdrFountainFillBlendType,
    ) -> HRESULT,
    fn get_Opacity(
        pVal: *mut u8,
    ) -> HRESULT,
    fn put_Opacity(
        pVal: u8,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580050, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPatternFill(IVGPatternFillVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrPatternFillType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrPatternFillType,
    ) -> HRESULT,
    fn get_FrontColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_FrontColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_BackColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_BackColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_Canvas(
        ppVal: *mut *mut IVGPatternCanvas,
    ) -> HRESULT,
    fn put_Canvas(
        ppVal: *const IVGPatternCanvas,
    ) -> HRESULT,
    fn get_FilePath(
        FileName: *mut BSTR,
    ) -> HRESULT,
    fn get_OriginX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OriginX(
        pVal: f64,
    ) -> HRESULT,
    fn get_OriginY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OriginY(
        pVal: f64,
    ) -> HRESULT,
    fn get_TileWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TileWidth(
        pVal: f64,
    ) -> HRESULT,
    fn get_TileHeight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TileHeight(
        pVal: f64,
    ) -> HRESULT,
    fn get_TileOffsetType(
        pVal: *mut cdrTileOffsetType,
    ) -> HRESULT,
    fn put_TileOffsetType(
        pVal: cdrTileOffsetType,
    ) -> HRESULT,
    fn get_TileOffset(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_TileOffset(
        pVal: i32,
    ) -> HRESULT,
    fn get_SkewAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_SkewAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_RotationAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RotationAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_TransformWithShape(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TransformWithShape(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn Load(
        FileName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MirrorFill(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MirrorFill(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MirrorFillX(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MirrorFillX(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MirrorFillY(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MirrorFillY(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058004e, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPatternCanvas(IVGPatternCanvasVtbl): IDispatch(IDispatchVtbl) {
    fn get_Size(
        pVal: *mut cdrPatternCanvasSize,
    ) -> HRESULT,
    fn put_Size(
        pVal: cdrPatternCanvasSize,
    ) -> HRESULT,
    fn get_Pixel(
        x: i32,
        y: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Pixel(
        x: i32,
        y: i32,
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn FillArea(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        State: VARIANT_BOOL,
    ) -> HRESULT,
    fn CopyArea(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x: i32,
        y: i32,
    ) -> HRESULT,
    fn FlipArea(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        Axes: cdrFlipAxes,
    ) -> HRESULT,
    fn RotateArea(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        Angle: f64,
    ) -> HRESULT,
    fn Select(
        Index: i32,
    ) -> HRESULT,
    fn Clear(
    ) -> HRESULT,
    fn PutCopy(
        PatternCanvas: *const IVGPatternCanvas,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Width(
        pVal: i32,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Height(
        pVal: i32,
    ) -> HRESULT,
    fn get_Data(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Data(
        pVal: BSTR,
    ) -> HRESULT,
    fn PSet(
        Step: i16,
        x: i32,
        y: i32,
        Color: VARIANT_BOOL,
    ) -> HRESULT,
    fn Line(
        Flags: i16,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        Color: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580079, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextureFill(IVGTextureFillVtbl): IDispatch(IDispatchVtbl) {
    fn get_OriginX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OriginX(
        pVal: f64,
    ) -> HRESULT,
    fn get_OriginY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OriginY(
        pVal: f64,
    ) -> HRESULT,
    fn get_TileWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TileWidth(
        pVal: f64,
    ) -> HRESULT,
    fn get_TileHeight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TileHeight(
        pVal: f64,
    ) -> HRESULT,
    fn get_TileOffsetType(
        pVal: *mut cdrTileOffsetType,
    ) -> HRESULT,
    fn put_TileOffsetType(
        pVal: cdrTileOffsetType,
    ) -> HRESULT,
    fn get_TileOffset(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_TileOffset(
        pVal: i32,
    ) -> HRESULT,
    fn get_SkewAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_SkewAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_RotationAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RotationAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_TransformWithShape(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TransformWithShape(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Resolution(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Resolution(
        pVal: i32,
    ) -> HRESULT,
    fn get_MaximumTileWidth(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_MaximumTileWidth(
        pVal: i32,
    ) -> HRESULT,
    fn get_LibraryName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_TextureName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_StyleName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn Select(
        Texture: BSTR,
        Library: BSTR,
    ) -> HRESULT,
    fn SetProperties(
        SettingArray: *const SAFEARRAY,
    ) -> HRESULT,
    fn get_MirrorFill(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MirrorFill(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Properties(
        ppVal: *mut *mut IVGTextureFillProperties,
    ) -> HRESULT,
    fn get_MirrorFillX(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MirrorFillX(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MirrorFillY(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MirrorFillY(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058007a, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextureFillProperties(IVGTextureFillPropertiesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGTextureFillProperty,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058007b, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextureFillProperty(IVGTextureFillPropertyVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrTexturePropertyType,
    ) -> HRESULT,
    fn get_Value(
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn put_Value(
        pVal: VARIANT,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580052, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPostScriptFill(IVGPostScriptFillVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Properties(
        Index: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Properties(
        Index: i32,
        pVal: i32,
    ) -> HRESULT,
    fn Select(
        IndexOrName: VARIANT,
    ) -> HRESULT,
    fn SetProperties(
        Param1: i32,
        Param2: i32,
        Param3: i32,
        Param4: i32,
        Param5: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580096, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPSScreenOptions(IVGPSScreenOptionsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Angle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Angle(
        pVal: f64,
    ) -> HRESULT,
    fn get_Frequency(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Frequency(
        pVal: i32,
    ) -> HRESULT,
    fn Select(
        Name: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn SetProperties(
        IndexOrName: VARIANT,
        Angle: f64,
        Frequency: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn NameByIndex(
        Index: i32,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
    fn UserAssign(
        ParentWindowHandle: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058009a, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGHatchFill(IVGHatchFillVtbl): IDispatch(IDispatchVtbl) {
    fn get_BackColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_BackColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_HasBackground(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TransformWithShape(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TransformWithShape(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ScaleLinesWithShape(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ScaleLinesWithShape(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_UseWorldCoordinates(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseWorldCoordinates(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_LibraryName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_HatchName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Patterns(
        ppVal: *mut *mut IVGHatchPatterns,
    ) -> HRESULT,
    fn get_FillScaleX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_FillScaleX(
        pVal: f64,
    ) -> HRESULT,
    fn get_FillScaleY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_FillScaleY(
        pVal: f64,
    ) -> HRESULT,
    fn get_RotationAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RotationAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_SkewAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_SkewAngle(
        pVal: f64,
    ) -> HRESULT,
    fn SetNoBackColor(
    ) -> HRESULT,
    fn SetFillScale(
        FillScale: f64,
    ) -> HRESULT,
    fn GetFillScale(
        pVal: *mut f64,
    ) -> HRESULT,
    fn SetLineScale(
        LineScale: f64,
    ) -> HRESULT,
    fn GetLineScale(
        pVal: *mut f64,
    ) -> HRESULT,
    fn AddPattern(
        Angle: f64,
        Spacing: f64,
        Shift: f64,
        OriginX: f64,
        OriginY: f64,
        Width: f64,
        Color: *const IVGColor,
        Style: *const IVGOutlineStyle,
        DashDotLength: f64,
        PenWidth: f64,
        ppVal: *mut *mut IVGHatchPattern,
    ) -> HRESULT,
    fn AddToLibrary(
        LibraryName: BSTR,
        HatchName: BSTR,
    ) -> HRESULT,
    fn Select(
        LibraryName: BSTR,
        HatchNameOrIndex: VARIANT,
        BackColor: *const IVGColor,
        TransformWithShape: VARIANT_BOOL,
        ScaleLinesWithShape: VARIANT_BOOL,
        UseWorldCoordinates: VARIANT_BOOL,
        FillScale: f64,
        LineScale: f64,
        FillAngle: f64,
        FillSkew: f64,
    ) -> HRESULT,
    fn get_IsFromLibrary(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Library(
        ppVal: *mut *mut IVGHatchLibrary,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058009b, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGHatchPatterns(IVGHatchPatternsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGHatchPattern,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058009c, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGHatchPattern(IVGHatchPatternVtbl): IDispatch(IDispatchVtbl) {
    fn get_Angle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Angle(
        pVal: f64,
    ) -> HRESULT,
    fn get_OriginX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OriginX(
        pVal: f64,
    ) -> HRESULT,
    fn get_OriginY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OriginY(
        pVal: f64,
    ) -> HRESULT,
    fn get_Spacing(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Spacing(
        pVal: f64,
    ) -> HRESULT,
    fn get_Shift(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Shift(
        pVal: f64,
    ) -> HRESULT,
    fn get_ShiftPercent(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_ShiftPercent(
        pVal: f64,
    ) -> HRESULT,
    fn get_Outline(
        ppVal: *mut *mut IVGOutline,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn SetOrigin(
        OriginX: f64,
        OriginY: f64,
    ) -> HRESULT,
    fn GetOrigin(
        OriginX: *mut f64,
        OriginY: *mut f64,
    ) -> HRESULT,
    fn SetProperties(
        Angle: VARIANT,
        Spacing: f64,
        Shift: VARIANT,
        OriginX: VARIANT,
        OriginY: VARIANT,
        Width: f64,
        Color: *const IVGColor,
        Style: *const IVGOutlineStyle,
        DashDotLength: f64,
        PenWidth: f64,
    ) -> HRESULT,
    fn Reset(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580045, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGOutline(IVGOutlineVtbl): IDispatch(IDispatchVtbl) {
    fn get_Width(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Width(
        pVal: f64,
    ) -> HRESULT,
    fn get_Color(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_Color(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn ConvertToObject(
        ppRetVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Type(
        Type: *mut cdrOutlineType,
    ) -> HRESULT,
    fn put_Type(
        Type: cdrOutlineType,
    ) -> HRESULT,
    fn get_Style(
        ppStyle: *mut *mut IVGOutlineStyle,
    ) -> HRESULT,
    fn put_Style(
        ppStyle: *const IVGOutlineStyle,
    ) -> HRESULT,
    fn get_StartArrow(
        ppArrowHead: *mut *mut IVGArrowHead,
    ) -> HRESULT,
    fn put_StartArrow(
        ppArrowHead: *const IVGArrowHead,
    ) -> HRESULT,
    fn get_EndArrow(
        ppArrowHead: *mut *mut IVGArrowHead,
    ) -> HRESULT,
    fn put_EndArrow(
        ppArrowHead: *const IVGArrowHead,
    ) -> HRESULT,
    fn get_NibStretch(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_NibStretch(
        pVal: i32,
    ) -> HRESULT,
    fn get_NibAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_NibAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_BehindFill(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_BehindFill(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_LineCaps(
        pVal: *mut cdrOutlineLineCaps,
    ) -> HRESULT,
    fn put_LineCaps(
        pVal: cdrOutlineLineCaps,
    ) -> HRESULT,
    fn get_LineJoin(
        pVal: *mut cdrOutlineLineJoin,
    ) -> HRESULT,
    fn put_LineJoin(
        pVal: cdrOutlineLineJoin,
    ) -> HRESULT,
    fn get_ScaleWithShape(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ScaleWithShape(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetProperties(
        Width: f64,
        Style: *const IVGOutlineStyle,
        Color: *const IVGColor,
        StartArrow: *const IVGArrowHead,
        EndArrow: *const IVGArrowHead,
        BehindFill: cdrTriState,
        ScaleWithShape: cdrTriState,
        LineCaps: cdrOutlineLineCaps,
        LineJoin: cdrOutlineLineJoin,
        NibAngle: f64,
        NibStretch: i32,
        DashDotLength: f64,
        PenWidth: f64,
        MiterLimit: f64,
    ) -> HRESULT,
    fn get_Size(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Size(
        pVal: f64,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGOutline,
    ) -> HRESULT,
    fn CopyAssign(
        SourceOutline: *const IVGOutline,
    ) -> HRESULT,
    fn UserAssign(
        ParentWindowHandle: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PSScreen(
        ppVal: *mut *mut IVGPSScreenOptions,
    ) -> HRESULT,
    fn CompareWith(
        Outline: *const IVGOutline,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DashDotLength(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_DashDotLength(
        pVal: f64,
    ) -> HRESULT,
    fn ToString(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn StringAssign(
        OutlineString: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PenWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PenWidth(
        pVal: f64,
    ) -> HRESULT,
    fn get_MiterLimit(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_MiterLimit(
        pVal: f64,
    ) -> HRESULT,
    fn SetNoOutline(
    ) -> HRESULT,
    fn get_StartArrowOptions(
        ppVal: *mut *mut IVGArrowHeadOptions,
    ) -> HRESULT,
    fn put_StartArrowOptions(
        ppVal: *const IVGArrowHeadOptions,
    ) -> HRESULT,
    fn get_EndArrowOptions(
        ppVal: *mut *mut IVGArrowHeadOptions,
    ) -> HRESULT,
    fn put_EndArrowOptions(
        ppVal: *const IVGArrowHeadOptions,
    ) -> HRESULT,
    fn get_Justification(
        pVal: *mut cdrOutlineJustification,
    ) -> HRESULT,
    fn put_Justification(
        pVal: cdrOutlineJustification,
    ) -> HRESULT,
    fn SetPropertiesEx(
        Width: f64,
        Style: *const IVGOutlineStyle,
        Color: *const IVGColor,
        StartArrow: *const IVGArrowHead,
        EndArrow: *const IVGArrowHead,
        BehindFill: cdrTriState,
        ScaleWithShape: cdrTriState,
        LineCaps: cdrOutlineLineCaps,
        LineJoin: cdrOutlineLineJoin,
        NibAngle: f64,
        NibStretch: i32,
        DashDotLength: f64,
        PenWidth: f64,
        MiterLimit: f64,
        Justification: cdrOutlineJustification,
    ) -> HRESULT,
    fn get_AdjustDashes(
        pVal: *mut cdrOutlineDashAdjust,
    ) -> HRESULT,
    fn put_AdjustDashes(
        pVal: cdrOutlineDashAdjust,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800a7, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGArrowHeadOptions(IVGArrowHeadOptionsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Length(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Length(
        pVal: f64,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Width(
        pVal: f64,
    ) -> HRESULT,
    fn get_OffsetX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OffsetX(
        pVal: f64,
    ) -> HRESULT,
    fn get_OffsetY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OffsetY(
        pVal: f64,
    ) -> HRESULT,
    fn get_FlipHorizontal(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FlipHorizontal(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FlipVertical(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FlipVertical(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn Flip(
        Axes: cdrFlipAxes,
    ) -> HRESULT,
    fn get_RotationAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RotationAngle(
        pVal: f64,
    ) -> HRESULT,
    fn CopyAssign(
        Source: *const IVGArrowHeadOptions,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGArrowHeadOptions,
    ) -> HRESULT,
    fn get_FlipVerical(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FlipVerical(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058009e, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGHatchLibrary(IVGHatchLibraryVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Fills(
        ppVal: *mut *mut IVGHatchFills,
    ) -> HRESULT,
    fn get_Active(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Activate(
    ) -> HRESULT,
    fn get_DisplayName(
        pVal: *mut BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058009f, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGHatchFills(IVGHatchFillsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGFill,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Find(
        Name: BSTR,
        ppVal: *mut *mut IVGFill,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580054, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGProperties(IVGPropertiesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        Name: BSTR,
        ID: i32,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn put_Item(
        Name: BSTR,
        ID: i32,
        pVal: VARIANT,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Index(
        Name: BSTR,
        ID: i32,
        pIndex: *mut i32,
    ) -> HRESULT,
    fn get_ItemByIndex(
        Index: i32,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn Delete(
        Name: BSTR,
        ID: i32,
    ) -> HRESULT,
    fn DeleteByIndex(
        Index: i32,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Description(
        Index: i32,
        Name: *mut BSTR,
        ID: *mut i32,
    ) -> HRESULT,
    fn PutFile(
        Name: BSTR,
        ID: i32,
        FileName: BSTR,
    ) -> HRESULT,
    fn GetFile(
        Name: BSTR,
        ID: i32,
        FileName: BSTR,
    ) -> HRESULT,
    fn Exists(
        Name: BSTR,
        ID: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn DeleteAll(
        Name: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetPoint(
        uuidName: BSTR,
        ppRet: *mut *mut IVGPoint,
    ) -> HRESULT,
    fn SetPoint(
        uuidName: BSTR,
        pVal: *const IVGPoint,
    ) -> HRESULT,
    fn GetVector(
        uuidName: BSTR,
        ppRet: *mut *mut IVGVector,
    ) -> HRESULT,
    fn SetVector(
        uuidName: BSTR,
        pVal: *const IVGVector,
    ) -> HRESULT,
    fn GetCurve(
        uuidName: BSTR,
        ppRet: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn SetCurve(
        uuidName: BSTR,
        pVal: *const IVGCurve,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800d8, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGImage(IVGImageVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrImageType,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Pixel(
        x: i32,
        y: i32,
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGImage,
    ) -> HRESULT,
    fn get_ReadOnly(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Tiles(
        ppVal: *mut *mut IVGImageTiles,
    ) -> HRESULT,
    fn Blit(
        DestX: i32,
        DestY: i32,
        DestWidth: i32,
        DestHeight: i32,
        SrcImage: *const IVGImage,
        SrcX: i32,
        SrcY: i32,
        SrcWidth: i32,
        SrcHeight: i32,
        MergeMode: cdrMergeMode,
    ) -> HRESULT,
    fn FillArea(
        x: i32,
        y: i32,
        Width: i32,
        Height: i32,
        Color: *const IVGColor,
    ) -> HRESULT,
    fn FlipArea(
        x: i32,
        y: i32,
        Width: i32,
        Height: i32,
        Axes: cdrFlipAxes,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800d9, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGImageTiles(IVGImageTilesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGImageTile,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGImageTile,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGImageTile,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800da, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGImageTile(IVGImageTileVtbl): IDispatch(IDispatchVtbl) {
    fn get_Left(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Top(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Right(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Bottom(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_BytesPerTile(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_BytesPerLine(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_BytesPerPixel(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_PixelData(
        pVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn put_PixelData(
        pVal: *const SAFEARRAY,
    ) -> HRESULT,
    fn get_ReadOnly(
        ppVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580071, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGText(IVGTextVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrTextType,
    ) -> HRESULT,
    fn get_FramesInLink(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_UnusedFramesInLink(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Overflow(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FontProperties(
        Frames: cdrTextFrames,
        ppVal: *mut *mut IVGStructFontProperties,
    ) -> HRESULT,
    fn put_FontProperties(
        Frames: cdrTextFrames,
        ppVal: *const IVGStructFontProperties,
    ) -> HRESULT,
    fn get_FontPropertiesInRange(
        StartIndex: i32,
        Count: i32,
        IndexingType: cdrTextIndexingType,
        ppVal: *mut *mut IVGStructFontProperties,
    ) -> HRESULT,
    fn put_FontPropertiesInRange(
        StartIndex: i32,
        Count: i32,
        IndexingType: cdrTextIndexingType,
        ppVal: *const IVGStructFontProperties,
    ) -> HRESULT,
    fn get_AlignProperties(
        Frames: cdrTextFrames,
        ppVal: *mut *mut IVGStructAlignProperties,
    ) -> HRESULT,
    fn put_AlignProperties(
        Frames: cdrTextFrames,
        ppVal: *const IVGStructAlignProperties,
    ) -> HRESULT,
    fn get_AlignPropertiesInRange(
        StartIndex: i32,
        Count: i32,
        IndexingType: cdrTextIndexingType,
        ppVal: *mut *mut IVGStructAlignProperties,
    ) -> HRESULT,
    fn put_AlignPropertiesInRange(
        StartIndex: i32,
        Count: i32,
        IndexingType: cdrTextIndexingType,
        ppVal: *const IVGStructAlignProperties,
    ) -> HRESULT,
    fn get_SpaceProperties(
        Frames: cdrTextFrames,
        ppVal: *mut *mut IVGStructSpaceProperties,
    ) -> HRESULT,
    fn put_SpaceProperties(
        Frames: cdrTextFrames,
        ppVal: *const IVGStructSpaceProperties,
    ) -> HRESULT,
    fn get_SpacePropertiesInRange(
        StartIndex: i32,
        Count: i32,
        IndexingType: cdrTextIndexingType,
        ppVal: *mut *mut IVGStructSpaceProperties,
    ) -> HRESULT,
    fn put_SpacePropertiesInRange(
        StartIndex: i32,
        Count: i32,
        IndexingType: cdrTextIndexingType,
        ppVal: *const IVGStructSpaceProperties,
    ) -> HRESULT,
    fn get_HyphenationSettings(
        Frames: cdrTextFrames,
        ppVal: *mut *mut IVGStructHyphenationSettings,
    ) -> HRESULT,
    fn put_HyphenationSettings(
        Frames: cdrTextFrames,
        ppVal: *const IVGStructHyphenationSettings,
    ) -> HRESULT,
    fn get_HyphenationSettingsInRange(
        StartIndex: i32,
        Count: i32,
        IndexingType: cdrTextIndexingType,
        ppVal: *mut *mut IVGStructHyphenationSettings,
    ) -> HRESULT,
    fn put_HyphenationSettingsInRange(
        StartIndex: i32,
        Count: i32,
        IndexingType: cdrTextIndexingType,
        ppVal: *const IVGStructHyphenationSettings,
    ) -> HRESULT,
    fn get_Contents(
        Frames: cdrTextFrames,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Contents(
        Frames: cdrTextFrames,
        pVal: BSTR,
    ) -> HRESULT,
    fn FitToPath(
        Path: *const IVGShape,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn Find(
        Text: BSTR,
        CaseSensitive: VARIANT_BOOL,
        StartIndex: i32,
        WrapAround: VARIANT_BOOL,
        IndexingType: cdrTextIndexingType,
        IndexFound: *mut i32,
    ) -> HRESULT,
    fn Replace(
        OldText: BSTR,
        NewText: BSTR,
        CaseSensitive: VARIANT_BOOL,
        StartIndex: i32,
        ReplaceAll: VARIANT_BOOL,
        WrapAround: VARIANT_BOOL,
        IndexingType: cdrTextIndexingType,
    ) -> HRESULT,
    fn ImportFromFile(
        FileName: BSTR,
        StartIndex: i32,
        IndexingType: cdrTextIndexingType,
    ) -> HRESULT,
    fn ExportToFile(
        FileName: BSTR,
        StartIndex: i32,
        Count: i32,
        IndexingType: cdrTextIndexingType,
    ) -> HRESULT,
    fn ConvertToArtistic(
    ) -> HRESULT,
    fn ConvertToParagraph(
    ) -> HRESULT,
    fn get_IsHTMLCompatible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn MakeHTMLCompatible(
        HTML: VARIANT_BOOL,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Story(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Selection(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn Range(
        Start: i32,
        End: i32,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_IsEditing(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn BeginEdit(
    ) -> HRESULT,
    fn get_Frame(
        ppVal: *mut *mut IVGTextFrame,
    ) -> HRESULT,
    fn get_Frames(
        ppVal: *mut *mut IVGTextFrames,
    ) -> HRESULT,
    fn get_IsArtisticText(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn FitTextToFrame(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580064, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStructFontProperties(IVGStructFontPropertiesVtbl): IDispatch(IDispatchVtbl) {
    fn put_Name(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Style(
        pVal: cdrFontStyle,
    ) -> HRESULT,
    fn get_Style(
        pVal: *mut cdrFontStyle,
    ) -> HRESULT,
    fn put_Size(
        pVal: f32,
    ) -> HRESULT,
    fn get_Size(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_Underline(
        pVal: cdrFontLine,
    ) -> HRESULT,
    fn get_Underline(
        pVal: *mut cdrFontLine,
    ) -> HRESULT,
    fn put_Overscore(
        pVal: cdrFontLine,
    ) -> HRESULT,
    fn get_Overscore(
        pVal: *mut cdrFontLine,
    ) -> HRESULT,
    fn put_Strikethru(
        pVal: cdrFontLine,
    ) -> HRESULT,
    fn get_Strikethru(
        pVal: *mut cdrFontLine,
    ) -> HRESULT,
    fn put_Uppercase(
        pVal: cdrFontCase,
    ) -> HRESULT,
    fn get_Uppercase(
        pVal: *mut cdrFontCase,
    ) -> HRESULT,
    fn put_Position(
        pVal: cdrFontPosition,
    ) -> HRESULT,
    fn get_Position(
        pVal: *mut cdrFontPosition,
    ) -> HRESULT,
    fn put_RangeKerning(
        pVal: i32,
    ) -> HRESULT,
    fn get_RangeKerning(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Fill(
        ppVal: *const IVGFill,
    ) -> HRESULT,
    fn get_Fill(
        ppVal: *mut *mut IVGFill,
    ) -> HRESULT,
    fn put_Outline(
        ppVal: *const IVGOutline,
    ) -> HRESULT,
    fn get_Outline(
        ppVal: *mut *mut IVGOutline,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580062, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStructAlignProperties(IVGStructAlignPropertiesVtbl): IDispatch(IDispatchVtbl) {
    fn put_Alignment(
        pVal: cdrAlignment,
    ) -> HRESULT,
    fn get_Alignment(
        pVal: *mut cdrAlignment,
    ) -> HRESULT,
    fn put_FirstLineIndent(
        pVal: f64,
    ) -> HRESULT,
    fn get_FirstLineIndent(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_LeftIndent(
        pVal: f64,
    ) -> HRESULT,
    fn get_LeftIndent(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RightIndent(
        pVal: f64,
    ) -> HRESULT,
    fn get_RightIndent(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_MaxWordSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_MaxWordSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_MinWordSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_MinWordSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_MaxCharacterSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_MaxCharacterSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_HorizontalCharacterShift(
        pVal: i32,
    ) -> HRESULT,
    fn get_HorizontalCharacterShift(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_VerticalCharacterShift(
        pVal: i32,
    ) -> HRESULT,
    fn get_VerticalCharacterShift(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CharacterRotation(
        pVal: f32,
    ) -> HRESULT,
    fn get_CharacterRotation(
        pVal: *mut f32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580069, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStructSpaceProperties(IVGStructSpacePropertiesVtbl): IDispatch(IDispatchVtbl) {
    fn put_CharacterSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_CharacterSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_WordSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_WordSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_LineSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_LineSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_LineSpacingType(
        pVal: cdrLineSpacingType,
    ) -> HRESULT,
    fn get_LineSpacingType(
        pVal: *mut cdrLineSpacingType,
    ) -> HRESULT,
    fn put_BeforeParagraphSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_BeforeParagraphSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_AfterParagraphSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_AfterParagraphSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580066, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStructHyphenationSettings(IVGStructHyphenationSettingsVtbl): IDispatch(IDispatchVtbl) {
    fn put_UseAutomaticHyphenation(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_UseAutomaticHyphenation(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_BreakCapitalized(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BreakCapitalized(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_HotZone(
        pVal: f64,
    ) -> HRESULT,
    fn get_HotZone(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_MinWordLength(
        pVal: i32,
    ) -> HRESULT,
    fn get_MinWordLength(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_MinCharactersBefore(
        pVal: i32,
    ) -> HRESULT,
    fn get_MinCharactersBefore(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_MinCharactersAfter(
        pVal: i32,
    ) -> HRESULT,
    fn get_MinCharactersAfter(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_BreakAllCapWords(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BreakAllCapWords(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580026, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffect(IVGEffectVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffects,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrEffectType,
    ) -> HRESULT,
    fn get_Blend(
        ppVal: *mut *mut IVGEffectBlend,
    ) -> HRESULT,
    fn get_ControlPath(
        ppVal: *mut *mut IVGEffectControlPath,
    ) -> HRESULT,
    fn get_Extrude(
        ppVal: *mut *mut IVGEffectExtrude,
    ) -> HRESULT,
    fn get_Envelope(
        pVal: *mut *mut IVGEffectEnvelope,
    ) -> HRESULT,
    fn get_TextOnPath(
        ppVal: *mut *mut IVGEffectTextOnPath,
    ) -> HRESULT,
    fn get_DropShadow(
        ppVal: *mut *mut IVGEffectDropShadow,
    ) -> HRESULT,
    fn get_Contour(
        ppVal: *mut *mut IVGEffectContour,
    ) -> HRESULT,
    fn get_Distortion(
        ppVal: *mut *mut IVGEffectDistortion,
    ) -> HRESULT,
    fn get_Lens(
        ppVal: *mut *mut IVGEffectLens,
    ) -> HRESULT,
    fn get_Perspective(
        ppVal: *mut *mut IVGEffectPerspective,
    ) -> HRESULT,
    fn get_Clones(
        ppVal: *mut *mut IVGEffects,
    ) -> HRESULT,
    fn get_CloneParent(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn Clear(
    ) -> HRESULT,
    fn Separate(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn get_Custom(
        ppVal: *mut *mut IVGCustomEffect,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580036, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffects(IVGEffectsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_BlendEffects(
        ppVal: *mut *mut IVGEffects,
    ) -> HRESULT,
    fn get_CustomEffects(
        ppVal: *mut *mut IVGEffects,
    ) -> HRESULT,
    fn get_DistortionEffects(
        ppVal: *mut *mut IVGEffects,
    ) -> HRESULT,
    fn get_EnvelopeEffects(
        ppVal: *mut *mut IVGEffects,
    ) -> HRESULT,
    fn get_PerspectiveEffects(
        ppVal: *mut *mut IVGEffects,
    ) -> HRESULT,
    fn get_ContourEffect(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_ControlPathEffect(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_DropShadowEffect(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_ExtrudeEffect(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_LensEffect(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_TextOnPathEffect(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580027, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectBlend(IVGEffectBlendVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_StartShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn put_StartShape(
        ppVal: *const IVGShape,
    ) -> HRESULT,
    fn get_EndShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn put_EndShape(
        ppVal: *const IVGShape,
    ) -> HRESULT,
    fn get_BlendGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Path(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn put_Path(
        ppVal: *const IVGShape,
    ) -> HRESULT,
    fn get_StartShapeOffset(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_StartShapeOffset(
        pVal: f64,
    ) -> HRESULT,
    fn get_EndShapeOffset(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_EndShapeOffset(
        pVal: f64,
    ) -> HRESULT,
    fn get_Mode(
        pVal: *mut cdrBlendMode,
    ) -> HRESULT,
    fn put_Mode(
        pVal: cdrBlendMode,
    ) -> HRESULT,
    fn get_Steps(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Steps(
        pVal: i32,
    ) -> HRESULT,
    fn get_Spacing(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Spacing(
        pVal: f64,
    ) -> HRESULT,
    fn get_Angle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Angle(
        pVal: f64,
    ) -> HRESULT,
    fn get_Loop(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Loop(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FullPath(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FullPath(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RotateShapes(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RotateShapes(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColorBlendType(
        pVal: *mut cdrFountainFillBlendType,
    ) -> HRESULT,
    fn put_ColorBlendType(
        pVal: cdrFountainFillBlendType,
    ) -> HRESULT,
    fn get_SpacingAcceleration(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_SpacingAcceleration(
        pVal: i32,
    ) -> HRESULT,
    fn get_ColorAcceleration(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_ColorAcceleration(
        pVal: i32,
    ) -> HRESULT,
    fn get_LinkAcceleration(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_LinkAcceleration(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AccelerateSize(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AccelerateSize(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MapNodes(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MapNodes(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_StartPoint(
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn put_StartPoint(
        ppVal: *const IVGSnapPoint,
    ) -> HRESULT,
    fn get_EndPoint(
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn put_EndPoint(
        ppVal: *const IVGSnapPoint,
    ) -> HRESULT,
    fn Split(
        StepNo: i32,
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn FuseStart(
        pbVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn FuseEnd(
        pbVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn CopyFrom(
        Source: *const IVGEffectBlend,
        pbVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580060, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSnapPoint(IVGSnapPointVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_PositionX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PositionX(
        pVal: f64,
    ) -> HRESULT,
    fn get_PositionY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PositionY(
        pVal: f64,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrPointType,
    ) -> HRESULT,
    fn get_Node(
        ppVal: *mut *mut IVGNode,
    ) -> HRESULT,
    fn get_IsDeletable(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsMovable(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanChangeDirection(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetPosition(
        PositionX: *mut f64,
        PositionY: *mut f64,
    ) -> HRESULT,
    fn get_UsesDirection(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UsesDirection(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSelectable(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Direction(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Direction(
        pVal: f64,
    ) -> HRESULT,
    fn SetPosition(
        PositionX: f64,
        PositionY: f64,
    ) -> HRESULT,
    fn get_Selected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Selected(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn CreateSelection(
    ) -> HRESULT,
    fn get_User(
        ppVal: *mut *mut IVGUserSnapPoint,
    ) -> HRESULT,
    fn get_Object(
        ppVal: *mut *mut IVGObjectSnapPoint,
    ) -> HRESULT,
    fn get_Shape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_BBox(
        ppVal: *mut *mut IVGBBoxSnapPoint,
    ) -> HRESULT,
    fn get_ReferenceData(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn Move(
        OffsetX: f64,
        OffsetY: f64,
    ) -> HRESULT,
    fn get_Edge(
        ppVal: *mut *mut IVGEdgeSnapPoint,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800a8, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGUserSnapPoint(IVGUserSnapPointVtbl): IDispatch(IDispatchVtbl) {
    fn get_ID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_AutoSnap(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AutoSnap(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800a9, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGObjectSnapPoint(IVGObjectSnapPointVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrObjectSnapPointType,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800aa, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGBBoxSnapPoint(IVGBBoxSnapPointVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrReferencePoint,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800ac, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEdgeSnapPoint(IVGEdgeSnapPointVtbl): IDispatch(IDispatchVtbl) {
    fn get_SegmentIndex(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_SegmentOffset(
        pVal: *mut f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580029, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectControlPath(IVGEffectControlPathVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_Effects(
        ppVal: *mut *mut IVGEffects,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058002d, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectExtrude(IVGEffectExtrudeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrExtrudeType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrExtrudeType,
    ) -> HRESULT,
    fn get_VanishingPoint(
        ppVal: *mut *mut IVGExtrudeVanishingPoint,
    ) -> HRESULT,
    fn put_VanishingPoint(
        ppVal: *const IVGExtrudeVanishingPoint,
    ) -> HRESULT,
    fn get_Depth(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Depth(
        pVal: i32,
    ) -> HRESULT,
    fn get_AngleX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_AngleX(
        pVal: f64,
    ) -> HRESULT,
    fn get_AngleY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_AngleY(
        pVal: f64,
    ) -> HRESULT,
    fn get_AngleZ(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_AngleZ(
        pVal: f64,
    ) -> HRESULT,
    fn get_Shading(
        pVal: *mut cdrExtrudeShading,
    ) -> HRESULT,
    fn put_Shading(
        pVal: cdrExtrudeShading,
    ) -> HRESULT,
    fn get_BaseColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_BaseColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_ShadingColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_ShadingColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_UseBevel(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseBevel(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ShowBevelOnly(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ShowBevelOnly(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BevelDepth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_BevelDepth(
        pVal: f64,
    ) -> HRESULT,
    fn get_BevelAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_BevelAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_UseExtrudeColorForBevel(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseExtrudeColorForBevel(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BevelColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_BevelColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_LightPresent(
        Index: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_LightPresent(
        Index: i32,
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_LightPosition(
        Index: i32,
        pVal: *mut cdrExtrudeLightPosition,
    ) -> HRESULT,
    fn put_LightPosition(
        Index: i32,
        pVal: cdrExtrudeLightPosition,
    ) -> HRESULT,
    fn get_LightIntensity(
        Index: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_LightIntensity(
        Index: i32,
        pVal: i32,
    ) -> HRESULT,
    fn get_UseFullColorRange(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseFullColorRange(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FaceVisible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FaceShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_BevelGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_ExtrudeGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn Rotate(
        AngleX: f64,
        AngleY: f64,
        AngleZ: f64,
    ) -> HRESULT,
    fn SetBevel(
        Depth: f64,
        Angle: f64,
        ShowBevelOnly: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetLight(
        Index: i32,
        Position: cdrExtrudeLightPosition,
        LightIntensity: i32,
    ) -> HRESULT,
    fn CopyFrom(
        Source: *const IVGEffectExtrude,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058002e, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGExtrudeVanishingPoint(IVGExtrudeVanishingPointVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffectExtrude,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrExtrudeVPType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrExtrudeVPType,
    ) -> HRESULT,
    fn get_PositionX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PositionX(
        pVal: f64,
    ) -> HRESULT,
    fn get_PositionY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PositionY(
        pVal: f64,
    ) -> HRESULT,
    fn get_Effects(
        ppVal: *mut *mut IVGEffects,
    ) -> HRESULT,
    fn Share(
        Source: *const IVGEffectExtrude,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058002c, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectEnvelope(IVGEffectEnvelopeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_Container(
        pVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn put_Container(
        pVal: *const IVGCurve,
    ) -> HRESULT,
    fn get_Mode(
        pVal: *mut cdrEnvelopeMode,
    ) -> HRESULT,
    fn put_Mode(
        pVal: cdrEnvelopeMode,
    ) -> HRESULT,
    fn get_KeepLines(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_KeepLines(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn Select(
        PresetIndex: i32,
    ) -> HRESULT,
    fn CopyFrom(
        Source: *const IVGEffectEnvelope,
    ) -> HRESULT,
    fn CreateFrom(
        Shape: *const IVGShape,
        Success: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn CopyFromShape(
        Source: *const IVGShape,
        Mode: cdrEnvelopeMode,
        KeepLines: VARIANT_BOOL,
        CopyMode: cdrEnvelopeCopyMode,
        CornerIndices: VARIANT,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn CopyFromCurve(
        Source: *const IVGCurve,
        Mode: cdrEnvelopeMode,
        KeepLines: VARIANT_BOOL,
        CopyMode: cdrEnvelopeCopyMode,
        CornerIndices: VARIANT,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CornerIndices(
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn put_CornerIndices(
        pVal: VARIANT,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580033, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectTextOnPath(IVGEffectTextOnPathVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_Text(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn put_Text(
        ppVal: *const IVGShape,
    ) -> HRESULT,
    fn get_Path(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn put_Path(
        ppVal: *const IVGShape,
    ) -> HRESULT,
    fn get_DistanceFromPath(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_DistanceFromPath(
        pVal: f64,
    ) -> HRESULT,
    fn get_Offset(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Offset(
        pVal: f64,
    ) -> HRESULT,
    fn get_Orientation(
        pVal: *mut cdrFittedOrientation,
    ) -> HRESULT,
    fn put_Orientation(
        pVal: cdrFittedOrientation,
    ) -> HRESULT,
    fn get_Placement(
        pVal: *mut cdrFittedPlacement,
    ) -> HRESULT,
    fn put_Placement(
        pVal: cdrFittedPlacement,
    ) -> HRESULT,
    fn get_PlaceOnOtherSide(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PlaceOnOtherSide(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Quadrant(
        pVal: *mut cdrFittedQuadrant,
    ) -> HRESULT,
    fn put_Quadrant(
        pVal: cdrFittedQuadrant,
    ) -> HRESULT,
    fn get_VertPlacement(
        pVal: *mut cdrFittedVertPlacement,
    ) -> HRESULT,
    fn put_VertPlacement(
        pVal: cdrFittedVertPlacement,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058002b, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectDropShadow(IVGEffectDropShadowVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_OffsetX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OffsetX(
        pVal: f64,
    ) -> HRESULT,
    fn get_OffsetY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OffsetY(
        pVal: f64,
    ) -> HRESULT,
    fn get_Opacity(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Opacity(
        pVal: i32,
    ) -> HRESULT,
    fn get_Feather(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Feather(
        pVal: i32,
    ) -> HRESULT,
    fn get_FeatherType(
        pVal: *mut cdrFeatherType,
    ) -> HRESULT,
    fn put_FeatherType(
        pVal: cdrFeatherType,
    ) -> HRESULT,
    fn get_FeatherEdge(
        pVal: *mut cdrEdgeType,
    ) -> HRESULT,
    fn put_FeatherEdge(
        pVal: cdrEdgeType,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrDropShadowType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrDropShadowType,
    ) -> HRESULT,
    fn get_PerspectiveAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PerspectiveAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_PerspectiveStretch(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PerspectiveStretch(
        pVal: f64,
    ) -> HRESULT,
    fn get_Fade(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Fade(
        pVal: i32,
    ) -> HRESULT,
    fn get_Color(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_Color(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn SetOffset(
        OffsetX: f64,
        OffsetY: f64,
    ) -> HRESULT,
    fn get_ShadowGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_MergeMode(
        pVal: *mut cdrMergeMode,
    ) -> HRESULT,
    fn put_MergeMode(
        pVal: cdrMergeMode,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580028, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectContour(IVGEffectContourVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_Direction(
        pVal: *mut cdrContourDirection,
    ) -> HRESULT,
    fn put_Direction(
        pVal: cdrContourDirection,
    ) -> HRESULT,
    fn get_Offset(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Offset(
        pVal: f64,
    ) -> HRESULT,
    fn get_Steps(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Steps(
        pVal: i32,
    ) -> HRESULT,
    fn get_ColorBlendType(
        pVal: *mut cdrFountainFillBlendType,
    ) -> HRESULT,
    fn put_ColorBlendType(
        pVal: cdrFountainFillBlendType,
    ) -> HRESULT,
    fn get_OutlineColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_OutlineColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_FillColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_FillColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_FillColorTo(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_FillColorTo(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_LinkAcceleration(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_LinkAcceleration(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColorAcceleration(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_ColorAcceleration(
        pVal: i32,
    ) -> HRESULT,
    fn get_SpacingAcceleration(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_SpacingAcceleration(
        pVal: i32,
    ) -> HRESULT,
    fn get_ContourGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_EndCapType(
        pVal: *mut cdrContourEndCapType,
    ) -> HRESULT,
    fn put_EndCapType(
        pVal: cdrContourEndCapType,
    ) -> HRESULT,
    fn get_CornerType(
        pVal: *mut cdrContourCornerType,
    ) -> HRESULT,
    fn put_CornerType(
        pVal: cdrContourCornerType,
    ) -> HRESULT,
    fn get_MiterLimit(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_MiterLimit(
        pVal: f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058002a, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectDistortion(IVGEffectDistortionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrDistortionType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrDistortionType,
    ) -> HRESULT,
    fn get_OriginX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OriginX(
        pVal: f64,
    ) -> HRESULT,
    fn get_OriginY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OriginY(
        pVal: f64,
    ) -> HRESULT,
    fn get_PushPull(
        ppVal: *mut *mut IVGEffectPushPullDistortion,
    ) -> HRESULT,
    fn put_PushPull(
        ppVal: *const IVGEffectPushPullDistortion,
    ) -> HRESULT,
    fn get_Zipper(
        ppVal: *mut *mut IVGEffectZipperDistortion,
    ) -> HRESULT,
    fn put_Zipper(
        ppVal: *const IVGEffectZipperDistortion,
    ) -> HRESULT,
    fn get_Twister(
        ppVal: *mut *mut IVGEffectTwisterDistortion,
    ) -> HRESULT,
    fn put_Twister(
        ppVal: *const IVGEffectTwisterDistortion,
    ) -> HRESULT,
    fn CenterDistortion(
    ) -> HRESULT,
    fn get_Custom(
        ppVal: *mut *mut IVGEffectCustomDistortion,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580032, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectPushPullDistortion(IVGEffectPushPullDistortionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffectDistortion,
    ) -> HRESULT,
    fn get_Amplitude(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Amplitude(
        pVal: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580035, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectZipperDistortion(IVGEffectZipperDistortionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffectDistortion,
    ) -> HRESULT,
    fn get_Amplitude(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Amplitude(
        pVal: i32,
    ) -> HRESULT,
    fn get_Frequency(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Frequency(
        pVal: i32,
    ) -> HRESULT,
    fn get_Random(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Random(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Smooth(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Smooth(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Local(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Local(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580034, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectTwisterDistortion(IVGEffectTwisterDistortionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffectDistortion,
    ) -> HRESULT,
    fn get_Angle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Angle(
        pVal: f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058001b, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectCustomDistortion(IVGEffectCustomDistortionVtbl): IDispatch(IDispatchVtbl) {
    fn get_DistortionID(
        pVal: *mut BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580030, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectLens(IVGEffectLensVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn Freeze(
    ) -> HRESULT,
    fn Unfreeze(
    ) -> HRESULT,
    fn Ungroup(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn get_Shapes(
        ppVal: *mut *mut IVGShapes,
    ) -> HRESULT,
    fn get_Frozen(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrLensType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrLensType,
    ) -> HRESULT,
    fn get_Rate(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Rate(
        pVal: i32,
    ) -> HRESULT,
    fn get_Color(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_Color(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_OutlineColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_OutlineColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_FillColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_FillColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_FromColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_FromColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_ToColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_ToColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_UseOutlineColor(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseOutlineColor(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_UseFillColor(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseFillColor(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColorMapPalette(
        pVal: *mut cdrFountainFillBlendType,
    ) -> HRESULT,
    fn put_ColorMapPalette(
        pVal: cdrFountainFillBlendType,
    ) -> HRESULT,
    fn get_Magnification(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Magnification(
        pVal: f64,
    ) -> HRESULT,
    fn get_UseViewPoint(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseViewPoint(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ViewPointX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_ViewPointX(
        pVal: f64,
    ) -> HRESULT,
    fn get_ViewPointY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_ViewPointY(
        pVal: f64,
    ) -> HRESULT,
    fn get_RemoveFace(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RemoveFace(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PaletteRotation(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_PaletteRotation(
        pVal: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580031, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEffectPerspective(IVGEffectPerspectiveVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGEffect,
    ) -> HRESULT,
    fn get_UseHorizVanishingPoint(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseHorizVanishingPoint(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_UseVertVanishingPoint(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseVertVanishingPoint(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_HorizVanishingPointX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_HorizVanishingPointX(
        pVal: f64,
    ) -> HRESULT,
    fn get_HorizVanishingPointY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_HorizVanishingPointY(
        pVal: f64,
    ) -> HRESULT,
    fn get_VertVanishingPointX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_VertVanishingPointX(
        pVal: f64,
    ) -> HRESULT,
    fn get_VertVanishingPointY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_VertVanishingPointY(
        pVal: f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058001a, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGCustomEffect(IVGCustomEffectVtbl): IDispatch(IDispatchVtbl) {
    fn get_EffectID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_EffectGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580078, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextRange(IVGTextRangeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Text(
        ppVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Text(
        ppVal: BSTR,
    ) -> HRESULT,
    fn get_WideText(
        ppVal: *mut BSTR,
    ) -> HRESULT,
    fn put_WideText(
        ppVal: BSTR,
    ) -> HRESULT,
    fn get_Characters(
        ppVal: *mut *mut IVGTextCharacters,
    ) -> HRESULT,
    fn get_Words(
        ppVal: *mut *mut IVGTextWords,
    ) -> HRESULT,
    fn get_Lines(
        ppVal: *mut *mut IVGTextLines,
    ) -> HRESULT,
    fn get_Paragraphs(
        ppVal: *mut *mut IVGTextParagraphs,
    ) -> HRESULT,
    fn get_Start(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Start(
        pVal: i32,
    ) -> HRESULT,
    fn get_End(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_End(
        pVal: i32,
    ) -> HRESULT,
    fn get_Length(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Length(
        pVal: i32,
    ) -> HRESULT,
    fn Duplicate(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Style(
        pVal: *mut cdrFontStyle,
    ) -> HRESULT,
    fn put_Style(
        pVal: cdrFontStyle,
    ) -> HRESULT,
    fn get_Bold(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Bold(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Italic(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Italic(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Underline(
        pVal: *mut cdrFontLine,
    ) -> HRESULT,
    fn put_Underline(
        pVal: cdrFontLine,
    ) -> HRESULT,
    fn get_Strikethru(
        pVal: *mut cdrFontLine,
    ) -> HRESULT,
    fn put_Strikethru(
        pVal: cdrFontLine,
    ) -> HRESULT,
    fn get_Overscore(
        pVal: *mut cdrFontLine,
    ) -> HRESULT,
    fn put_Overscore(
        pVal: cdrFontLine,
    ) -> HRESULT,
    fn get_Font(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Font(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Size(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_Size(
        pVal: f32,
    ) -> HRESULT,
    fn get_Position(
        pVal: *mut cdrFontPosition,
    ) -> HRESULT,
    fn put_Position(
        pVal: cdrFontPosition,
    ) -> HRESULT,
    fn get_Case(
        pVal: *mut cdrFontCase,
    ) -> HRESULT,
    fn put_Case(
        pVal: cdrFontCase,
    ) -> HRESULT,
    fn get_CharAngle(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_CharAngle(
        pVal: f32,
    ) -> HRESULT,
    fn get_Alignment(
        pVal: *mut cdrAlignment,
    ) -> HRESULT,
    fn put_Alignment(
        pVal: cdrAlignment,
    ) -> HRESULT,
    fn get_FirstLineIndent(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_FirstLineIndent(
        pVal: f64,
    ) -> HRESULT,
    fn get_HorizShift(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_HorizShift(
        pVal: i32,
    ) -> HRESULT,
    fn get_VertShift(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_VertShift(
        pVal: i32,
    ) -> HRESULT,
    fn get_LeftIndent(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_LeftIndent(
        pVal: f64,
    ) -> HRESULT,
    fn get_RightIndent(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RightIndent(
        pVal: f64,
    ) -> HRESULT,
    fn get_MinWordSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_MinWordSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_MaxWordSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_MaxWordSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_MaxCharSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_MaxCharSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_ParaSpacingBefore(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_ParaSpacingBefore(
        pVal: f32,
    ) -> HRESULT,
    fn get_ParaSpacingAfter(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_ParaSpacingAfter(
        pVal: f32,
    ) -> HRESULT,
    fn get_CharSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_CharSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_LineSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_LineSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_LineSpacingType(
        pVal: *mut cdrLineSpacingType,
    ) -> HRESULT,
    fn get_WordSpacing(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_WordSpacing(
        pVal: f32,
    ) -> HRESULT,
    fn get_AutoHyphenate(
        pVal: *mut cdrTriState,
    ) -> HRESULT,
    fn put_AutoHyphenate(
        pVal: cdrTriState,
    ) -> HRESULT,
    fn get_HyphenHotZone(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_HyphenHotZone(
        pVal: f64,
    ) -> HRESULT,
    fn get_HyphenMinCharsBefore(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_HyphenMinCharsBefore(
        pVal: i32,
    ) -> HRESULT,
    fn get_HyphenMinCharsAfter(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_HyphenMinCharsAfter(
        pVal: i32,
    ) -> HRESULT,
    fn get_HyphenMinWordLength(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_HyphenMinWordLength(
        pVal: i32,
    ) -> HRESULT,
    fn get_HyphenateCapitals(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_HyphenateCapitals(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn Select(
    ) -> HRESULT,
    fn Copy(
    ) -> HRESULT,
    fn Paste(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn ChangeCase(
        Case: cdrTextChangeCase,
    ) -> HRESULT,
    fn SetRange(
        Start: i32,
        End: i32,
    ) -> HRESULT,
    fn get_LanguageID(
        pVal: *mut cdrTextLanguage,
    ) -> HRESULT,
    fn put_LanguageID(
        pVal: cdrTextLanguage,
    ) -> HRESULT,
    fn get_CharSet(
        pVal: *mut cdrTextCharSet,
    ) -> HRESULT,
    fn put_CharSet(
        pVal: cdrTextCharSet,
    ) -> HRESULT,
    fn InsertBefore(
        Text: BSTR,
        LanguageID: cdrTextLanguage,
        CharSet: cdrTextCharSet,
        Font: BSTR,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn InsertBeforeWide(
        Text: BSTR,
        LanguageID: cdrTextLanguage,
        CharSet: cdrTextCharSet,
        Font: BSTR,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn InsertAfter(
        Text: BSTR,
        LanguageID: cdrTextLanguage,
        CharSet: cdrTextCharSet,
        Font: BSTR,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn InsertAfterWide(
        Text: BSTR,
        LanguageID: cdrTextLanguage,
        CharSet: cdrTextCharSet,
        Font: BSTR,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn Replace(
        Text: BSTR,
        LanguageID: cdrTextLanguage,
        CharSet: cdrTextCharSet,
        Font: BSTR,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn ReplaceWide(
        Text: BSTR,
        LanguageID: cdrTextLanguage,
        CharSet: cdrTextCharSet,
        Font: BSTR,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn Range(
        Start: i32,
        End: i32,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn SetLineSpacing(
        Type: cdrLineSpacingType,
        LineSpacing: f32,
        ParaBefore: f32,
        ParaAfter: f32,
    ) -> HRESULT,
    fn get_Columns(
        ppVal: *mut *mut IVGTextColumns,
    ) -> HRESULT,
    fn get_Frames(
        ppVal: *mut *mut IVGTextFrames,
    ) -> HRESULT,
    fn Collapse(
        ToEnd: VARIANT_BOOL,
    ) -> HRESULT,
    fn Combine(
        Range: *const IVGTextRange,
    ) -> HRESULT,
    fn InRange(
        Range: *const IVGTextRange,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn IsSame(
        Range: *const IVGTextRange,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Fill(
        ppVal: *mut *mut IVGFill,
    ) -> HRESULT,
    fn get_Outline(
        ppVal: *mut *mut IVGOutline,
    ) -> HRESULT,
    fn get_RangeKerning(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_RangeKerning(
        pVal: i32,
    ) -> HRESULT,
    fn get_Tabs(
        ppVal: *mut *mut IVGTextTabPositions,
    ) -> HRESULT,
    fn get_Effect(
        pVal: *mut cdrTextEffect,
    ) -> HRESULT,
    fn get_DropCapLinesDropped(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_DropCapLinesDropped(
        pVal: i32,
    ) -> HRESULT,
    fn get_DropCapDistanceFromText(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_DropCapDistanceFromText(
        pVal: f64,
    ) -> HRESULT,
    fn get_DropCapHangingIndent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DropCapHangingIndent(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BulletFont(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_BulletFont(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_BulletSymbol(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_BulletSymbol(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_BulletSize(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_BulletSize(
        pVal: f32,
    ) -> HRESULT,
    fn get_BulletBaselineShift(
        pVal: *mut f32,
    ) -> HRESULT,
    fn put_BulletBaselineShift(
        pVal: f32,
    ) -> HRESULT,
    fn get_BulletHorizontalPosition(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_BulletHorizontalPosition(
        pVal: f64,
    ) -> HRESULT,
    fn get_BulletHangingIndent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_BulletHangingIndent(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsEmpty(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ApplyNoEffect(
    ) -> HRESULT,
    fn ApplyBulletEffect(
        Symbol: BSTR,
        Font: BSTR,
        Size: f32,
        BaselineShift: f32,
        HorizontalPosition: f64,
        HangingIndent: VARIANT_BOOL,
    ) -> HRESULT,
    fn ApplyDropCapEffect(
        LinesDropped: i32,
        DistanceFromText: f64,
        HangingIndent: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_HyphenateAllCapWords(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_HyphenateAllCapWords(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn EnumRanges(
        PropertyFilter: cdrTextPropertySet,
        ppVal: *mut *mut IVGTextRanges,
    ) -> HRESULT,
    fn Evaluate(
        Expression: BSTR,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn FindRanges(
        Query: BSTR,
        ppVal: *mut *mut IVGTextRanges,
    ) -> HRESULT,
    fn get_Baselines(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn Straighten(
    ) -> HRESULT,
    fn AlignToBaseline(
    ) -> HRESULT,
    fn get_TextLineRects(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_CharBackFill(
        ppVal: *mut *mut IVGFill,
    ) -> HRESULT,
    fn CopyAttributes(
        SourceRange: *const IVGTextRange,
    ) -> HRESULT,
    fn GetOpenTypeFeature(
        Feature: BSTR,
        pVal: *mut i32,
    ) -> HRESULT,
    fn SetOpenTypeFeature(
        Feature: BSTR,
        State: i32,
    ) -> HRESULT,
    fn get_TextFormatter(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_TextFormatter(
        pVal: i32,
    ) -> HRESULT,
    fn ApplyStyle(
        StyleName: BSTR,
    ) -> HRESULT,
    fn get_ObjectStyle(
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580072, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextCharacters(IVGTextCharactersVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        Count: i32,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_All(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058007c, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextWords(IVGTextWordsVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        Count: i32,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_All(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580076, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextLines(IVGTextLinesVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        Count: i32,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_All(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580077, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextParagraphs(IVGTextParagraphsVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        Count: i32,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_All(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580073, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextColumns(IVGTextColumnsVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        Count: i32,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_All(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580075, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextFrames(IVGTextFramesVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGTextFrame,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Range(
        Index: i32,
        Count: i32,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_All(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGTextFrame,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGTextFrame,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580074, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextFrame(IVGTextFrameVtbl): IDispatch(IDispatchVtbl) {
    fn get_Range(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Previous(
        ppVal: *mut *mut IVGTextFrame,
    ) -> HRESULT,
    fn get_Next(
        ppVal: *mut *mut IVGTextFrame,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Empty(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFirst(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsLast(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_VerticalAlignment(
        pVal: *mut cdrVerticalAlignment,
    ) -> HRESULT,
    fn put_VerticalAlignment(
        pVal: cdrVerticalAlignment,
    ) -> HRESULT,
    fn get_Multicolumn(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColumnCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_ColumnWidth(
        Index: i32,
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_ColumnGutter(
        Index: i32,
        pVal: *mut f64,
    ) -> HRESULT,
    fn SetColumns(
        NumColumns: i32,
        EqualColumns: VARIANT_BOOL,
        WidthsAndGutters: *const SAFEARRAY,
    ) -> HRESULT,
    fn LinkTo(
        Shape: *const IVGShape,
    ) -> HRESULT,
    fn UnLink(
    ) -> HRESULT,
    fn get_Container(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_IsInsideContainer(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFittedToPath(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Path(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_FrameShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Fill(
        ppVal: *mut *mut IVGFill,
    ) -> HRESULT,
    fn put_Fill(
        ppVal: *const IVGFill,
    ) -> HRESULT,
    fn get_Outline(
        ppVal: *mut *mut IVGOutline,
    ) -> HRESULT,
    fn put_Outline(
        ppVal: *const IVGOutline,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580095, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextTabPositions(IVGTextTabPositionsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGTextTabPosition,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_LeaderSpacing(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_LeaderSpacing(
        pVal: i32,
    ) -> HRESULT,
    fn get_LeaderCharacter(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_LeaderCharacter(
        pVal: BSTR,
    ) -> HRESULT,
    fn Clear(
    ) -> HRESULT,
    fn Add(
        Position: f64,
        Alignment: cdrTextTabAlignment,
        Leadered: VARIANT_BOOL,
        ppVal: *mut *mut IVGTextTabPosition,
    ) -> HRESULT,
    fn AddEvery(
        Position: f64,
        Alignment: cdrTextTabAlignment,
        Leadered: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580094, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextTabPosition(IVGTextTabPositionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Position(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Position(
        pVal: f64,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Alignment(
        pVal: *mut cdrTextTabAlignment,
    ) -> HRESULT,
    fn put_Alignment(
        pVal: cdrTextTabAlignment,
    ) -> HRESULT,
    fn get_Leadered(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Leadered(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800a4, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTextRanges(IVGTextRangesVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGTextRange,
    ) -> HRESULT,
    fn Reverse(
        ppVal: *mut *mut IVGTextRanges,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800b8, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStyle(IVGStyleVtbl): IDispatch(IDispatchVtbl) {
    fn get_CategoryName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn GetAllPropertyNames(
        pVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetOverridePropertyNames(
        pVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsPropertyInherited(
        Name: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetProperty(
        Name: BSTR,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn SetProperty(
        Name: BSTR,
        Value: VARIANT,
    ) -> HRESULT,
    fn ClearProperty(
        Name: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Outline(
        ppVal: *mut *mut IVGStyleOutline,
    ) -> HRESULT,
    fn get_Fill(
        ppVal: *mut *mut IVGStyleFill,
    ) -> HRESULT,
    fn get_Character(
        ppVal: *mut *mut IVGStyleCharacter,
    ) -> HRESULT,
    fn get_Paragraph(
        ppVal: *mut *mut IVGStyleParagraph,
    ) -> HRESULT,
    fn get_Frame(
        ppVal: *mut *mut IVGStyleFrame,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_DisplayName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_IsStyleSet(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsObjectDefaults(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DisplayCategoryName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_BasedOn(
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
    fn get_DerivedStyles(
        ppVal: *mut *mut IVGStyles,
    ) -> HRESULT,
    fn ToString(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn StringAssign(
        StyleString: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetPropertyAsString(
        Name: BSTR,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn SetPropertyAsString(
        Name: BSTR,
        Value: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Rename(
        NewName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn SetBasedOn(
        NewParent: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Delete(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Assign(
        pVal: *const IVGStyle,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
    fn get_Transparency(
        ppVal: *mut *mut IVGStyleTransparency,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800bb, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStyleOutline(IVGStyleOutlineVtbl): IDispatch(IDispatchVtbl) {
    fn get_Style(
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrOutlineType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrOutlineType,
    ) -> HRESULT,
    fn get_Overprint(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Overprint(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BehindFill(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_BehindFill(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ScaleWithShape(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ScaleWithShape(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Width(
        pVal: f64,
    ) -> HRESULT,
    fn get_Color(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_Color(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_OverlapArrow(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_OverlapArrow(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ShareArrow(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ShareArrow(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MiterLimit(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_MiterLimit(
        pVal: f64,
    ) -> HRESULT,
    fn get_NibStretch(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_NibStretch(
        pVal: i32,
    ) -> HRESULT,
    fn get_NibAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_NibAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_WidelineWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_WidelineWidth(
        pVal: f64,
    ) -> HRESULT,
    fn get_LineCaps(
        pVal: *mut cdrOutlineLineCaps,
    ) -> HRESULT,
    fn put_LineCaps(
        pVal: cdrOutlineLineCaps,
    ) -> HRESULT,
    fn get_LineJoin(
        pVal: *mut cdrOutlineLineJoin,
    ) -> HRESULT,
    fn put_LineJoin(
        pVal: cdrOutlineLineJoin,
    ) -> HRESULT,
    fn get_Justification(
        pVal: *mut cdrOutlineJustification,
    ) -> HRESULT,
    fn put_Justification(
        pVal: cdrOutlineJustification,
    ) -> HRESULT,
    fn get_AdjustDashes(
        pVal: *mut cdrOutlineDashAdjust,
    ) -> HRESULT,
    fn put_AdjustDashes(
        pVal: cdrOutlineDashAdjust,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800bc, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStyleFill(IVGStyleFillVtbl): IDispatch(IDispatchVtbl) {
    fn get_Style(
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrFillStyleType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrFillStyleType,
    ) -> HRESULT,
    fn get_Overprint(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Overprint(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_WindingFill(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_WindingFill(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FountainFillType(
        pVal: *mut cdrFountainFillType,
    ) -> HRESULT,
    fn put_FountainFillType(
        pVal: cdrFountainFillType,
    ) -> HRESULT,
    fn get_EdgePad(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_EdgePad(
        pVal: i32,
    ) -> HRESULT,
    fn get_FountainCenterOffsetX(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_FountainCenterOffsetX(
        pVal: i32,
    ) -> HRESULT,
    fn get_FountainCenterOffsetY(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_FountainCenterOffsetY(
        pVal: i32,
    ) -> HRESULT,
    fn get_FountainSteps(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_FountainSteps(
        pVal: i32,
    ) -> HRESULT,
    fn get_FountainBlendType(
        pVal: *mut cdrFountainFillBlendType,
    ) -> HRESULT,
    fn put_FountainBlendType(
        pVal: cdrFountainFillBlendType,
    ) -> HRESULT,
    fn get_MidPoint(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_MidPoint(
        pVal: i32,
    ) -> HRESULT,
    fn get_FlipColors(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FlipColors(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PostScriptName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_PostScriptName(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_TileWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TileWidth(
        pVal: f64,
    ) -> HRESULT,
    fn get_TileHeight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TileHeight(
        pVal: f64,
    ) -> HRESULT,
    fn get_TileOriginX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TileOriginX(
        pVal: f64,
    ) -> HRESULT,
    fn get_TileOriginY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TileOriginY(
        pVal: f64,
    ) -> HRESULT,
    fn get_TileOffsetType(
        pVal: *mut cdrTileOffsetType,
    ) -> HRESULT,
    fn put_TileOffsetType(
        pVal: cdrTileOffsetType,
    ) -> HRESULT,
    fn get_TileOffset(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_TileOffset(
        pVal: i32,
    ) -> HRESULT,
    fn get_RotationAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_RotationAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_SkewAngle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_SkewAngle(
        pVal: f64,
    ) -> HRESULT,
    fn get_MirrorFill(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MirrorFill(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TransformWithShape(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TransformWithShape(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PrimaryColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn get_SecondaryColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn get_MirrorFillX(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MirrorFillX(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MirrorFillY(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MirrorFillY(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FountainCenterXOffset(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_FountainCenterXOffset(
        pVal: f64,
    ) -> HRESULT,
    fn get_FountainCenterYOffset(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_FountainCenterYOffset(
        pVal: f64,
    ) -> HRESULT,
    fn get_FountainBlendAcceleration(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_FountainBlendAcceleration(
        pVal: f64,
    ) -> HRESULT,
    fn get_FountainScaleX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_FountainScaleX(
        pVal: f64,
    ) -> HRESULT,
    fn get_FountainScaleY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_FountainScaleY(
        pVal: f64,
    ) -> HRESULT,
    fn get_FountainAnisotropic(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FountainAnisotropic(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FountainSpreadMethod(
        pVal: *mut cdrFountainFillSpreadMethod,
    ) -> HRESULT,
    fn put_FountainSpreadMethod(
        pVal: cdrFountainFillSpreadMethod,
    ) -> HRESULT,
    fn get_PrimaryOpacity(
        pVal: *mut u8,
    ) -> HRESULT,
    fn put_PrimaryOpacity(
        pVal: u8,
    ) -> HRESULT,
    fn get_SecondaryOpacity(
        pVal: *mut u8,
    ) -> HRESULT,
    fn put_SecondaryOpacity(
        pVal: u8,
    ) -> HRESULT,
    fn get_MergeMode(
        pVal: *mut cdrMergeMode,
    ) -> HRESULT,
    fn put_MergeMode(
        pVal: cdrMergeMode,
    ) -> HRESULT,
    fn SaveFill(
        FileName: BSTR,
        Metadata: *const IVGFillMetadata,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn LoadFill(
        FileName: BSTR,
        Metadata: *mut *mut IVGFillMetadata,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800c0, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGFillMetadata(IVGFillMetadataVtbl): IDispatch(IDispatchVtbl) {
    fn get_Title(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
    fn get_Description(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
    fn get_Keywords(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
    fn get_Subject(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
    fn get_Copyright(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
    fn get_CreatorTool(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CreationDate(
        pVal: *mut DATE,
    ) -> HRESULT,
    fn get_ModificationDate(
        pVal: *mut DATE,
    ) -> HRESULT,
    fn get_DocumentID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_InstanceID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_DerivedFrom(
        pVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_Category(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800c1, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGLocalizableString(IVGLocalizableStringVtbl): IDispatch(IDispatchVtbl) {
    fn Clear(
    ) -> HRESULT,
    fn get_IsEmpty(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetLangString(
        Language: BSTR,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn SetLangString(
        Language: BSTR,
        Value: BSTR,
    ) -> HRESULT,
    fn HasLangString(
        Language: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_HasDefaultLangString(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_HasDefaultLangStringOnly(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_HasNonDefaultLangStrings(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DefaultLangString(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_DefaultLangString(
        pVal: BSTR,
    ) -> HRESULT,
    fn GetLanguages(
        pVal: *mut SAFEARRAY,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800bd, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStyleCharacter(IVGStyleCharacterVtbl): IDispatch(IDispatchVtbl) {
    fn get_Style(
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800be, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStyleParagraph(IVGStyleParagraphVtbl): IDispatch(IDispatchVtbl) {
    fn get_Style(
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800bf, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStyleFrame(IVGStyleFrameVtbl): IDispatch(IDispatchVtbl) {
    fn get_Style(
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800b9, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStyles(IVGStylesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Find(
        Name: BSTR,
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800d7, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStyleTransparency(IVGStyleTransparencyVtbl): IDispatch(IDispatchVtbl) {
    fn get_Style(
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
    fn get_Fill(
        ppVal: *mut *mut IVGStyleFill,
    ) -> HRESULT,
    fn get_Mode(
        pVal: *mut cdrMergeMode,
    ) -> HRESULT,
    fn put_Mode(
        pVal: cdrMergeMode,
    ) -> HRESULT,
    fn get_UniformTransparency(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_UniformTransparency(
        pVal: f64,
    ) -> HRESULT,
    fn get_WhiteTransparency(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_WhiteTransparency(
        pVal: f64,
    ) -> HRESULT,
    fn get_BlackTransparency(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_BlackTransparency(
        pVal: f64,
    ) -> HRESULT,
    fn get_AppliesTo(
        pVal: *mut cdrTransparencyAppliedTo,
    ) -> HRESULT,
    fn put_AppliesTo(
        pVal: cdrTransparencyAppliedTo,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580061, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSnapPoints(IVGSnapPointsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn User(
        ID: BSTR,
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn BBox(
        Type: cdrReferencePoint,
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn Object(
        Type: cdrObjectSnapPointType,
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn FindClosest(
        TypeSet: cdrPointType,
        PositionX: f64,
        PositionY: f64,
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn Range(
        References: *const SAFEARRAY,
        ppVal: *mut *mut IVGSnapPointRange,
    ) -> HRESULT,
    fn AddUserSnapPoint(
        PositionX: f64,
        PositionY: f64,
        Direction: f64,
        UseDirection: VARIANT_BOOL,
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn AddUserSnapPointEx(
        ID: BSTR,
        PositionX: f64,
        PositionY: f64,
        Direction: f64,
        UseDirection: VARIANT_BOOL,
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn get_Selection(
        ppVal: *mut *mut IVGSnapPointRange,
    ) -> HRESULT,
    fn ClearSelection(
    ) -> HRESULT,
    fn get_All(
        ppVal: *mut *mut IVGSnapPointRange,
    ) -> HRESULT,
    fn Edge(
        SegmentIndex: i32,
        SegmentOffset: f64,
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn Auto(
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800ab, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSnapPointRange(IVGSnapPointRangeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn Move(
        OffsetX: f64,
        OffsetY: f64,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Add(
        SnapPoint: *const IVGSnapPoint,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Remove(
        Index: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn RemoveByReference(
        ReferenceData: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Find(
        ReferenceData: BSTR,
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn CreateSelection(
    ) -> HRESULT,
    fn AddToSelection(
    ) -> HRESULT,
    fn RemoveFromSelection(
    ) -> HRESULT,
    fn ChangeDirection(
        Direction: f64,
        UsesDirection: cdrTriState,
    ) -> HRESULT,
    fn SetAutoSnap(
        AutoSnap: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580016, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGConnector(IVGConnectorVtbl): IDispatch(IDispatchVtbl) {
    fn get_StartPoint(
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn put_StartPoint(
        ppVal: *const IVGSnapPoint,
    ) -> HRESULT,
    fn get_EndPoint(
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn put_EndPoint(
        ppVal: *const IVGSnapPoint,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrConnectorType,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058003f, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGGuide(IVGGuideVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrGuideType,
    ) -> HRESULT,
    fn get_Preset(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn MakeEditable(
    ) -> HRESULT,
    fn get_Point1X(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Point1Y(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Point2X(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Point2Y(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_Angle(
        pVal: *mut f64,
    ) -> HRESULT,
    fn GetPoints(
        Point1X: *mut f64,
        Point1Y: *mut f64,
        Point2X: *mut f64,
        Point2Y: *mut f64,
    ) -> HRESULT,
    fn GetPointAndAngle(
        PointX: *mut f64,
        PointY: *mut f64,
        Angle: *mut f64,
    ) -> HRESULT,
    fn SetPoints(
        Point1X: f64,
        Point1Y: f64,
        Point2X: f64,
        Point2Y: f64,
    ) -> HRESULT,
    fn SetPointAndAngle(
        PointX: f64,
        PointY: f64,
        Angle: f64,
    ) -> HRESULT,
    fn get_InterceptX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_InterceptY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_CenterX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_CenterY(
        pVal: *mut f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580053, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPowerClip(IVGPowerClipVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Shapes(
        ppVal: *mut *mut IVGShapes,
    ) -> HRESULT,
    fn get_ContentsLocked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ContentsLocked(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn EnterEditMode(
    ) -> HRESULT,
    fn LeaveEditMode(
    ) -> HRESULT,
    fn ExtractShapes(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058007e, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGURL(IVGURLVtbl): IDispatch(IDispatchVtbl) {
    fn get_Address(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Address(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_TargetFrame(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_TargetFrame(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_AltComment(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_AltComment(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_BookMark(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_BookMark(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Region(
        pVal: *mut cdrURLRegion,
    ) -> HRESULT,
    fn put_Region(
        pVal: cdrURLRegion,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580020, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDataItems(IVGDataItemsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGDataItem,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Add(
        DataField: *const IVGDataField,
        Value: VARIANT,
        ppVal: *mut *mut IVGDataItem,
    ) -> HRESULT,
    fn CopyFrom(
        Shape: *const IVGShape,
    ) -> HRESULT,
    fn Clear(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058001f, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDataItem(IVGDataItemVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGDataItems,
    ) -> HRESULT,
    fn get_Value(
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn put_Value(
        pVal: VARIANT,
    ) -> HRESULT,
    fn get_DataField(
        pVal: *mut *mut IVGDataField,
    ) -> HRESULT,
    fn Clear(
    ) -> HRESULT,
    fn get_FormattedValue(
        pbstrValue: *mut BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058001d, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDataField(IVGDataFieldVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGDataFields,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_FormatType(
        pVal: *mut cdrDataFormatType,
    ) -> HRESULT,
    fn get_Format(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Format(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_FieldWidth(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_FieldWidth(
        pVal: i32,
    ) -> HRESULT,
    fn get_AppDefault(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AppDefault(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DocDefault(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DocDefault(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_SummarizeGroup(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_SummarizeGroup(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn Reorder(
        NewIndex: i32,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Target(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Target(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_DefaultValue(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_DefaultValue(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Constraint(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Constraint(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_DataType(
        pVal: *mut cdrDataType,
    ) -> HRESULT,
    fn put_DataType(
        pVal: cdrDataType,
    ) -> HRESULT,
    fn get_ParentName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_ParentName(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_ElementName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_ElementName(
        pVal: BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058001e, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDataFields(IVGDataFieldsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGDataField,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Add(
        Name: BSTR,
        Format: BSTR,
        AppDefault: VARIANT_BOOL,
        DocDefault: VARIANT_BOOL,
        SummarizeGroup: VARIANT_BOOL,
        ppVal: *mut *mut IVGDataField,
    ) -> HRESULT,
    fn AddEx(
        Name: BSTR,
        DataType: cdrDataType,
        DefaultValue: BSTR,
        Constraint: BSTR,
        Target: BSTR,
        Format: BSTR,
        AppDefault: VARIANT_BOOL,
        DocDefault: VARIANT_BOOL,
        SummarizeGroup: VARIANT_BOOL,
        FieldWidth: i32,
        ppVal: *mut *mut IVGDataField,
    ) -> HRESULT,
    fn IsPresent(
        FieldName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn AddEx2(
        ParentName: BSTR,
        ElementName: BSTR,
        Name: BSTR,
        DataType: cdrDataType,
        DefaultValue: BSTR,
        Constraint: BSTR,
        Target: BSTR,
        Format: BSTR,
        AppDefault: VARIANT_BOOL,
        DocDefault: VARIANT_BOOL,
        SummarizeGroup: VARIANT_BOOL,
        FieldWidth: i32,
        ppVal: *mut *mut IVGDataField,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580011, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGCloneLink(IVGCloneLinkVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_CloneParent(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_FillLinked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FillLinked(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_OutlineLinked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_OutlineLinked(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ShapeLinked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ShapeLinked(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TransformLinked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TransformLinked(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BitmapColorMaskLinked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_BitmapColorMaskLinked(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn RestoreAllLinks(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058007d, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTransparency(IVGTransparencyVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrTransparencyType,
    ) -> HRESULT,
    fn get_Uniform(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Uniform(
        pVal: i32,
    ) -> HRESULT,
    fn get_Fountain(
        ppVal: *mut *mut IVGFountainFill,
    ) -> HRESULT,
    fn put_Fountain(
        ppVal: *const IVGFountainFill,
    ) -> HRESULT,
    fn get_Pattern(
        ppVal: *mut *mut IVGPatternFill,
    ) -> HRESULT,
    fn put_Pattern(
        ppVal: *const IVGPatternFill,
    ) -> HRESULT,
    fn get_Texture(
        ppVal: *mut *mut IVGTextureFill,
    ) -> HRESULT,
    fn put_Texture(
        ppVal: *const IVGTextureFill,
    ) -> HRESULT,
    fn get_Start(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Start(
        pVal: i32,
    ) -> HRESULT,
    fn get_End(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_End(
        pVal: i32,
    ) -> HRESULT,
    fn get_Frozen(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ApplyNoTransparency(
    ) -> HRESULT,
    fn ApplyUniformTransparency(
        Value: i32,
    ) -> HRESULT,
    fn ApplyFountainTransparency(
        Start: i32,
        End: i32,
        Type: cdrFountainFillType,
        Angle: f64,
        Steps: i32,
        EdgePad: i32,
        MidPoint: i32,
        CenterOffsetX: f64,
        CenterOffsetY: f64,
        ppVal: *mut *mut IVGFountainFill,
    ) -> HRESULT,
    fn ApplyPatternTransparency(
        Type: cdrPatternFillType,
        FileName: BSTR,
        PatternCanvasIndex: i32,
        Front: i32,
        Back: i32,
        TransformWithShape: VARIANT_BOOL,
        ppVal: *mut *mut IVGPatternFill,
    ) -> HRESULT,
    fn ApplyTextureTransparency(
        TextureName: BSTR,
        LibraryName: BSTR,
        Front: i32,
        Back: i32,
        ppVal: *mut *mut IVGTextureFill,
    ) -> HRESULT,
    fn Freeze(
    ) -> HRESULT,
    fn Unfreeze(
    ) -> HRESULT,
    fn get_AppliedTo(
        pVal: *mut cdrTransparencyAppliedTo,
    ) -> HRESULT,
    fn put_AppliedTo(
        pVal: cdrTransparencyAppliedTo,
    ) -> HRESULT,
    fn get_MergeMode(
        pVal: *mut cdrMergeMode,
    ) -> HRESULT,
    fn put_MergeMode(
        pVal: cdrMergeMode,
    ) -> HRESULT,
    fn UserAssign(
        TransparencyType: cdrTransparencyType,
        PatternType: cdrPatternFillType,
        ParentWindowHandle: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058001c, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGCustomShape(IVGCustomShapeVtbl): IDispatch(IDispatchVtbl) {
    fn get_TypeID(
        pVal: *mut BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580021, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDimension(IVGDimensionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrDimensionType,
    ) -> HRESULT,
    fn get_Linear(
        ppVal: *mut *mut IVGDimensionLinear,
    ) -> HRESULT,
    fn get_Angular(
        ppVal: *mut *mut IVGDimensionAngular,
    ) -> HRESULT,
    fn get_Precision(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Precision(
        pVal: i32,
    ) -> HRESULT,
    fn get_BoxedText(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_BoxedText(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_LeadingZero(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_LeadingZero(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TextShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Prefix(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Prefix(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Suffix(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Suffix(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Outline(
        ppVal: *mut *mut IVGOutline,
    ) -> HRESULT,
    fn get_TextCentered(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TextCentered(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ShowUnits(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ShowUnits(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Placement(
        pVal: *mut cdrDimensionPlacement,
    ) -> HRESULT,
    fn put_Placement(
        pVal: cdrDimensionPlacement,
    ) -> HRESULT,
    fn get_HorizontalText(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_HorizontalText(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DynamicText(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DynamicText(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DistanceFromObject(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_UseDistanceFromObject(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseDistanceFromObject(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetDistanceFromObject(
        Distance: f64,
    ) -> HRESULT,
    fn get_Overhang(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_UseOverhang(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseOverhang(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetOverhang(
        Overhang: f64,
    ) -> HRESULT,
    fn get_TextLabelGap(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_UseTextLabelGap(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseTextLabelGap(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetTextLabelGap(
        Gap: f64,
    ) -> HRESULT,
    fn get_ExtensionLinesVisible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ExtensionLinesVisible(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_InnerExtensionLinesVisible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_InnerExtensionLinesVisible(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_GapOnFreeExtensionVisible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_GapOnFreeExtensionVisible(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580023, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDimensionLinear(IVGDimensionLinearVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrLinearDimensionType,
    ) -> HRESULT,
    fn get_Point1(
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn put_Point1(
        ppVal: *const IVGSnapPoint,
    ) -> HRESULT,
    fn get_Point2(
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn put_Point2(
        ppVal: *const IVGSnapPoint,
    ) -> HRESULT,
    fn get_TextCentered(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TextCentered(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TextX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TextX(
        pVal: f64,
    ) -> HRESULT,
    fn get_TextY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TextY(
        pVal: f64,
    ) -> HRESULT,
    fn get_Style(
        pVal: *mut cdrDimensionStyle,
    ) -> HRESULT,
    fn put_Style(
        pVal: cdrDimensionStyle,
    ) -> HRESULT,
    fn get_ShowUnits(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ShowUnits(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Units(
        pVal: *mut cdrDimensionLinearUnits,
    ) -> HRESULT,
    fn put_Units(
        pVal: cdrDimensionLinearUnits,
    ) -> HRESULT,
    fn get_Placement(
        pVal: *mut cdrDimensionPlacement,
    ) -> HRESULT,
    fn put_Placement(
        pVal: cdrDimensionPlacement,
    ) -> HRESULT,
    fn get_HorizontalText(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_HorizontalText(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ReverseTerminators(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ReverseTerminators(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AutoReverseTerminators(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AutoReverseTerminators(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ReverseTerminatorLength(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_UseReverseTerminatorLength(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseReverseTerminatorLength(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetReverseTerminatorLength(
        Length: f64,
    ) -> HRESULT,
    fn get_InnerDimensionLineVisible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_InnerDimensionLineVisible(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580022, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDimensionAngular(IVGDimensionAngularVtbl): IDispatch(IDispatchVtbl) {
    fn get_Center(
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn put_Center(
        ppVal: *const IVGSnapPoint,
    ) -> HRESULT,
    fn get_Point1(
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn put_Point1(
        ppVal: *const IVGSnapPoint,
    ) -> HRESULT,
    fn get_Point2(
        ppVal: *mut *mut IVGSnapPoint,
    ) -> HRESULT,
    fn put_Point2(
        ppVal: *const IVGSnapPoint,
    ) -> HRESULT,
    fn get_TextX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TextX(
        pVal: f64,
    ) -> HRESULT,
    fn get_TextY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_TextY(
        pVal: f64,
    ) -> HRESULT,
    fn get_ShowUnits(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ShowUnits(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Units(
        pVal: *mut cdrDimensionAngularUnits,
    ) -> HRESULT,
    fn put_Units(
        pVal: cdrDimensionAngularUnits,
    ) -> HRESULT,
    fn get_Clockwise(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Clockwise(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_InnerExtensionLinesVisible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_InnerExtensionLinesVisible(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058006c, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSymbol(IVGSymbolVtbl): IDispatch(IDispatchVtbl) {
    fn get_Definition(
        ppVal: *mut *mut IVGSymbolDefinition,
    ) -> HRESULT,
    fn RevertToShapes(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058006d, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSymbolDefinition(IVGSymbolDefinitionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Description(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Description(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Linked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_LinkLibraryPath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Nested(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_HasLinks(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsLinkBroken(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsLinkUpdated(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_HasBrokenLinks(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_HasUpdatedLinks(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_NestedSymbols(
        ppVal: *mut *mut IVGSymbolDefinitions,
    ) -> HRESULT,
    fn get_Editable(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_InstanceCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Instances(
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn EnterEditMode(
    ) -> HRESULT,
    fn LeaveEditMode(
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn Copy(
    ) -> HRESULT,
    fn Duplicate(
        Name: BSTR,
        ppVal: *mut *mut IVGSymbolDefinition,
    ) -> HRESULT,
    fn BreakLink(
    ) -> HRESULT,
    fn UpdateLinks(
    ) -> HRESULT,
    fn FixLink(
        LibraryPath: BSTR,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrSymbolType,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058006e, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSymbolDefinitions(IVGSymbolDefinitionsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGSymbolDefinition,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580088, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGOLE(IVGOLEVtbl): IDispatch(IDispatchVtbl) {
    fn get_ClassID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_ProgID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrOLEType,
    ) -> HRESULT,
    fn get_FullName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_ShortName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_ServerName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Modified(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsInPlaceActive(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsOpen(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsServerRunning(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_InPlaceWindowHandle(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Edit(
    ) -> HRESULT,
    fn Open(
    ) -> HRESULT,
    fn Activate(
    ) -> HRESULT,
    fn Deactivate(
    ) -> HRESULT,
    fn DoVerb(
        VerbID: i32,
    ) -> HRESULT,
    fn get_IsLinkUpToDate(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_LinkPath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn UpdateLink(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058008f, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTreeNode(IVGTreeNodeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pVal: *mut cdrTreeNodeType,
    ) -> HRESULT,
    fn get_ShapeType(
        pVal: *mut cdrShapeType,
    ) -> HRESULT,
    fn get_Shape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_VirtualShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_Page(
        ppVal: *mut *mut IVGPage,
    ) -> HRESULT,
    fn get_Layer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_Document(
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get_Next(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn get_Previous(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn get_FirstChild(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn get_LastChild(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn get_Children(
        ppVal: *mut *mut IVGTreeNodes,
    ) -> HRESULT,
    fn get_IsGroupChild(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Selected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_NextSelected(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn UnLink(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn LinkBefore(
        TreeNode: *const IVGTreeNode,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn LinkAfter(
        TreeNode: *const IVGTreeNode,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn LinkAsChildOf(
        TreeNode: *const IVGTreeNode,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn MoveToFirst(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn MoveToLast(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn MoveBefore(
        TreeNode: *const IVGTreeNode,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn MoveAfter(
        TreeNode: *const IVGTreeNode,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn IsDescendentOf(
        TreeNode: *const IVGTreeNode,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn SwapData(
        TreeNode: *const IVGTreeNode,
    ) -> HRESULT,
    fn SwapGroupData(
        GroupNode: *const IVGTreeNode,
    ) -> HRESULT,
    fn get_Handle(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pVal: BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580090, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTreeNodes(IVGTreeNodesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800a0, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGEPS(IVGEPSVtbl): IDispatch(IDispatchVtbl) {
    fn get_PreviewBitmap(
        ppVal: *mut *mut IVGBitmap,
    ) -> HRESULT,
    fn get_Data(
        pVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_DataAsString(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CropEnvelope(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn ResetCropEnvelope(
    ) -> HRESULT,
    fn get_CropEnvelopeModified(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_BoundingBoxPath(
        ppVal: *mut *mut IVGCurve,
    ) -> HRESULT,
    fn get_LinkFileName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_LinkFileName(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_DCSFileNames(
        pVal: *mut SAFEARRAY,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800a5, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSpread(IVGSpreadVtbl): IDispatch(IDispatchVtbl) {
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_SizeWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_SizeHeight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_LeftX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_RightX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_TopY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_BottomY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_CenterX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_CenterY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn get_BoundingBox(
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn GetBoundingBox(
        x: *mut f64,
        y: *mut f64,
        Width: *mut f64,
        Height: *mut f64,
    ) -> HRESULT,
    fn get_Pages(
        ppVal: *mut *mut IVGPages,
    ) -> HRESULT,
    fn get_Next(
        ppVal: *mut *mut IVGSpread,
    ) -> HRESULT,
    fn get_Previous(
        ppVal: *mut *mut IVGSpread,
    ) -> HRESULT,
    fn get_Layers(
        ppVal: *mut *mut IVGLayers,
    ) -> HRESULT,
    fn get_AllLayers(
        ppVal: *mut *mut IVGLayers,
    ) -> HRESULT,
    fn get_ActiveLayer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_Shapes(
        ppVal: *mut *mut IVGShapes,
    ) -> HRESULT,
    fn get_SelectableShapes(
        ppVal: *mut *mut IVGShapes,
    ) -> HRESULT,
    fn CreateLayer(
        LayerName: BSTR,
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_Guides(
        Type: cdrGuideType,
        ppVal: *mut *mut IVGShapeRange,
    ) -> HRESULT,
    fn get_TreeNode(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn get_GuidesLayer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_DesktopLayer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_GridLayer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800ad, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGBSpline(IVGBSplineVtbl): IDispatch(IDispatchVtbl) {
    fn get_ControlPoints(
        ppVal: *mut *mut IVGBSplineControlPoints,
    ) -> HRESULT,
    fn get_Closed(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Closed(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn InsertControlPoint(
        Index: i32,
        x: f64,
        y: f64,
        Clamped: VARIANT_BOOL,
    ) -> HRESULT,
    fn InsertControlPoints(
        Index: i32,
        HowMany: i32,
        x: f64,
        y: f64,
        Clamped: VARIANT_BOOL,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGBSpline,
    ) -> HRESULT,
    fn CopyAssign(
        rhs: *const IVGBSpline,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800af, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGBSplineControlPoints(IVGBSplineControlPointsVtbl): IDispatch(IDispatchVtbl) {
    fn get_First(
        ppVal: *mut *mut IVGBSplineControlPoint,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGBSplineControlPoint,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGBSplineControlPoint,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Resize(
        HowMany: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800ae, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGBSplineControlPoint(IVGBSplineControlPointVtbl): IDispatch(IDispatchVtbl) {
    fn get_x(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_x(
        pVal: f64,
    ) -> HRESULT,
    fn get_y(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_y(
        pVal: f64,
    ) -> HRESULT,
    fn get_Clamped(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Clamped(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn Move(
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn GetPosition(
        x: *mut f64,
        y: *mut f64,
    ) -> HRESULT,
    fn SetPosition(
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn SetProperties(
        x: f64,
        y: f64,
        Clamped: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580085, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStructImportOptions(IVGStructImportOptionsVtbl): IDispatch(IDispatchVtbl) {
    f
}}

RIDL!{#[uuid(0xf5200005, 0x8d23, 0x0001, 0x89, 0xe7, 0x00, 0x00, 0x86, 0x1e, 0xbb, 0xd6)]
interface IImportCropHandler(IImportCropHandlerVtbl): IDispatch(IDispatchVtbl) {
    fn Crop(
        Options: *const IStructImportCropOptions,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580087, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IStructImportCropOptions(IStructImportCropOptionsVtbl): IDispatch(IDispatchVtbl) {
    fn get_FileName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_FilterID(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_CustomData(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_ImageWidth(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_ImageHeight(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_DpiX(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_DpiY(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Width(
        pVal: i32,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Height(
        pVal: i32,
    ) -> HRESULT,
    fn get_Left(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Left(
        pVal: i32,
    ) -> HRESULT,
    fn get_Top(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Top(
        pVal: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xf5200004, 0x8d23, 0x0001, 0x89, 0xe7, 0x00, 0x00, 0x86, 0x1e, 0xbb, 0xd6)]
interface IImportResampleHandler(IImportResampleHandlerVtbl): IDispatch(IDispatchVtbl) {
    fn Resample(
        Options: *const IStructImportResampleOptions,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580086, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IStructImportResampleOptions(IStructImportResampleOptionsVtbl): IDispatch(IDispatchVtbl) {
    fn get_FileName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_FilterID(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_CustomData(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Width(
        pVal: i32,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Height(
        pVal: i32,
    ) -> HRESULT,
    fn get_DpiX(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_DpiX(
        pVal: i32,
    ) -> HRESULT,
    fn get_DpiY(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_DpiY(
        pVal: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800b2, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStructColorConversionOptions(IVGStructColorConversionOptionsVtbl): IDispatch(IDispatchVtbl) {
    fn get_ColorPolicy(
        ppVal: *mut *mut IVGColorManagementPolicy,
    ) -> HRESULT,
    fn get_SourceColorProfileList(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_SourceColorProfileList(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_TargetColorProfileList(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_TargetColorProfileList(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_ColorConversionHandler(
        ppVal: *mut *mut IColorConversionHandler,
    ) -> HRESULT,
    fn putref_ColorConversionHandler(
        ppVal: *const *const IColorConversionHandler,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800b1, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGColorManagementPolicy(IVGColorManagementPolicyVtbl): IDispatch(IDispatchVtbl) {
    fn get_ActionForRGB(
        pVal: *mut clrColorPolicyAction,
    ) -> HRESULT,
    fn put_ActionForRGB(
        pVal: clrColorPolicyAction,
    ) -> HRESULT,
    fn get_ActionForCMYK(
        pVal: *mut clrColorPolicyAction,
    ) -> HRESULT,
    fn put_ActionForCMYK(
        pVal: clrColorPolicyAction,
    ) -> HRESULT,
    fn get_ActionForGrayscale(
        pVal: *mut clrColorPolicyAction,
    ) -> HRESULT,
    fn put_ActionForGrayscale(
        pVal: clrColorPolicyAction,
    ) -> HRESULT,
    fn get_WarnOnMismatchedProfiles(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_WarnOnMismatchedProfiles(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_WarnOnMissingProfiles(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_WarnOnMissingProfiles(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x7cb12d17, 0xeca6, 0x0e87, 0x46, 0xf7, 0xe4, 0xd3, 0x1c, 0x0d, 0x05, 0x00)]
interface IColorConversionHandler(IColorConversionHandlerVtbl): IDispatch(IDispatchVtbl) {
}}

RIDL!{#[uuid(0xb0580070, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSymbolLibrary(IVGSymbolLibraryVtbl): IDispatch(IDispatchVtbl) {
    fn get_Symbols(
        ppVal: *mut *mut IVGSymbolDefinitions,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_FilePath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_ReadOnly(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn PurgeUnusedSymbols(
    ) -> HRESULT,
    fn Paste(
        Name: BSTR,
        ppVal: *mut *mut IVGSymbolDefinition,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800b5, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStructPasteOptions(IVGStructPasteOptionsVtbl): IDispatch(IDispatchVtbl) {
    fn get_ColorConversionOptions(
        ppVal: *mut *mut IVGStructColorConversionOptions,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580081, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGWindow(IVGWindowVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGWindows,
    ) -> HRESULT,
    fn Activate(
    ) -> HRESULT,
    fn Close(
    ) -> HRESULT,
    fn get_FullScreen(
        FullScreen: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FullScreen(
        FullScreen: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Page(
        Page: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Active(
        pActive: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Caption(
        pbstrName: *mut BSTR,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Height(
        pVal: i32,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Width(
        pVal: i32,
    ) -> HRESULT,
    fn get_Left(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Left(
        pVal: i32,
    ) -> HRESULT,
    fn get_Top(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Top(
        pVal: i32,
    ) -> HRESULT,
    fn get_WindowState(
        pVal: *mut cdrWindowState,
    ) -> HRESULT,
    fn put_WindowState(
        pVal: cdrWindowState,
    ) -> HRESULT,
    fn get_Previous(
        pWindow: *mut *mut IVGWindow,
    ) -> HRESULT,
    fn get_Next(
        pWindow: *mut *mut IVGWindow,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn NewWindow(
        pWindow: *mut *mut IVGWindow,
    ) -> HRESULT,
    fn Refresh(
    ) -> HRESULT,
    fn get_Document(
        pDoc: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get_ActiveView(
        pActiveView: *mut *mut IVGActiveView,
    ) -> HRESULT,
    fn ScreenToDocument(
        XScreen: i32,
        YScreen: i32,
        XDoc: *mut f64,
        YDoc: *mut f64,
    ) -> HRESULT,
    fn DocumentToScreen(
        XDoc: f64,
        YDoc: f64,
        XScreen: *mut i32,
        YScreen: *mut i32,
    ) -> HRESULT,
    fn get_Handle(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_ViewWindow(
        ppVal: *mut *mut ICUIViewWindow,
    ) -> HRESULT,
    fn ScreenDistanceToDocumentDistance(
        ScreenDistance: f64,
        DocumentDistance: *mut f64,
    ) -> HRESULT,
    fn DocumentDistanceToScreenDistance(
        DocumentDistance: f64,
        ScreenDistance: *mut f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580082, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGWindows(IVGWindowsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGWindow,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn CloseAll(
    ) -> HRESULT,
    fn Arrange(
        Style: cdrWindowArrangeStyle,
    ) -> HRESULT,
    fn Refresh(
    ) -> HRESULT,
    fn FindWindow(
        Caption: BSTR,
        ppVal: *mut *mut IVGWindow,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058000a, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGActiveView(IVGActiveViewVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGWindow,
    ) -> HRESULT,
    fn get_Type(
        pType: *mut cdrViewType,
    ) -> HRESULT,
    fn put_Type(
        pType: cdrViewType,
    ) -> HRESULT,
    fn get_OriginX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OriginX(
        pVal: f64,
    ) -> HRESULT,
    fn get_OriginY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OriginY(
        pVal: f64,
    ) -> HRESULT,
    fn get_Zoom(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Zoom(
        pVal: f64,
    ) -> HRESULT,
    fn ToFitPage(
    ) -> HRESULT,
    fn ToFitPageWidth(
    ) -> HRESULT,
    fn ToFitPageHeight(
    ) -> HRESULT,
    fn ToFitShape(
        Shape: *const IVGShape,
    ) -> HRESULT,
    fn ToFitSelection(
    ) -> HRESULT,
    fn ToFitArea(
        Left: f64,
        Top: f64,
        Right: f64,
        Bottom: f64,
    ) -> HRESULT,
    fn ToFitAllObjects(
    ) -> HRESULT,
    fn ToFitShapeRange(
        ShapeRange: *const IVGShapeRange,
    ) -> HRESULT,
    fn SetViewPoint(
        x: f64,
        y: f64,
        Zoom: f64,
    ) -> HRESULT,
    fn SetActualSize(
    ) -> HRESULT,
    fn ZoomIn(
    ) -> HRESULT,
    fn ZoomInAtPoint(
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn ZoomOut(
    ) -> HRESULT,
    fn GetViewArea(
        x: *mut f64,
        y: *mut f64,
        Width: *mut f64,
        Height: *mut f64,
    ) -> HRESULT,
    fn SetViewArea(
        x: f64,
        y: f64,
        Width: f64,
        Height: f64,
    ) -> HRESULT,
    fn get_SimulateOverprints(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_SimulateOverprints(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ProofColorSettings(
        ppVal: *mut *mut IVGProofColorSettings,
    ) -> HRESULT,
    fn put_ProofColorSettings(
        ppVal: *const IVGProofColorSettings,
    ) -> HRESULT,
    fn get_ShowProofColors(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ShowProofColors(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800b6, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGProofColorSettings(IVGProofColorSettingsVtbl): IDispatch(IDispatchVtbl) {
    fn get_ColorContext(
        ppVal: *mut *mut IVGColorContext,
    ) -> HRESULT,
    fn put_ColorContext(
        ppVal: *const IVGColorContext,
    ) -> HRESULT,
    fn get_ShowOutOfGamutWarning(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ShowOutOfGamutWarning(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_OutOfGamutColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_OutOfGamutColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_OutOfGamutTransparency(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OutOfGamutTransparency(
        pVal: f64,
    ) -> HRESULT,
    fn get_PreserveColorValues(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PreserveColorValues(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn GetCopy(
        ppVal: *mut *mut IVGProofColorSettings,
    ) -> HRESULT,
    fn CopyAssign(
        Source: *const IVGProofColorSettings,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580000, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface ICorelScriptTools(ICorelScriptToolsVtbl): IDispatch(IDispatchVtbl) {
    fn AngleConvert(
        FromUnit: i32,
        ToUnit: i32,
        Value: f64,
        Result: *mut f64,
    ) -> HRESULT,
    fn ASin(
        Value: f64,
        Result: *mut f64,
    ) -> HRESULT,
    fn BeginWaitCursor(
    ) -> HRESULT,
    fn EndWaitCursor(
    ) -> HRESULT,
    fn BuildDate(
        Year: i32,
        Month: i32,
        Day: i32,
        pVal: *mut DATE,
    ) -> HRESULT,
    fn BuildTime(
        Hour: i32,
        Minute: i32,
        Second: i32,
        pVal: *mut DATE,
    ) -> HRESULT,
    fn Dec(
        Hex: BSTR,
        pVal: *mut i32,
    ) -> HRESULT,
    fn FileAttr(
        FolderFile: BSTR,
        pVal: *mut i32,
    ) -> HRESULT,
    fn FindFirstFolder(
        SearchCriteria: BSTR,
        Attributes: i32,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn FindNextFolder(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn FormatTime(
        Time: DATE,
        Format: BSTR,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn FromCentimeters(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn FromCiceros(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn FromDidots(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn FromInches(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn FromPicas(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn FromPoints(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn GetAppHandle(
        pVal: *mut i32,
    ) -> HRESULT,
    fn GetColor(
        Red: *mut i32,
        Green: *mut i32,
        Blue: *mut i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetCommandLine(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn GetCurrFolder(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn GetDateInfo(
        Date: DATE,
        Year: *mut i32,
        Month: *mut i32,
        Day: *mut i32,
        DayOfWeek: *mut i32,
    ) -> HRESULT,
    fn GetFileBox(
        Filter: BSTR,
        Title: BSTR,
        Type: i32,
        File: BSTR,
        Extension: BSTR,
        Folder: BSTR,
        Button: BSTR,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn GetFolder(
        InitFolder: BSTR,
        Title: BSTR,
        ParentWindowHandle: i32,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn GetFont(
        FaceName: *mut BSTR,
        PointSize: *mut i32,
        Weight: *mut i32,
        Italic: *mut VARIANT_BOOL,
        Underline: *mut VARIANT_BOOL,
        StrikeOut: *mut VARIANT_BOOL,
        Red: *mut i32,
        Green: *mut i32,
        Blue: *mut i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetProcessInfo(
        ProcessHandle: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn GetScriptFolder(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn GetTempFolder(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn GetTimeInfo(
        Time: DATE,
        Hour: *mut i32,
        Minute: *mut i32,
        Second: *mut i32,
    ) -> HRESULT,
    fn GetType(
        Expression: VARIANT,
        Type: *mut i32,
    ) -> HRESULT,
    fn GetVersion(
        Option: i32,
        pVal: *mut i32,
    ) -> HRESULT,
    fn GetWinHandle(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Kill(
        FileName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn LengthConvert(
        FromUnit: i32,
        ToUnit: i32,
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn Log(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn MkFolder(
        Folder: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn RegistryQuery(
        MainKey: i32,
        SubKey: BSTR,
        Value: BSTR,
        pVal: *mut VARIANT,
    ) -> HRESULT,
    fn Rename(
        Src: BSTR,
        Dst: BSTR,
        Overwrite: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn RmFolder(
        Folder: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ToCentimeters(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn ToCiceros(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn ToDidots(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn ToInches(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn ToPicas(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
    fn ToPoints(
        Value: f64,
        pVal: *mut f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580083, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGWorkspace(IVGWorkspaceVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGWorkspaces,
    ) -> HRESULT,
    fn get_Name(
        Name: *mut BSTR,
    ) -> HRESULT,
    fn get_Description(
        Description: *mut BSTR,
    ) -> HRESULT,
    // fn get_Default(
    //     Default: *mut VARIANT_BOOL,
    // ) -> HRESULT,
    fn Activate(
    ) -> HRESULT,
    fn get_Active(
        Current: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580084, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGWorkspaces(IVGWorkspacesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGWorkspace,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058004d, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPalettes(IVGPalettesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGPalette,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Open(
        FileName: BSTR,
        ppPalette: *mut *mut IVGPalette,
    ) -> HRESULT,
    fn OpenFixed(
        PaletteID: cdrPaletteID,
        ppPalette: *mut *mut IVGPalette,
    ) -> HRESULT,
    fn CreateFromDocument(
        Name: BSTR,
        FileName: BSTR,
        Overwrite: VARIANT_BOOL,
        ppPalette: *mut *mut IVGPalette,
    ) -> HRESULT,
    fn CreateFromSelection(
        Name: BSTR,
        FileName: BSTR,
        Overwrite: VARIANT_BOOL,
        ppPalette: *mut *mut IVGPalette,
    ) -> HRESULT,
    fn Create(
        Name: BSTR,
        FileName: BSTR,
        Overwrite: VARIANT_BOOL,
        ppPalette: *mut *mut IVGPalette,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580039, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGFontList(IVGFontListVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058000c, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGAppWindow(IVGAppWindowVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn Activate(
    ) -> HRESULT,
    fn get_Active(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Caption(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Caption(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Height(
        pVal: i32,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Width(
        pVal: i32,
    ) -> HRESULT,
    fn get_Left(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Left(
        pVal: i32,
    ) -> HRESULT,
    fn get_Top(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Top(
        pVal: i32,
    ) -> HRESULT,
    fn get_WindowState(
        pVal: *mut cdrWindowState,
    ) -> HRESULT,
    fn put_WindowState(
        pVal: cdrWindowState,
    ) -> HRESULT,
    fn get_ClientWidth(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_ClientHeight(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Handle(
        pVal: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580056, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGRecentFiles(IVGRecentFilesVtbl): IDispatch(IDispatchVtbl) {
    fn Add(
        Name: BSTR,
        Path: BSTR,
        ppVal: *mut *mut IVGRecentFile,
    ) -> HRESULT,
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGRecentFile,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Maximum(
        pVal: *mut i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580055, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGRecentFile(IVGRecentFileVtbl): IDispatch(IDispatchVtbl) {
    fn Delete(
    ) -> HRESULT,
    fn Open(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGRecentFiles,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Path(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Path(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_FullName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_FullName(
        pVal: BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058000e, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGArrowHeads(IVGArrowHeadsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGArrowHead,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Remove(
        Index: i32,
    ) -> HRESULT,
    fn Add(
        ArrowHead: *const IVGArrowHead,
        ppVal: *mut *mut IVGArrowHead,
    ) -> HRESULT,
    fn Replace(
        Index: i32,
        ArrowHead: *const IVGArrowHead,
        ppVal: *mut *mut IVGArrowHead,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580047, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGOutlineStyles(IVGOutlineStylesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGOutlineStyle,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Add(
        ppVal: *mut *mut IVGOutlineStyle,
    ) -> HRESULT,
    fn Remove(
        Index: i32,
    ) -> HRESULT,
    fn Save(
    ) -> HRESULT,
    fn AddStyle(
        Style: *const IVGOutlineStyle,
        ppVal: *mut *mut IVGOutlineStyle,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058004f, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPatternCanvases(IVGPatternCanvasesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGPatternCanvas,
    ) -> HRESULT,
    fn put_Item(
        Index: i32,
        ppVal: *const IVGPatternCanvas,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Add(
        PatternCanvas: *const IVGPatternCanvas,
        pVal: *mut i32,
    ) -> HRESULT,
    fn Remove(
        Index: i32,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580010, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGClipboard(IVGClipboardVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut LPDISPATCH,
    ) -> HRESULT,
    fn get_Valid(
        pbValid: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Empty(
        pbEmpty: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Clear(
    ) -> HRESULT,
    fn DataPresent(
        FormatName: BSTR,
        pRet: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058003d, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGGMSManager(IVGGMSManagerVtbl): IDispatch(IDispatchVtbl) {
    fn get_GMSPath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn RunMacro(
        ModuleName: BSTR,
        MacroName: BSTR,
        Parameters: *mut SAFEARRAY,
        ppVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_UserGMSPath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Projects(
        ppVal: *mut *mut IVGGMSProjects,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058008a, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGGMSProjects(IVGGMSProjectsVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGGMSProject,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Load(
        FileName: BSTR,
        CopyFile: VARIANT_BOOL,
        ForAllUsers: VARIANT_BOOL,
        ppVal: *mut *mut IVGGMSProject,
    ) -> HRESULT,
    fn Create(
        Name: BSTR,
        ForAllUsers: VARIANT_BOOL,
        FileName: BSTR,
        ppVal: *mut *mut IVGGMSProject,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058008b, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGGMSProject(IVGGMSProjectVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_DisplayName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Macros(
        ppVal: *mut *mut IVGGMSMacros,
    ) -> HRESULT,
    fn Unload(
    ) -> HRESULT,
    fn get_FileName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_FilePath(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_FullFileName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Dirty(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Dirty(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Locked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PasswordProtected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058008c, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGGMSMacros(IVGGMSMacrosVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGGMSMacro,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Create(
        Name: BSTR,
        ppVal: *mut *mut IVGGMSMacro,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058008d, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGGMSMacro(IVGGMSMacroVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn Run(
    ) -> HRESULT,
    fn Edit(
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580068, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStructSaveAsOptions(IVGStructSaveAsOptionsVtbl): IDispatch(IDispatchVtbl) {
    fn put_Filter(
        pVal: cdrFilter,
    ) -> HRESULT,
    fn get_Filter(
        pVal: *mut cdrFilter,
    ) -> HRESULT,
    fn put_Version(
        pVal: cdrFileVersion,
    ) -> HRESULT,
    fn get_Version(
        pVal: *mut cdrFileVersion,
    ) -> HRESULT,
    fn put_ThumbnailSize(
        pVal: cdrThumbnailSize,
    ) -> HRESULT,
    fn get_ThumbnailSize(
        pVal: *mut cdrThumbnailSize,
    ) -> HRESULT,
    fn put_Range(
        pVal: cdrExportRange,
    ) -> HRESULT,
    fn get_Range(
        pVal: *mut cdrExportRange,
    ) -> HRESULT,
    fn put_Overwrite(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Overwrite(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_EmbedICCProfile(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_EmbedICCProfile(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_EmbedVBAProject(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_EmbedVBAProject(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_IncludeCMXData(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IncludeCMXData(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_KeepAppearance(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_KeepAppearance(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580063, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStructExportOptions(IVGStructExportOptionsVtbl): IDispatch(IDispatchVtbl) {
    fn put_SizeX(
        pVal: i32,
    ) -> HRESULT,
    fn get_SizeX(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_SizeY(
        pVal: i32,
    ) -> HRESULT,
    fn get_SizeY(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_ResolutionX(
        pVal: i32,
    ) -> HRESULT,
    fn get_ResolutionX(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_ResolutionY(
        pVal: i32,
    ) -> HRESULT,
    fn get_ResolutionY(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_AntiAliasingType(
        pVal: cdrAntiAliasingType,
    ) -> HRESULT,
    fn get_AntiAliasingType(
        pVal: *mut cdrAntiAliasingType,
    ) -> HRESULT,
    fn put_Overwrite(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Overwrite(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ImageType(
        pVal: cdrImageType,
    ) -> HRESULT,
    fn get_ImageType(
        pVal: *mut cdrImageType,
    ) -> HRESULT,
    fn put_Dithered(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Dithered(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Transparent(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Transparent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseColorProfile(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_UseColorProfile(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Compression(
        pVal: cdrCompressionType,
    ) -> HRESULT,
    fn get_Compression(
        pVal: *mut cdrCompressionType,
    ) -> HRESULT,
    fn put_MaintainLayers(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MaintainLayers(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MaintainAspect(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MaintainAspect(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ExportArea(
        ppVal: *mut *mut IVGRect,
    ) -> HRESULT,
    fn putref_ExportArea(
        ppVal: *const *const IVGRect,
    ) -> HRESULT,
    fn put_Matted(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Matted(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MatteColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_MatteColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn put_MatteMaskedOnly(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MatteMaskedOnly(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AlwaysOverprintBlack(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AlwaysOverprintBlack(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ProofColorSettings(
        ppVal: *mut *mut IVGProofColorSettings,
    ) -> HRESULT,
    fn put_ProofColorSettings(
        ppVal: *const IVGProofColorSettings,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580015, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGComponents(IVGComponentsVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGComponent,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn IsComponentInstalled(
        ComponentID: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580014, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGComponent(IVGComponentVtbl): IDispatch(IDispatchVtbl) {
    fn get_ComponentID(
        pVal: *mut BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058006f, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSymbolLibraries(IVGSymbolLibrariesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGSymbolLibrary,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Add(
        FileName: BSTR,
        CopyLocally: VARIANT_BOOL,
        ppVal: *mut *mut IVGSymbolLibrary,
    ) -> HRESULT,
    fn AddFromFolder(
        Folder: BSTR,
        Recursive: VARIANT_BOOL,
        CopyLocally: VARIANT_BOOL,
        pVal: *mut i32,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580089, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGAppStatus(IVGAppStatusVtbl): IDispatch(IDispatchVtbl) {
    fn BeginProgress(
        Message: BSTR,
        CanAbort: VARIANT_BOOL,
    ) -> HRESULT,
    fn UpdateProgress(
        Step: i32,
    ) -> HRESULT,
    fn SetProgressMessage(
        Message: BSTR,
    ) -> HRESULT,
    fn EndProgress(
    ) -> HRESULT,
    fn get_Progress(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Progress(
        pVal: i32,
    ) -> HRESULT,
    fn get_Aborted(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058004b, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPageSizes(IVGPageSizesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Parent(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGPageSize,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Add(
        Name: BSTR,
        Width: f64,
        Height: f64,
        ppVal: *mut *mut IVGPageSize,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058004a, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPageSize(IVGPageSizeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Parent(
        ppVal: *mut *mut IVGPageSizes,
    ) -> HRESULT,
    fn get_BuiltIn(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Width(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Width(
        pVal: f64,
    ) -> HRESULT,
    fn get_Height(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Height(
        pVal: f64,
    ) -> HRESULT,
    fn get_Index(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
    fn get_FixedOrientation(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DefaultUnit(
        pVal: *mut cdrUnit,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058008e, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGTreeManager(IVGTreeManagerVtbl): IDispatch(IDispatchVtbl) {
    fn get_VirtualLayer(
        ppVal: *mut *mut IVGLayer,
    ) -> HRESULT,
    fn get_SelectedNodeCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_FirstSelectedNode(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn CreateGroupNode(
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
    fn CleanGroupNode(
        GroupNode: *const IVGTreeNode,
        ppVal: *mut *mut IVGTreeNode,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580097, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGColorManager(IVGColorManagerVtbl): IDispatch(IDispatchVtbl) {
    fn get_ScannerCalibrated(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ScannerCalibrated(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_SeparationPrinterCalibrated(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_SeparationPrinterCalibrated(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CompositePrinterCalibrated(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CompositePrinterCalibration(
        pVal: *mut clrCompPrnCalibration,
    ) -> HRESULT,
    fn put_CompositePrinterCalibration(
        pVal: clrCompPrnCalibration,
    ) -> HRESULT,
    fn get_MonitorCalibration(
        pVal: *mut clrMonitorCalibration,
    ) -> HRESULT,
    fn put_MonitorCalibration(
        pVal: clrMonitorCalibration,
    ) -> HRESULT,
    fn get_CompositePrinterSimulatesSeparation(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ShowOutOfGamut(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_ShowOutOfGamut(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_OutOfGamutColor(
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn put_OutOfGamutColor(
        ppVal: *const IVGColor,
    ) -> HRESULT,
    fn get_OutOfGamutTransparency(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_OutOfGamutTransparency(
        pVal: i32,
    ) -> HRESULT,
    fn get_CMYKInPercents(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_CMYKInPercents(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CMYKGamutForSpotColors(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_CMYKGamutForSpotColors(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RenderingIntent(
        pVal: *mut clrRenderingIntent,
    ) -> HRESULT,
    fn put_RenderingIntent(
        pVal: clrRenderingIntent,
    ) -> HRESULT,
    fn get_ColorEngine(
        pVal: *mut clrColorEngine,
    ) -> HRESULT,
    fn put_ColorEngine(
        pVal: clrColorEngine,
    ) -> HRESULT,
    fn get_StyleCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_StyleByIndex(
        Index: i32,
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_CurrentProfile(
        DeviceType: clrDeviceType,
        ppVal: *mut *mut IVGColorProfile,
    ) -> HRESULT,
    fn get_InstalledProfiles(
        DeviceType: clrDeviceType,
        ppVal: *mut *mut IVGColorProfiles,
    ) -> HRESULT,
    fn LoadStyle(
        StyleName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn DeleteStyle(
        StyleName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn SaveStyle(
        StyleName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsICM2Available(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_UnsavedStyleName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn get_GenericProfileName(
        DeviceType: clrDeviceType,
        ppVal: *mut BSTR,
    ) -> HRESULT,
    fn get_IsCompositePrinterCMYK(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ColorCorrectionOnImport(
        pVal: *mut clrImportColorCorrection,
    ) -> HRESULT,
    fn put_ColorCorrectionOnImport(
        pVal: clrImportColorCorrection,
    ) -> HRESULT,
    fn get_ColorCorrectionOnExport(
        pVal: *mut clrExportColorCorrection,
    ) -> HRESULT,
    fn put_ColorCorrectionOnExport(
        pVal: clrExportColorCorrection,
    ) -> HRESULT,
    fn get_DefaultImportProfile(
        ppVal: *mut *mut IVGColorProfile,
    ) -> HRESULT,
    fn putref_DefaultImportProfile(
        ppVal: *const *const IVGColorProfile,
    ) -> HRESULT,
    fn get_CustomImportProfile(
        ppVal: *mut *mut IVGColorProfile,
    ) -> HRESULT,
    fn putref_CustomImportProfile(
        ppVal: *const *const IVGColorProfile,
    ) -> HRESULT,
    fn get_CustomExportProfile(
        ppVal: *mut *mut IVGColorProfile,
    ) -> HRESULT,
    fn putref_CustomExportProfile(
        ppVal: *const *const IVGColorProfile,
    ) -> HRESULT,
    fn get_MonitorColorProfiles(
        ppVal: *mut *mut IVGColorProfiles,
    ) -> HRESULT,
    fn GetProfilesByColorModel(
        ColorModel: clrColorModel,
        ppVal: *mut *mut IVGColorProfiles,
    ) -> HRESULT,
    fn GetProfilesForDevice(
        DeviceType: clrDeviceType,
        DeviceName: BSTR,
        ppVal: *mut *mut IVGColorProfiles,
    ) -> HRESULT,
    fn get_DefaultColorContext(
        ppVal: *mut *mut IVGColorContext,
    ) -> HRESULT,
    fn ColorEnginePresent(
        ColorEngine: clrColorEngine,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn CanDeleteStyle(
        StyleName: BSTR,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MapGrayToCMYKBlack(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MapGrayToCMYKBlack(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PreservePureBlack(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PreservePureBlack(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_SpotColorDefinition(
        pVal: *mut clrColorModel,
    ) -> HRESULT,
    fn put_SpotColorDefinition(
        pVal: clrColorModel,
    ) -> HRESULT,
    fn get_ColorProfiles(
        ppVal: *mut *mut IVGColorProfiles,
    ) -> HRESULT,
    fn get_PolicyForOpen(
        ppVal: *mut *mut IVGColorManagementPolicy,
    ) -> HRESULT,
    fn get_PolicyForImport(
        ppVal: *mut *mut IVGColorManagementPolicy,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800b3, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStructOpenOptions(IVGStructOpenOptionsVtbl): IDispatch(IDispatchVtbl) {
    fn get_CodePage(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_CodePage(
        pVal: i32,
    ) -> HRESULT,
    fn get_ColorConversionOptions(
        ppVal: *mut *mut IVGStructColorConversionOptions,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800b4, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStructCreateOptions(IVGStructCreateOptionsVtbl): IDispatch(IDispatchVtbl) {
    fn get_ColorContext(
        ppVal: *mut *mut IVGColorContext,
    ) -> HRESULT,
    fn put_ColorContext(
        ppVal: *const IVGColorContext,
    ) -> HRESULT,
    fn get_Name(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_PageWidth(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PageWidth(
        pVal: f64,
    ) -> HRESULT,
    fn get_PageHeight(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_PageHeight(
        pVal: f64,
    ) -> HRESULT,
    fn get_Units(
        pVal: *mut cdrUnit,
    ) -> HRESULT,
    fn put_Units(
        pVal: cdrUnit,
    ) -> HRESULT,
    fn get_Resolution(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Resolution(
        pVal: f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800b7, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGPaletteManager(IVGPaletteManagerVtbl): IDispatch(IDispatchVtbl) {
    fn get_PaletteCount(
        pVal: *mut i32,
    ) -> HRESULT,
    fn GetPalette(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGPalette,
    ) -> HRESULT,
    fn get_DefaultPalette(
        ppVal: *mut *mut IVGPalette,
    ) -> HRESULT,
    fn get_OpenPalettes(
        ppVal: *mut *mut IVGPalettes,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn LoadAllPalettes(
    ) -> HRESULT,
    fn LoadPalette(
        FileName: BSTR,
        ppVal: *mut *mut IVGPalette,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800cf, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGToolState(IVGToolStateVtbl): IDispatch(IDispatchVtbl) {
    fn OnStartState(
        StateAttributes: *const IVGToolStateAttributes,
    ) -> HRESULT,
    fn OnExitState(
    ) -> HRESULT,
    fn OnMouseMove(
        pt: *const IVGPoint,
    ) -> HRESULT,
    fn OnLButtonDown(
        pt: *const IVGPoint,
    ) -> HRESULT,
    fn OnLButtonDownLeaveGrace(
        pt: *const IVGPoint,
    ) -> HRESULT,
    fn OnLButtonUp(
        pt: *const IVGPoint,
    ) -> HRESULT,
    fn OnLButtonDblClick(
        pt: *const IVGPoint,
    ) -> HRESULT,
    fn OnClick(
        pt: *const IVGPoint,
        Handled: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn OnRButtonDown(
        pt: *const IVGPoint,
        Handled: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn OnRButtonUp(
        pt: *const IVGPoint,
        Handled: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn OnKeyDown(
        KeyCode: i32,
        Handled: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn OnKeyUp(
        KeyCode: i32,
        Handled: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn OnDelete(
        Handled: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn OnAbort(
    ) -> HRESULT,
    fn OnCommit(
        pt: *const IVGPoint,
    ) -> HRESULT,
    fn OnSnapMouse(
        pt: *mut IVGPoint,
        Handled: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn OnTimer(
        TimerId: i32,
        TimeEllapsed: i32,
    ) -> HRESULT,
    fn get_IsDrawing(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800ce, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGToolStateAttributes(IVGToolStateAttributesVtbl): IDispatch(IDispatchVtbl) {
    fn get_PropertyBarGuid(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_PropertyBarGuid(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_ContextMenuGuid(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_ContextMenuGuid(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_UseTabletPressure(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseTabletPressure(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AllowTempPickState(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AllowTempPickState(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AllowAutopan(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AllowAutopan(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AllowContextMenu(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AllowContextMenu(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanUpdateSelectionOnMouseClick(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_CanUpdateSelectionOnMouseClick(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DeselectOnLButtonDown(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DeselectOnLButtonDown(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_EnterGraceStateOnLButtonDown(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_EnterGraceStateOnLButtonDown(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn put_StatusInfo(
        value: BSTR,
    ) -> HRESULT,
    fn SetCursor(
        CursorShape: cdrCursorShape,
    ) -> HRESULT,
    fn StartTimer(
        TimerId: i32,
        TimeToTick: i32,
        OneTime: VARIANT_BOOL,
    ) -> HRESULT,
    fn StopTimer(
        TimerId: i32,
    ) -> HRESULT,
    fn SnapMouse(
        pt: *mut IVGPoint,
    ) -> HRESULT,
    fn AnchoredSnapMouse(
        pt: *mut IVGPoint,
        AnchorPoint: *const IVGPoint,
    ) -> HRESULT,
    fn ConstrainMouse(
        pt: *mut IVGPoint,
        AnchorPoint: *const IVGPoint,
    ) -> HRESULT,
    fn IsKeyDown(
        KeyCode: i32,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CurrentPressure(
        pVal: *mut f64,
    ) -> HRESULT,
    fn SetCursorGuid(
        newVal: BSTR,
    ) -> HRESULT,
    fn SetStateHintsPage(
        newVal: BSTR,
    ) -> HRESULT,
    fn ExitTemporaryToolState(
    ) -> HRESULT,
    fn get_Document(
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn SetFocus(
    ) -> HRESULT,
    fn CaptureMouse(
    ) -> HRESULT,
    fn ReleaseMouse(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800cb, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGOnScreenCurve(IVGOnScreenCurveVtbl): IDispatch(IDispatchVtbl) {
    fn Show(
    ) -> HRESULT,
    fn Hide(
    ) -> HRESULT,
    fn SetPen(
        Color: i32,
        WidthInPixels: i32,
        Style: cdrOnScreenCurvePenStyle,
    ) -> HRESULT,
    fn SetNoPen(
    ) -> HRESULT,
    fn SetBrush(
        Color: i32,
    ) -> HRESULT,
    fn SetNoBrush(
    ) -> HRESULT,
    fn SetCurve(
        Curve: *const IVGCurve,
    ) -> HRESULT,
    fn SetLine(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> HRESULT,
    fn SetRectangle(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> HRESULT,
    fn SetCircle(
        CenterX: f64,
        CenterY: f64,
        Radius: f64,
    ) -> HRESULT,
    fn SetPoints(
        Points: *const IVGPointRange,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800cc, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGOnScreenHandle(IVGOnScreenHandleVtbl): IDispatch(IDispatchVtbl) {
    fn Show(
    ) -> HRESULT,
    fn Hide(
    ) -> HRESULT,
    fn SetHandleColor(
        Color: i32,
    ) -> HRESULT,
    fn SetPosition(
        x: f64,
        y: f64,
    ) -> HRESULT,
    fn UpdateHotTracking(
        MouseX: f64,
        MouseY: f64,
    ) -> HRESULT,
    fn get_IsHotTracked(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsOnHandle(
        MouseX: f64,
        MouseY: f64,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800cd, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGOnScreenText(IVGOnScreenTextVtbl): IDispatch(IDispatchVtbl) {
    fn Show(
    ) -> HRESULT,
    fn Hide(
    ) -> HRESULT,
    fn SetTextColor(
        Color: i32,
    ) -> HRESULT,
    fn SetTextAndPosition(
        Text: BSTR,
        x: f64,
        y: f64,
        align: cdrOnScreenTextAlign,
        xRef: f64,
        yRef: f64,
    ) -> HRESULT,
    fn SetText(
        Text: BSTR,
    ) -> HRESULT,
    fn SetPixelOffset(
        x: i32,
        y: i32,
    ) -> HRESULT,
    fn UpdatePosition(
        x: f64,
        y: f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800d4, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGMathUtils(IVGMathUtilsVtbl): IDispatch(IDispatchVtbl) {
    fn Interpolate(
        S: *const IVGPoint,
        E: *const IVGPoint,
        t: f64,
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
    fn IntersectLineSegments(
        S1: *const IVGPoint,
        E1: *const IVGPoint,
        S2: *const IVGPoint,
        E2: *const IVGPoint,
        ppVal: *mut *mut IVGPoint,
        pResult: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn DistanceToLineSegment(
        S: *const IVGPoint,
        E: *const IVGPoint,
        Point: *const IVGPoint,
        ppVal: *mut f64,
    ) -> HRESULT,
    fn ClosestPointToLineSegment(
        S: *const IVGPoint,
        E: *const IVGPoint,
        Point: *const IVGPoint,
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
    fn IntersectInfiniteLines(
        S1: *const IVGPoint,
        E1: *const IVGPoint,
        S2: *const IVGPoint,
        E2: *const IVGPoint,
        ppVal: *mut *mut IVGPoint,
        pResult: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn DistanceToInfiniteLine(
        S: *const IVGPoint,
        E: *const IVGPoint,
        Point: *const IVGPoint,
        ppVal: *mut f64,
    ) -> HRESULT,
    fn ClosestPointToInfiniteLine(
        S: *const IVGPoint,
        E: *const IVGPoint,
        Point: *const IVGPoint,
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
    fn GetRandomReal(
        low: f64,
        High: f64,
        ppVal: *mut f64,
    ) -> HRESULT,
    fn GetRandomInteger(
        low: i32,
        High: i32,
        ppVal: *mut i32,
    ) -> HRESULT,
    fn FitLineToPoints(
        Points: *const IVGPointRange,
        Origin: *mut *mut IVGPoint,
        Direction: *mut *mut IVGVector,
    ) -> HRESULT,
    fn MidPoint(
        S: *const IVGPoint,
        E: *const IVGPoint,
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
    fn CreatePoint(
        x: f64,
        y: f64,
        ppVal: *mut *mut IVGPoint,
    ) -> HRESULT,
    fn CreateVector(
        x: f64,
        y: f64,
        ppVal: *mut *mut IVGVector,
    ) -> HRESULT,
    fn CreatePointRange(
        ppVal: *mut *mut IVGPointRange,
    ) -> HRESULT,
    fn CreateIdentityTransformMatrix(
        ppVal: *mut *mut IVGTransformMatrix,
    ) -> HRESULT,
    fn CreateRotationTransformMatrix(
        Angle: f64,
        OriginX: f64,
        OriginY: f64,
        ppVal: *mut *mut IVGTransformMatrix,
    ) -> HRESULT,
    fn CreateTranslationTransformMatrix(
        TranslateX: f64,
        TranslateY: f64,
        ppVal: *mut *mut IVGTransformMatrix,
    ) -> HRESULT,
    fn CreateScaleTransformMatrix(
        ScaleX: f64,
        ScaleY: f64,
        OriginX: f64,
        OriginY: f64,
        ppVal: *mut *mut IVGTransformMatrix,
    ) -> HRESULT,
    fn CreateLineSegmentTransformMatrix(
        FromStart: *const IVGPoint,
        FromEnd: *const IVGPoint,
        ToStart: *const IVGPoint,
        ToEnd: *const IVGPoint,
        ppVal: *mut *mut IVGTransformMatrix,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800d5, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGToolShapeAttributes(IVGToolShapeAttributesVtbl): IDispatch(IDispatchVtbl) {
    fn SetCanResize(
        val: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetCanRotate(
        val: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetCanSkew(
        val: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetCanSizeDisproportionally(
        val: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetCanApplyNonlinearTransforms(
        val: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetRegenerateOnTransform(
        val: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetRegenerateOnStyleChange(
        val: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetPropertyBarGuid(
        val: BSTR,
    ) -> HRESULT,
    fn SetContextMenuGuid(
        val: BSTR,
    ) -> HRESULT,
    fn SetObjectManagerBitmapGuid(
        val: BSTR,
    ) -> HRESULT,
    fn SetEditStateGuid(
        val: BSTR,
    ) -> HRESULT,
    fn SetDefaultShapename(
        val: BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800d6, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGToolShape(IVGToolShapeVtbl): IDispatch(IDispatchVtbl) {
    fn OnGenerateShape(
        Parent: *const IVGLayer,
        ObjectProperties: *const IVGProperties,
        pStyleAttributes: *const IVGStyle,
        pTransformation: *const IVGTransformMatrix,
        IsPreviewOnly: VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580058, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGRulers(IVGRulersVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get_VUnits(
        pVal: *mut cdrUnit,
    ) -> HRESULT,
    fn put_VUnits(
        pVal: cdrUnit,
    ) -> HRESULT,
    fn get_HUnits(
        pVal: *mut cdrUnit,
    ) -> HRESULT,
    fn put_HUnits(
        pVal: cdrUnit,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058003e, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGGrid(IVGGridVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get_Visible(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Visible(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Type(
        pVal: *mut cdrGridType,
    ) -> HRESULT,
    fn put_Type(
        pVal: cdrGridType,
    ) -> HRESULT,
    fn get_Snap(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Snap(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn SetFrequency(
        GridX: f64,
        GridY: f64,
    ) -> HRESULT,
    fn get_SpacingX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_SpacingX(
        pVal: f64,
    ) -> HRESULT,
    fn get_SpacingY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_SpacingY(
        pVal: f64,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580080, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGViews(IVGViewsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGView,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn AddActiveView(
        Name: BSTR,
        ppVal: *mut *mut IVGView,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058007f, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGView(IVGViewVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGViews,
    ) -> HRESULT,
    fn get_Name(
        Name: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        Name: BSTR,
    ) -> HRESULT,
    fn get_OriginX(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OriginX(
        pVal: f64,
    ) -> HRESULT,
    fn get_OriginY(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_OriginY(
        pVal: f64,
    ) -> HRESULT,
    fn get_UsePage(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UsePage(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Page(
        Page: *mut *mut IVGPage,
    ) -> HRESULT,
    fn put_Page(
        Page: *const IVGPage,
    ) -> HRESULT,
    fn get_UseZoom(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseZoom(
        pVal: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Zoom(
        pVal: *mut f64,
    ) -> HRESULT,
    fn put_Zoom(
        pVal: f64,
    ) -> HRESULT,
    fn Activate(
    ) -> HRESULT,
    fn Delete(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058005c, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSelectionInformation(IVGSelectionInformationVtbl): IDispatch(IDispatchVtbl) {
    fn get_Application(
        ppVal: *mut *mut IVGApplication,
    ) -> HRESULT,
    fn get_Parent(
        ppVal: *mut *mut IVGDocument,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get_FirstShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_SecondShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_BlendTopShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_BlendBottomShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_BlendPath(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_CanCreateBlend(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DistortionShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_DistortionType(
        pVal: *mut cdrDistortionType,
    ) -> HRESULT,
    fn get_ExtrudeFaceShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_ExtrudeGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_ExtrudeBevelGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_ContourControlShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_ContourGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_DropShadowControlShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_DropShadowGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_DimensionControlShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_DimensionGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_ConnectorLines(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_FittedTextControlShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_FittedText(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_FirstShapeWithOutline(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_FirstShapeWithFill(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_NaturalMediaControlShape(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_NaturalMediaGroup(
        ppVal: *mut *mut IVGShape,
    ) -> HRESULT,
    fn get_CanPrint(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsEditingText(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsTextSelection(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsOnPowerClipContents(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsEditingRollOver(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanApplyFillOutline(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsControlSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanDeleteControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsGroup(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsRegularShape(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsControlShape(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsBlendControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsBlendGroup(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsCloneControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsContourControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsContourGroup(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsDropShadowControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsDropShadowGroup(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsDimensionControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsExtrudeControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsExtrudeGroup(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsBevelGroup(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_HasAutoLabelText(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsEnvelope(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsPerspective(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsDistortion(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsConnectorLine(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsConnector(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFittedText(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFittedTextControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsNaturalMediaControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsNaturalMediaGroup(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSecondExtrudeControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSecondContourControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSecondDropShadowControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSecondNaturalMediaControl(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsArtisticTextSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsParagraphTextSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsTextSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsOLESelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsBitmapSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsBitmapPresent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsLensPresent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsMaskedBitmapPresent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsGroupSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanUngroup(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsLinkGroupSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsLinkControlSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAttachedToDimension(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFittedTextSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsConnectorLineSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsConnectorSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsPerspectivePresent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsEnvelopePresent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsDistortionPresent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsGuidelineSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsInternetObjectSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsSoundObjectSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsExternalBitmapSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsNonExternalBitmapSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsMeshFillSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsMeshFillPresent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsRollOverSelected(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ContainsRollOverParent(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanClone(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanApplyBlend(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanApplyContour(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanApplyFill(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanApplyOutline(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanApplyTransparency(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanAssignURL(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanApplyDistortion(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanApplyEnvelope(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCopyBlend(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCloneBlend(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCopyExtrude(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCloneExtrude(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCopyContour(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCloneContour(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCopyDropShadow(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCloneDropShadow(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCopyLens(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCopyPerspective(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCopyEnvelope(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCopyPowerclip(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanCopyDistortion(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanLockShapes(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanUnlockShapes(
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb058009d, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGHatchLibraries(IVGHatchLibrariesVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        IndexOrName: VARIANT,
        ppVal: *mut *mut IVGHatchLibrary,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn Find(
        Name: BSTR,
        ppVal: *mut *mut IVGHatchLibrary,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_ActiveLibrary(
        ppVal: *mut *mut IVGHatchLibrary,
    ) -> HRESULT,
    fn get_DefaultLibrary(
        ppVal: *mut *mut IVGHatchLibrary,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800a2, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGMetadata(IVGMetadataVtbl): IDispatch(IDispatchVtbl) {
    fn get_Keywords(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Keywords(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Notes(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Notes(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Author(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Author(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_LastAuthor(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_LastAuthor(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Subject(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Subject(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Copyright(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Copyright(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Revision(
        pVal: *mut i32,
    ) -> HRESULT,
    fn put_Revision(
        pVal: i32,
    ) -> HRESULT,
    fn get_TemplateSided(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_TemplateSided(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_TemplateFolds(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_TemplateFolds(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_TemplateType(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_TemplateType(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_TemplateIndustry(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_TemplateIndustry(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_Title(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_Title(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_DocID(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_DocID(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_DocLanguage(
        pVal: *mut cdrTextLanguage,
    ) -> HRESULT,
    fn put_DocLanguage(
        pVal: cdrTextLanguage,
    ) -> HRESULT,
    fn get_TemplateDesignerNotes(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put_TemplateDesignerNotes(
        pVal: BSTR,
    ) -> HRESULT,
    fn get_LocalizableKeywords(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
    fn get_LocalizableNotes(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
    fn get_LocalizableTitle(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
    fn get_LocalizableSubject(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
    fn get_LocalizableCopyright(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
    fn get_LocalizableTemplateDesignerNotes(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
    fn get_Category(
        ppVal: *mut *mut IVGLocalizableString,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800a6, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGSpreads(IVGSpreadsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Item(
        Index: i32,
        ppVal: *mut *mut IVGSpread,
    ) -> HRESULT,
    fn get_Count(
        pVal: *mut i32,
    ) -> HRESULT,
    fn get__NewEnum(
        pVal: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_First(
        ppVal: *mut *mut IVGSpread,
    ) -> HRESULT,
    fn get_Last(
        ppVal: *mut *mut IVGSpread,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800ba, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGStyleSheet(IVGStyleSheetVtbl): IDispatch(IDispatchVtbl) {
    fn get_Styles(
        ppVal: *mut *mut IVGStyles,
    ) -> HRESULT,
    fn get_StyleSets(
        ppVal: *mut *mut IVGStyles,
    ) -> HRESULT,
    fn get_ObjectDefaults(
        ppVal: *mut *mut IVGStyles,
    ) -> HRESULT,
    fn get_AllStyles(
        ppVal: *mut *mut IVGStyles,
    ) -> HRESULT,
    fn get_AllStyleSets(
        ppVal: *mut *mut IVGStyles,
    ) -> HRESULT,
    fn FindStyle(
        Name: BSTR,
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
    fn CreateStyleFromShape(
        Shape: *const IVGShape,
        Category: BSTR,
        Name: BSTR,
        ReplaceExisting: VARIANT_BOOL,
        ppVal: *mut *mut IVGStyles,
    ) -> HRESULT,
    fn CreateStyleFromShapeRange(
        ShapeRange: *const IVGShapeRange,
        Category: BSTR,
        Name: BSTR,
        ReplaceExisting: VARIANT_BOOL,
        ppVal: *mut *mut IVGStyles,
    ) -> HRESULT,
    fn CreateStyleFromTextRange(
        TextRange: *const IVGTextRange,
        Category: BSTR,
        Name: BSTR,
        ReplaceExisting: VARIANT_BOOL,
        ppVal: *mut *mut IVGStyles,
    ) -> HRESULT,
    fn CreateStyleSetFromShape(
        Shape: *const IVGShape,
        Name: BSTR,
        ReplaceExisting: VARIANT_BOOL,
        ppVal: *mut *mut IVGStyles,
    ) -> HRESULT,
    fn CreateStyleSetFromShapeRange(
        ShapeRange: *const IVGShapeRange,
        Name: BSTR,
        ReplaceExisting: VARIANT_BOOL,
        ppVal: *mut *mut IVGStyles,
    ) -> HRESULT,
    fn CreateStyleSetFromTextRange(
        TextRange: *const IVGTextRange,
        Name: BSTR,
        ReplaceExisting: VARIANT_BOOL,
        ppVal: *mut *mut IVGStyles,
    ) -> HRESULT,
    fn CreateStyle(
        Category: BSTR,
        BasedOn: BSTR,
        Name: BSTR,
        ReplaceExisting: VARIANT_BOOL,
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
    fn CreateStyleSet(
        BasedOn: BSTR,
        Name: BSTR,
        ReplaceExisting: VARIANT_BOOL,
        ppVal: *mut *mut IVGStyle,
    ) -> HRESULT,
    fn Export(
        FileName: BSTR,
        Styles: VARIANT_BOOL,
        StyleSets: VARIANT_BOOL,
        ObjectDefaults: VARIANT_BOOL,
        ColorStyles: VARIANT_BOOL,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Import(
        FileName: BSTR,
        MergeStyles: VARIANT_BOOL,
        Styles: VARIANT_BOOL,
        StyleSets: VARIANT_BOOL,
        ObjectDefaults: VARIANT_BOOL,
        ColorStyles: VARIANT_BOOL,
        pVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AllColorStyles(
        ppVal: *mut *mut IVGColors,
    ) -> HRESULT,
    fn CreateColorStyle(
        Name: BSTR,
        Color: *const IVGColor,
        HarmonyIndex: i32,
        IndexInHarmony: i32,
        ReplaceExisting: VARIANT_BOOL,
        ppVal: *mut *mut IVGColor,
    ) -> HRESULT,
    fn DeleteAllColorStyles(
    ) -> HRESULT,
    fn DeleteColorStyle(
        Name: BSTR,
    ) -> HRESULT,
    fn RenameColorStyle(
        OldName: BSTR,
        NewName: BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800c2, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface _IGlobalMacroStorage(_IGlobalMacroStorageVtbl): IDispatch(IDispatchVtbl) {
    fn get__CodeName(
        pVal: *mut BSTR,
    ) -> HRESULT,
    fn put__CodeName(
        pVal: BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb0580005, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGAppPlugin(IVGAppPluginVtbl): IDispatch(IDispatchVtbl) {
    fn OnLoad(
        Application: *const IVGApplication,
    ) -> HRESULT,
    fn StartSession(
    ) -> HRESULT,
    fn StopSession(
    ) -> HRESULT,
    fn OnUnload(
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xb05800c3, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGApplicationEvents(IVGApplicationEventsVtbl): IDispatch(IDispatchVtbl) {
    fn QueryDocumentClose(
        Doc: *const Document,
        Cancel: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn QueryDocumentSave(
        Doc: *const Document,
        Cancel: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn QueryDocumentPrint(
        Doc: *const Document,
        Cancel: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn QueryDocumentExport(
        Doc: *const Document,
        Cancel: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn QueryQuit(
        Cancel: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn DocumentOpen(
        Doc: *const Document,
        FileName: BSTR,
    ) -> HRESULT,
    fn DocumentNew(
        Doc: *const Document,
        FromTemplate: VARIANT_BOOL,
        Template: BSTR,
        IncludeGraphics: VARIANT_BOOL,
    ) -> HRESULT,
    fn DocumentClose(
        Doc: *const Document,
    ) -> HRESULT,
    fn DocumentBeforeSave(
        Doc: *const Document,
        SaveAs: VARIANT_BOOL,
        FileName: BSTR,
    ) -> HRESULT,
    fn DocumentAfterSave(
        Doc: *const Document,
        SaveAs: VARIANT_BOOL,
        FileName: BSTR,
    ) -> HRESULT,
    fn DocumentBeforePrint(
        Doc: *const Document,
    ) -> HRESULT,
    fn DocumentAfterPrint(
        Doc: *const Document,
    ) -> HRESULT,
    fn DocumentBeforeExport(
        Doc: *const Document,
        FileName: BSTR,
        Filter: cdrFilter,
        SaveBitmap: VARIANT_BOOL,
    ) -> HRESULT,
    fn DocumentAfterExport(
        Doc: *const Document,
        FileName: BSTR,
        Filter: cdrFilter,
        SaveBitmap: VARIANT_BOOL,
    ) -> HRESULT,
    fn WindowActivate(
        Doc: *const Document,
        Window: *const Window,
    ) -> HRESULT,
    fn WindowDeactivate(
        Doc: *const Document,
        Window: *const Window,
    ) -> HRESULT,
    fn SelectionChange(
    ) -> HRESULT,
    fn Start(
    ) -> HRESULT,
    fn Quit(
    ) -> HRESULT,
    fn OnPluginCommand(
        CommandID: BSTR,
    ) -> HRESULT,
    fn OnUpdatePluginCommand(
        CommandID: BSTR,
        Enabled: *mut VARIANT_BOOL,
        Checked: *mut cdrCommandCheckState,
    ) -> HRESULT,
    fn OnApplicationEvent(
        EventName: BSTR,
        Parameters: *const SAFEARRAY,
    ) -> HRESULT,
}}

// Implements IVGDocument
// Implements DIVGDocumentEvents
// Implements IPrnVBAPrintDocument
RIDL!{#[uuid(0xde3d0026, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Document; }

RIDL!{#[uuid(0xb05800c4, 0x9aa4, 0x44fd, 0x95, 0x47, 0x4f, 0x91, 0xeb, 0x75, 0x7a, 0xc4)]
interface IVGDocumentEvents(IVGDocumentEventsVtbl): IDispatch(IDispatchVtbl) {
    fn QueryClose(
        Cancel: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn QuerySave(
        Cancel: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn QueryPrint(
        Cancel: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn QueryExport(
        Cancel: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Open(
    ) -> HRESULT,
    fn Close(
    ) -> HRESULT,
    fn BeforeSave(
        SaveAs: VARIANT_BOOL,
        FileName: BSTR,
    ) -> HRESULT,
    fn AfterSave(
        SaveAs: VARIANT_BOOL,
        FileName: BSTR,
    ) -> HRESULT,
    fn BeforePrint(
    ) -> HRESULT,
    fn AfterPrint(
    ) -> HRESULT,
    fn BeforeExport(
        FileName: BSTR,
        Filter: cdrFilter,
        SaveBitmap: VARIANT_BOOL,
    ) -> HRESULT,
    fn AfterExport(
        FileName: BSTR,
        Filter: cdrFilter,
        SaveBitmap: VARIANT_BOOL,
    ) -> HRESULT,
    fn LayerCreate(
        Layer: *const Layer,
    ) -> HRESULT,
    fn LayerDelete(
        Count: i32,
    ) -> HRESULT,
    fn LayerActivate(
        Layer: *const Layer,
    ) -> HRESULT,
    fn LayerChange(
        Layer: *const Layer,
    ) -> HRESULT,
    fn PageCreate(
        Page: *const Page,
    ) -> HRESULT,
    fn PageDelete(
        Count: i32,
    ) -> HRESULT,
    fn PageActivate(
        Page: *const Page,
    ) -> HRESULT,
    fn PageChange(
        Page: *const Page,
    ) -> HRESULT,
    fn ShapeCreate(
        Shape: *const Shape,
    ) -> HRESULT,
    fn ShapeDelete(
        Count: i32,
    ) -> HRESULT,
    fn ShapeMove(
        Shape: *const Shape,
    ) -> HRESULT,
    fn ShapeTransform(
        Shape: *const Shape,
    ) -> HRESULT,
    fn ShapeDistort(
        Shape: *const Shape,
    ) -> HRESULT,
    fn ShapeChange(
        Shape: *const Shape,
        Scope: cdrShapeChangeScope,
    ) -> HRESULT,
    fn SelectionChange(
    ) -> HRESULT,
}}

// Implements IVGLayer
RIDL!{#[uuid(0xde3d0051, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Layer; }

// Implements IVGPage
// Implements IPrnVBAPrintPage
RIDL!{#[uuid(0xde3d005d, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Page; }

// Implements IVGShape
RIDL!{#[uuid(0xde3d0076, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Shape; }

// Implements IVGWindow
RIDL!{#[uuid(0xde3d00b3, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Window; }

// Implements IVGActiveView
RIDL!{#[uuid(0xde3d0001, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ActiveView; }

// Implements IVGApplication
// Implements DIVGApplicationEvents
// Implements ICUIApplication
// {1F880002-01A1-470B-A07E-FF9136793812}
// {00020424-0000-0000-C000-000000000046}
RIDL!{#[uuid(0xde3d0002, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Application; }
// 
// RIDL!{#[uuid(0x1f880002, 0x01a1, 0x470b, 0xa0, 0x7e, 0xff, 0x91, 0x36, 0x79, 0x38, 0x12)]
//     class Application; }

// Implements IVGAppStatus
RIDL!{#[uuid(0xde3d0003, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class AppStatus; }

// Implements IVGAppWindow
RIDL!{#[uuid(0xde3d0004, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class AppWindow; }

// Implements IVGArrowHead
RIDL!{#[uuid(0xde3d0005, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ArrowHead; }

// Implements IVGArrowHeadOptions
RIDL!{#[uuid(0xde3d0006, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ArrowHeadOptions; }

// Implements IVGArrowHeads
RIDL!{#[uuid(0xde3d0007, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ArrowHeads; }

// Implements IVGBBoxSnapPoint
RIDL!{#[uuid(0xde3d0008, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class BBoxSnapPoint; }

// Implements IVGBitmap
RIDL!{#[uuid(0xde3d0009, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Bitmap; }

// Implements IVGBSpline
RIDL!{#[uuid(0xde3d000a, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class BSpline; }

// Implements IVGBSplineControlPoint
RIDL!{#[uuid(0xde3d000b, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class BSplineControlPoint; }

// Implements IVGBSplineControlPoints
RIDL!{#[uuid(0xde3d000c, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class BSplineControlPoints; }

// Implements IVGClipboard
RIDL!{#[uuid(0xde3d000d, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Clipboard; }

// Implements IVGCloneLink
RIDL!{#[uuid(0xde3d000e, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class CloneLink; }

// Implements IVGColor
RIDL!{#[uuid(0xde3d000f, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Color; }

// Implements IVGColorContext
RIDL!{#[uuid(0xde3d0010, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ColorContext; }

// Implements IVGColorManagementPolicy
RIDL!{#[uuid(0xde3d0011, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ColorManagementPolicy; }

// Implements IVGColorManager
RIDL!{#[uuid(0xde3d0012, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ColorManager; }

// Implements IVGColorProfile
RIDL!{#[uuid(0xde3d0013, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ColorProfile; }

// Implements IVGColorProfiles
RIDL!{#[uuid(0xde3d0014, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ColorProfiles; }

// Implements IVGColors
RIDL!{#[uuid(0xde3d0015, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Colors; }

// Implements IVGComponent
RIDL!{#[uuid(0xde3d0016, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Component; }

// Implements IVGComponents
RIDL!{#[uuid(0xde3d0017, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Components; }

// Implements IVGConnector
RIDL!{#[uuid(0xde3d0018, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Connector; }

// Implements IVGCrossPoint
RIDL!{#[uuid(0xde3d001a, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class CrossPoint; }

// Implements IVGCrossPoints
RIDL!{#[uuid(0xde3d001b, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class CrossPoints; }

// Implements IVGCurve
RIDL!{#[uuid(0xde3d001c, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Curve; }

// Implements IVGCustomEffect
RIDL!{#[uuid(0xde3d001d, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class CustomEffect; }

// Implements IVGCustomShape
RIDL!{#[uuid(0xde3d001e, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class CustomShape; }

// Implements IVGDataField
RIDL!{#[uuid(0xde3d001f, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DataField; }

// Implements IVGDataFields
RIDL!{#[uuid(0xde3d0020, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DataFields; }

// Implements IVGDataItem
RIDL!{#[uuid(0xde3d0021, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DataItem; }

// Implements IVGDataItems
RIDL!{#[uuid(0xde3d0022, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DataItems; }

// Implements IVGDimension
RIDL!{#[uuid(0xde3d0023, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Dimension; }

// Implements IVGDimensionAngular
RIDL!{#[uuid(0xde3d0024, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DimensionAngular; }

// Implements IVGDimensionLinear
RIDL!{#[uuid(0xde3d0025, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DimensionLinear; }

// Implements IVGDocuments
RIDL!{#[uuid(0xde3d0027, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Documents; }

// Implements IVGDuotone
RIDL!{#[uuid(0xde3d0028, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Duotone; }

// Implements IVGDuotoneInk
RIDL!{#[uuid(0xde3d0029, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DuotoneInk; }

// Implements IVGDuotoneOverprint
RIDL!{#[uuid(0xde3d002a, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DuotoneOverprint; }

// Implements IVGEdgeSnapPoint
RIDL!{#[uuid(0xde3d002b, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EdgeSnapPoint; }

// Implements IVGEffect
RIDL!{#[uuid(0xde3d002c, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Effect; }

// Implements IVGEffectBlend
RIDL!{#[uuid(0xde3d002d, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectBlend; }

// Implements IVGEffectContour
RIDL!{#[uuid(0xde3d002e, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectContour; }

// Implements IVGEffectControlPath
RIDL!{#[uuid(0xde3d002f, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectControlPath; }

// Implements IVGEffectCustomDistortion
RIDL!{#[uuid(0xde3d0030, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectCustomDistortion; }

// Implements IVGEffectDistortion
RIDL!{#[uuid(0xde3d0031, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectDistortion; }

// Implements IVGEffectDropShadow
RIDL!{#[uuid(0xde3d0032, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectDropShadow; }

// Implements IVGEffectEnvelope
RIDL!{#[uuid(0xde3d0033, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectEnvelope; }

// Implements IVGEffectExtrude
RIDL!{#[uuid(0xde3d0034, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectExtrude; }

// Implements IVGEffectLens
RIDL!{#[uuid(0xde3d0035, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectLens; }

// Implements IVGEffectPerspective
RIDL!{#[uuid(0xde3d0036, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectPerspective; }

// Implements IVGEffectPushPullDistortion
RIDL!{#[uuid(0xde3d0037, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectPushPullDistortion; }

// Implements IVGEffects
RIDL!{#[uuid(0xde3d0038, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Effects; }

// Implements IVGEffectTextOnPath
RIDL!{#[uuid(0xde3d0039, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectTextOnPath; }

// Implements IVGEffectTwisterDistortion
RIDL!{#[uuid(0xde3d003a, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectTwisterDistortion; }

// Implements IVGEffectZipperDistortion
RIDL!{#[uuid(0xde3d003b, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EffectZipperDistortion; }

// Implements IVGEllipse
RIDL!{#[uuid(0xde3d003c, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Ellipse; }

// Implements IVGEPS
RIDL!{#[uuid(0xde3d003d, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class EPS; }

// Implements ICorelExportFilter
RIDL!{#[uuid(0xde3d003e, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ExportFilter; }

// Implements IVGExtrudeVanishingPoint
RIDL!{#[uuid(0xde3d003f, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ExtrudeVanishingPoint; }

// Implements IVGFill
RIDL!{#[uuid(0xde3d0040, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Fill; }

// Implements IVGFillMetadata
RIDL!{#[uuid(0xde3d0041, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class FillMetadata; }

// Implements IVGFontList
RIDL!{#[uuid(0xde3d0042, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class FontList; }

// Implements IVGFountainColor
RIDL!{#[uuid(0xde3d0043, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class FountainColor; }

// Implements IVGFountainColors
RIDL!{#[uuid(0xde3d0044, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class FountainColors; }

// Implements IVGFountainFill
RIDL!{#[uuid(0xde3d0045, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class FountainFill; }

// Implements _IGlobalMacroStorage
// Implements DIVGApplicationEvents
RIDL!{#[uuid(0xde3d0046, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class GlobalMacroStorage; }

// Implements IVGGMSManager
RIDL!{#[uuid(0xde3d0047, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class GMSManager; }

// Implements IVGGrid
RIDL!{#[uuid(0xde3d0048, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Grid; }

// Implements IVGGuide
RIDL!{#[uuid(0xde3d0049, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Guide; }

// Implements IVGHatchFill
RIDL!{#[uuid(0xde3d004a, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class HatchFill; }

// Implements IVGHatchFills
RIDL!{#[uuid(0xde3d004b, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class HatchFills; }

// Implements IVGHatchLibraries
RIDL!{#[uuid(0xde3d004c, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class HatchLibraries; }

// Implements IVGHatchLibrary
RIDL!{#[uuid(0xde3d004d, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class HatchLibrary; }

// Implements IVGHatchPattern
RIDL!{#[uuid(0xde3d004e, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class HatchPattern; }

// Implements IVGHatchPatterns
RIDL!{#[uuid(0xde3d004f, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class HatchPatterns; }

// Implements ICorelImportFilter
RIDL!{#[uuid(0xde3d0050, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ImportFilter; }

// Implements IVGLayers
RIDL!{#[uuid(0xde3d0052, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Layers; }

// Implements IVGLocalizableString
RIDL!{#[uuid(0xde3d0053, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class LocalizableString; }

// Implements IVGMetadata
RIDL!{#[uuid(0xde3d0054, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Metadata; }

// Implements IVGNode
RIDL!{#[uuid(0xde3d0055, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Node; }

// Implements IVGNodeRange
RIDL!{#[uuid(0xde3d0056, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class NodeRange; }

// Implements IVGNodes
RIDL!{#[uuid(0xde3d0057, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Nodes; }

// Implements IVGObjectSnapPoint
RIDL!{#[uuid(0xde3d0058, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ObjectSnapPoint; }

// Implements IVGOLE
RIDL!{#[uuid(0xde3d0059, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class OLE; }

// Implements IVGOutline
RIDL!{#[uuid(0xde3d005a, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Outline; }

// Implements IVGOutlineStyle
RIDL!{#[uuid(0xde3d005b, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class OutlineStyle; }

// Implements IVGOutlineStyles
RIDL!{#[uuid(0xde3d005c, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class OutlineStyles; }

// Implements IVGPages
RIDL!{#[uuid(0xde3d005e, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Pages; }

// Implements IVGPageSize
RIDL!{#[uuid(0xde3d005f, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class PageSize; }

// Implements IVGPageSizes
RIDL!{#[uuid(0xde3d0060, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class PageSizes; }

// Implements IVGPalette
RIDL!{#[uuid(0xde3d0061, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Palette; }

// Implements IVGPaletteManager
RIDL!{#[uuid(0xde3d0062, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class PaletteManager; }

// Implements IVGPalettes
RIDL!{#[uuid(0xde3d0063, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Palettes; }

// Implements IVGPatternCanvas
RIDL!{#[uuid(0xde3d0064, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class PatternCanvas; }

// Implements IVGPatternCanvases
RIDL!{#[uuid(0xde3d0065, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class PatternCanvases; }

// Implements IVGPatternFill
RIDL!{#[uuid(0xde3d0066, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class PatternFill; }

// Implements IVGPolygon
RIDL!{#[uuid(0xde3d0067, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Polygon; }

// Implements IVGPostScriptFill
RIDL!{#[uuid(0xde3d0068, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class PostScriptFill; }

// Implements IVGPowerClip
RIDL!{#[uuid(0xde3d0069, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class PowerClip; }

// Implements IVGProofColorSettings
RIDL!{#[uuid(0xde3d006a, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ProofColorSettings; }

// Implements IVGProperties
RIDL!{#[uuid(0xde3d006b, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Properties; }

// Implements IVGPSScreenOptions
RIDL!{#[uuid(0xde3d006c, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class PSScreenOptions; }

// Implements IVGRecentFile
RIDL!{#[uuid(0xde3d006d, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class RecentFile; }

// Implements IVGRecentFiles
RIDL!{#[uuid(0xde3d006e, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class RecentFiles; }

// Implements IVGRect
RIDL!{#[uuid(0xde3d006f, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Rect; }

// Implements IVGRectangle
RIDL!{#[uuid(0xde3d0070, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Rectangle; }

// Implements IVGRulers
RIDL!{#[uuid(0xde3d0071, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Rulers; }

// Implements IVGSegment
RIDL!{#[uuid(0xde3d0072, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Segment; }

// Implements IVGSegmentRange
RIDL!{#[uuid(0xde3d0073, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class SegmentRange; }

// Implements IVGSegments
RIDL!{#[uuid(0xde3d0074, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Segments; }

// Implements IVGSelectionInformation
RIDL!{#[uuid(0xde3d0075, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class SelectionInfo; }

// Implements IVGShapeRange
RIDL!{#[uuid(0xde3d0077, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ShapeRange; }

// Implements IVGShapes
RIDL!{#[uuid(0xde3d0078, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Shapes; }

// Implements IVGSnapPoint
RIDL!{#[uuid(0xde3d0079, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class SnapPoint; }

// Implements IVGSnapPointRange
RIDL!{#[uuid(0xde3d007a, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class SnapPointRange; }

// Implements IVGSnapPoints
RIDL!{#[uuid(0xde3d007b, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class SnapPoints; }

// Implements IVGSpread
RIDL!{#[uuid(0xde3d007c, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Spread; }

// Implements IVGSpreads
RIDL!{#[uuid(0xde3d007d, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Spreads; }

// Implements IVGStructAlignProperties
RIDL!{#[uuid(0xde3d007e, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructAlignProperties; }

// Implements IVGStructColorConversionOptions
RIDL!{#[uuid(0xde3d007f, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructColorConversionOptions; }

// Implements IVGStructCreateOptions
RIDL!{#[uuid(0xde3d0080, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructCreateOptions; }

// Implements IVGStructExportOptions
RIDL!{#[uuid(0xde3d0081, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructExportOptions; }

// Implements IVGStructFontProperties
RIDL!{#[uuid(0xde3d0082, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructFontProperties; }

// Implements IVGStructHyphenationSettings
RIDL!{#[uuid(0xde3d0083, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructHyphenationSettings; }

// Implements IStructImportCropOptions
RIDL!{#[uuid(0xde3d0084, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructImportCropOptions; }

// Implements IVGStructImportOptions
RIDL!{#[uuid(0xde3d0085, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructImportOptions; }

// Implements IStructImportResampleOptions
RIDL!{#[uuid(0xde3d0086, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructImportResampleOptions; }

// Implements IVGStructOpenOptions
RIDL!{#[uuid(0xde3d0087, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructOpenOptions; }

// Implements IVGStructPaletteOptions
RIDL!{#[uuid(0xde3d0088, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructPaletteOptions; }

// Implements IVGStructPasteOptions
RIDL!{#[uuid(0xde3d0089, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructPasteOptions; }

// Implements IVGStructSaveAsOptions
RIDL!{#[uuid(0xde3d008a, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructSaveAsOptions; }

// Implements IVGStructSpaceProperties
RIDL!{#[uuid(0xde3d008b, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StructSpaceProperties; }

// Implements IVGStyle
RIDL!{#[uuid(0xde3d008c, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Style; }

// Implements IVGStyleCharacter
RIDL!{#[uuid(0xde3d008d, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StyleCharacter; }

// Implements IVGStyleFill
RIDL!{#[uuid(0xde3d008e, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StyleFill; }

// Implements IVGStyleFrame
RIDL!{#[uuid(0xde3d008f, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StyleFrame; }

// Implements IVGStyleOutline
RIDL!{#[uuid(0xde3d0090, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StyleOutline; }

// Implements IVGStyleParagraph
RIDL!{#[uuid(0xde3d0091, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StyleParagraph; }

// Implements IVGStyles
RIDL!{#[uuid(0xde3d0092, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Styles; }

// Implements IVGStyleSheet
RIDL!{#[uuid(0xde3d0093, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StyleSheet; }

// Implements IVGSubPath
RIDL!{#[uuid(0xde3d0094, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class SubPath; }

// Implements IVGSubPaths
RIDL!{#[uuid(0xde3d0095, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class SubPaths; }

// Implements IVGSymbol
RIDL!{#[uuid(0xde3d0096, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Symbol; }

// Implements IVGSymbolDefinition
RIDL!{#[uuid(0xde3d0097, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class SymbolDefinition; }

// Implements IVGSymbolDefinitions
RIDL!{#[uuid(0xde3d0098, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class SymbolDefinitions; }

// Implements IVGSymbolLibraries
RIDL!{#[uuid(0xde3d0099, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class SymbolLibraries; }

// Implements IVGSymbolLibrary
RIDL!{#[uuid(0xde3d009a, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class SymbolLibrary; }

// Implements IVGText
RIDL!{#[uuid(0xde3d009b, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Text; }

// Implements IVGTextCharacters
RIDL!{#[uuid(0xde3d009c, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextCharacters; }

// Implements IVGTextColumns
RIDL!{#[uuid(0xde3d009d, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextColumns; }

// Implements IVGTextFrame
RIDL!{#[uuid(0xde3d009e, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextFrame; }

// Implements IVGTextFrames
RIDL!{#[uuid(0xde3d009f, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextFrames; }

// Implements IVGTextLines
RIDL!{#[uuid(0xde3d00a0, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextLines; }

// Implements IVGTextParagraphs
RIDL!{#[uuid(0xde3d00a1, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextParagraphs; }

// Implements IVGTextRange
RIDL!{#[uuid(0xde3d00a2, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextRange; }

// Implements IVGTextRanges
RIDL!{#[uuid(0xde3d00a3, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextRanges; }

// Implements IVGTextTabPosition
RIDL!{#[uuid(0xde3d00a4, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextTabPosition; }

// Implements IVGTextTabPositions
RIDL!{#[uuid(0xde3d00a5, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextTabPositions; }

// Implements IVGTextureFill
RIDL!{#[uuid(0xde3d00a6, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextureFill; }

// Implements IVGTextureFillProperties
RIDL!{#[uuid(0xde3d00a7, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextureFillProperties; }

// Implements IVGTextureFillProperty
RIDL!{#[uuid(0xde3d00a8, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextureFillProperty; }

// Implements IVGTextWords
RIDL!{#[uuid(0xde3d00a9, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TextWords; }

// Implements IVGTraceSettings
RIDL!{#[uuid(0xde3d00aa, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TraceSettings; }

// Implements IVGTransparency
RIDL!{#[uuid(0xde3d00ab, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Transparency; }

// Implements IVGTreeManager
RIDL!{#[uuid(0xde3d00ac, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TreeManager; }

// Implements IVGTreeNode
RIDL!{#[uuid(0xde3d00ad, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TreeNode; }

// Implements IVGTreeNodes
RIDL!{#[uuid(0xde3d00ae, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TreeNodes; }

// Implements IVGURL
RIDL!{#[uuid(0xde3d00af, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class URL; }

// Implements IVGUserSnapPoint
RIDL!{#[uuid(0xde3d00b0, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class UserSnapPoint; }

// Implements IVGView
RIDL!{#[uuid(0xde3d00b1, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class View; }

// Implements IVGViews
RIDL!{#[uuid(0xde3d00b2, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Views; }

// Implements IVGWindows
RIDL!{#[uuid(0xde3d00b4, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Windows; }

// Implements IVGWorkspace
RIDL!{#[uuid(0xde3d00b5, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Workspace; }

// Implements IVGWorkspaces
RIDL!{#[uuid(0xde3d00b6, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Workspaces; }

// Implements ICorelScriptTools
RIDL!{#[uuid(0xde3d00b7, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class CorelScriptTools; }

// Implements IVGGMSMacro
RIDL!{#[uuid(0xde3d00b8, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class GMSMacro; }

// Implements IVGGMSMacros
RIDL!{#[uuid(0xde3d00b9, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class GMSMacros; }

// Implements IVGGMSProject
RIDL!{#[uuid(0xde3d00ba, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class GMSProject; }

// Implements IVGGMSProjects
RIDL!{#[uuid(0xde3d00bb, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class GMSProjects; }

// Implements IVGOnScreenCurve
RIDL!{#[uuid(0xde3d00bc, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class OnScreenCurve; }

// Implements IVGOnScreenHandle
RIDL!{#[uuid(0xde3d00bd, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class OnScreenHandle; }

// Implements IVGOnScreenText
RIDL!{#[uuid(0xde3d00be, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class OnScreenText; }

// Implements IVGToolStateAttributes
RIDL!{#[uuid(0xde3d00bf, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ToolStateAttributes; }

// Implements IVGToolState
RIDL!{#[uuid(0xde3d00c0, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ToolState; }

// Implements IVGPoint
RIDL!{#[uuid(0xde3d00c1, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Point; }

// Implements IVGPointRange
RIDL!{#[uuid(0xde3d00c2, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class PointRange; }

// Implements IVGVector
RIDL!{#[uuid(0xde3d00c3, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Vector; }

// Implements IVGTransformMatrix
RIDL!{#[uuid(0xde3d00c4, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class TransformMatrix; }

// Implements IVGMathUtils
RIDL!{#[uuid(0xde3d00c5, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class MathUtils; }

// Implements IVGToolShapeAttributes
RIDL!{#[uuid(0xde3d00c6, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ToolShapeAttributes; }

// Implements IVGToolShape
RIDL!{#[uuid(0xde3d00c7, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ToolShape; }

// Implements IVGStyleTransparency
RIDL!{#[uuid(0xde3d00c8, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StyleTransparency; }

// Implements IVGImage
RIDL!{#[uuid(0xde3d00c9, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Image; }

// Implements IVGImageTiles
RIDL!{#[uuid(0xde3d00ca, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ImageTiles; }

// Implements IVGImageTile
RIDL!{#[uuid(0xde3d00cb, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ImageTile; }

// Implements ICUIFrameWork
RIDL!{#[uuid(0xde3d1001, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class FrameWork; }

// Implements ICUICommandBars
RIDL!{#[uuid(0xde3d1002, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class CommandBars; }

// Implements ICUICommandBarModes
RIDL!{#[uuid(0xde3d1003, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class CommandBarModes; }

// Implements ICUICommandBarMode
RIDL!{#[uuid(0xde3d1004, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class CommandBarMode; }

// Implements ICUIControls
RIDL!{#[uuid(0xde3d1005, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Controls; }

// Implements ICUIControl
RIDL!{#[uuid(0xde3d1006, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class Control; }

// Implements ICUICommandBar
RIDL!{#[uuid(0xde3d1007, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class CommandBar; }

// Implements ICUIDataContext
RIDL!{#[uuid(0xde3d1008, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DataContext; }

// Implements ICUIDataSourceProxy
RIDL!{#[uuid(0xde3d1009, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DataSourceProxy; }

// Implements ICUIImageList
RIDL!{#[uuid(0xde3d100a, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ImageList; }

// Implements ICUIFrameWindows
RIDL!{#[uuid(0xde3d100b, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class FrameWindows; }

// Implements ICUIFrameWindow
RIDL!{#[uuid(0xde3d100c, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class FrameWindow; }

// Implements ICUIViewHosts
RIDL!{#[uuid(0xde3d100d, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ViewHosts; }

// Implements ICUIViewHost
RIDL!{#[uuid(0xde3d100e, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ViewHost; }

// Implements ICUIDockHosts
RIDL!{#[uuid(0xde3d100f, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DockHosts; }

// Implements ICUIDockHost
RIDL!{#[uuid(0xde3d1010, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DockHost; }

// Implements ICUIViewWindows
RIDL!{#[uuid(0xde3d1011, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ViewWindows; }

// Implements ICUIViewWindow
RIDL!{#[uuid(0xde3d1012, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ViewWindow; }

// Implements ICUIDockItems
RIDL!{#[uuid(0xde3d1013, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DockItems; }

// Implements ICUIDockItem
RIDL!{#[uuid(0xde3d1014, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class DockItem; }

// Implements ICUIScreenRect
RIDL!{#[uuid(0xde3d1015, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class ScreenRect; }

// Implements ICUIBitmapImage
RIDL!{#[uuid(0xde3d1016, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class BitmapImage; }

// Implements ICUIStatusText
RIDL!{#[uuid(0xde3d1017, 0xfee3, 0x4ec0, 0x98, 0x45, 0x53, 0xd1, 0x73, 0x54, 0xae, 0x24)]
class StatusText; }

// Implements IPDFVBASettings
RIDL!{#[uuid(0x1af50001, 0x1359, 0x11d7, 0x93, 0xda, 0x00, 0x90, 0x27, 0x58, 0x61, 0xfc)]
class PDFVBASettings; }

// Implements IPrnVBAPrinters
RIDL!{#[uuid(0x0a9f0001, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class SystemPrinters; }

// Implements IPrnVBAPrinter
RIDL!{#[uuid(0x0a9f0002, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class Printer; }

// Implements IPrnVBAPrintJob
RIDL!{#[uuid(0x0a9f0003, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class PrintJob; }

// Implements IPrnVBAPrintDocuments
RIDL!{#[uuid(0x0a9f0004, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class PrintDocuments; }

// Implements IPrnVBAPrintPages
RIDL!{#[uuid(0x0a9f0005, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class PrintPages; }

// Implements IPrnVBAPrintSettings
RIDL!{#[uuid(0x0a9f0006, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class PrintSettings; }

// Implements IPrnVBAPrintSeparations
RIDL!{#[uuid(0x0a9f0007, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class PrintSeparations; }

// Implements IPrnVBASeparationPlates
RIDL!{#[uuid(0x0a9f0008, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class SeparationPlates; }

// Implements IPrnVBASeparationPlate
RIDL!{#[uuid(0x0a9f0009, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class SeparationPlate; }

// Implements IPrnVBAPrintPrepress
RIDL!{#[uuid(0x0a9f000a, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class PrintPrepress; }

// Implements IPrnVBAPrintPostScript
RIDL!{#[uuid(0x0a9f000b, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class PrintPostScript; }

// Implements IPrnVBAPrintTrapping
RIDL!{#[uuid(0x0a9f000c, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class PrintTrapping; }

// Implements IPrnVBATrapLayers
RIDL!{#[uuid(0x0a9f000d, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class TrapLayers; }

// Implements IPrnVBATrapLayer
RIDL!{#[uuid(0x0a9f000e, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class TrapLayer; }

// Implements IPrnVBAPrintOptions
RIDL!{#[uuid(0x0a9f000f, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class PrintOptions; }

// Implements IPrnVBAPrintLayout
RIDL!{#[uuid(0x0a9f0010, 0x6a30, 0x4fb2, 0xb5, 0x15, 0x6e, 0x64, 0x6d, 0x90, 0x39, 0x99)]
class PrintLayout; }


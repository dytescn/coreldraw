# coreldraw —— cyctron libary 插件

把 `cdrsdk` 的核心接口打包成动态库，通过 [cyctron](../README.md) 的
libary 机制以 C ABI 暴露。仓库根目录是 SDK，本目录是插件壳。

## 构建与安装

```bash
# 仓库根目录执行
bash ffi/package.sh
# 等价于
cargo build --release --manifest-path ffi/Cargo.toml
cp ffi/target/release/coreldraw.dll <cyctron>/libary/
```

默认安装到 `D:/codes/github/funxdata/cyctron/libary/coreldraw.dll`，
可通过环境变量覆盖：

```bash
CYCTRON_LIBARY=/path/to/libary bash ffi/package.sh
```

> **每次改动接口后**：重新执行 `ffi/package.sh`，然后重启 cyctron。
> cyctron 对已加载的动态库做了句柄缓存（`dlopen` 只成功一次），
> 单纯覆盖 DLL 不会让运行中的进程重新加载。

## HTTP 调用约定

```http
POST /coreldraw
Content-Type: application/json
FFI-Symbol: shape_create_rectangle

{"name":"rect-1","x":0.5,"y":0.5,"width":2,"height":1.5}
```

- 每个接口对应一个导出符号：`post_<interface>`。
- 入参：UTF-8 JSON，可为 `NULL`。
- 出参：64 KB 缓冲区中的紧凑 JSON，以 `\0` 结尾。
- 返回值：固定 `0`；业务错误放在 JSON 里。
- 输出指针为空返回 `-1`（cyctron 侧表现为 404 `Plugin Not Found`）。

统一信封：

```json
{"ok":true,"method":"POST","interface":"app_info","data":{...}}
{"ok":false,"method":"POST","interface":"app_info","error":"no_document","message":"..."}
```

`health` 不需要 CorelDRAW，返回插件信息与全部接口名（`data.interfaces`）。

## 参数约定

| 选择器 | 含义 |
|---|---|
| `doc` | 文档，数字为 1-based 索引，字符串为名称；省略取活动文档 |
| `page` | 页面，数字为索引，字符串为名称；省略取活动页 |
| `layer` | 图层，数字为索引，字符串为名称；省略取活动图层 |
| `name` | 图形名称（创建接口写入唯一名称，后续操作按名称定位） |
| `index` | 图形在活动页 `Shapes` 集合中的 1-based 索引（`shape_*`） |

尺寸/坐标使用文档当前单位（新建文档默认英寸）。典型用法是先
`page_set_size` 设成合适尺寸，再按英寸传坐标。

## 接口清单（41）

| 分组 | 接口 |
|---|---|
| meta | `health` |
| app | `app_info` `app_documents` `app_create_document` `app_open_document` `app_convert_units` `app_create_color` `app_set_visible` |
| document | `doc_info` `doc_pages` `doc_create_page` `doc_delete_page` `doc_layers` `doc_create_layer` `doc_shapes` `doc_selection` `doc_clear_selection` `doc_save_as` `doc_export` `doc_close` |
| page | `page_info` `page_set_size` |
| layer | `layer_info` `layer_shapes` `layer_delete` |
| shape | `shape_create_rectangle` `shape_create_ellipse` `shape_create_polygon` `shape_create_text` `shape_info` `shape_move` `shape_set_size` `shape_rotate` `shape_set_fill` `shape_set_outline` `shape_select` `shape_delete` |
| text | `text_info` `text_set_content` |
| view | `view_info` `view_set_zoom` |

### 常用参数

| 接口 | 参数 |
|---|---|
| `app_open_document` | `{"path":"C:/tmp/a.cdr"}` |
| `app_convert_units` | `{"value":1,"from":1,"to":3}`（1=英寸，3=毫米） |
| `app_create_color` | `{"r":255,"g":128,"b":0}` |
| `page_set_size` | `{"page":1,"width":6,"height":4}` |
| `doc_create_layer` | `{"name":"ffi-layer"}` |
| `shape_create_rectangle` | `{"name":"r1","x":0.5,"y":0.5,"width":2,"height":1.5,"radius":0}` |
| `shape_create_ellipse` | `{"name":"e1","cx":4,"cy":2,"rx":0.5,"ry":0.5}` |
| `shape_create_polygon` | `{"name":"p1","x":0.5,"y":2.5,"width":1.5,"height":1.5,"sides":6,"star":false}` |
| `shape_create_text` | `{"name":"t1","x":3,"y":3,"text":"Hello","font":"Arial","size":18}` |
| `shape_set_fill` | `{"name":"r1","r":255,"g":128,"b":0}` |
| `shape_set_outline` | `{"name":"r1","width":0.05,"r":0,"g":0,"b":0}` |
| `doc_export` | `{"path":"C:/tmp/a.png","filter":"png"}`（也支持 `jpg` 与数字 filter） |
| `doc_save_as` | `{"path":"C:/tmp/a.cdr"}` |
| `doc_close` | `{}` 或 `{"doc":1,"save":false}` |
| `view_set_zoom` | `{"zoom":75}` |

## 测试

```bash
# 1. 安装插件并重启 cyctron
bash ffi/package.sh
# ./cyctron start -port 44944

# 2. 运行接口测试（首次调用会自动启动 CorelDRAW）
deno run --allow-net --allow-env --allow-read --allow-write \
  ts/coreldraw_request.ts 44944
```

测试会自建临时文档、导出 PNG、另存 CDR，结束后关闭文档并删除临时文件。
若本机已打开其它文档，测试以“初始文档数”为基准，不会关闭它们。

## 实现说明

- **COM 初始化**：cyctron 的 libffi 调用固定在主线程，插件用
  `thread_local!` 做一次 `CoInitializeEx(STA)`，并缓存 `IvgApplication`；
  缓存失效（CorelDRAW 被关闭）时自动重新绑定。首次绑定会启动 CorelDRAW。
- **图形定位**：CorelDRAW 的 `Shape.Name` 默认不唯一（多个矩形都叫
  `Rectangle`），创建接口会写入唯一名称（`name` 参数或 `ffi_<n>`）。
- **类型库签名**：`Document.SaveAs` 需要 `(FileName, Options)`；
  `Document.Export` 是五参数 `(FileName, Filter, Range, Options,
  PaletteOptions)`；页面创建用 `AddPages(1)` 而不是 `CreatePage`。
  这些都在 `cdrsdk` 内修正。

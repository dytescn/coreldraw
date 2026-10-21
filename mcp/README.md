# `mcp/` —— CorelDRAW 接口元数据

本目录存放 **cdrsdk 公开接口的机器可读元数据**，供 MCP server、AI 助手或
内部工具检索：有哪些 COM 包装类型、每个类型有哪些方法、参数和返回值，
以及 `VGCore` 枚举的成员与真实整数值。

> 这里不是独立的 MCP server，也不参与 crate 编译；它只是数据目录。
> 运行时绑定仍然由 `src/` 中的 `Ivg*` / `ICui*` 包装类型通过
> `IDispatch` 后期绑定完成。

## 目录结构

```
mcp/
├── README.md          本文件
├── schema.json        JSON 结构说明
├── generate.py        从 src/ 重新生成元数据的脚本
├── interfaces.json    索引：模块、类型数、方法数、测试与覆盖率
├── enums.json         全部枚举、成员值、重复值别名
├── coverage.json      测试覆盖统计（机器可读）
├── coverage.md        测试覆盖统计（表格）
└── modules/           每个模块一份完整类型/方法清单
    ├── app.json
    ├── shape.json
    ├── curve.json
    └── ...
```

当前规模（由 `generate.py` 统计）：

| 项 | 数量 |
|---|---|
| 模块 | 21 |
| COM 包装类型 | 253 |
| 公开方法 | 4649 |
| 枚举 | 146 |
| 测试用例 | 66 |
| 全局方法名覆盖（去重） | 357 / 2701（13.2%） |

## 数据格式

### `interfaces.json`

```json
{
  "crate": "cdrsdk",
  "version": "0.3.0",
  "type_count": 253,
  "method_count": 4649,
  "modules": [
    {
      "module": "shape",
      "files": ["src/shape/shape.rs", "..."],
      "types": 15,
      "methods": 420,
      "tests": {
        "files": ["tests/shape.rs", "tests/smoke.rs", "tests/e2e.rs"],
        "cases": 11
      },
      "coverage": {
        "method_name_count": 435,
        "covered_method_name_count": 70,
        "coverage_pct": 16.1
      }
    }
  ]
}
```

### `modules/<module>.json`

```json
{
  "module": "shape",
  "files": ["src/shape/shape.rs"],
  "types": [
    {
      "name": "IvgShape",
      "file": "src/shape/shape.rs",
      "methods": [
        {
          "name": "get_size",
          "params": "&self",
          "returns": "Option<(f64, f64)>"
        }
      ]
    }
  ]
}
```

### `enums.json`

```json
{
  "enum_count": 146,
  "enums": [
    {
      "name": "cdrFillType",
      "module": "enums/fill",
      "variant_count": 7,
      "variants": [
        { "name": "NoFill", "value": 0 },
        { "name": "UniformFill", "value": 1 }
      ],
      "aliases": []
    }
  ]
}
```

`aliases` 保存类型库中与主成员同值的别名（Rust 端以关联常量暴露），
例如 `cdrFilter::PDF = cdrFilter::AI9`。

## MCP / AI 检索建议

1. **按模块定位**：先读 `interfaces.json` 找模块，再读 `modules/<module>.json`。
2. **按方法名搜索**：对 `modules/*.json` 做全文检索（`name`、`params`、`returns`）。
3. **查枚举值**：读 `enums.json`，不要把 `cdr*` 常量写成魔数。
4. **生成调用代码**：方法签名已经包含 `Option<T>` / `bool` 返回约定：
   - 读属性 → `Option<T>`
   - 写属性 / 无返回调用 → `bool`
   - 返回 COM 对象 → `Option<IvgXxx>`

示例问题 → 定位：

| 问题 | 应读取 |
|---|---|
| 怎么给图形上渐变？ | `modules/fill.json` → `apply_fountain_fill` |
| 页面尺寸怎么改？ | `modules/page.json` → `set_size` |
| `cdrShapeType` 有哪些值？ | `enums.json` → `cdrShapeType` |
| 文本怎么读内容？ | `modules/text.json` → `story` / `contents` |

## 测试覆盖

`coverage.json` / `coverage.md` 由 `generate.py` 扫描 `tests/*.rs` 生成：

- **测试用例数**：每个测试文件里的 `#[test]` 数量；
- **模块级覆盖**：该模块的方法名与该模块关联测试文件中出现的方法调用名求交集；
- **全局覆盖**：所有模块方法名去重后，再与全部测试调用名求交集。

口径是**静态近似**：同名方法跨类型会被计为已覆盖，因此数字是上限；真实的
“类型 + 方法”覆盖需要逐类型统计。当前全局约 **13.2%**，模块级累加约 **22.2%**。

覆盖较完整的模块：`app`、`page`、`document`、`layer`、`tree`、`view`、`symbol`、
`style`、`print`、`import_export`、`ui`、`enums` 都有独立测试文件；`effect`、`misc`、
`shape`、`fill`、`text` 等仍以代表性调用为主。

## 重新生成

在仓库根目录执行：

```bash
python mcp/generate.py
```

脚本只读 `src/**/*.rs` 和 `tests/*.rs`，重写 `interfaces.json`、`enums.json`、
`modules/*.json`、`coverage.json` 和 `coverage.md`。新增模块、方法或测试后跑一次即可同步。

> 说明：方法签名由源码静态解析，复杂的跨行泛型签名会做空白归一化；
> 完整的、可直接编译的调用示例见 `tests/`。

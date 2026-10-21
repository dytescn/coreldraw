# 接口测试覆盖（静态近似）

- 测试用例：66
- 全局方法名覆盖（去重）：357 / 2701（13.2%）
- 模块级累加覆盖：788 / 3552（22.2%）
- 口径：按去重后的 `pub fn` 名称与 `tests/*.rs` 中的调用名求交集；同名方法跨类型会高估。

| 模块 | 测试文件 | 用例数 | 方法名覆盖 |
|---|---|---:|---:|
| app | app.rs<br>e2e.rs<br>effect.rs<br>shape.rs<br>smoke.rs | 22 | 51/81（63.0%） |
| color | color.rs | 6 | 27/197（13.7%） |
| curve | curve.rs | 2 | 30/144（20.8%） |
| document | document.rs<br>e2e.rs<br>effect.rs<br>shape.rs<br>smoke.rs | 20 | 60/142（42.3%） |
| effect | effect.rs | 2 | 26/261（10.0%） |
| fill | fill_outline.rs | 2 | 39/230（17.0%） |
| geometry | geometry.rs | 3 | 35/171（20.5%） |
| import_export | import_export.rs | 1 | 7/27（25.9%） |
| layer | e2e.rs<br>effect.rs<br>page_layer.rs<br>shape.rs<br>smoke.rs | 15 | 33/89（37.1%） |
| misc | misc.rs | 3 | 50/298（16.8%） |
| outline | fill_outline.rs | 2 | 29/94（30.9%） |
| page | e2e.rs<br>page_layer.rs<br>smoke.rs | 10 | 37/83（44.6%） |
| print | print.rs | 2 | 49/361（13.6%） |
| shape | e2e.rs<br>shape.rs<br>smoke.rs | 11 | 70/435（16.1%） |
| structs | structs.rs | 8 | 56/183（30.6%） |
| style | style.rs | 1 | 38/173（22.0%） |
| symbol | symbol.rs | 1 | 13/43（30.2%） |
| text | text.rs | 1 | 42/221（19.0%） |
| tree | view_tree.rs | 2 | 15/44（34.1%） |
| ui | ui.rs | 1 | 53/182（29.1%） |
| view | view_tree.rs | 2 | 28/93（30.1%） |

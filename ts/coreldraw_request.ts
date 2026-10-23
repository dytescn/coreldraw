// coreldraw_request.ts —— coreldraw 动态库（ffi/coreldraw.dll）接口验证
//
// 前置：
//   1) 把 ffi/target/release/coreldraw.dll 放到 cyctron 的 ./libary/ 目录
//      （仓库内执行 `bash ffi/package.sh` 即可）
//   2) 启动 cyctron：./cyctron start -port 44944
//   3) 本机安装 CorelDRAW 2018+；首次调用会自动启动 CorelDRAW
//
// 运行：
//   deno run --allow-net --allow-env --allow-read --allow-write \
//     ts/coreldraw_request.ts [port] [timeoutSec]
//
// 说明：--allow-env 读取 TEMP 目录；--allow-read/--allow-write 用于
// 校验并清理导出/另存生成的临时文件。
// 覆盖 INTERFACES 中的全部 41 个接口：health / app_* / doc_* / page_* /
// layer_* / shape_* / text_* / view_*，以及 libary 协议负例。
// 脚本自行创建临时文档与临时文件，结束时关闭文档不保存并删除临时文件。

const port = Deno.args[0] ?? "44944";
const timeoutSec = Number(Deno.args[1] ?? "300");
const base = `http://127.0.0.1:${port}`;
const PLUGIN = "/coreldraw";
const timeoutMs =
  Number.isFinite(timeoutSec) && timeoutSec > 0 ? timeoutSec * 1000 : 300_000;

let passed = 0;
let failed = 0;

interface Reply {
  status: number;
  text: string;
  data: any;
}

async function post(
  symbol: string,
  params?: unknown,
  headers: Record<string, string> = {},
): Promise<Reply> {
  const body = params === undefined ? undefined : JSON.stringify(params);
  const res = await fetch(base + PLUGIN, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "FFI-Symbol": symbol,
      ...headers,
    },
    body,
    signal: AbortSignal.timeout(timeoutMs),
  });
  const text = await res.text();
  let data: any = null;
  try {
    data = JSON.parse(text);
  } catch {
    // 404 静态文本等由调用方按 text 断言
  }
  return { status: res.status, text, data };
}

function check(name: string, ok: boolean, detail = "") {
  if (ok) {
    passed++;
    console.log(`  [PASS] ${name}`);
  } else {
    failed++;
    console.log(`  [FAIL] ${name}  ${detail}`);
  }
}

/** 调用接口并断言 ok:true，返回 data（失败时返回 null，避免后续空指针）。 */
async function ok(
  name: string,
  symbol: string,
  params?: unknown,
  pred: (d: any) => boolean = () => true,
): Promise<any> {
  const r = await post(symbol, params);
  const d = r.data?.data;
  const good = r.status === 200 && r.data?.ok === true && pred(d);
  check(
    name,
    good,
    `status=${r.status} body=${r.text.slice(0, 240)}`,
  );
  return good ? d : null;
}

/** 调用接口并断言业务错误码。 */
async function fails(name: string, symbol: string, params: unknown, code: string) {
  const r = await post(symbol, params);
  check(
    name,
    r.status === 200 && r.data?.ok === false && r.data?.error === code,
    `status=${r.status} body=${r.text.slice(0, 240)}`,
  );
}

const near = (a: number | null | undefined, b: number, eps = 1e-6) =>
  typeof a === "number" && Math.abs(a - b) <= eps;

const tmp = (Deno.env.get("TEMP") ?? Deno.env.get("TMP") ?? ".").replace(
  /\\/g,
  "/",
);
const stamp = `${Date.now()}-${Math.floor(Math.random() * 1e6)}`;
const pngPath = `${tmp}/cdrsdk_ffi_${stamp}.png`;
const cdrPath = `${tmp}/cdrsdk_ffi_${stamp}.cdr`;

const EXPECTED_INTERFACES = [
  "health",
  "app_info",
  "app_documents",
  "app_create_document",
  "app_open_document",
  "app_convert_units",
  "app_create_color",
  "app_set_visible",
  "doc_info",
  "doc_pages",
  "doc_create_page",
  "doc_delete_page",
  "doc_layers",
  "doc_create_layer",
  "doc_shapes",
  "doc_selection",
  "doc_clear_selection",
  "doc_save_as",
  "doc_export",
  "doc_close",
  "page_info",
  "page_set_size",
  "layer_info",
  "layer_shapes",
  "layer_delete",
  "shape_create_rectangle",
  "shape_create_ellipse",
  "shape_create_polygon",
  "shape_create_text",
  "shape_info",
  "shape_move",
  "shape_set_size",
  "shape_rotate",
  "shape_set_fill",
  "shape_set_outline",
  "shape_select",
  "shape_delete",
  "text_info",
  "text_set_content",
  "view_info",
  "view_set_zoom",
];

console.log(`== coreldraw 接口验证 ${base}${PLUGIN} ==`);

try {
  // ------------------------------------------------------------
  // 1. health（无 COM）
  // ------------------------------------------------------------
  const health = await ok("health → 插件信息", "health", undefined, (d) =>
    d?.plugin === "coreldraw" && d?.sdk === "cdrsdk" &&
    Array.isArray(d?.interfaces));
  const listed: string[] = health?.interfaces ?? [];
  check(
    "health → 41 个接口全部登记",
    listed.length === EXPECTED_INTERFACES.length &&
      EXPECTED_INTERFACES.every((n) => listed.includes(n)),
    `listed=${listed.length} expect=${EXPECTED_INTERFACES.length} missing=${
      EXPECTED_INTERFACES.filter((n) => !listed.includes(n)).join(",")
    }`,
  );

  // ------------------------------------------------------------
  // 2. app_*：信息、文档、单位、颜色
  // ------------------------------------------------------------
  await ok("app_info → CorelDRAW 信息", "app_info", undefined, (d) =>
    typeof d?.version === "string" && d.version.length > 0 &&
    typeof d?.version_major === "number");
  const appVisible = (await post("app_info")).data?.data?.visible === true;
  await ok("app_set_visible → true", "app_set_visible", { visible: true }, (d) =>
    d?.visible === true);
  await ok("app_set_visible → 恢复", "app_set_visible", {
    visible: appVisible,
  }, (d) => d?.visible === appVisible);

  await ok("app_convert_units 1in → mm", "app_convert_units", {
    value: 1,
    from: 1,
    to: 3,
  }, (d) => near(d?.result, 25.4, 1e-3));
  await ok("app_create_color 255,128,0", "app_create_color", {
    r: 255,
    g: 128,
    b: 0,
  }, (d) =>
    d?.rgb?.r === 255 && d?.rgb?.g === 128 && d?.rgb?.b === 0 &&
    d?.hex === "#FF8000");

  const initialDocs = (await post("app_documents")).data?.data?.count ?? 0;
  const doc = await ok("app_create_document → 新文档", "app_create_document",
    undefined, (d) =>
      typeof d?.name === "string" && d?.pages >= 1 &&
      typeof d?.active_page === "string");
  // 后续接口都依赖这个文档
  if (!doc) throw new Error("app_create_document failed; aborting");

  const docList = await ok("app_documents → 包含新文档", "app_documents", undefined, (d) =>
    d?.count === initialDocs + 1 && Array.isArray(d?.items) &&
    d.items.some((it: any) => it.name === doc.name));
  const docIndex = docList?.items?.find((it: any) => it.name === doc.name)?.index;

  await ok("doc_info → 与创建结果一致", "doc_info", undefined, (d) =>
    d?.name === doc.name && d?.pages >= 1 && d?.shapes === 0 &&
    typeof d?.active_layer === "string");
  await ok("doc_info → 按 doc 索引选择", "doc_info", { doc: docIndex }, (d) =>
    d?.name === doc.name);

  // ------------------------------------------------------------
  // 3. page_*：页面列表 / 新建 / 删除 / 尺寸
  // ------------------------------------------------------------
  const pages = await ok("doc_pages → 页面列表", "doc_pages", undefined, (d) =>
    d?.count >= 1 && Array.isArray(d?.items) && d.items[0]?.index === 1);
  const firstPage = pages?.items?.[0];
  check(
    "doc_pages → A4 页面尺寸有效",
    firstPage && firstPage.size?.width > 0 && firstPage.size?.height > 0,
    `page=${JSON.stringify(firstPage?.size)}`,
  );

  const page2 = await ok("doc_create_page → 第 2 页", "doc_create_page",
    undefined, (d) => d?.index === 2);
  await ok("page_info → 第 2 页", "page_info", { page: 2 }, (d) =>
    d?.index === 2 && d?.size?.width > 0);
  await ok("doc_delete_page → 删除第 2 页", "doc_delete_page", { page: 2 },
    (d) => d?.deleted?.index === 2 && d?.remaining === 1);
  await fails("doc_delete_page → 唯一页不可删", "doc_delete_page", {
    page: 1,
  }, "last_page");

  await ok("page_info → 活动页", "page_info", undefined, (d) =>
    d?.index === 1 && d?.layers >= 1);
  await ok("page_set_size 6x4", "page_set_size", {
    width: 6,
    height: 4,
  }, (d) => near(d?.size?.width, 6, 1e-3) && near(d?.size?.height, 4, 1e-3));

  // ------------------------------------------------------------
  // 4. layer_*：列表 / 新建 / 查询 / 删除
  // ------------------------------------------------------------
  await ok("doc_layers → 图层列表", "doc_layers", undefined, (d) =>
    d?.count >= 1 && Array.isArray(d?.items));
  await ok("doc_create_layer → ffi-layer", "doc_create_layer", {
    name: "ffi-layer",
  }, (d) => d?.name === "ffi-layer");
  await ok("layer_info → 新建图层", "layer_info", { layer: "ffi-layer" },
    (d) => d?.name === "ffi-layer" && d?.shapes === 0);
  await ok("layer_shapes → 新图层为空", "layer_shapes", { layer: "ffi-layer" },
    (d) => d?.count === 0);
  await ok("layer_delete → 删除新图层", "layer_delete", { layer: "ffi-layer" },
    (d) => d?.deleted?.name === "ffi-layer");
  await fails("layer_info → 不存在图层", "layer_info", {
    layer: "no-such-layer",
  }, "layer_not_found");

  // ------------------------------------------------------------
  // 5. shape_*：工厂 + 属性操作
  // ------------------------------------------------------------
  const rect = await ok("shape_create_rectangle", "shape_create_rectangle", {
    name: "ffi-rect",
    x: 0.5,
    y: 0.5,
    width: 2,
    height: 1.5,
    radius: 0.1,
  }, (d) =>
    d?.name === "ffi-rect" && near(d?.size?.width, 2, 1e-3) &&
    near(d?.size?.height, 1.5, 1e-3));

  await ok("shape_create_ellipse", "shape_create_ellipse", {
    name: "ffi-ellipse",
    cx: 4,
    cy: 2,
    rx: 0.5,
    ry: 0.5,
  }, (d) =>
    d?.name === "ffi-ellipse" && near(d?.size?.width, 1, 1e-3) &&
    near(d?.size?.height, 1, 1e-3));

  await ok("shape_create_polygon 6 边", "shape_create_polygon", {
    name: "ffi-polygon",
    x: 0.5,
    y: 2.5,
    width: 1.5,
    height: 1.5,
    sides: 6,
  }, (d) =>
    d?.name === "ffi-polygon" && d?.type === 4 &&
    near(d?.size?.height, 1.5, 1e-3) && d?.size?.width > 0);

  await ok("shape_create_text", "shape_create_text", {
    name: "ffi-text",
    x: 3,
    y: 3,
    text: "Hello FFI",
    size: 18,
  }, (d) =>
    d?.name === "ffi-text" && d?.type === 6 &&
    d?.text?.content?.includes("Hello FFI"));

  await ok("doc_shapes → 4 个图形", "doc_shapes", undefined, (d) =>
    d?.count === 4 &&
    ["ffi-rect", "ffi-ellipse", "ffi-polygon", "ffi-text"].every((n) =>
      d.items.some((it: any) => it.name === n)
    ));
  await ok("layer_shapes → 活动图层 4 个图形", "layer_shapes", undefined, (d) =>
    d?.count === 4);

  await ok("shape_info → 按名称", "shape_info", { name: "ffi-rect" }, (d) =>
    d?.name === "ffi-rect" && near(d?.size?.width, 2, 1e-3));
  await ok("shape_info → 按 index", "shape_info", { index: 1 }, (d) =>
    typeof d?.name === "string");

  await ok("shape_move dx=0.5", "shape_move", {
    name: "ffi-rect",
    dx: 0.5,
    dy: 0,
  }, (d) => near(d?.position?.x, 1.0, 1e-3));
  await ok("shape_set_size 2.5x1.5", "shape_set_size", {
    name: "ffi-rect",
    width: 2.5,
    height: 1.5,
  }, (d) => near(d?.size?.width, 2.5, 1e-3) && near(d?.size?.height, 1.5, 1e-3));
  await ok("shape_rotate 15°", "shape_rotate", {
    name: "ffi-rect",
    angle: 15,
  }, (d) => near(d?.rotation?.angle, 15, 1e-3));

  await ok("shape_set_fill RGB(255,128,0)", "shape_set_fill", {
    name: "ffi-rect",
    r: 255,
    g: 128,
    b: 0,
  }, (d) =>
    d?.fill?.uniform?.rgb?.r === 255 &&
    d?.fill?.uniform?.rgb?.g === 128 &&
    d?.fill?.uniform?.rgb?.b === 0);

  await ok("shape_set_outline 0.05in 黑色", "shape_set_outline", {
    name: "ffi-rect",
    width: 0.05,
    r: 0,
    g: 0,
    b: 0,
  }, (d) =>
    d?.outline?.width > 0 &&
    d?.outline?.color?.rgb?.r === 0);

  await ok("doc_clear_selection", "doc_clear_selection", undefined, (d) =>
    d?.cleared === true);
  await ok("doc_selection → 空选择", "doc_selection", undefined, (d) =>
    d?.count === 0);
  await ok("shape_select → 选中矩形", "shape_select", {
    name: "ffi-rect",
    selected: true,
  }, (d) => d?.selected === true);
  await ok("doc_selection → 1 个选中", "doc_selection", undefined, (d) =>
    d?.count >= 1 && d.items.some((it: any) => it.name === "ffi-rect"));
  await ok("doc_clear_selection → 清空", "doc_clear_selection", undefined, (d) =>
    d?.cleared === true);

  // ------------------------------------------------------------
  // 6. text_*：读取 / 改写
  // ------------------------------------------------------------
  await ok("text_info → 初始文本", "text_info", { name: "ffi-text" }, (d) =>
    d?.is_artistic === true && d?.content?.includes("Hello FFI"));
  await ok("text_set_content → 更新", "text_set_content", {
    name: "ffi-text",
    text: "Updated FFI text",
  }, (d) => d?.text?.content?.includes("Updated FFI text"));
  await ok("text_info → 更新后", "text_info", { name: "ffi-text" }, (d) =>
    d?.content?.includes("Updated FFI text") &&
    !d.content.includes("Hello FFI"));

  // ------------------------------------------------------------
  // 7. shape_delete
  // ------------------------------------------------------------
  await ok("shape_create_rectangle → 待删除", "shape_create_rectangle", {
    name: "ffi-delete-me",
    x: 1,
    y: 1,
    width: 0.5,
    height: 0.5,
  });
  await ok("shape_delete → 删除", "shape_delete", {
    name: "ffi-delete-me",
  }, (d) => d?.deleted === "ffi-delete-me" && d?.shapes === 4);
  await fails("shape_info → 已删除图形", "shape_info", {
    name: "ffi-delete-me",
  }, "shape_not_found");
  await fails("shape_info → 未知图形", "shape_info", {
    name: "no-such-shape",
  }, "shape_not_found");

  // ------------------------------------------------------------
  // 8. view_*：活动视图 / 缩放
  // ------------------------------------------------------------
  const view = await ok("view_info → 活动视图", "view_info", undefined, (d) =>
    typeof d?.zoom === "number" && d?.zoom > 0);
  const originalZoom: number = view?.zoom ?? 100;
  const zoomed = await ok("view_set_zoom 75", "view_set_zoom", { zoom: 75 },
    (d) =>
      typeof d?.zoom === "number" &&
      (near(d.zoom, 75, 1e-2) || near(d.zoom, 0.75, 1e-4)));
  if (zoomed) {
    await ok("view_set_zoom → 恢复原缩放", "view_set_zoom", {
      zoom: originalZoom,
    }, (d) => typeof d?.zoom === "number" && d.zoom > 0);
  }

  // ------------------------------------------------------------
  // 9. 导出 / 另存为
  // ------------------------------------------------------------
  await ok("doc_export → PNG", "doc_export", { path: pngPath }, (d) =>
    d?.path === pngPath && d?.size > 0);
  await ok("doc_export → 文件已写盘", "doc_export", { path: pngPath }, (d) =>
    d?.size > 0);
  check(
    "doc_export → PNG 文件存在",
    await exists(pngPath),
    `path=${pngPath}`,
  );

  await ok("doc_save_as → CDR", "doc_save_as", { path: cdrPath }, (d) =>
    d?.path === cdrPath && d?.size > 0);
  check("doc_save_as → CDR 文件存在", await exists(cdrPath), `path=${cdrPath}`);

  // ------------------------------------------------------------
  // 10. 关闭 / 重新打开
  // ------------------------------------------------------------
  await ok("doc_close → 关闭（不保存）", "doc_close", undefined, (d) =>
    d?.closed === true);
  await ok("app_documents → 恢复到初始文档数", "app_documents", undefined, (d) =>
    d?.count === initialDocs);

  const reopened = await ok("app_open_document → 打开 CDR", "app_open_document",
    { path: cdrPath }, (d) => typeof d?.name === "string" && d?.shapes === 4);
  if (reopened) {
    await ok("doc_info → 打开后的文档", "doc_info", undefined, (d) =>
      d?.name === reopened.name && d?.shapes === 4);
    await ok("doc_close → 关闭重开的文档", "doc_close", undefined, (d) =>
      d?.closed === true);
  }

  // ------------------------------------------------------------
  // 11. 参数与协议负例
  // ------------------------------------------------------------
  await fails("doc_save_as → 缺少 path", "doc_save_as", {}, "missing_param");
  await fails("page_set_size → 缺少 height", "page_set_size", {
    width: 5,
  }, "missing_param");

  const badSymbol = await post("no_such_symbol", {});
  check(
    "FFI-Symbol 负例 → 404",
    badSymbol.status === 404 &&
      badSymbol.text.toLowerCase().includes("plugin not found"),
    `status=${badSymbol.status} body=${badSymbol.text.slice(0, 120)}`,
  );
  const unknownPlugin = await fetch(`${base}/no_such_lib`);
  check(
    "未知插件 → 404",
    unknownPlugin.status === 404,
    `status=${unknownPlugin.status}`,
  );
} catch (e) {
  failed++;
  console.log(`  [FAIL] 脚本异常：${e instanceof Error ? e.message : e}`);
  // 尽力清理：关闭可能残留的文档
  try {
    await post("doc_close", {});
  } catch {
    // ignore
  }
} finally {
  for (const p of [pngPath, cdrPath]) {
    try {
      await Deno.remove(p);
    } catch {
      // 文件不存在时忽略
    }
  }
}

async function exists(path: string): Promise<boolean> {
  try {
    const st = await Deno.stat(path);
    return st.isFile && (st.size ?? 0) > 0;
  } catch {
    return false;
  }
}

console.log("=================================");
console.log(`Passed: ${passed}  Failed: ${failed}`);
if (failed > 0) Deno.exit(1);

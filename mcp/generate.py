import re, glob, json, os
from pathlib import Path

def brace_block(text, start):
    i = text.index('{', start); d = 0
    for j in range(i, len(text)):
        if text[j] == '{': d += 1
        elif text[j] == '}':
            d -= 1
            if d == 0: return text[i+1:j]
    return ''

def parse_methods(block):
    lines = block.split('\n'); out = []; i = 0
    while i < len(lines):
        s = lines[i].strip()
        if s.startswith('pub fn '):
            sig = s
            while sig.count('(') > sig.count(')') and i + 1 < len(lines):
                i += 1; sig += ' ' + lines[i].strip()
            sig = sig.split('{')[0].strip()
            m = re.match(r'pub fn (\w+)\s*\((.*)\)\s*(?:->\s*(.+))?$', sig, re.S)
            if m:
                out.append({'name': m.group(1), 'params': ' '.join(m.group(2).split()),
                            'returns': ' '.join((m.group(3) or '').split()) or None})
        i += 1
    return out

# ---------- source types ----------
mods = {}
for path in sorted(glob.glob('src/**/*.rs', recursive=True)):
    p = Path(path).as_posix(); parts = p.split('/')
    if len(parts) < 2 or parts[1] == 'enums' or parts[-1] in ('lib.rs', 'prelude.rs', 'types.rs'):
        continue
    text = Path(path).read_text(encoding='utf-8')
    entry = mods.setdefault(parts[1], {'module': parts[1], 'files': [], 'types': []})
    entry['files'].append(p)
    for sm in re.finditer(r'pub struct (\w+)\s*\{', text):
        name = sm.group(1)
        im = re.search(r'impl\s+' + re.escape(name) + r'\s*\{', text[sm.end():])
        methods = parse_methods(brace_block(text, sm.end() + im.start())) if im else []
        entry['types'].append({'name': name, 'file': p, 'methods': methods})

# ---------- tests ----------
tests = {}
for path in sorted(glob.glob('tests/*.rs')):
    p = Path(path).as_posix(); txt = Path(path).read_text(encoding='utf-8')
    cases = re.findall(r'#\[test\]\s*fn\s+(\w+)', txt)
    types = sorted(set(re.findall(r'\b(Ivg[A-Za-z0-9_]+|ICui[A-Za-z0-9_]+)\b', txt)))
    calls = sorted(set(re.findall(r'\.([a-z_][a-z0-9_]*)\(', txt)))
    tests[Path(path).stem] = {'file': p, 'cases': cases, 'types': types, 'calls': calls}

all_calls = set()
for t in tests.values(): all_calls |= set(t['calls'])

# 文件名包含关系 + 类型引用；smoke/e2e 作为跨模块冒烟测试
ALIAS = {'page': ['page_layer'], 'layer': ['page_layer'], 'fill': ['fill_outline'],
         'outline': ['fill_outline'], 'view': ['view_tree'], 'tree': ['view_tree']}
SMOKE = {'app', 'document', 'page', 'layer', 'shape'}

def test_files_for(mod):
    out = []
    for stem, info in tests.items():
        names = ALIAS.get(mod, []) + [mod]
        if any(n in stem for n in names) or (stem in ('smoke', 'e2e') and mod in SMOKE):
            out.append(info['file'])
        elif any(t in info['types'] for t in [x['name'] for x in mods[mod]['types']]):
            out.append(info['file'])
    return sorted(set(out))

# ---------- coverage ----------
coverage = []
for mod, m in sorted(mods.items()):
    names = {fn['name'] for t in m['types'] for fn in t['methods']}
    hit = names & all_calls
    files = test_files_for(mod)
    cases = sum(len(tests[Path(f).stem]['cases']) for f in files if Path(f).stem in tests)
    coverage.append({
        'module': mod,
        'test_files': files,
        'test_cases': cases,
        'method_name_count': len(names),
        'covered_method_name_count': len(hit),
        'coverage_pct': round(len(hit) * 100.0 / len(names), 1) if names else 0.0,
    })

# 把测试信息写进模块 JSON 和索引
cov_by_mod = {c['module']: c for c in coverage}
for mod, m in mods.items():
    m['tests'] = {'files': cov_by_mod[mod]['test_files'], 'cases': cov_by_mod[mod]['test_cases']}
    m['coverage'] = {k: cov_by_mod[mod][k] for k in
                     ('method_name_count', 'covered_method_name_count', 'coverage_pct')}
    with open(os.path.join('mcp', 'modules', mod + '.json'), 'w', encoding='utf-8', newline='') as f:
        json.dump(m, f, ensure_ascii=False, indent=2); f.write('\n')

for old in glob.glob('mcp/modules/*.json'):
    if Path(old).stem not in mods: os.remove(old)

type_count = sum(len(m['types']) for m in mods.values())
method_count = sum(len(t['methods']) for m in mods.values() for t in m['types'])
index = {
    'crate': 'cdrsdk', 'version': '0.3.0', 'source': 'src/',
    'description': 'CorelDRAW COM IDispatch late-binding SDK interface metadata',
    'type_count': type_count, 'method_count': method_count,
    'modules': [{'module': m['module'], 'files': m['files'], 'types': len(m['types']),
                 'methods': sum(len(t['methods']) for t in m['types']),
                 'tests': m['tests'], 'coverage': m['coverage']}
                for m in sorted(mods.values(), key=lambda x: x['module'])],
}
with open('mcp/interfaces.json', 'w', encoding='utf-8', newline='') as f:
    json.dump(index, f, ensure_ascii=False, indent=2); f.write('\n')

# ---------- enums ----------
enums = []
for path in sorted(glob.glob('src/enums/*.rs')):
    if path.endswith('mod.rs'): continue
    module = 'enums/' + Path(path).stem
    text = Path(path).read_text(encoding='utf-8')
    for em in re.finditer(r'pub enum (\w+)\s*\{(.*?)\n\}', text, re.S):
        name, body = em.group(1), em.group(2)
        variants = [{'name': vm.group(1), 'value': int(vm.group(2))}
                    for vm in re.finditer(r'^\s*([A-Za-z_]\w*|r#\w+)\s*=\s*(-?\d+)', body, re.M)]
        impl = re.search(r'impl\s+' + re.escape(name) + r'\s*\{', text[em.end():])
        aliases = []
        if impl:
            blk = brace_block(text, em.end() + impl.start())
            aliases = [{'name': am.group(1), 'alias_of': am.group(2)}
                       for am in re.finditer(r'pub const (\w+)\s*:\s*Self\s*=\s*Self::(\w+)\s*;', blk)]
        enums.append({'name': name, 'module': module, 'variants': variants,
                      'aliases': aliases, 'variant_count': len(variants)})
with open('mcp/enums.json', 'w', encoding='utf-8', newline='') as f:
    json.dump({'enum_count': len(enums), 'enums': enums}, f, ensure_ascii=False, indent=2); f.write('\n')

# ---------- coverage files ----------
all_method_names = set()
for m in mods.values():
    for t in m['types']:
        for fn in t['methods']:
            all_method_names.add(fn['name'])
global_covered = all_method_names & all_calls
covered = sum(c['covered_method_name_count'] for c in coverage)
total = sum(c['method_name_count'] for c in coverage)
cov = {
    'method_name_count': total,
    'covered_method_name_count': covered,
    'coverage_pct': round(covered * 100.0 / total, 1) if total else 0.0,
    'global_method_name_count': len(all_method_names),
    'global_covered_method_name_count': len(global_covered),
    'global_coverage_pct': round(len(global_covered) * 100.0 / len(all_method_names), 1) if all_method_names else 0.0,
    'test_case_count': sum(len(t['cases']) for t in tests.values()),
    'test_files': sorted(t['file'] for t in tests.values()),
    'note': '方法名按“去重后的 pub fn 名称”统计；同名方法跨类型会被高估，仅用于趋势参考。',
    'modules': coverage,
}
with open('mcp/coverage.json', 'w', encoding='utf-8', newline='') as f:
    json.dump(cov, f, ensure_ascii=False, indent=2); f.write('\n')

lines = ['# 接口测试覆盖（静态近似）', '',
         f'- 测试用例：{cov["test_case_count"]}',
         f'- 全局方法名覆盖（去重）：{len(global_covered)} / {len(all_method_names)}（{cov["global_coverage_pct"]}%）',
         f'- 模块级累加覆盖：{covered} / {total}（{cov["coverage_pct"]}%）',
         '- 口径：按去重后的 `pub fn` 名称与 `tests/*.rs` 中的调用名求交集；同名方法跨类型会高估。',
         '', '| 模块 | 测试文件 | 用例数 | 方法名覆盖 |', '|---|---|---:|---:|']
for c in coverage:
    files = '<br>'.join(Path(f).name for f in c['test_files']) or '—'
    lines.append(f'| {c["module"]} | {files} | {c["test_cases"]} | {c["covered_method_name_count"]}/{c["method_name_count"]}（{c["coverage_pct"]}%） |')
Path('mcp/coverage.md').write_text('\n'.join(lines) + '\n', encoding='utf-8')

print('types', type_count, 'methods', method_count, 'modules', len(mods), 'enums', len(enums))
print('tests', cov['test_case_count'], 'coverage', f'{cov["coverage_pct"]}%')

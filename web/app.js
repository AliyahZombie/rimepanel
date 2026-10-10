'use strict';
/* RimePanel frontend — dependency free. Line icons only.
 * (c) AliyahZombie */

// ------------------------------------------------------------------ icons
const P = {
  rime: '<path d="M12.00 3.00 L19.79 7.50 L19.79 16.50 L12.00 21.00 L4.21 16.50 L4.21 7.50 ZM12 12 L4.21 7.50 M12 12 L19.79 7.50 M12 12 L12.00 21.00 " stroke-width="1.6"/><path d="M12.00 5.45 L12.00 9.55 M13.78 6.48 L10.22 8.52 M13.78 8.53 L10.22 6.47 M8.10 12.20 L8.10 16.30 M9.88 13.22 L6.32 15.28 M9.88 15.28 L6.32 13.22 M15.90 12.20 L15.90 16.30 M17.68 13.22 L14.12 15.28 M17.68 15.28 L14.12 13.22 " stroke-width="1.25"/>',
  cube: '<path d="M12 2.5 3.5 7v10l8.5 4.5 8.5-4.5V7z"/><path d="m3.5 7 8.5 4.5L20.5 7M12 11.5v10"/>',
  home: '<path d="M3 10.5 12 3l9 7.5"/><path d="M5 9.5V21h14V9.5"/><path d="M9.5 21v-6h5v6"/>',
  server: '<rect x="3" y="4" width="18" height="7" rx="2"/><rect x="3" y="13" width="18" height="7" rx="2"/><path d="M7 7.5h.01M7 16.5h.01M11 7.5h6M11 16.5h6"/>',
  layers: '<path d="m12 3 9 5-9 5-9-5z"/><path d="m3 12.5 9 5 9-5"/><path d="m3 17 9 5 9-5"/>',
  box: '<path d="M12 2.8 3.5 7.4v9.2L12 21.2l8.5-4.6V7.4z"/><path d="m3.5 7.4 8.5 4.6 8.5-4.6M12 12v9.2M7.8 5.1l8.4 4.6"/>',
  globe: '<circle cx="12" cy="12" r="9"/><path d="M3 12h18M12 3c2.5 2.6 3.8 5.6 3.8 9s-1.3 6.4-3.8 9c-2.5-2.6-3.8-5.6-3.8-9S9.5 5.6 12 3z"/>',
  activity: '<path d="M3 12h4l3-8 4 16 3-8h4"/>',
  sliders: '<path d="M4 6h10M18 6h2M4 12h4M12 12h8M4 18h11M19 18h1"/><circle cx="16" cy="6" r="2"/><circle cx="10" cy="12" r="2"/><circle cx="17" cy="18" r="2"/>',
  play: '<path d="M7 4.5v15l12-7.5z"/>',
  stop: '<rect x="5.5" y="5.5" width="13" height="13" rx="2"/>',
  restart: '<path d="M3.5 12a8.5 8.5 0 1 0 2.6-6.1"/><path d="M3.5 4v4.5H8"/>',
  zap: '<path d="M13 2.5 4.5 13.5H12l-1 8 8.5-11H12z"/>',
  terminal: '<rect x="3" y="4" width="18" height="16" rx="2"/><path d="m7 9 3 3-3 3M13 15h4"/>',
  prompt: '<path d="m6 8 4 4-4 4M12 16h6"/>',
  folder: '<path d="M3 7a2 2 0 0 1 2-2h4l2 2.5h8a2 2 0 0 1 2 2V18a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>',
  folderPlus: '<path d="M3 7a2 2 0 0 1 2-2h4l2 2.5h8a2 2 0 0 1 2 2V18a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><path d="M12 11v5M9.5 13.5h5"/>',
  file: '<path d="M14 3H7a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8z"/><path d="M14 3v5h5"/>',
  fileText: '<path d="M14 3H7a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8z"/><path d="M14 3v5h5M9 13h6M9 17h4"/>',
  filePlus: '<path d="M14 3H7a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8z"/><path d="M14 3v5h5M12 11v6M9 14h6"/>',
  fileZip: '<path d="M14 3H7a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8z"/><path d="M14 3v5h5M10 5h1M10 8h1M10 11h1M9.5 14h2v3h-2z"/>',
  upload: '<path d="M12 16V4M7 9l5-5 5 5"/><path d="M4 16v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2"/>',
  download: '<path d="M12 4v12M7 11l5 5 5-5"/><path d="M4 16v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2"/>',
  trash: '<path d="M4 7h16M10 11v6M14 11v6"/><path d="M6 7l1 12a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2l1-12M9 7V4h6v3"/>',
  edit: '<path d="M4 20h4L19 9a2.1 2.1 0 0 0-3-3L5 17z"/><path d="m14 8 3 3"/>',
  rename: '<path d="M4 7h10M4 12h7M4 17h5"/><path d="M14 20h2.5l5-5a1.8 1.8 0 0 0-2.5-2.5l-5 5z"/>',
  archive: '<rect x="3" y="4" width="18" height="5" rx="1.5"/><path d="M5 9v9a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V9M10 13h4"/>',
  extract: '<rect x="3" y="4" width="18" height="5" rx="1.5"/><path d="M5 9v9a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V9M12 12v5M9.5 14.5 12 17l2.5-2.5"/>',
  plus: '<path d="M12 5v14M5 12h14"/>',
  x: '<path d="M6 6l12 12M18 6 6 18"/>',
  check: '<path d="m5 12.5 4.5 4.5L19 7"/>',
  checkCircle: '<circle cx="12" cy="12" r="9"/><path d="m8 12.5 2.8 2.8L16.5 9.5"/>',
  xCircle: '<circle cx="12" cy="12" r="9"/><path d="m9 9 6 6M15 9l-6 6"/>',
  minusCircle: '<circle cx="12" cy="12" r="9"/><path d="M8 12h8"/>',
  search: '<circle cx="11" cy="11" r="6.5"/><path d="m20 20-4.2-4.2"/>',
  chevR: '<path d="m9 6 6 6-6 6"/>',
  chevL: '<path d="m15 6-6 6 6 6"/>',
  chevD: '<path d="m6 9 6 6 6-6"/>',
  arrowUp: '<path d="M12 19V5M6 11l6-6 6 6"/>',
  link: '<path d="M10 14a4 4 0 0 0 5.7 0l3-3a4 4 0 0 0-5.7-5.7l-1 1"/><path d="M14 10a4 4 0 0 0-5.7 0l-3 3a4 4 0 0 0 5.7 5.7l1-1"/>',
  cpu: '<rect x="6" y="6" width="12" height="12" rx="2"/><rect x="9.5" y="9.5" width="5" height="5"/><path d="M9 3v3M15 3v3M9 18v3M15 18v3M3 9h3M3 15h3M18 9h3M18 15h3"/>',
  memory: '<rect x="3" y="7" width="18" height="10" rx="1.5"/><path d="M7 11v2M11 11v2M15 11v2M6 17v3M18 17v3"/>',
  disk: '<rect x="3" y="13" width="18" height="7" rx="2"/><path d="M5 13 7.5 5h9L19 13"/><path d="M7 16.5h.01M11 16.5h.01"/>',
  users: '<circle cx="9" cy="8" r="3.5"/><path d="M2.5 20c.6-3.4 3.2-5.5 6.5-5.5s5.9 2.1 6.5 5.5"/><path d="M16 4.6a3.5 3.5 0 0 1 0 6.8M18 14.7c2 .7 3.2 2.6 3.5 5.3"/>',
  clock: '<circle cx="12" cy="12" r="9"/><path d="M12 7v5l3 2"/>',
  logout: '<path d="M15 4h3a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-3"/><path d="M10 16l-4-4 4-4M6 12h10"/>',
  refresh: '<path d="M20 11a8 8 0 0 0-14.8-3.5M4 13a8 8 0 0 0 14.8 3.5"/><path d="M4 4v4h4M20 20v-4h-4"/>',
  save: '<path d="M5 3h11l4 4v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V5a2 2 0 0 1 1-2z"/><path d="M8 3v5h7V3M8 21v-7h8v7"/>',
  shield: '<path d="M12 3 4.5 6v6c0 4.5 3.2 7.8 7.5 9 4.3-1.2 7.5-4.5 7.5-9V6z"/><path d="m9 12 2 2 4-4"/>',
  puzzle: '<path d="M9 4.5a2 2 0 1 1 4 0V6h4v4h-1.5a2 2 0 1 0 0 4H17v4h-4v-1.5a2 2 0 1 0-4 0V18H5v-4h1.5a2 2 0 1 0 0-4H5V6h4z"/>',
  image: '<rect x="3" y="4" width="18" height="16" rx="2"/><circle cx="9" cy="9.5" r="1.8"/><path d="m21 16-5-5-9 9"/>',
  database: '<ellipse cx="12" cy="5.5" rx="7.5" ry="2.5"/><path d="M4.5 5.5v13c0 1.4 3.4 2.5 7.5 2.5s7.5-1.1 7.5-2.5v-13M4.5 12c0 1.4 3.4 2.5 7.5 2.5s7.5-1.1 7.5-2.5"/>',
  java: '<path d="M5 9h11v5a5 5 0 0 1-5 5h-1a5 5 0 0 1-5-5z"/><path d="M16 10h1.5a2.5 2.5 0 0 1 0 5H16M8 3v3M11 3v3"/>',
  info: '<circle cx="12" cy="12" r="9"/><path d="M12 11v5M12 8h.01"/>',
  alert: '<path d="M12 3.5 2.5 20h19z"/><path d="M12 10v4M12 17h.01"/>',
  loader: '<path d="M12 3a9 9 0 1 0 9 9"/>',
  bookmark: '<path d="M6 3h12v18l-6-4-6 4z"/>',
  key: '<circle cx="8" cy="15" r="4"/><path d="m11 12 9-9M17 6l3 3M14.5 8.5l2 2"/>',
  broom: '<path d="M14 4 20 10M17 7l-6.5 6.5"/><path d="M10.5 13.5 4 20h6l3.5-3.5"/><path d="m8 15 3 3"/>',
  ban: '<circle cx="12" cy="12" r="9"/><path d="m5.6 5.6 12.8 12.8"/>',
  undo: '<path d="M9 14 4 9l5-5"/><path d="M4 9h10.5a5.5 5.5 0 0 1 0 11H11"/>',
  wand: '<path d="m4 20 11-11M14 4v2M19 9h2M18 4l-1.5 1.5M18 14l-1.5-1.5M10 4l1.5 1.5"/>',
  modrinth: '<circle cx="12" cy="12" r="9"/><path d="M12 3a9 9 0 0 1 7.7 13.6M8 9.5l4-2 4 2v5l-4 2-4-2z"/>',
};
const I = (n, c = '') => `<svg class="ico ${c}" viewBox="0 0 24 24" aria-hidden="true">${P[n] || ''}</svg>`;

// ------------------------------------------------------------------ helpers
const $ = (s, r = document) => r.querySelector(s);
const $$ = (s, r = document) => [...r.querySelectorAll(s)];
const esc = (s) => String(s ?? '').replace(/[&<>"']/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[c]));
const fmtSize = (n) => { n = Number(n) || 0; const u = ['B', 'KB', 'MB', 'GB', 'TB']; let i = 0; while (n >= 1024 && i < u.length - 1) { n /= 1024; i++; } return `${n.toFixed(i ? 1 : 0)} ${u[i]}`; };
const fmtTime = (ts) => { if (!ts) return '-'; const d = new Date(ts * 1000); const p = (x) => String(x).padStart(2, '0'); return `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())} ${p(d.getHours())}:${p(d.getMinutes())}`; };
const fmtDur = (s) => { if (s == null) return '-'; const h = Math.floor(s / 3600), m = Math.floor((s % 3600) / 60); return h ? `${h}h ${m}m` : `${m}m ${s % 60}s`; };
const fmtNum = (n) => n >= 1e6 ? (n / 1e6).toFixed(1) + 'M' : n >= 1e3 ? (n / 1e3).toFixed(1) + 'K' : String(n);
const debounce = (fn, ms = 300) => { let t; return (...a) => { clearTimeout(t); t = setTimeout(() => fn(...a), ms); }; };
const enc = encodeURIComponent;
const STATUS_TEXT = { stopped: '已停止', starting: '启动中', running: '运行中', stopping: '停止中', crashed: '已崩溃' };
const CORE_NAMES = { vanilla: 'Vanilla', paper: 'Paper', purpur: 'Purpur', fabric: 'Fabric', quilt: 'Quilt', forge: 'Forge', neoforge: 'NeoForge', custom: '自定义' };
const KIND_NAMES = { mod: '模组', plugin: '插件', resourcepack: '资源包', datapack: '数据包', modpack: '整合包', server: '服务端核心', installer: '安装器', other: '其他' };
const KIND_ICON = { mod: 'puzzle', plugin: 'puzzle', resourcepack: 'image', datapack: 'database', modpack: 'layers', server: 'server', installer: 'java', other: 'file' };

class ApiError extends Error {}
async function api(method, url, body, opts = {}) {
  const init = { method, headers: {}, credentials: 'same-origin' };
  if (body instanceof FormData) init.body = body;
  else if (typeof body === 'string' && opts.text) { init.body = body; init.headers['Content-Type'] = 'text/plain; charset=utf-8'; }
  else if (body !== undefined) { init.body = JSON.stringify(body); init.headers['Content-Type'] = 'application/json'; }
  const r = await fetch(url, init);
  let data = null;
  try { data = await r.json(); } catch { /* empty */ }
  if (r.status === 401 && !url.startsWith('/api/auth/')) { boot(); throw new ApiError('未登录'); }
  if (!r.ok) throw new ApiError((data && data.error) || `请求失败 (${r.status})`);
  return data;
}
const GET = (u) => api('GET', u);
const POST = (u, b) => api('POST', u, b ?? {});

/* upload with progress (XHR gives upload progress events) */
function upload(url, files, onProgress) {
  return new Promise((resolve, reject) => {
    const fd = new FormData();
    for (const f of files) fd.append('file', f, f.relPath || f.name);
    const x = new XMLHttpRequest();
    x.open('POST', url);
    x.upload.onprogress = (e) => e.lengthComputable && onProgress && onProgress(e.loaded / e.total);
    x.onload = () => { let d = {}; try { d = JSON.parse(x.responseText); } catch { } x.status < 300 ? resolve(d) : reject(new ApiError(d.error || `上传失败 (${x.status})`)); };
    x.onerror = () => reject(new ApiError('网络错误'));
    x.send(fd);
  });
}

function toast(msg, type = 'ok', ms = 3200) {
  const el = document.createElement('div');
  el.className = `toast ${type}`;
  el.innerHTML = `${I(type === 'err' ? 'xCircle' : type === 'info' ? 'info' : 'checkCircle')}<div>${esc(msg)}</div>`;
  $('#toasts').append(el);
  setTimeout(() => { el.classList.add('out'); setTimeout(() => el.remove(), 260); }, ms);
}
const fail = (e) => toast(e.message || String(e), 'err', 5000);

function modal({ title, body = '', foot = '', size = '', onMount, onClose }) {
  const back = document.createElement('div');
  back.className = 'modal-back';
  back.innerHTML = `<div class="modal ${size}" role="dialog"><div class="modal-head"><h2 class="ellip">${title}</h2><button class="btn icon sm ghost" data-close>${I('x')}</button></div><div class="modal-body">${body}</div>${foot ? `<div class="modal-foot">${foot}</div>` : ''}</div>`;
  const m = { el: back.firstElementChild, closed: false };
  m.close = () => {
    if (m.closed) return; m.closed = true;
    document.removeEventListener('keydown', onKey);
    back.classList.add('out'); setTimeout(() => back.remove(), 190);
    onClose && onClose();
  };
  const onKey = (e) => { if (e.key === 'Escape') m.close(); };
  document.addEventListener('keydown', onKey);
  back.addEventListener('mousedown', (e) => { if (e.target === back) m.close(); });
  back.addEventListener('click', (e) => { if (e.target.closest('[data-close]')) m.close(); });
  $('#modal-root').append(back);
  onMount && onMount(m.el, m);
  return m;
}

function confirmDlg(text, { danger = false, ok = '确定', title = '确认' } = {}) {
  return new Promise((res) => {
    let v = false;
    const m = modal({
      title: esc(title), body: `<div>${text}</div>`,
      foot: `<button class="btn" data-close>取消</button><button class="btn ${danger ? 'danger' : 'primary'}" data-ok>${esc(ok)}</button>`,
      onClose: () => res(v),
      onMount: (el) => { const b = $('[data-ok]', el); b.focus(); b.onclick = () => { v = true; m.close(); }; },
    });
  });
}

function promptDlg(title, label, value = '', { placeholder = '' } = {}) {
  return new Promise((res) => {
    let v = null;
    const m = modal({
      title: esc(title),
      body: `<label class="field"><span>${esc(label)}</span><input class="input" value="${esc(value)}" placeholder="${esc(placeholder)}"></label>`,
      foot: `<button class="btn" data-close>取消</button><button class="btn primary" data-ok>确定</button>`,
      onClose: () => res(v),
      onMount: (el) => {
        const inp = $('input', el);
        setTimeout(() => { inp.focus(); const dot = value.lastIndexOf('.'); inp.setSelectionRange(0, dot > 0 ? dot : value.length); }, 30);
        const go = () => { v = inp.value.trim(); m.close(); };
        $('[data-ok]', el).onclick = go;
        inp.onkeydown = (e) => { if (e.key === 'Enter') go(); };
      },
    });
  });
}

function pickFiles({ multiple = true, accept = '', dir = false } = {}) {
  return new Promise((res) => {
    const i = document.createElement('input');
    i.type = 'file'; i.multiple = multiple; if (accept) i.accept = accept;
    if (dir) i.webkitdirectory = true;
    i.onchange = () => res([...i.files].map((f) => { if (f.webkitRelativePath) f.relPath = f.webkitRelativePath; return f; }));
    i.click();
  });
}

// ------------------------------------------------------------------ global state
const S = { instances: [], tasks: new Map(), taskWaiters: new Map(), sse: null, settings: {}, page: null };

function onTask(t) {
  const prev = S.tasks.get(t.id);
  S.tasks.set(t.id, t);
  if (t.state !== 'running' && (!prev || prev.state === 'running')) {
    const w = S.taskWaiters.get(t.id);
    if (w) { S.taskWaiters.delete(t.id); w(t); }
    else if (prev) toast(`${t.title}：${t.message}`, t.state === 'done' ? 'ok' : 'err', t.state === 'done' ? 3200 : 6000);
  }
  updateTaskBadge();
  S.page && S.page.onTask && S.page.onTask(t);
}
/** Wait for a task; toasts the result. */
function waitTask(id, { silent = false } = {}) {
  return new Promise((res) => {
    const t = S.tasks.get(id);
    const done = (t) => {
      if (!silent) toast(`${t.title}：${t.message}`, t.state === 'done' ? 'ok' : 'err', t.state === 'done' ? 3500 : 7000);
      res(t);
    };
    if (t && t.state !== 'running') return done(t);
    S.taskWaiters.set(id, done);
  });
}
function updateTaskBadge() {
  const n = [...S.tasks.values()].filter((t) => t.state === 'running').length;
  const b = $('#nav-task-count');
  if (b) { b.textContent = n; b.style.display = n ? '' : 'none'; }
}

function connectEvents() {
  if (S.sse) S.sse.close();
  const es = new EventSource('/api/events');
  S.sse = es;
  es.addEventListener('task', (e) => onTask(JSON.parse(e.data)));
  es.addEventListener('instances', (e) => {
    S.instances = JSON.parse(e.data).sort((a, b) => a.created - b.created);
    S.page && S.page.onInstances && S.page.onInstances();
  });
  es.onerror = () => { /* EventSource auto-reconnects */ };
}

const instById = (id) => S.instances.find((i) => i.id === id);
const statusBadge = (st) => `<span class="badge ${st === 'running' ? 'ok' : st === 'crashed' ? 'err' : (st === 'starting' || st === 'stopping') ? 'warn' : ''}"><i class="dot ${st}"></i>${STATUS_TEXT[st] || st}</span>`;
const coreLabel = (c) => `${CORE_NAMES[c.kind] || c.kind || '-'} ${c.mc_version || ''}`;

// ------------------------------------------------------------------ auth / boot
async function boot() {
  if (S.sse) { S.sse.close(); S.sse = null; }
  let st;
  try { st = await api('GET', '/api/auth/state'); } catch (e) { $('#app').innerHTML = `<div class="auth"><div class="card">${esc(e.message)}</div></div>`; return; }
  if (!st.logged_in) return renderAuth(st.setup_needed);
  renderShell();
  connectEvents();
  try {
    S.instances = await GET('/api/instances');
    (await GET('/api/tasks')).forEach((t) => S.tasks.set(t.id, t));
    S.settings = await GET('/api/settings');
  } catch (e) { fail(e); }
  updateTaskBadge();
  route();
}

function renderAuth(setup) {
  $('#app').innerHTML = `<div class="auth"><form class="card" autocomplete="on">
    <img class="logo-img lg" src="/icon.svg" alt="">
    <h1>${setup ? '初始化 RimePanel' : '登录 RimePanel'}</h1>
    <p class="muted" style="margin:0 0 20px">${setup ? '设置管理员密码，之后用它登录面板。' : '输入管理员密码。'}</p>
    <label class="field"><span>密码</span><input class="input" type="password" name="pw" autocomplete="${setup ? 'new-password' : 'current-password'}" required minlength="${setup ? 6 : 1}"></label>
    ${setup ? `<label class="field" style="margin-top:12px"><span>确认密码</span><input class="input" type="password" name="pw2" autocomplete="new-password" required></label>` : ''}
    <button class="btn primary" style="width:100%;justify-content:center;margin-top:20px;height:38px">${setup ? '完成设置' : '登录'}</button>
  </form></div>`;
  const f = $('.auth form');
  setTimeout(() => f.pw.focus(), 50);
  f.onsubmit = async (e) => {
    e.preventDefault();
    if (setup && f.pw.value !== f.pw2.value) return toast('两次输入的密码不一致', 'err');
    try { await POST(setup ? '/api/auth/setup' : '/api/auth/login', { password: f.pw.value }); boot(); }
    catch (err) { fail(err); f.pw.select(); }
  };
}

// ------------------------------------------------------------------ shell / router
const NAV = [
  ['', 'home', '概览'],
  ['instances', 'server', '实例'],
  ['assemble', 'layers', '拼装服务器'],
  ['library', 'box', '资源库'],
  ['worlds', 'globe', '世界'],
  ['tasks', 'activity', '任务'],
  ['settings', 'sliders', '设置'],
];
const BRAND = { name: 'RimePanel', author: 'AliyahZombie' };
const brandFoot = () => `<footer class="app-foot"><img src="/icon.svg" alt="" width="14" height="14">${BRAND.name}${S.version ? ' v' + esc(S.version) : ''}<span>·</span>${BRAND.author}</footer>`;
/* Watch detection by real viewport geometry, never by user agent:
 * any small, near-square screen (Apple Watch 162x197 .. 205x251, Wear OS round
 * 192..240 squares, Galaxy Watch 360x360 class) gets the compact watch UI. */
const isWatch = () => {
  const w = window.innerWidth, h = window.innerHeight;
  const short = Math.min(w, h), long = Math.max(w, h);
  if (w <= 240) return true;                       // anything this narrow is unusable as a desktop/phone layout
  return short <= 400 && long <= 480 && long / short <= 1.35;
};
const isRound = () => { const w = window.innerWidth, h = window.innerHeight; return Math.max(w, h) / Math.min(w, h) <= 1.06; };
function renderShell() {
  S.watchMode = isWatch();
  document.documentElement.classList.toggle('is-watch', S.watchMode);
  document.documentElement.classList.toggle('is-round', S.watchMode && isRound());
  if (S.watchMode) { $('#app').innerHTML = '<main class="main" id="main"></main>'; return; }
  $('#app').innerHTML = `<div class="shell">
    <aside class="side">
      <div class="brand"><img class="logo-img" src="/icon.svg" alt="">${BRAND.name}</div>
      <nav class="nav">${NAV.map(([h, ic, t]) => `<a href="#/${h}" data-nav="${h}">${I(ic)}<span class="lbl">${t}</span>${h === 'tasks' ? '<span class="count" id="nav-task-count" style="display:none"></span>' : ''}</a>`).join('')}</nav>
      <div class="grow"></div>
      <div class="foot"><span id="ver">${BRAND.name}<br><small>${BRAND.author}</small></span><button class="btn icon sm ghost" id="logout" title="退出登录">${I('logout', 'sm')}</button></div>
    </aside>
    <main class="main" id="main"></main>
  </div>`;
  $('#logout').onclick = async () => { await POST('/api/auth/logout'); boot(); };
}

window.addEventListener('hashchange', route);
window.addEventListener('resize', debounce(() => { if ($('#main') && S.watchMode !== isWatch()) { renderShell(); route(); } }, 200));
function route() {
  if (!$('#main')) return;
  const parts = location.hash.replace(/^#\/?/, '').split('/').map(decodeURIComponent);
  const [top] = parts;
  $$('[data-nav]').forEach((a) => a.classList.toggle('active', a.dataset.nav === (top || '') || (top === 'instance' && a.dataset.nav === 'instances')));
  if (S.page && S.page.destroy) S.page.destroy();
  S.page = null;
  const main = $('#main');
  main.scrollTop = 0;
  if (S.watchMode) {
    S.page = (top === 'instance' && parts[1] ? WatchInstance(main, parts[1]) : WatchHome(main)) || {};
    return;
  }
  const views = { '': Dashboard, instances: Instances, instance: InstanceView, assemble: Assemble, library: Library, worlds: Worlds, tasks: Tasks, settings: Settings };
  const V = views[top] || Dashboard;
  S.page = V(main, parts.slice(1)) || {};
  const pg = $('.page', main); if (pg && !$('.app-foot', pg)) pg.insertAdjacentHTML('beforeend', brandFoot());
}

function pageShell(main, { title, sub = '', actions = '' }) {
  main.innerHTML = `<div class="page"><div class="page-head"><div><h1>${title}</h1>${sub ? `<div class="sub">${sub}</div>` : ''}</div><div class="actions">${actions}</div></div><div class="page-body"></div></div>`;
  return $('.page-body', main);
}

// ------------------------------------------------------------------ dashboard
function Dashboard(main) {
  const body = pageShell(main, { title: '概览', sub: '服务器与主机状态', actions: `<a class="btn primary" href="#/assemble">${I('plus')}新建实例</a>` });
  body.innerHTML = `<div class="grid g4 stats-grid" id="sys"></div><div class="section-title">实例</div><div id="dash-inst"></div>`;
  let timer;
  const loadSys = async () => {
    try {
      const s = await GET('/api/system');
      const memUsed = s.mem_total_kb - s.mem_avail_kb;
      const memP = s.mem_total_kb ? memUsed / s.mem_total_kb : 0;
      const diskP = s.disk_total ? 1 - s.disk_free / s.disk_total : 0;
      if (S.version !== s.version) { S.version = s.version; $('#ver') && ($('#ver').innerHTML = `${BRAND.name} v${esc(s.version)}<br><small>${BRAND.author}</small>`); }
      const el = $('#sys'); if (!el) return;
      el.innerHTML = `
        <div class="card stat"><div class="k">${I('cpu', 'sm')}系统负载</div><div class="v">${esc(s.load[0] || '-')}<small>/ ${s.cpus} 核</small></div><div class="meter"><i style="width:${Math.min(100, (parseFloat(s.load[0]) || 0) / s.cpus * 100)}%"></i></div></div>
        <div class="card stat"><div class="k">${I('memory', 'sm')}内存</div><div class="v">${fmtSize(memUsed * 1024)}<small>/ ${fmtSize(s.mem_total_kb * 1024)}</small></div><div class="meter"><i style="width:${memP * 100}%"></i></div></div>
        <div class="card stat"><div class="k">${I('disk', 'sm')}磁盘</div><div class="v">${fmtSize(s.disk_free)}<small>可用</small></div><div class="meter"><i style="width:${diskP * 100}%"></i></div></div>
        <div class="card stat"><div class="k">${I('cube', 'sm')}面板占用</div><div class="v">${fmtSize(s.panel_rss_kb * 1024)}<small>RSS</small></div><div class="meter"><i style="width:${Math.min(100, s.panel_rss_kb / 1024)}%"></i></div></div>`;
    } catch (e) { /* ignore */ }
  };
  const renderInst = () => renderInstanceGrid($('#dash-inst'));
  loadSys(); renderInst();
  timer = setInterval(loadSys, 5000);
  return { onInstances: renderInst, destroy: () => clearInterval(timer) };
}

function cardStats(i) {
  return `<span>${I('users', 'sm')}${i.players ? `${i.players[0]} / ${i.players[1]}` : '-'}</span>
        <span>${I('memory', 'sm')}${i.mem_kb ? fmtSize(i.mem_kb * 1024) : '-'}</span>
        <span>${I('cpu', 'sm')}${i.status === 'running' ? i.cpu.toFixed(0) + '%' : '-'}</span>
        <span>${I('link', 'sm')}:${i.port}</span>`;
}
const cardBadge = (i) => i.installing ? '<span class="badge warn"><i class="dot starting"></i>安装中</span>'
  : i.incomplete && i.status !== 'running' ? `<span class="badge err">${I('alert', 'sm')}安装未完成</span>` : statusBadge(i.status);
function instanceGrid() {
  if (!S.instances.length) return `<div class="card empty">${I('server')}<div>还没有实例</div><div style="margin-top:14px"><a class="btn primary" href="#/assemble">${I('plus')}新建实例</a></div></div>`;
  return `<div class="inst-grid">${S.instances.map((i) => `
    <a class="card inst-card" data-id="${esc(i.id)}" href="#/instance/${enc(i.id)}/console">
      <div class="top"><div class="thumb">${I('server')}</div><div style="min-width:0;flex:1"><div class="name ellip">${esc(i.name)}</div><div class="muted core" style="font-size:12px">${esc(coreLabel(i.core))}</div></div><span class="st">${cardBadge(i)}</span></div>
      <div class="stats">${cardStats(i)}</div>
    </a>`).join('')}</div>`;
}
/** Render once, then patch in place so hover states and animations stay smooth. */
function renderInstanceGrid(el) {
  if (!el) return;
  const ids = S.instances.map((i) => i.id).join(',');
  if (el._ids !== ids) { el._ids = ids; el.innerHTML = instanceGrid(); return; }
  for (const i of S.instances) {
    const c = el.querySelector(`[data-id="${CSS.escape(i.id)}"]`);
    if (!c) continue;
    const b = cardBadge(i), st = cardStats(i);
    if (c._b !== b) { $('.st', c).innerHTML = b; c._b = b; }
    if (c._s !== st) { $('.stats', c).innerHTML = st; c._s = st; }
    const n = $('.name', c); if (n.textContent !== i.name) n.textContent = i.name;
  }
}

function Instances(main) {
  const body = pageShell(main, { title: '实例', sub: `${S.instances.length} 个服务器`, actions: `<a class="btn primary" href="#/assemble">${I('plus')}新建实例</a>` });
  const render = () => renderInstanceGrid(body);
  render();
  return { onInstances: render };
}

// ------------------------------------------------------------------ instance view
const INST_TABS = [
  ['console', 'terminal', '控制台'],
  ['files', 'folder', '文件'],
  ['content', 'puzzle', '模组 / 插件'],
  ['backups', 'archive', '备份'],
  ['world', 'globe', '世界与资源包'],
  ['settings', 'sliders', '设置'],
];

function InstanceView(main, [id, tab = 'console', ...rest]) {
  const inst = instById(id);
  if (!inst) { main.innerHTML = `<div class="page"><div class="card empty">${I('alert')}<div>实例不存在</div></div></div>`; return {}; }
  main.innerHTML = `<div class="page">
    <div class="page-head">
      <a class="btn icon ghost" href="#/instances">${I('chevL')}</a>
      <div style="min-width:0"><h1 class="ellip" id="iv-name">${esc(inst.name)}</h1><div class="sub" id="iv-sub">${esc(coreLabel(inst.core))}${inst.core.loader_version ? ' · ' + esc(inst.core.loader_version) : ''}</div></div>
      <span id="iv-status">${cardBadge(inst)}</span>
      <div class="actions" id="iv-ctrl"></div>
    </div>
    <div class="tabs" id="iv-tabs">${INST_TABS.map(([t, ic, l]) => `<a href="#/instance/${enc(id)}/${t}" data-tab="${t}" class="${t === tab ? 'active' : ''}">${I(ic, 'sm')}${l}</a>`).join('')}<i class="ink"></i></div>
    <div id="iv-body"></div>
  </div>`;
  requestAnimationFrame(() => moveInk($('#iv-tabs')));
  const ctrl = $('#iv-ctrl');
  const renderCtrl = () => {
    const i = instById(id);
    if (!i) { if (!ctrl._gone) { ctrl._gone = true; toast('实例已移除', 'info'); location.hash = '#/instances'; } return; }
    const st = i.status;
    const running = st === 'running' || st === 'starting';
    const html = running
      ? `<button class="btn" data-act="restart">${I('restart')}重启</button><button class="btn" data-act="stop">${I('stop')}停止</button><button class="btn icon danger" data-act="kill" title="强制结束">${I('zap')}</button>`
      : st === 'stopping'
        ? `<button class="btn" disabled>${I('loader', 'st-running')}停止中</button><button class="btn icon danger" data-act="kill" title="强制结束">${I('zap')}</button>`
        : i.installing
          ? `<button class="btn danger" data-act="cancel-install">${I('x')}取消安装</button>`
          : i.incomplete
            ? `<a class="btn" href="#/instance/${enc(id)}/settings">${I('sliders')}修复或删除</a>`
            : `<button class="btn primary" data-act="start">${I('play')}启动</button>`;
    if (ctrl._h !== html) { ctrl.innerHTML = html; ctrl._h = html; }
    const sb = cardBadge(i); const se = $('#iv-status'); if (se._h !== sb) { se.innerHTML = sb; se._h = sb; }
  };
  ctrl.onclick = async (e) => {
    const b = e.target.closest('[data-act]'); if (!b) return;
    const act = b.dataset.act;
    if (act === 'kill' && !(await confirmDlg('强制结束会直接杀掉进程，未保存的数据可能丢失。', { danger: true, ok: '强制结束' }))) return;
    if (act === 'cancel-install') {
      if (!(await confirmDlg('取消安装？新建中的实例会被一并移除；更换核心则保留原实例。', { danger: true, ok: '取消安装' }))) return;
      try { const r = await POST(`/api/instances/${enc(id)}/cancel-install`); toast(r.removed ? '已取消并移除实例' : '已取消安装'); } catch (err) { fail(err); }
      return;
    }
    try {
      const r = await POST(`/api/instances/${enc(id)}/${act}`);
      if (r && r.java_installing) { toast(`缺少 Java ${r.java_installing}，正在自动安装，完成后启动`, 'info', 5000); waitTask(r.task); }
    } catch (err) {
      if (err.message === 'EULA_REQUIRED') {
        if (await confirmDlg('启动前需要同意 <a style="color:var(--accent)" target="_blank" rel="noopener" href="https://aka.ms/MinecraftEULA">Minecraft 最终用户许可协议 (EULA)</a>。', { ok: '同意并启动', title: 'EULA' })) {
          try { await POST(`/api/instances/${enc(id)}/eula`); const r = await POST(`/api/instances/${enc(id)}/start`); if (r && r.java_installing) { toast(`缺少 Java ${r.java_installing}，正在自动安装，完成后启动`, 'info', 5000); waitTask(r.task); } } catch (e2) { fail(e2); }
        }
      } else fail(err);
    }
  };
  renderCtrl();
  const tabs = { console: ConsoleTab, files: FilesTab, content: ContentTab, backups: BackupsTab, world: WorldTab, settings: SettingsTab };
  const sub = (tabs[tab] || ConsoleTab)($('#iv-body'), id, rest) || {};
  return {
    onInstances: () => {
      renderCtrl();
      const i = instById(id); if (!i) return;
      const n = $('#iv-name'); if (n && n.textContent !== i.name) n.textContent = i.name;
      sub.onInstances && sub.onInstances(i);
    },
    onTask: (t) => sub.onTask && sub.onTask(t),
    destroy: () => sub.destroy && sub.destroy(),
  };
}

function moveInk(tabs) {
  if (!tabs) return;
  const a = $('a.active', tabs), ink = $('.ink', tabs);
  if (!a || !ink) return;
  ink.style.left = a.offsetLeft + 'px'; ink.style.width = a.offsetWidth + 'px';
}

// ---- console
function classifyLine(l) {
  if (l.startsWith('[RimePanel]')) return 'panel';
  if (l.startsWith('> ')) return 'cmd';
  if (l.startsWith('[安装器]')) return 'dim';
  if (/\b(ERROR|FATAL|SEVERE)\b|Exception|^\s+at /.test(l)) return 'error';
  if (/\bWARN(ING)?\b/.test(l)) return 'warn';
  return '';
}
const stripAnsi = (s) => s.replace(/\x1b\[[0-9;]*[A-Za-z]/g, '');

function ConsoleTab(el, id) {
  const i = instById(id);
  el.innerHTML = `
    <div class="ctrl-bar"><div class="metrics" id="c-metrics"></div></div>
    <div class="console-wrap">
      <div class="console" id="con"></div>
      <div class="console-input">${I('prompt')}<input id="con-in" placeholder="输入命令，回车发送（上下键翻历史）" autocomplete="off" spellcheck="false"></div>
    </div>`;
  const con = $('#con'), input = $('#con-in');
  const MAX = 3000;
  let stick = true;
  con.addEventListener('scroll', () => { stick = con.scrollHeight - con.scrollTop - con.clientHeight < 40; });
  const append = (lines, animate) => {
    const frag = document.createDocumentFragment();
    for (let l of lines) {
      l = stripAnsi(l);
      const d = document.createElement('div');
      const c = classifyLine(l);
      d.className = (animate ? 'l ' : '') + c;
      d.textContent = l;
      frag.append(d);
    }
    con.append(frag);
    while (con.childElementCount > MAX) con.firstElementChild.remove();
    if (stick) con.scrollTop = con.scrollHeight;
  };
  const metrics = () => {
    const i = instById(id); if (!i) return;
    $('#c-metrics').innerHTML = `<span>${I('users', 'sm')}${i.players ? `${i.players[0]} / ${i.players[1]}` : '-'}</span><span>${I('memory', 'sm')}${i.mem_kb ? fmtSize(i.mem_kb * 1024) : '-'}</span><span>${I('cpu', 'sm')}${i.status === 'running' ? i.cpu.toFixed(1) + '%' : '-'}</span><span>${I('clock', 'sm')}${i.uptime != null ? fmtDur(i.uptime) : '-'}</span>`;
  };
  metrics();
  let ws, closed = false, retry;
  const connect = () => {
    const proto = location.protocol === 'https:' ? 'wss' : 'ws';
    ws = new WebSocket(`${proto}://${location.host}/api/instances/${enc(id)}/console`);
    ws.onmessage = (e) => {
      const m = JSON.parse(e.data);
      if (m.type === 'history') { con.innerHTML = ''; append(m.lines, false); if (!m.lines.length) con.innerHTML = `<div class="dim">${i && i.status === 'stopped' ? '服务器未运行。点击右上角「启动」。' : '暂无输出'}</div>`; }
      else if (m.type === 'lines') { const ph = con.querySelector('.dim:only-child'); if (ph && !ph.textContent.startsWith('[')) ph.remove(); append(m.lines, true); }
    };
    ws.onclose = () => { if (!closed) retry = setTimeout(connect, 1500); };
  };
  connect();
  const hist = JSON.parse(localStorage.getItem('mcp-hist-' + id) || '[]');
  let hi = hist.length;
  input.onkeydown = (e) => {
    if (e.key === 'Enter' && input.value.trim()) {
      const c = input.value.trim();
      if (ws && ws.readyState === 1) ws.send(c); else POST(`/api/instances/${enc(id)}/command`, { command: c }).catch(fail);
      if (hist[hist.length - 1] !== c) hist.push(c);
      while (hist.length > 100) hist.shift();
      localStorage.setItem('mcp-hist-' + id, JSON.stringify(hist));
      hi = hist.length; input.value = ''; stick = true;
    } else if (e.key === 'ArrowUp') { if (hi > 0) { hi--; input.value = hist[hi]; } e.preventDefault(); }
    else if (e.key === 'ArrowDown') { if (hi < hist.length) { hi++; input.value = hist[hi] || ''; } e.preventDefault(); }
  };
  input.focus();
  return { onInstances: metrics, destroy: () => { closed = true; clearTimeout(retry); ws && ws.close(); } };
}

// ---- monaco
let monacoP = null;
function loadMonaco() {
  if (monacoP) return monacoP;
  const base = (S.settings.monaco_cdn || 'https://cdn.jsdelivr.net/npm/monaco-editor@0.52.2/min').replace(/\/$/, '');
  monacoP = new Promise((res, rej) => {
    window.MonacoEnvironment = {
      getWorkerUrl: () => `data:text/javascript;charset=utf-8,${encodeURIComponent(`self.MonacoEnvironment={baseUrl:'${base}/'};importScripts('${base}/vs/base/worker/workerMain.js');`)}`,
    };
    const s = document.createElement('script');
    s.src = base + '/vs/loader.js';
    s.onload = () => {
      window.require.config({ paths: { vs: base + '/vs' } });
      window.require(['vs/editor/editor.main'], () => {
        const m = window.monaco;
        m.editor.defineTheme('mcp-dark', { base: 'vs-dark', inherit: true, rules: [], colors: { 'editor.background': '#0d0e10', 'editorGutter.background': '#0d0e10', 'editor.lineHighlightBackground': '#16181c' } });
        res(m);
      }, (e) => { monacoP = null; rej(e); });
    };
    const fallback = (msg) => { monacoP = null; rej(new Error(msg)); };
    s.onerror = () => fallback('Monaco 编辑器加载失败，已回退为纯文本编辑');
    // a CDN that hangs (common behind firewalls) must not block editing
    setTimeout(() => { if (!window.monaco) fallback('Monaco 加载超时，已回退为纯文本编辑（可在设置中更换 CDN）'); }, 12000);
    document.head.append(s);
  });
  return monacoP;
}
const LANG = { json: 'json', json5: 'json', mcmeta: 'json', yml: 'yaml', yaml: 'yaml', toml: 'ini', properties: 'ini', cfg: 'ini', conf: 'ini', ini: 'ini', md: 'markdown', js: 'javascript', ts: 'typescript', xml: 'xml', html: 'html', css: 'css', sh: 'shell', bat: 'bat', java: 'java', py: 'python', csv: 'plaintext' };
const TEXT_EXT = new Set(['txt', 'json', 'json5', 'mcmeta', 'yml', 'yaml', 'toml', 'properties', 'cfg', 'conf', 'ini', 'log', 'md', 'js', 'ts', 'xml', 'html', 'css', 'sh', 'bat', 'csv', 'snbt', 'mcfunction', 'lang', 'java', 'py', 'secret', 'list']);
const ARCHIVE_RE = /\.(zip|tar|tar\.gz|tgz|tar\.zst|mrpack)$/i;
const ext = (n) => (n.includes('.') ? n.split('.').pop().toLowerCase() : '');
const isText = (n) => TEXT_EXT.has(ext(n)) || !n.includes('.');

async function openEditor(id, path) {
  let data;
  try { data = await GET(`/api/instances/${enc(id)}/files/read?path=${enc(path)}`); } catch (e) { return fail(e); }
  let editor = null, dirty = false;
  const isDark = !matchMedia('(prefers-color-scheme: light)').matches;
  const m = modal({
    title: `${I('fileText', 'sm')} <span class="mono">${esc(path)}</span>`,
    size: 'full',
    body: `${data.linked ? `<div class="badge" style="margin-bottom:10px">${I('link', 'sm')}此文件硬链接到资源库，保存时会自动断开链接，资源库原件不受影响</div>` : ''}<div class="editor-host" id="ed" style="height:calc(100% - ${data.linked ? 34 : 0}px)"></div>`,
    foot: `<span class="hint" id="ed-st" style="margin-right:auto">Ctrl+S 保存</span><button class="btn" data-close>关闭</button><button class="btn primary" id="ed-save">${I('save')}保存</button>`,
    onMount: (el) => { $('.modal-body', el).style.cssText = 'padding:0 0 0 0;display:flex;flex-direction:column;overflow:hidden'; },
  });
  const host = $('#ed', m.el);
  const getVal = () => (editor ? editor.getValue() : $('textarea', host).value);
  const save = async () => {
    try {
      await api('PUT', `/api/instances/${enc(id)}/files/write?path=${enc(path)}`, getVal(), { text: true });
      dirty = false; $('#ed-st', m.el).textContent = '已保存 ' + new Date().toLocaleTimeString(); toast('已保存');
    } catch (e) { fail(e); }
  };
  $('#ed-save', m.el).onclick = save;
  m.el.addEventListener('keydown', (e) => { if ((e.ctrlKey || e.metaKey) && e.key === 's') { e.preventDefault(); save(); } });
  try {
    const monaco = await loadMonaco();
    if (m.closed) return;
    editor = monaco.editor.create(host, {
      value: data.content, language: LANG[ext(path)] || 'plaintext', theme: isDark ? 'mcp-dark' : 'vs',
      automaticLayout: true, minimap: { enabled: data.content.length < 200000 }, fontSize: 13, smoothScrolling: true,
      cursorSmoothCaretAnimation: 'on', scrollBeyondLastLine: false, renderWhitespace: 'selection', tabSize: 2,
    });
    editor.onDidChangeModelContent(() => { if (!dirty) { dirty = true; $('#ed-st', m.el).textContent = '未保存'; } });
    editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, save);
    editor.focus();
  } catch (e) {
    toast(e.message || 'Monaco 加载失败', 'info');
    host.innerHTML = '<textarea class="input" spellcheck="false" style="width:100%;height:100%;border:0;border-radius:0;resize:none"></textarea>';
    $('textarea', host).value = data.content;
  }
}

// ---- files
function FilesTab(el, id, rest) {
  let cwd = rest.filter(Boolean).join('/');
  let entries = [];
  const sel = new Set();
  el.innerHTML = `
    <div class="row" style="margin-bottom:12px;flex-wrap:wrap">
      <div class="crumbs" id="crumbs"></div><div class="spacer"></div>
      <div class="row" id="sel-acts" style="gap:6px;display:none"></div>
      <button class="btn sm" id="f-up">${I('upload', 'sm')}上传</button>
      <button class="btn sm icon" id="f-updir" title="上传文件夹">${I('folder', 'sm')}</button>
      <button class="btn sm icon" id="f-mkdir" title="新建文件夹">${I('folderPlus', 'sm')}</button>
      <button class="btn sm icon" id="f-new" title="新建文件">${I('filePlus', 'sm')}</button>
      <button class="btn sm icon" id="f-ref" title="刷新">${I('refresh', 'sm')}</button>
    </div>
    <div class="card dropzone" id="dz"><div class="file-scroll"><table class="tbl"><thead><tr><th style="width:34px"><input type="checkbox" class="chk" id="f-all"></th><th>名称</th><th style="width:110px" class="hide-m">大小</th><th style="width:150px" class="hide-m">修改时间</th><th style="width:200px"></th></tr></thead><tbody id="f-body"></tbody></table></div><div id="up-prog"></div></div>`;
  const join = (n) => (cwd ? cwd + '/' + n : n);
  const setHash = () => history.replaceState(null, '', `#/instance/${enc(id)}/files${cwd ? '/' + cwd.split('/').map(enc).join('/') : ''}`);
  const crumbs = () => {
    const parts = cwd ? cwd.split('/') : [];
    $('#crumbs').innerHTML = `<a href="javascript:void 0" data-p="">${I('home', 'sm')}</a>` + parts.map((p, i) => `${I('chevR', 'sm')}<a href="javascript:void 0" data-p="${esc(parts.slice(0, i + 1).join('/'))}">${esc(p)}</a>`).join('');
  };
  $('#crumbs').onclick = (e) => { const a = e.target.closest('[data-p]'); if (a) go(a.dataset.p); };
  const renderSel = () => {
    const box = $('#sel-acts');
    box.style.display = sel.size ? 'flex' : 'none';
    box.innerHTML = sel.size ? `<span class="muted" style="font-size:12px">已选 ${sel.size} 项</span><button class="btn sm" data-sa="zip">${I('archive', 'sm')}压缩</button><button class="btn sm danger" data-sa="del">${I('trash', 'sm')}删除</button>` : '';
    $$('#f-body tr').forEach((tr) => tr.classList.toggle('sel', sel.has(tr.dataset.n)));
    $('#f-all').checked = sel.size && sel.size === entries.length;
  };
  const row = (e) => {
    const icon = e.dir ? I('folder', 'dir') : ARCHIVE_RE.test(e.name) ? I('fileZip') : isText(e.name) ? I('fileText') : I('file');
    const acts = [
      `<button class="btn icon sm ghost" data-a="rename" title="重命名">${I('rename', 'sm')}</button>`,
      !e.dir && isText(e.name) ? `<button class="btn icon sm ghost" data-a="edit" title="编辑">${I('edit', 'sm')}</button>` : '',
      !e.dir ? `<button class="btn icon sm ghost" data-a="dl" title="下载">${I('download', 'sm')}</button>` : '',
      !e.dir && ARCHIVE_RE.test(e.name) ? `<button class="btn icon sm ghost" data-a="extract" title="解压到此处">${I('extract', 'sm')}</button>` : '',
      !e.dir && !e.linked && /\.(jar|zip|mrpack)$/i.test(e.name) ? `<button class="btn icon sm ghost" data-a="lib" title="存入资源库（替换为硬链接）">${I('box', 'sm')}</button>` : '',
      `<button class="btn icon sm ghost danger" data-a="del" title="删除">${I('trash', 'sm')}</button>`,
    ].join('');
    return `<tr data-n="${esc(e.name)}" data-dir="${e.dir ? 1 : 0}"><td><input type="checkbox" class="chk" data-ck></td>
      <td><div class="name">${icon}<span class="ellip">${esc(e.name)}</span>${e.linked ? `<span class="tag-link" title="硬链接自资源库，不额外占用空间">${I('link', 'sm')}资源库</span>` : ''}</div></td>
      <td class="muted hide-m">${e.dir ? '-' : fmtSize(e.size)}</td><td class="muted hide-m">${fmtTime(e.mtime)}</td><td><div class="acts">${acts}</div></td></tr>`;
  };
  const load = async () => {
    try {
      const d = await GET(`/api/instances/${enc(id)}/files?path=${enc(cwd)}`);
      entries = d.entries;
      sel.clear();
      const body = $('#f-body');
      body.innerHTML = (cwd ? `<tr data-up="1"><td></td><td><div class="name">${I('arrowUp')}<span class="muted">上级目录</span></div></td><td class="hide-m"></td><td class="hide-m"></td><td></td></tr>` : '') +
        (entries.length ? entries.map(row).join('') : `<tr><td colspan="5"><div class="empty">${I('folder')}<div>空目录，拖拽文件到这里上传</div></div></td></tr>`);
      renderSel();
    } catch (e) { fail(e); if (cwd) go(cwd.split('/').slice(0, -1).join('/')); }
  };
  const go = (p) => { cwd = p; setHash(); crumbs(); load(); };
  const doUpload = async (files) => {
    if (!files.length) return;
    const prog = $('#up-prog');
    prog.innerHTML = `<div style="padding:10px 14px;border-top:1px solid var(--border)"><div class="row" style="font-size:12px"><span>上传 ${files.length} 个文件</span><span class="spacer"></span><span id="up-p">0%</span></div><div class="meter"><i id="up-bar" style="width:0"></i></div></div>`;
    try {
      await upload(`/api/instances/${enc(id)}/files/upload?path=${enc(cwd)}`, files, (p) => { $('#up-bar').style.width = p * 100 + '%'; $('#up-p').textContent = Math.round(p * 100) + '%'; });
      toast(`已上传 ${files.length} 个文件`);
    } catch (e) { fail(e); }
    prog.innerHTML = ''; load();
  };
  const del = async (names) => {
    if (!(await confirmDlg(`删除 ${names.length > 1 ? names.length + ' 项' : '<b>' + esc(names[0]) + '</b>'}？此操作不可撤销。<div class="hint" style="margin-top:6px">硬链接文件只会移除实例中的链接，资源库原件保留。</div>`, { danger: true, ok: '删除' }))) return;
    try { await POST(`/api/instances/${enc(id)}/files/delete`, { paths: names.map(join) }); toast('已删除'); load(); } catch (e) { fail(e); }
  };
  const compress = async (names) => {
    const def = (names.length === 1 ? names[0] : (cwd.split('/').pop() || 'archive')) + '.zip';
    const n = await promptDlg('压缩', '压缩包名称', def); if (!n) return;
    try { const r = await POST(`/api/instances/${enc(id)}/files/compress`, { paths: names.map(join), dest: join(n) }); await waitTask(r.task); load(); } catch (e) { fail(e); }
  };
  $('#f-body').onclick = async (e) => {
    const tr = e.target.closest('tr'); if (!tr) return;
    if (tr.dataset.up) return go(cwd.split('/').slice(0, -1).join('/'));
    const n = tr.dataset.n; if (!n) return;
    const ent = entries.find((x) => x.name === n);
    if (e.target.matches('[data-ck]')) { e.target.checked ? sel.add(n) : sel.delete(n); return renderSel(); }
    const a = e.target.closest('[data-a]');
    if (!a) {
      if (e.target.closest('.name')) { if (ent.dir) go(join(n)); else if (isText(n)) openEditor(id, join(n)); }
      return;
    }
    const act = a.dataset.a;
    try {
      if (act === 'rename') { const to = await promptDlg('重命名', '新名称', n); if (to && to !== n) { await POST(`/api/instances/${enc(id)}/files/rename`, { from: join(n), to: join(to) }); load(); } }
      else if (act === 'edit') openEditor(id, join(n));
      else if (act === 'dl') location.href = `/api/instances/${enc(id)}/files/download?path=${enc(join(n))}`;
      else if (act === 'extract') { const d = await promptDlg('解压', '解压到目录（相对当前目录）', n.replace(ARCHIVE_RE, '')); if (d !== null) { const r = await POST(`/api/instances/${enc(id)}/files/extract`, { path: join(n), dest: join(d) }); await waitTask(r.task); load(); } }
      else if (act === 'lib') { await POST(`/api/instances/${enc(id)}/files/to-library`, { path: join(n) }); toast('已存入资源库，并替换为硬链接'); load(); }
      else if (act === 'del') del([n]);
    } catch (err) { fail(err); }
  };
  $('#f-all').onchange = (e) => { sel.clear(); if (e.target.checked) entries.forEach((x) => sel.add(x.name)); $$('#f-body [data-ck]').forEach((c) => (c.checked = e.target.checked)); renderSel(); };
  $('#sel-acts').onclick = (e) => { const b = e.target.closest('[data-sa]'); if (!b) return; const names = [...sel]; b.dataset.sa === 'del' ? del(names) : compress(names); };
  $('#f-up').onclick = async () => doUpload(await pickFiles());
  $('#f-updir').onclick = async () => doUpload(await pickFiles({ dir: true }));
  $('#f-ref').onclick = load;
  $('#f-mkdir').onclick = async () => { const n = await promptDlg('新建文件夹', '名称', ''); if (n) { try { await POST(`/api/instances/${enc(id)}/files/mkdir`, { path: join(n) }); load(); } catch (e) { fail(e); } } };
  $('#f-new').onclick = async () => { const n = await promptDlg('新建文件', '文件名', ''); if (n) { try { await api('PUT', `/api/instances/${enc(id)}/files/write?path=${enc(join(n))}`, '', { text: true }); load(); openEditor(id, join(n)); } catch (e) { fail(e); } } };
  const dz = $('#dz');
  dz.addEventListener('dragover', (e) => { e.preventDefault(); dz.classList.add('over'); });
  dz.addEventListener('dragleave', (e) => { if (!dz.contains(e.relatedTarget)) dz.classList.remove('over'); });
  dz.addEventListener('drop', (e) => { e.preventDefault(); dz.classList.remove('over'); doUpload([...e.dataTransfer.files]); });
  crumbs(); load();
  return {};
}

// ---- shared pickers
/** Modrinth browser. onPick(version, project) returns true to close. */
function modrinthBrowser({ type = 'mod', types = null, mc = '', loader = '', title = 'Modrinth', action = '选择', onPick }) {
  let q = '', offset = 0, curType = type, total = 0;
  const m = modal({
    title: `${I('modrinth', 'sm')} ${esc(title)}`, size: 'wide',
    body: `<div class="row" style="margin-bottom:12px;flex-wrap:wrap">
        <div style="position:relative;flex:1;min-width:200px"><input class="input" id="mr-q" placeholder="搜索 Modrinth" style="padding-left:34px"><span style="position:absolute;left:10px;top:8px;color:var(--faint)">${I('search', 'sm')}</span></div>
        ${types ? `<div class="seg" id="mr-t">${types.map((t) => `<button data-t="${t}" class="${t === curType ? 'on' : ''}">${KIND_NAMES[t] || t}</button>`).join('')}</div>` : ''}
      </div>
      <div class="row hint" style="margin-bottom:10px">${mc ? `游戏版本 ${esc(mc)}` : '全部版本'}${loader ? ` · ${esc(CORE_NAMES[loader] || loader)}` : ''}</div>
      <div class="card list" id="mr-res" style="min-height:200px"></div>
      <div class="row" style="justify-content:center;margin-top:12px" id="mr-pg"></div>`,
  });
  const res = $('#mr-res', m.el);
  const search = async () => {
    res.innerHTML = Array.from({ length: 5 }, () => `<div class="list-row"><div class="thumb"></div><div class="meta"><div class="skeleton" style="width:40%"></div><div class="skeleton" style="width:70%;margin-top:6px;height:10px"></div></div></div>`).join('');
    try {
      const d = await GET(`/api/modrinth/search?q=${enc(q)}&type=${curType}&mc=${enc(mc)}&loader=${enc(curType === 'mod' || curType === 'plugin' || curType === 'modpack' ? loader : '')}&offset=${offset}`);
      total = d.total_hits || 0;
      res.innerHTML = d.hits.length ? d.hits.map((h) => `<div data-pid="${esc(h.project_id)}">
          <div class="list-row" style="cursor:pointer"><div class="thumb">${h.icon_url ? `<img loading="lazy" src="${esc(h.icon_url)}" alt="">` : I(KIND_ICON[curType] || 'box')}</div>
          <div class="meta"><div class="t">${esc(h.title)} <span class="muted" style="font-weight:400;font-size:12px">by ${esc(h.author)}</span></div><div class="d ellip">${esc(h.description)}</div></div>
          <span class="muted" style="font-size:12px;display:flex;gap:4px;align-items:center">${I('download', 'sm')}${fmtNum(h.downloads)}</span>${I('chevD', 'sm')}</div>
          <div class="vers" style="display:none"></div></div>`).join('') : `<div class="empty">${I('search')}<div>没有结果</div></div>`;
      const pages = Math.ceil(total / 20), cur = offset / 20;
      $('#mr-pg', m.el).innerHTML = pages > 1 ? `<button class="btn sm" data-pg="-1" ${cur === 0 ? 'disabled' : ''}>${I('chevL', 'sm')}</button><span class="muted" style="font-size:12px">${cur + 1} / ${pages}</span><button class="btn sm" data-pg="1" ${cur + 1 >= pages ? 'disabled' : ''}>${I('chevR', 'sm')}</button>` : '';
    } catch (e) { res.innerHTML = `<div class="empty">${I('alert')}<div>${esc(e.message)}</div></div>`; }
  };
  $('#mr-q', m.el).oninput = debounce((e) => { q = e.target.value; offset = 0; search(); }, 350);
  $('#mr-pg', m.el).onclick = (e) => { const b = e.target.closest('[data-pg]'); if (b) { offset = Math.max(0, offset + Number(b.dataset.pg) * 20); search(); } };
  if (types) $('#mr-t', m.el).onclick = (e) => { const b = e.target.closest('[data-t]'); if (!b) return; curType = b.dataset.t; $$('#mr-t button', m.el).forEach((x) => x.classList.toggle('on', x === b)); offset = 0; search(); };
  res.onclick = async (e) => {
    const pick = e.target.closest('[data-vid]');
    const wrap = e.target.closest('[data-pid]'); if (!wrap) return;
    if (pick) {
      pick.disabled = true;
      const vs = wrap._vers || [];
      const v = vs.find((x) => x.id === pick.dataset.vid);
      const proj = { id: wrap.dataset.pid, title: $('.t', wrap).firstChild.textContent.trim(), icon_url: $('img', wrap)?.src || null };
      try { const close = await onPick(v, proj, curType); if (close !== false) m.close(); else pick.disabled = false; } catch (err) { fail(err); pick.disabled = false; }
      return;
    }
    if (!e.target.closest('.list-row')) return;
    const box = $('.vers', wrap);
    if (box.style.display !== 'none') { box.style.display = 'none'; return; }
    box.style.display = '';
    box.innerHTML = `<div style="padding:10px 14px 10px 62px" class="muted">加载版本...</div>`;
    try {
      const lp = curType === 'resourcepack' || curType === 'datapack' ? '' : loader;
      const vs = await GET(`/api/modrinth/project/${enc(wrap.dataset.pid)}/versions?mc=${enc(curType === 'datapack' || curType === 'resourcepack' ? mc : mc)}&loader=${enc(lp)}`);
      wrap._vers = vs;
      box.innerHTML = vs.length ? `<div style="padding:4px 14px 10px 62px;display:flex;flex-direction:column;gap:4px;max-height:240px;overflow:auto">${vs.slice(0, 30).map((v) => `
        <div class="row" style="gap:10px;font-size:13px"><span class="badge ${v.version_type === 'release' ? 'ok' : 'warn'}">${esc(v.version_type)}</span><span class="ellip" style="flex:1">${esc(v.name || v.version_number)}</span><span class="muted ellip" style="font-size:12px;max-width:220px">${esc((v.game_versions || []).slice(-3).join(', '))} · ${esc((v.loaders || []).join('/'))}</span><button class="btn sm primary" data-vid="${esc(v.id)}">${esc(action)}</button></div>`).join('')}</div>`
        : `<div style="padding:10px 14px 10px 62px" class="muted">没有与 ${esc(mc || '当前')} 兼容的版本</div>`;
    } catch (err) { box.innerHTML = `<div style="padding:10px 14px 10px 62px;color:var(--danger)">${esc(err.message)}</div>`; }
  };
  search();
  setTimeout(() => $('#mr-q', m.el).focus(), 50);
  return m;
}

/** Pick items from the resource library. */
async function libraryPicker({ kinds, multi = true, title = '从资源库选择', selected = [], onPick }) {
  let items = [];
  try { for (const k of kinds) items = items.concat(await GET(`/api/library?kind=${k}`)); } catch (e) { return fail(e); }
  const sel = new Set(selected);
  const m = modal({
    title: `${I('box', 'sm')} ${esc(title)}`, size: 'wide',
    body: items.length ? `<input class="input" id="lp-q" placeholder="筛选" style="margin-bottom:10px"><div class="pick-list" id="lp">${items.map((it) => `
      <div class="list-row ${sel.has(it.sha1) ? 'on' : ''}" data-sha="${it.sha1}"><div class="thumb">${it.meta && it.meta.icon_url ? `<img src="${esc(it.meta.icon_url)}" alt="">` : I(KIND_ICON[it.kind] || 'file')}</div>
      <div class="meta"><div class="t ellip">${esc(it.meta && it.meta.title ? it.meta.title : it.name)}</div><div class="d ellip">${esc(it.name)} · ${KIND_NAMES[it.kind]} · ${fmtSize(it.size)}</div></div>${I('check', 'ck')}</div>`).join('')}</div>`
      : `<div class="empty">${I('box')}<div>资源库中还没有${kinds.map((k) => KIND_NAMES[k]).join('/')}</div></div>`,
    foot: `<span class="hint" style="margin-right:auto" id="lp-n"></span><button class="btn" data-close>取消</button><button class="btn primary" id="lp-ok">确定</button>`,
  });
  const upd = () => { $('#lp-n', m.el).textContent = multi ? `已选 ${sel.size} 项` : ''; };
  upd();
  const lp = $('#lp', m.el);
  if (lp) lp.onclick = (e) => {
    const r = e.target.closest('[data-sha]'); if (!r) return;
    const s = r.dataset.sha;
    if (!multi) { sel.clear(); $$('.list-row', lp).forEach((x) => x.classList.remove('on')); }
    sel.has(s) ? sel.delete(s) : sel.add(s);
    r.classList.toggle('on', sel.has(s)); upd();
  };
  const q = $('#lp-q', m.el);
  if (q) q.oninput = () => { const v = q.value.toLowerCase(); $$('.list-row', lp).forEach((r) => (r.style.display = r.textContent.toLowerCase().includes(v) ? '' : 'none')); };
  $('#lp-ok', m.el).onclick = () => { onPick(items.filter((i) => sel.has(i.sha1))); m.close(); };
}

// ---- content
function ContentTab(el, id) {
  const inst = instById(id);
  const isPlugin = ['paper', 'purpur'].includes(inst.core.kind);
  const kind = isPlugin ? 'plugin' : 'mod';
  el.innerHTML = `
    <div class="row" style="margin-bottom:14px;flex-wrap:wrap">
      <button class="btn primary" id="c-mr">${I('modrinth')}从 Modrinth 安装</button>
      <button class="btn" id="c-lib">${I('box')}从资源库添加</button>
      <button class="btn" id="c-up">${I('upload')}上传${KIND_NAMES[kind]}</button>
      <span class="spacer"></span>
      ${isPlugin ? '' : `<button class="btn" id="c-scan">${I('shield')}识别客户端模组</button>`}
    </div>
    <div class="card" id="c-list"><div class="empty">${I('loader', 'st-running')}</div></div>`;
  const load = async () => {
    try {
      const rows = await GET(`/api/instances/${enc(id)}/content`);
      const groups = {};
      rows.forEach((r) => (groups[r.folder] = groups[r.folder] || []).push(r));
      const label = (f) => f === 'mods' ? '模组' : f === 'mods-disabled' ? '已禁用' : f === 'plugins' ? '插件' : '数据包';
      $('#c-list').innerHTML = rows.length ? Object.entries(groups).map(([f, rs]) => `
        <div class="section-title" style="padding:12px 14px 6px;margin:0">${label(f)} <span class="muted">(${rs.length})</span></div>
        <div class="list">${rs.map((r) => `<div class="list-row" data-f="${esc(r.folder)}" data-file="${esc(r.file)}">
          <div class="thumb">${r.icon_url ? `<img loading="lazy" src="${esc(r.icon_url)}" alt="">` : I(f.includes('datapacks') ? 'database' : 'puzzle')}</div>
          <div class="meta"><div class="t ellip" style="${f === 'mods-disabled' ? 'opacity:.55' : ''}">${esc(r.title || r.file)}</div><div class="d ellip">${esc(r.file)} · ${fmtSize(r.size)}${r.source ? ' · ' + esc(r.source) : ''}</div></div>
          ${r.linked ? `<span class="tag-link">${I('link', 'sm')}资源库</span>` : ''}
          ${f === 'mods' ? `<button class="btn icon sm ghost" data-a="off" title="禁用">${I('ban', 'sm')}</button>` : ''}
          ${f === 'mods-disabled' ? `<button class="btn icon sm ghost" data-a="on" title="启用">${I('undo', 'sm')}</button>` : ''}
          <button class="btn icon sm ghost danger" data-a="rm" title="移除">${I('trash', 'sm')}</button></div>`).join('')}</div>`).join('')
        : `<div class="empty">${I('puzzle')}<div>还没有安装任何${KIND_NAMES[kind]}</div></div>`;
    } catch (e) { fail(e); }
  };
  $('#c-list').onclick = async (e) => {
    const b = e.target.closest('[data-a]'); if (!b) return;
    const r = b.closest('[data-file]'); const f = r.dataset.f, file = r.dataset.file;
    try {
      if (b.dataset.a === 'rm') { if (!(await confirmDlg(`移除 <b>${esc(file)}</b>？`, { danger: true, ok: '移除' }))) return; await POST(`/api/instances/${enc(id)}/content/remove`, { folder: f, file }); }
      else if (b.dataset.a === 'off') await POST(`/api/instances/${enc(id)}/clientmods/disable`, { files: [file] });
      else if (b.dataset.a === 'on') await POST(`/api/instances/${enc(id)}/clientmods/enable`, { files: [file] });
      r.style.transition = 'opacity .2s'; r.style.opacity = '0'; setTimeout(load, 180);
    } catch (err) { fail(err); }
  };
  $('#c-mr').onclick = () => modrinthBrowser({
    type: kind, types: isPlugin ? ['plugin', 'datapack'] : ['mod', 'datapack', 'resourcepack'], mc: inst.core.mc_version, loader: inst.core.kind, title: '安装到实例', action: '安装',
    onPick: async (v) => { const r = await POST(`/api/instances/${enc(id)}/modrinth`, { version_id: v.id, deps: true }); toast('已开始安装，可在任务中查看进度', 'info'); waitTask(r.task).then(load); return false; },
  });
  $('#c-lib').onclick = () => libraryPicker({
    kinds: [kind, 'datapack'], onPick: async (rows) => {
      try { for (const r of rows) await POST(`/api/instances/${enc(id)}/content/link`, { sha1: r.sha1 }); if (rows.length) toast(`已链接 ${rows.length} 个文件（硬链接，不占额外空间）`); load(); } catch (e) { fail(e); }
    },
  });
  $('#c-up').onclick = async () => {
    const files = await pickFiles({ accept: '.jar,.zip' }); if (!files.length) return;
    try {
      const r = await upload(`/api/library/upload?kind=${kind}`, files);
      for (const sha of r.sha1) await POST(`/api/instances/${enc(id)}/content/link`, { sha1: sha });
      toast(`已上传到资源库并链接 ${r.sha1.length} 个文件`); load();
    } catch (e) { fail(e); }
  };
  const scan = $('#c-scan');
  if (scan) scan.onclick = async () => {
    scan.disabled = true;
    try {
      const r = await POST(`/api/instances/${enc(id)}/clientmods/scan`);
      const t = await waitTask(r.task, { silent: true });
      if (t.state !== 'done') return toast(t.message, 'err');
      const found = t.result || [];
      if (!found.length) return toast('没有发现客户端模组');
      const mm = modal({
        title: `${I('shield', 'sm')} 发现 ${found.length} 个客户端模组`, size: 'wide',
        body: `<p class="muted" style="margin-top:0">选中的文件会移动到 <span class="mono">mods-disabled/</span>，随时可以恢复。</p><div class="pick-list">${found.map((f) => `
          <label class="list-row" style="cursor:pointer"><input type="checkbox" class="chk" value="${esc(f.file)}" ${f.confidence === 'high' ? 'checked' : ''}><div class="meta"><div class="t ellip">${esc(f.file)}</div><div class="d">${esc(f.reason)}</div></div><span class="badge ${f.confidence === 'high' ? 'err' : 'warn'}">${f.confidence === 'high' ? '确定' : '疑似'}</span></label>`).join('')}</div>`,
        foot: `<button class="btn" data-close>取消</button><button class="btn primary" id="cm-ok">${I('ban')}禁用选中</button>`,
      });
      $('#cm-ok', mm.el).onclick = async () => {
        const files = $$('input:checked', mm.el).map((i) => i.value);
        try { const x = await POST(`/api/instances/${enc(id)}/clientmods/disable`, { files }); toast(`已禁用 ${x.moved} 个模组`); mm.close(); load(); } catch (e) { fail(e); }
      };
    } catch (e) { fail(e); } finally { scan.disabled = false; }
  };
  load();
  return {};
}

// ---- backups
function BackupsTab(el, id) {
  const inst = instById(id);
  const b = inst.backup;
  el.innerHTML = `
    <div class="grid" style="grid-template-columns:1fr 320px;align-items:start" id="bk-grid">
      <div>
        <div class="row" style="margin-bottom:12px"><button class="btn primary" id="bk-new">${I('archive')}立即备份</button><span class="spacer"></span><span class="hint">备份仅包含世界目录，使用 zstd 压缩</span></div>
        <div class="card" id="bk-list"></div>
      </div>
      <div class="card pad">
        <h3>${I('clock', 'sm')}自动备份</h3>
        <div class="toggle-row"><div class="t"><div>无人时备份</div><div>有玩家进出后，服务器空闲达到设定时长自动备份</div></div><label class="switch"><input type="checkbox" id="bp-idle" ${b.idle_enabled ? 'checked' : ''}><i></i></label></div>
        <label class="field"><span>空闲时长（分钟）</span><input class="input" type="number" min="1" id="bp-idle-m" value="${b.idle_minutes}"></label>
        <div class="toggle-row" style="margin-top:8px"><div class="t"><div>定时备份</div><div>服务器运行期间按间隔备份（有过玩家活动时）</div></div><label class="switch"><input type="checkbox" id="bp-int" ${b.interval_enabled ? 'checked' : ''}><i></i></label></div>
        <label class="field"><span>间隔（分钟）</span><input class="input" type="number" min="5" id="bp-int-m" value="${b.interval_minutes}"></label>
        <label class="field" style="margin-top:12px"><span>保留自动备份份数</span><input class="input" type="number" min="1" id="bp-keep" value="${b.keep}"></label>
        <button class="btn primary" style="margin-top:16px;width:100%;justify-content:center" id="bp-save">${I('save')}保存策略</button>
      </div>
    </div>`;
  if (innerWidth < 900) $('#bk-grid').style.gridTemplateColumns = '1fr';
  const TAGS = { manual: '手动', auto: '自动', 'pre-restore': '恢复前快照', 'pre-world': '替换前快照' };
  const load = async () => {
    try {
      const list = await GET(`/api/instances/${enc(id)}/backups`);
      $('#bk-list').innerHTML = list.length ? `<div class="list">${list.map((x) => `<div class="list-row" data-n="${esc(x.name)}">
        <div class="thumb">${I('archive')}</div><div class="meta"><div class="t">${fmtTime(x.created)}</div><div class="d">${fmtSize(x.size)}</div></div>
        <span class="badge ${x.tag === 'auto' ? 'ok' : ''}">${esc(TAGS[x.tag] || x.tag)}</span>
        <button class="btn sm" data-a="restore">${I('undo', 'sm')}恢复</button>
        <button class="btn icon sm ghost" data-a="world" title="另存为世界模板">${I('globe', 'sm')}</button>
        <button class="btn icon sm ghost" data-a="dl" title="下载">${I('download', 'sm')}</button>
        <button class="btn icon sm ghost danger" data-a="del" title="删除">${I('trash', 'sm')}</button></div>`).join('')}</div>`
        : `<div class="empty">${I('archive')}<div>还没有备份</div></div>`;
    } catch (e) { fail(e); }
  };
  $('#bk-new').onclick = async () => { try { const r = await POST(`/api/instances/${enc(id)}/backups`); await waitTask(r.task); load(); } catch (e) { fail(e); } };
  $('#bk-list').onclick = async (e) => {
    const btn = e.target.closest('[data-a]'); if (!btn) return;
    const n = btn.closest('[data-n]').dataset.n;
    const base = `/api/instances/${enc(id)}/backups/${enc(n)}`;
    try {
      if (btn.dataset.a === 'restore') {
        if (!(await confirmDlg('恢复会替换当前世界（会先自动做一份快照）。服务器需处于停止状态。', { ok: '恢复' }))) return;
        const r = await POST(base + '/restore'); await waitTask(r.task); load();
      } else if (btn.dataset.a === 'dl') location.href = base + '/download';
      else if (btn.dataset.a === 'world') { const nm = await promptDlg('另存为世界模板', '名称', `${inst.name} ${fmtTime(Number(n.split('-')[0]))}`); if (nm) { const r = await POST(base + '/to-world', { name: nm }); waitTask(r.task); } }
      else if (btn.dataset.a === 'del') { if (await confirmDlg('删除这份备份？', { danger: true, ok: '删除' })) { await api('DELETE', base); load(); } }
    } catch (err) { fail(err); }
  };
  $('#bp-save').onclick = async () => {
    try {
      await api('PATCH', `/api/instances/${enc(id)}`, { backup: { idle_enabled: $('#bp-idle').checked, idle_minutes: +$('#bp-idle-m').value || 10, interval_enabled: $('#bp-int').checked, interval_minutes: +$('#bp-int-m').value || 360, keep: +$('#bp-keep').value || 10 } });
      toast('备份策略已保存');
    } catch (e) { fail(e); }
  };
  load();
  return {};
}

// ---- world & resource pack
function WorldTab(el, id) {
  const inst = instById(id);
  el.innerHTML = `
    <div class="grid g2">
      <div class="card pad"><h3>${I('globe', 'sm')}世界</h3>
        <p class="muted" style="margin-top:0">世界库中的地图以复制方式放入实例（不使用硬链接），替换前会自动做一份快照。</p>
        <div class="row"><button class="btn" id="w-apply">${I('globe')}从世界库替换</button><button class="btn" id="w-save">${I('save')}保存当前世界到世界库</button></div>
      </div>
      <div class="card pad"><h3>${I('image', 'sm')}服务器资源包</h3>
        <p class="muted" style="margin-top:0">资源包由面板直接提供下载，自动写入 <span class="mono">resource-pack</span> 和 <span class="mono">resource-pack-sha1</span>。需在设置中填写面板公网地址。</p>
        <div id="rp-cur" class="muted" style="margin-bottom:12px"></div>
        <div class="row"><button class="btn" id="rp-lib">${I('box')}从资源库选择</button><button class="btn" id="rp-mr">${I('modrinth')}从 Modrinth 选择</button><button class="btn ghost danger" id="rp-clear">${I('x')}清除</button></div>
      </div>
    </div>`;
  const showRp = async () => {
    const i = instById(id);
    const sha = i && i.resource_pack;
    if (!sha) { $('#rp-cur').textContent = '未设置'; return; }
    const lib = (await GET('/api/library?kind=resourcepack').catch(() => [])).find((x) => x.sha1 === sha);
    $('#rp-cur').innerHTML = `${I('image', 'sm')} ${esc(lib ? lib.name : sha.slice(0, 12))}`;
  };
  const setRp = async (sha1) => { try { await POST(`/api/instances/${enc(id)}/resourcepack`, { sha1 }); toast(sha1 ? '已设置资源包' : '已清除资源包'); setTimeout(showRp, 600); } catch (e) { fail(e); } };
  $('#rp-lib').onclick = () => libraryPicker({ kinds: ['resourcepack'], multi: false, onPick: (r) => r[0] && setRp(r[0].sha1) });
  $('#rp-mr').onclick = () => modrinthBrowser({ type: 'resourcepack', mc: inst.core.mc_version, title: '选择资源包', action: '使用', onPick: async (v) => { const r = await POST(`/api/instances/${enc(id)}/modrinth`, { version_id: v.id, deps: false }); waitTask(r.task).then(() => setTimeout(showRp, 600)); } });
  $('#rp-clear').onclick = () => setRp(null);
  $('#w-save').onclick = async () => { const n = await promptDlg('保存世界', '世界名称', `${inst.name} 的世界`); if (n) { try { const r = await POST(`/api/instances/${enc(id)}/world/save`, { name: n }); waitTask(r.task); } catch (e) { fail(e); } } };
  $('#w-apply').onclick = async () => {
    const ws = await GET('/api/worlds').catch(() => []);
    if (!ws.length) return toast('世界库为空，可在「世界」页面上传', 'info');
    const m = modal({
      title: '选择世界', body: `<div class="pick-list">${ws.map((w) => `<div class="list-row" data-w="${esc(w.id)}"><div class="thumb">${I('globe')}</div><div class="meta"><div class="t">${esc(w.name)}</div><div class="d">${fmtSize(w.size)} · ${esc(w.source)}</div></div>${I('chevR', 'sm')}</div>`).join('')}</div>`,
      onMount: (el2) => {
        el2.onclick = async (e) => {
          const r = e.target.closest('[data-w]'); if (!r) return;
          m.close();
          if (!(await confirmDlg('替换当前世界？服务器需处于停止状态，会先自动备份。', { ok: '替换' }))) return;
          try { const t = await POST(`/api/instances/${enc(id)}/world/apply`, { world_id: r.dataset.w }); waitTask(t.task); } catch (err) { fail(err); }
        };
      },
    });
  };
  showRp();
  return {};
}

// ---- settings
function SettingsTab(el, id) {
  const inst = instById(id);
  const L = inst.launch;
  el.innerHTML = `
    <div class="grid g2" style="align-items:start">
      <div class="card pad"><h3>${I('sliders', 'sm')}基本与启动</h3>
        <div class="grid">
          <label class="field"><span>名称</span><input class="input" id="s-name" value="${esc(inst.name)}"></label>
          <div class="grid g2"><label class="field"><span>最小内存</span><input class="input" id="s-min" value="${esc(L.min_mem)}"></label><label class="field"><span>最大内存</span><input class="input" id="s-max" value="${esc(L.max_mem)}"></label></div>
          <div class="grid g2"><label class="field"><span>Java 运行时</span><select class="input" id="s-java"><option value="auto">自动匹配</option></select></label><label class="field"><span>需要的主版本</span><input class="input" type="number" id="s-jmaj" value="${L.java_major}"></label></div>
          <div id="s-jwarn"></div>
          <label class="field"><span>JVM 参数</span><input class="input mono" id="s-jvm" value="${esc(L.jvm_args)}" placeholder="-XX:+UseG1GC ..."></label>
          <div class="grid g2"><label class="field"><span>启动 Jar</span><input class="input mono" id="s-jar" value="${esc(L.jar || '')}"></label><label class="field"><span>参数文件 (@args)</span><input class="input mono" id="s-args" value="${esc(L.args_file || '')}"></label></div>
          <label class="field"><span>服务端参数</span><input class="input mono" id="s-sargs" value="${esc(L.server_args)}"></label>
          <div class="toggle-row"><div class="t"><div>面板启动时自动开服</div></div><label class="switch"><input type="checkbox" id="s-auto" ${inst.auto_start ? 'checked' : ''}><i></i></label></div>
          <div class="toggle-row" style="padding-top:0"><div class="t"><div>崩溃自动重启</div><div>连续 5 次失败后停止</div></div><label class="switch"><input type="checkbox" id="s-restart" ${inst.auto_restart ? 'checked' : ''}><i></i></label></div>
          <button class="btn primary" id="s-save" style="justify-self:start">${I('save')}保存</button>
        </div>
      </div>
      <div class="grid">
        <div class="card pad"><h3>${I('fileText', 'sm')}server.properties</h3><div id="props" style="max-height:420px;overflow:auto"></div><button class="btn primary" id="p-save" style="margin-top:12px">${I('save')}保存属性</button></div>
        <div class="card pad"><h3>${I('server', 'sm')}服务端核心</h3><p class="muted" style="margin-top:0">当前 ${esc(coreLabel(inst.core))}${inst.core.loader_version ? ' · ' + esc(inst.core.loader_version) : ''}。更换核心不会删除世界与配置。</p><button class="btn" id="s-core">${I('refresh')}更换 / 重装核心</button></div>
        <div class="card pad" style="border-color:var(--danger-soft)"><h3 style="color:var(--danger)">${I('alert', 'sm')}危险操作</h3><div class="row"><button class="btn danger" id="s-del">${I('trash')}删除实例</button><label class="row" style="gap:6px;font-size:13px"><input type="checkbox" class="chk" id="s-keepbk">保留备份</label></div></div>
      </div>
    </div>`;
  let runtimes = [];
  const jwarn = () => {
    const sel = $('#s-java').value, maj = +$('#s-jmaj').value;
    const rt = sel === 'auto' ? runtimes.find((r) => r.major === maj) : runtimes.find((r) => r.path === sel);
    $('#s-jwarn').innerHTML = !rt ? `<div class="badge warn" style="white-space:normal">${I('alert', 'sm')}${sel === 'auto' ? `Java ${maj} 未安装，启动时会自动安装` : '所选 Java 不可用'}</div>`
      : rt.major !== maj ? `<div class="badge warn" style="white-space:normal">${I('alert', 'sm')}所选是 Java ${rt.major}，该版本游戏需要 Java ${maj}</div>`
      : `<div class="badge ok">${I('check', 'sm')}将使用 Java ${rt.major} · ${esc(rt.version)}</div>`;
  };
  GET('/api/java').then((d) => {
    runtimes = d.runtimes;
    const sel = $('#s-java'); if (!sel) return;
    sel.innerHTML = `<option value="auto">自动匹配（按主版本）</option>` + runtimes.map((r) => `<option value="${esc(r.path)}" ${r.path === L.java ? 'selected' : ''}>Java ${r.major} · ${r.managed ? '托管' : r.source === 'custom' ? '自定义' : '系统'} · ${esc(r.version)}</option>`).join('');
    if (L.java !== 'auto' && !runtimes.find((r) => r.path === L.java)) sel.insertAdjacentHTML('beforeend', `<option value="${esc(L.java)}" selected>${esc(L.java)}</option>`);
    jwarn();
  }).catch(() => {});
  $('#s-java').onchange = jwarn; $('#s-jmaj').oninput = jwarn;
  const loadProps = async () => {
    try {
      const p = await GET(`/api/instances/${enc(id)}/properties`);
      const keys = Object.keys(p);
      $('#props').innerHTML = keys.length ? `<table class="tbl">${keys.map((k) => `<tr><td class="mono muted" style="width:45%">${esc(k)}</td><td><input class="input mono" data-k="${esc(k)}" value="${esc(p[k])}" style="height:28px"></td></tr>`).join('')}</table>` : '<div class="muted">首次启动后生成</div>';
    } catch (e) { fail(e); }
  };
  $('#p-save').onclick = async () => { const b = {}; $$('#props [data-k]').forEach((i) => (b[i.dataset.k] = i.value)); try { await api('PUT', `/api/instances/${enc(id)}/properties`, b); toast('已保存，重启服务器后生效'); } catch (e) { fail(e); } };
  $('#s-save').onclick = async () => {
    try {
      await api('PATCH', `/api/instances/${enc(id)}`, {
        name: $('#s-name').value, auto_start: $('#s-auto').checked, auto_restart: $('#s-restart').checked,
        launch: { java: $('#s-java').value || 'auto', java_major: +$('#s-jmaj').value || 25, min_mem: $('#s-min').value.trim(), max_mem: $('#s-max').value.trim(), jvm_args: $('#s-jvm').value, jar: $('#s-jar').value.trim() || null, args_file: $('#s-args').value.trim() || null, server_args: $('#s-sargs').value },
      });
      toast('已保存');
    } catch (e) { fail(e); }
  };
  $('#s-core').onclick = () => coreDialog(inst.core, async (core) => { try { const r = await POST(`/api/instances/${enc(id)}/core`, core); waitTask(r.task); } catch (e) { fail(e); } });
  $('#s-del').onclick = async () => {
    if (!(await confirmDlg(`删除实例 <b>${esc(inst.name)}</b> 及其全部文件？此操作不可撤销。`, { danger: true, ok: '删除', title: '删除实例' }))) return;
    try { await api('DELETE', `/api/instances/${enc(id)}?keep_backups=${$('#s-keepbk').checked}`); toast('实例已删除'); location.hash = '#/instances'; } catch (e) { fail(e); }
  };
  loadProps();
  return {};
}

// ------------------------------------------------------------------ core picker
const CORE_KINDS = ['vanilla', 'paper', 'purpur', 'fabric', 'quilt', 'forge', 'neoforge'];
const LOADER_LABEL = { fabric: 'Fabric Loader', quilt: 'Quilt Loader', forge: 'Forge 版本', neoforge: 'NeoForge 版本', paper: '构建号', purpur: '构建号' };
const verCache = {};
function corePicker(el, init, onChange) {
  const st = { kind: init.kind || 'paper', mc_version: init.mc_version || '', loader_version: init.loader_version || null, snapshots: false };
  el.innerHTML = `<div class="seg" id="cp-k" style="margin-bottom:14px">${CORE_KINDS.map((k) => `<button data-k="${k}" class="${k === st.kind ? 'on' : ''}">${CORE_NAMES[k]}</button>`).join('')}</div>
    <div class="grid g2"><label class="field"><span>游戏版本</span><select class="input" id="cp-mc"><option>加载中...</option></select></label>
    <label class="field" id="cp-lw"><span id="cp-ll"></span><select class="input" id="cp-l"></select></label></div>
    <label class="row" style="gap:6px;margin-top:10px;font-size:13px" id="cp-snapw"><input type="checkbox" class="chk" id="cp-snap">显示快照 / 测试版本</label>`;
  const emit = () => onChange && onChange({ kind: st.kind, mc_version: st.mc_version, loader_version: st.loader_version || null });
  const loadLoaders = async () => {
    const lw = $('#cp-lw', el), sel = $('#cp-l', el);
    if (st.kind === 'vanilla') { lw.style.visibility = 'hidden'; st.loader_version = null; emit(); return; }
    lw.style.visibility = ''; $('#cp-ll', el).textContent = LOADER_LABEL[st.kind];
    sel.innerHTML = '<option>加载中...</option>'; sel.disabled = true;
    try {
      const key = `l:${st.kind}:${st.mc_version}`;
      const ls = verCache[key] || (verCache[key] = await GET(`/api/versions/${st.kind}/loaders?mc=${enc(st.mc_version)}`));
      const shown = ls.filter((l) => st.snapshots || !['snapshot', 'beta', 'alpha', 'experimental'].includes(l.type));
      const list = shown.length ? shown : ls;
      sel.innerHTML = `<option value="">最新${list[0] ? `（${esc(list[0].id)}）` : ''}</option>` + list.slice(0, 200).map((l) => `<option value="${esc(l.id)}" ${l.id === st.loader_version ? 'selected' : ''}>${esc(l.id)}${l.type && l.type !== 'release' ? ' · ' + esc(l.type) : ''}</option>`).join('');
      if (!list.length) sel.innerHTML = '<option value="">该版本没有可用的加载器</option>';
    } catch (e) { sel.innerHTML = `<option value="">${esc(e.message)}</option>`; }
    sel.disabled = false; st.loader_version = sel.value || null; emit();
  };
  const loadMc = async () => {
    const sel = $('#cp-mc', el);
    sel.innerHTML = '<option>加载中...</option>'; sel.disabled = true;
    try {
      const vs = verCache[st.kind] || (verCache[st.kind] = await GET(`/api/versions/${st.kind}`));
      const list = vs.filter((v) => st.snapshots || v.type === 'release');
      if (!list.find((v) => v.id === st.mc_version)) st.mc_version = (list[0] || {}).id || '';
      sel.innerHTML = list.map((v) => `<option value="${esc(v.id)}" ${v.id === st.mc_version ? 'selected' : ''}>${esc(v.id)}${v.type !== 'release' ? ' · ' + esc(v.type) : ''}</option>`).join('');
    } catch (e) { sel.innerHTML = `<option>${esc(e.message)}</option>`; }
    sel.disabled = false;
    loadLoaders();
  };
  $('#cp-k', el).onclick = (e) => { const b = e.target.closest('[data-k]'); if (!b || b.dataset.k === st.kind) return; st.kind = b.dataset.k; st.loader_version = null; $$('#cp-k button', el).forEach((x) => x.classList.toggle('on', x === b)); loadMc(); };
  $('#cp-mc', el).onchange = (e) => { st.mc_version = e.target.value; st.loader_version = null; loadLoaders(); };
  $('#cp-l', el).onchange = (e) => { st.loader_version = e.target.value || null; emit(); };
  $('#cp-snap', el).onchange = (e) => { st.snapshots = e.target.checked; loadMc(); };
  loadMc();
  return st;
}

function coreDialog(init, onOk) {
  let core = { ...init };
  const m = modal({
    title: '更换服务端核心', size: 'wide', body: '<div id="cd"></div><p class="hint" style="margin-bottom:0">更换后旧核心文件保留在资源库中（服务端 Jar 存档），随时可切回。</p>',
    foot: `<button class="btn" data-close>取消</button><button class="btn primary" id="cd-ok">${I('check')}安装</button>`,
  });
  corePicker($('#cd', m.el), init.kind === 'custom' ? {} : init, (c) => (core = c));
  $('#cd-ok', m.el).onclick = () => { if (!core.mc_version) return toast('请选择版本', 'err'); m.close(); onOk(core); };
}

// ------------------------------------------------------------------ assemble
function Assemble(main) {
  const body = pageShell(main, { title: '拼装服务器', sub: '核心 + 整合包 + 模组 + 资源包 + 世界，组合出一个服务器', actions: `<button class="btn" id="as-recipes">${I('bookmark')}配方</button>` });
  const R = { name: '', core: { kind: 'paper', mc_version: '', loader_version: null }, modpack: null, mods: [], resourcepack: null, datapacks: [], world: null, max_mem: '4G', port: 25565, eula: false, strip_client: true, motd: '' };
  const label = {}; // display labels keyed by ref
  body.innerHTML = `<div class="steps">
    <div>
      <div class="card pad step"><div class="step-head"><span class="step-num">1</span><h3>基本信息</h3></div>
        <div class="grid g3"><label class="field"><span>实例名称</span><input class="input" id="as-name" placeholder="我的服务器"></label>
        <label class="field"><span>端口</span><input class="input" type="number" id="as-port" value="25565"></label>
        <label class="field"><span>最大内存</span><input class="input" id="as-mem" value="4G"></label></div>
        <label class="field" style="margin-top:12px"><span>MOTD</span><input class="input" id="as-motd" placeholder="A Minecraft Server"></label>
      </div>
      <div class="card pad step"><div class="step-head"><span class="step-num">2</span><h3>整合包 <span class="muted" style="font-weight:400;font-size:12px">可选，选择后由整合包决定核心</span></h3><button class="btn sm" id="as-pack-mr">${I('modrinth', 'sm')}Modrinth</button><button class="btn sm" id="as-pack-lib">${I('box', 'sm')}资源库</button></div>
        <div id="as-pack" class="muted">未选择。支持 Modrinth (.mrpack) 与 CurseForge 整合包（上传到资源库后选择）。</div>
      </div>
      <div class="card pad step" id="as-core-card"><div class="step-head"><span class="step-num">3</span><h3>服务端核心</h3></div><div id="as-core"></div></div>
      <div class="card pad step"><div class="step-head"><span class="step-num">4</span><h3>模组 / 插件</h3><button class="btn sm" id="as-mod-mr">${I('modrinth', 'sm')}Modrinth</button><button class="btn sm" id="as-mod-lib">${I('box', 'sm')}资源库</button></div>
        <div class="chips" id="as-mods"></div>
        <div class="toggle-row"><div class="t"><div>自动剔除客户端模组</div><div>按 Modrinth 元数据、jar 声明与已知名单识别，移入 mods-disabled</div></div><label class="switch"><input type="checkbox" id="as-strip" checked><i></i></label></div>
      </div>
      <div class="card pad step"><div class="step-head"><span class="step-num">5</span><h3>资源包与数据包</h3><button class="btn sm" id="as-rp">${I('image', 'sm')}资源包</button><button class="btn sm" id="as-dp">${I('database', 'sm')}数据包</button></div>
        <div class="chips" id="as-packs"></div>
      </div>
      <div class="card pad step"><div class="step-head"><span class="step-num">6</span><h3>世界</h3><button class="btn sm" id="as-world">${I('globe', 'sm')}从世界库选择</button></div>
        <div id="as-w" class="muted">新世界（首次启动时生成）</div>
      </div>
    </div>
    <div class="card pad summary">
      <h3>${I('layers', 'sm')}配方预览</h3>
      <div id="as-sum"></div>
      <label class="row" style="gap:8px;margin:16px 0 6px;font-size:13px"><input type="checkbox" class="chk" id="as-eula">我同意 <a href="https://aka.ms/MinecraftEULA" target="_blank" rel="noopener" style="color:var(--accent)">Minecraft EULA</a></label>
      <label class="row" style="gap:8px;margin-bottom:16px;font-size:13px"><input type="checkbox" class="chk" id="as-save">同时保存为配方</label>
      <button class="btn primary" style="width:100%;justify-content:center;height:40px" id="as-go">${I('wand')}拼装并创建</button>
      <p class="hint" style="margin-bottom:0">所有下载进入本地资源库，再以硬链接放入实例，相同文件只占一份空间。</p>
    </div>
  </div>`;
  const refKey = (r) => r.source === 'library' ? 'lib:' + r.sha1 : 'mr:' + r.version_id;
  const chip = (r, group) => `<span class="chip">${I(group === 'dp' ? 'database' : group === 'rp' ? 'image' : 'puzzle', 'sm')}${esc(label[refKey(r)] || refKey(r))}<button data-rm="${group}:${esc(refKey(r))}" title="移除">${I('x', 'sm')}</button></span>`;
  const summary = () => {
    const c = R.core;
    $('#as-sum').innerHTML = [
      ['名称', esc(R.name || '-')],
      ['核心', R.modpack ? '<span class="muted">由整合包决定</span>' : `${esc(CORE_NAMES[c.kind])} ${esc(c.mc_version)}${c.loader_version ? ' · ' + esc(c.loader_version) : ''}`],
      ['整合包', R.modpack ? esc(label[refKey(R.modpack)]) : '-'],
      ['模组', R.mods.length ? `${R.mods.length} 个` : '-'],
      ['资源包', R.resourcepack ? esc(label[refKey(R.resourcepack)]) : '-'],
      ['数据包', R.datapacks.length ? `${R.datapacks.length} 个` : '-'],
      ['世界', R.world ? esc(label['w:' + R.world]) : '新世界'],
      ['内存', esc(R.max_mem)],
      ['端口', String(R.port)],
    ].map(([k, v]) => `<div class="item"><span class="k">${k}</span><span class="ellip">${v}</span></div>`).join('');
    $('#as-mods').innerHTML = R.mods.length ? R.mods.map((r) => chip(r, 'mod')).join('') : '<span class="muted">未选择</span>';
    $('#as-packs').innerHTML = (R.resourcepack ? chip(R.resourcepack, 'rp') : '') + R.datapacks.map((r) => chip(r, 'dp')).join('') || '<span class="muted">未选择</span>';
    $('#as-pack').innerHTML = R.modpack ? `<span class="chip">${I('layers', 'sm')}${esc(label[refKey(R.modpack)])}<button data-rm="pack:x">${I('x', 'sm')}</button></span>` : '未选择。支持 Modrinth (.mrpack) 与 CurseForge 整合包（上传到资源库后选择）。';
    $('#as-core-card').style.opacity = R.modpack ? '.5' : '1';
    $('#as-core-card').style.pointerEvents = R.modpack ? 'none' : '';
    $('#as-w').innerHTML = R.world ? `<span class="chip">${I('globe', 'sm')}${esc(label['w:' + R.world])}<button data-rm="world:x">${I('x', 'sm')}</button></span>` : '新世界（首次启动时生成）';
  };
  body.addEventListener('click', (e) => {
    const b = e.target.closest('[data-rm]'); if (!b) return;
    const [g, k] = [b.dataset.rm.slice(0, b.dataset.rm.indexOf(':')), b.dataset.rm.slice(b.dataset.rm.indexOf(':') + 1)];
    if (g === 'mod') R.mods = R.mods.filter((r) => refKey(r) !== k);
    if (g === 'dp') R.datapacks = R.datapacks.filter((r) => refKey(r) !== k);
    if (g === 'rp') R.resourcepack = null;
    if (g === 'pack') R.modpack = null;
    if (g === 'world') R.world = null;
    summary();
  });
  const bind = (sel, key, f = (v) => v) => { $(sel).oninput = (e) => { R[key] = f(e.target.value); summary(); }; };
  bind('#as-name', 'name'); bind('#as-mem', 'max_mem'); bind('#as-port', 'port', (v) => +v || 25565); bind('#as-motd', 'motd');
  $('#as-strip').onchange = (e) => (R.strip_client = e.target.checked);
  $('#as-eula').onchange = (e) => (R.eula = e.target.checked);
  corePicker($('#as-core'), R.core, (c) => { R.core = c; summary(); });
  const mcOf = () => R.core.mc_version;
  $('#as-pack-mr').onclick = () => modrinthBrowser({ type: 'modpack', title: '选择整合包', action: '选择', onPick: (v, p) => { R.modpack = { source: 'modrinth', version_id: v.id }; label[refKey(R.modpack)] = `${p.title} ${v.version_number}`; summary(); } });
  $('#as-pack-lib').onclick = () => libraryPicker({ kinds: ['modpack'], multi: false, onPick: (rows) => { if (!rows[0]) return; R.modpack = { source: 'library', sha1: rows[0].sha1 }; label[refKey(R.modpack)] = rows[0].name; summary(); } });
  const modKind = () => ['paper', 'purpur'].includes(R.core.kind) ? 'plugin' : 'mod';
  $('#as-mod-mr').onclick = () => modrinthBrowser({
    type: modKind(), mc: mcOf(), loader: R.core.kind, title: '添加模组 / 插件', action: '添加',
    onPick: (v, p) => { const r = { source: 'modrinth', version_id: v.id }; if (!R.mods.find((x) => refKey(x) === refKey(r))) { R.mods.push(r); label[refKey(r)] = p.title; summary(); toast(`已添加 ${p.title}`); } return false; },
  });
  $('#as-mod-lib').onclick = () => libraryPicker({ kinds: ['mod', 'plugin'], selected: R.mods.filter((r) => r.source === 'library').map((r) => r.sha1), onPick: (rows) => {
    R.mods = R.mods.filter((r) => r.source !== 'library');
    rows.forEach((x) => { const r = { source: 'library', sha1: x.sha1 }; R.mods.push(r); label[refKey(r)] = (x.meta && x.meta.title) || x.name; });
    summary();
  } });
  $('#as-rp').onclick = () => libraryPicker({ kinds: ['resourcepack'], multi: false, onPick: (rows) => { if (!rows[0]) return; R.resourcepack = { source: 'library', sha1: rows[0].sha1 }; label[refKey(R.resourcepack)] = rows[0].name; summary(); } });
  $('#as-dp').onclick = () => libraryPicker({ kinds: ['datapack'], onPick: (rows) => { rows.forEach((x) => { const r = { source: 'library', sha1: x.sha1 }; if (!R.datapacks.find((y) => refKey(y) === refKey(r))) { R.datapacks.push(r); label[refKey(r)] = x.name; } }); summary(); } });
  $('#as-world').onclick = async () => {
    const ws = await GET('/api/worlds').catch(() => []);
    if (!ws.length) return toast('世界库为空，可在「世界」页面上传或从实例保存', 'info');
    const m = modal({ title: '选择世界', body: `<div class="pick-list">${ws.map((w) => `<div class="list-row" data-w="${esc(w.id)}" data-n="${esc(w.name)}"><div class="thumb">${I('globe')}</div><div class="meta"><div class="t">${esc(w.name)}</div><div class="d">${fmtSize(w.size)} · ${esc(w.source)}</div></div></div>`).join('')}</div>`,
      onMount: (el) => { el.onclick = (e) => { const r = e.target.closest('[data-w]'); if (!r) return; R.world = r.dataset.w; label['w:' + R.world] = r.dataset.n; summary(); m.close(); }; } });
  };
  const buildReq = () => {
    const props = {}; if (R.motd) props.motd = R.motd;
    return { name: R.name.trim(), core: R.modpack ? null : R.core, modpack: R.modpack, mods: R.mods, resourcepack: R.resourcepack, datapacks: R.datapacks, world: R.world, max_mem: R.max_mem, port: R.port, eula: R.eula, strip_client: R.strip_client, properties: props };
  };
  $('#as-go').onclick = async () => {
    const req = buildReq();
    if (!req.name) { $('#as-name').focus(); return toast('请填写实例名称', 'err'); }
    if (!req.modpack && !req.core.mc_version) return toast('请选择游戏版本', 'err');
    if ($('#as-save').checked) req.save_recipe = req.name;
    const btn = $('#as-go'); btn.disabled = true;
    try {
      const r = await POST('/api/instances', req);
      toast('开始拼装，进度可在任务中查看', 'info');
      S.instances = await GET('/api/instances');
      location.hash = `#/instance/${enc(r.id)}/console`;
      waitTask(r.task);
    } catch (e) { fail(e); btn.disabled = false; }
  };
  $('#as-recipes').onclick = async () => {
    const list = await GET('/api/recipes').catch(() => []);
    const m = modal({
      title: `${I('bookmark', 'sm')} 配方`, size: 'wide',
      body: list.length ? `<div class="list card">${list.map((x) => `<div class="list-row" data-id="${esc(x.id)}"><div class="thumb">${I('layers')}</div><div class="meta"><div class="t">${esc(x.name)}</div><div class="d">${x.data.modpack ? '整合包' : esc(coreLabel(x.data.core || {}))} · ${(x.data.mods || []).length} 个模组 · ${fmtTime(x.created)}</div></div><button class="btn sm primary" data-a="use">使用</button><button class="btn icon sm ghost danger" data-a="del">${I('trash', 'sm')}</button></div>`).join('')}</div>` : `<div class="empty">${I('bookmark')}<div>还没有配方。创建实例时勾选「同时保存为配方」。</div></div>`,
      onMount: (el) => {
        el.onclick = async (e) => {
          const b = e.target.closest('[data-a]'); if (!b) return;
          const rid = b.closest('[data-id]').dataset.id;
          const rec = list.find((x) => x.id === rid);
          if (b.dataset.a === 'del') { await api('DELETE', `/api/recipes/${enc(rid)}`); b.closest('.list-row').remove(); return; }
          const nm = await promptDlg('用配方创建实例', '新实例名称', rec.name + ' 副本');
          if (!nm) return;
          try { const r = await POST('/api/instances', { ...rec.data, name: nm, eula: false }); m.close(); toast('开始拼装', 'info'); S.instances = await GET('/api/instances'); location.hash = `#/instance/${enc(r.id)}/console`; waitTask(r.task); } catch (err) { fail(err); }
        };
      },
    });
  };
  summary();
  return {};
}

// ------------------------------------------------------------------ library
function Library(main) {
  const body = pageShell(main, { title: '资源库', sub: '所有下载与上传的资源只存一份，实例通过硬链接使用', actions: `<button class="btn" id="lb-gc">${I('broom')}清理未使用</button><button class="btn" id="lb-mr">${I('modrinth')}从 Modrinth 添加</button><button class="btn primary" id="lb-up">${I('upload')}上传</button>` });
  let kind = '';
  body.innerHTML = `<div class="grid g4 stats-grid" id="lb-stats" style="margin-bottom:18px"></div>
    <div class="row" style="margin-bottom:12px;flex-wrap:wrap"><div class="seg" id="lb-k"><button data-k="" class="on">全部</button>${['mod', 'plugin', 'modpack', 'resourcepack', 'datapack', 'server', 'installer', 'other'].map((k) => `<button data-k="${k}">${KIND_NAMES[k]}</button>`).join('')}</div><span class="spacer"></span><input class="input" id="lb-q" placeholder="筛选" style="width:220px"></div>
    <div class="card dropzone" id="lb-dz"><div class="list" id="lb-list"></div></div>`;
  const stats = async () => {
    try {
      const s = await GET('/api/library/stats');
      $('#lb-stats').innerHTML = `
        <div class="card stat"><div class="k">${I('box', 'sm')}资源数</div><div class="v">${s.objects}</div></div>
        <div class="card stat"><div class="k">${I('disk', 'sm')}占用空间</div><div class="v">${fmtSize(s.bytes)}</div></div>
        <div class="card stat"><div class="k">${I('link', 'sm')}硬链接节省</div><div class="v" style="color:var(--accent)">${fmtSize(s.saved_bytes)}</div></div>
        <div class="card stat"><div class="k">${I('globe', 'sm')}世界模板</div><div class="v">${s.worlds}</div></div>`;
    } catch (e) { /* */ }
  };
  let items = [];
  const render = () => {
    const q = $('#lb-q').value.toLowerCase();
    const rows = items.filter((i) => !q || (i.name + ' ' + ((i.meta && i.meta.title) || '')).toLowerCase().includes(q));
    $('#lb-list').innerHTML = rows.length ? rows.map((i) => `<div class="list-row" data-sha="${i.sha1}">
      <div class="thumb">${i.meta && i.meta.icon_url ? `<img loading="lazy" src="${esc(i.meta.icon_url)}" alt="">` : I(KIND_ICON[i.kind] || 'file')}</div>
      <div class="meta"><div class="t ellip">${esc((i.meta && i.meta.title) || i.name)}</div><div class="d ellip">${esc(i.name)} · ${fmtSize(i.size)} · <span class="mono">${i.sha1.slice(0, 10)}</span> · ${esc(i.source)}</div></div>
      <span class="badge">${KIND_NAMES[i.kind] || i.kind}</span>
      <span class="badge ${i.links ? 'ok' : ''}" title="被实例引用次数">${I('link', 'sm')}${i.links}</span>
      <select class="input hide-m" data-kind style="width:110px;height:28px">${Object.keys(KIND_NAMES).map((k) => `<option value="${k}" ${k === i.kind ? 'selected' : ''}>${KIND_NAMES[k]}</option>`).join('')}</select>
      <button class="btn icon sm ghost danger" data-del title="删除">${I('trash', 'sm')}</button></div>`).join('')
      : `<div class="empty">${I('box')}<div>没有资源。拖拽文件到这里上传。</div></div>`;
  };
  const load = async () => { try { items = await GET(`/api/library${kind ? '?kind=' + kind : ''}`); render(); } catch (e) { fail(e); } stats(); };
  $('#lb-k').onclick = (e) => { const b = e.target.closest('[data-k]'); if (!b) return; kind = b.dataset.k; $$('#lb-k button').forEach((x) => x.classList.toggle('on', x === b)); load(); };
  $('#lb-q').oninput = debounce(render, 150);
  $('#lb-list').onclick = async (e) => {
    const d = e.target.closest('[data-del]'); if (!d) return;
    const r = d.closest('[data-sha]');
    const it = items.find((x) => x.sha1 === r.dataset.sha);
    if (!(await confirmDlg(`从资源库删除 <b>${esc(it.name)}</b>？${it.links ? `<div class="hint" style="margin-top:6px">有 ${it.links} 个实例正在使用它，删除后实例中的文件仍保留（硬链接独立存在）。</div>` : ''}`, { danger: true, ok: '删除' }))) return;
    try { await api('DELETE', `/api/library/${it.sha1}`); r.style.transition = 'opacity .2s, transform .2s'; r.style.opacity = 0; r.style.transform = 'translateX(10px)'; setTimeout(load, 200); } catch (err) { fail(err); }
  };
  $('#lb-list').onchange = async (e) => {
    if (!e.target.matches('[data-kind]')) return;
    const sha = e.target.closest('[data-sha]').dataset.sha;
    try { await api('PATCH', `/api/library/${sha}`, { kind: e.target.value }); toast('已更新类型'); } catch (err) { fail(err); }
  };
  const doUp = async (files) => {
    if (!files.length) return;
    try { const r = await upload(`/api/library/upload${kind ? '?kind=' + kind : ''}`, files); toast(`已入库 ${r.sha1.length} 个文件`); load(); } catch (e) { fail(e); }
  };
  $('#lb-up').onclick = async () => doUp(await pickFiles());
  const dz = $('#lb-dz');
  dz.addEventListener('dragover', (e) => { e.preventDefault(); dz.classList.add('over'); });
  dz.addEventListener('dragleave', (e) => { if (!dz.contains(e.relatedTarget)) dz.classList.remove('over'); });
  dz.addEventListener('drop', (e) => { e.preventDefault(); dz.classList.remove('over'); doUp([...e.dataTransfer.files]); });
  $('#lb-gc').onclick = async () => {
    if (!(await confirmDlg('删除所有未被任何实例引用的资源？服务端核心存档默认保留。', { ok: '清理' }))) return;
    try { const r = await POST('/api/library/gc', { include_servers: false }); toast(`已清理 ${r.removed} 个，释放 ${fmtSize(r.freed)}`); load(); } catch (e) { fail(e); }
  };
  $('#lb-mr').onclick = () => modrinthBrowser({
    type: 'mod', types: ['mod', 'plugin', 'modpack', 'resourcepack', 'datapack'], title: '添加到资源库', action: '下载',
    onPick: async (v, p, t) => { const r = await POST('/api/library/modrinth', { version_id: v.id, kind: t === 'mod' || t === 'plugin' ? null : t }); toast(`开始下载 ${p.title}`, 'info'); waitTask(r.task).then(load); return false; },
  });
  load();
  return { onTask: (t) => { if (t.kind === 'library' && t.state === 'done') load(); } };
}

// ------------------------------------------------------------------ worlds
function Worlds(main) {
  const body = pageShell(main, { title: '世界', sub: '世界模板会以复制方式放入实例', actions: `<button class="btn primary" id="w-up">${I('upload')}上传世界</button>` });
  body.innerHTML = `<div class="card dropzone" id="w-dz"><div class="list" id="w-list"></div></div><p class="hint">支持包含 level.dat 的 .zip / .tar.gz / .tar.zst 压缩包。也可以在实例的「备份」或「世界与资源包」中保存。</p>`;
  const load = async () => {
    try {
      const ws = await GET('/api/worlds');
      $('#w-list').innerHTML = ws.length ? ws.map((w) => `<div class="list-row" data-id="${esc(w.id)}"><div class="thumb">${I('globe')}</div><div class="meta"><div class="t">${esc(w.name)}</div><div class="d">${fmtSize(w.size)} · ${esc(w.source)} · ${fmtTime(w.created)}</div></div>
        <button class="btn icon sm ghost" data-a="rename" title="重命名">${I('rename', 'sm')}</button><button class="btn icon sm ghost danger" data-a="del" title="删除">${I('trash', 'sm')}</button></div>`).join('')
        : `<div class="empty">${I('globe')}<div>世界库为空</div></div>`;
    } catch (e) { fail(e); }
  };
  $('#w-list').onclick = async (e) => {
    const b = e.target.closest('[data-a]'); if (!b) return;
    const r = b.closest('[data-id]'); const id = r.dataset.id;
    try {
      if (b.dataset.a === 'rename') { const n = await promptDlg('重命名世界', '名称', $('.t', r).textContent); if (n) { await api('PATCH', `/api/worlds/${enc(id)}`, { name: n }); load(); } }
      else if (await confirmDlg('删除这个世界模板？已使用它的实例不受影响。', { danger: true, ok: '删除' })) { await api('DELETE', `/api/worlds/${enc(id)}`); load(); }
    } catch (err) { fail(err); }
  };
  const doUp = async (files) => {
    if (!files.length) return;
    try { for (const f of files) { const r = await upload('/api/worlds/upload', [f]); waitTask(r.task).then(load); } toast('上传完成，正在导入', 'info'); } catch (e) { fail(e); }
  };
  $('#w-up').onclick = async () => doUp(await pickFiles({ accept: '.zip,.gz,.tgz,.zst,.tar', multiple: false }));
  const dz = $('#w-dz');
  dz.addEventListener('dragover', (e) => { e.preventDefault(); dz.classList.add('over'); });
  dz.addEventListener('dragleave', (e) => { if (!dz.contains(e.relatedTarget)) dz.classList.remove('over'); });
  dz.addEventListener('drop', (e) => { e.preventDefault(); dz.classList.remove('over'); doUp([...e.dataTransfer.files]); });
  load();
  return { onTask: (t) => { if (t.kind === 'world' && t.state === 'done') load(); } };
}

// ------------------------------------------------------------------ tasks
function Tasks(main) {
  const body = pageShell(main, { title: '任务', sub: '后台下载、安装与文件处理', actions: `<button class="btn" id="t-clear">${I('broom')}清除已完成</button>` });
  body.innerHTML = '<div class="card" id="t-list"></div>';
  const ICON = { running: 'loader', done: 'checkCircle', failed: 'xCircle', cancelled: 'minusCircle' };
  const row = (t) => `<div class="task" data-id="${t.id}">${I(ICON[t.state], 'st-' + t.state)}
    <div class="meta"><div class="row" style="gap:8px"><span class="ellip" style="font-weight:500">${esc(t.title)}</span>${t.instance && instById(t.instance) ? `<a class="badge" href="#/instance/${enc(t.instance)}/console">${esc(instById(t.instance).name)}</a>` : ''}<span class="spacer"></span><span class="muted" style="font-size:12px">${fmtTime(t.created)}</span></div>
    <div class="msg" title="${esc(t.message)}">${esc(t.message)}</div>
    ${t.state === 'running' ? `<div class="meter ${t.progress < 0 ? 'indet' : ''}"><i style="width:${Math.max(0, t.progress) * 100}%"></i></div>` : ''}</div>
    ${t.state === 'running' ? `<button class="btn icon sm ghost" data-cancel title="取消">${I('x', 'sm')}</button>` : ''}</div>`;
  const render = () => {
    const ts = [...S.tasks.values()].sort((a, b) => b.created - a.created);
    $('#t-list').innerHTML = ts.length ? ts.map(row).join('') : `<div class="empty">${I('activity')}<div>暂无任务</div></div>`;
  };
  const patch = (t) => {
    const el = $(`#t-list [data-id="${t.id}"]`);
    if (!el) return render();
    // update in place so progress bars animate smoothly
    const meter = $('.meter i', el);
    if (t.state === 'running' && meter) {
      $('.meter', el).classList.toggle('indet', t.progress < 0);
      meter.style.width = Math.max(0, t.progress) * 100 + '%';
      $('.msg', el).textContent = t.message;
    } else { const tmp = document.createElement('div'); tmp.innerHTML = row(t); el.replaceWith(tmp.firstElementChild); }
  };
  $('#t-list').onclick = async (e) => { const b = e.target.closest('[data-cancel]'); if (b) { await POST(`/api/tasks/${b.closest('[data-id]').dataset.id}/cancel`); } };
  $('#t-clear').onclick = async () => { await POST('/api/tasks/clear'); [...S.tasks.values()].filter((t) => t.state !== 'running').forEach((t) => S.tasks.delete(t.id)); render(); };
  render();
  return { onTask: patch };
}

// ------------------------------------------------------------------ settings
function Settings(main) {
  const body = pageShell(main, { title: '设置', sub: '下载源、API 与运行环境' });
  const s = S.settings;
  body.innerHTML = `<div class="grid g2" style="align-items:start">
    <div class="grid">
      <div class="card pad"><h3>${I('download', 'sm')}下载</h3>
        <div class="grid">
          <label class="field"><span>下载源</span><select class="input" id="st-mirror"><option value="official">官方</option><option value="bmclapi" ${s.mirror === 'bmclapi' ? 'selected' : ''}>BMCLAPI 镜像（中国大陆推荐）</option></select><span class="hint">镜像失败时自动回退到官方源</span></label>
          <label class="field"><span>并发下载数</span><input class="input" type="number" min="1" max="32" id="st-conc" value="${esc(s.download_concurrency || '8')}"><span class="hint">重启面板后生效</span></label>
          <label class="field"><span>CurseForge API Key</span><input class="input" type="password" id="st-cf" value="${esc(s.curseforge_key || '')}" placeholder="留空则使用 MCIM 镜像"><span class="hint">用于解析 CurseForge 整合包</span></label>
        </div>
      </div>
      <div class="card pad"><h3>${I('globe', 'sm')}面板</h3>
        <div class="grid">
          <label class="field"><span>面板公网地址</span><input class="input" id="st-url" value="${esc(s.public_url || '')}" placeholder="https://panel.example.com"><span class="hint">玩家下载服务器资源包时使用</span></label>
          <label class="field"><span>Monaco 编辑器 CDN</span><input class="input" id="st-monaco" value="${esc(s.monaco_cdn || '')}" placeholder="https://cdn.jsdelivr.net/npm/monaco-editor@0.52.2/min"></label>
          <button class="btn primary" id="st-save" style="justify-self:start">${I('save')}保存设置</button>
        </div>
      </div>
    </div>
    <div class="grid">
      <div class="card pad"><h3>${I('java', 'sm')}Java 运行时</h3>
        <div id="st-java"><div class="skeleton"></div></div>
        <div class="row" style="margin-top:14px;flex-wrap:wrap;gap:8px">
          <select class="input" id="jv-sel" style="flex:1;min-width:150px"><option>加载可用版本...</option></select>
          <button class="btn primary" id="jv-inst">${I('download', 'sm')}安装</button>
        </div>
        <div class="row" style="margin-top:8px;gap:8px">
          <input class="input mono" id="jv-path" placeholder="/usr/lib/jvm/xxx/bin/java" style="flex:1;min-width:0">
          <button class="btn" id="jv-add">${I('plus', 'sm')}添加路径</button>
        </div>
        <p class="hint" style="margin-bottom:0">托管版本来自 Eclipse Temurin。实例按游戏版本自动匹配主版本（26.x 需要 Java 25，1.20.5–1.21.x 需要 21，1.18–1.20.4 需要 17，更早需要 8），缺失时启动会自动安装。系统中已有的 Java 会自动识别。</p>
      </div>
      <div class="card pad"><h3>${I('key', 'sm')}修改密码</h3>
        <div class="grid"><input class="input" type="password" id="pw-old" placeholder="当前密码" autocomplete="current-password"><input class="input" type="password" id="pw-new" placeholder="新密码（至少 6 位）" autocomplete="new-password"><button class="btn" id="pw-go" style="justify-self:start">${I('check')}修改</button></div>
      </div>
    </div></div>`;
  const loadJava = async () => {
    try {
      const d = await GET('/api/java');
      const have = new Set(d.runtimes.map((r) => r.major));
      const need = Object.entries(d.required || {});
      const missing = need.filter(([m]) => !have.has(+m));
      const SRC = { managed: '托管', system: '系统', custom: '自定义' };
      $('#st-java').innerHTML = (missing.length ? `<div class="badge warn" style="margin-bottom:10px;white-space:normal">${I('alert', 'sm')}缺少 ${missing.map(([m, n]) => `Java ${m}（${n.map(esc).join('、')}）`).join('，')}</div>` : '') +
        (d.runtimes.length ? `<div class="list">${d.runtimes.map((j) => {
          const users = (d.required || {})[j.major] || [];
          return `<div class="list-row" style="padding:8px 0" data-major="${j.major}" data-path="${esc(j.path)}">
          <div class="thumb">${I('java')}</div>
          <div class="meta"><div class="t">Java ${j.major} <span class="muted" style="font-weight:400">${esc(j.version)}</span></div><div class="d mono ellip" title="${esc(j.path)}">${esc(j.path)}</div></div>
          <span class="badge ${j.managed ? 'ok' : ''}">${SRC[j.source] || j.source}</span>
          ${users.length ? `<span class="badge" title="${esc(users.join('、'))}">${I('server', 'sm')}${users.length}</span>` : ''}
          ${j.managed ? `<button class="btn icon sm ghost danger" data-jdel title="卸载">${I('trash', 'sm')}</button>` : j.source === 'custom' ? `<button class="btn icon sm ghost" data-jrm title="移除">${I('x', 'sm')}</button>` : ''}
        </div>`; }).join('')}</div>` : '<div class="muted">还没有可用的 Java</div>');
    } catch (e) { fail(e); }
  };
  const loadAvail = async () => {
    try {
      const a = await GET('/api/java/available');
      const lts = new Set(a.available_lts_releases || []);
      const list = (a.available_releases || []).slice().sort((x, y) => y - x);
      $('#jv-sel').innerHTML = list.map((v) => `<option value="${v}" ${v === a.most_recent_lts ? 'selected' : ''}>Java ${v}${lts.has(v) ? '  LTS' : ''}${v === a.most_recent_lts ? '（推荐）' : ''}</option>`).join('');
    } catch (e) { $('#jv-sel').innerHTML = [25, 21, 17, 8].map((v) => `<option value="${v}">Java ${v} LTS</option>`).join(''); }
  };
  $('#jv-inst').onclick = async () => {
    const b = $('#jv-inst'); b.disabled = true;
    try { const r = await POST('/api/java/install', { major: +$('#jv-sel').value }); await waitTask(r.task); loadJava(); } catch (e) { fail(e); }
    b.disabled = false;
  };
  $('#jv-add').onclick = async () => {
    const p = $('#jv-path').value.trim(); if (!p) return;
    try { const r = await POST('/api/java/custom', { path: p }); toast(`已添加 Java ${r.major}（${r.version}）`); $('#jv-path').value = ''; loadJava(); } catch (e) { fail(e); }
  };
  $('#st-java').onclick = async (e) => {
    const row = e.target.closest('[data-major]'); if (!row) return;
    try {
      if (e.target.closest('[data-jdel]')) {
        if (!(await confirmDlg(`卸载托管的 Java ${row.dataset.major}？需要它的实例下次启动时会自动重新安装。`, { danger: true, ok: '卸载' }))) return;
        await api('DELETE', `/api/java/${row.dataset.major}`); toast('已卸载'); loadJava();
      } else if (e.target.closest('[data-jrm]')) { await POST('/api/java/custom/remove', { path: row.dataset.path }); loadJava(); }
    } catch (err) { fail(err); }
  };
  $('#st-save').onclick = async () => {
    const b = { mirror: $('#st-mirror').value, download_concurrency: $('#st-conc').value, curseforge_key: $('#st-cf').value, public_url: $('#st-url').value, monaco_cdn: $('#st-monaco').value };
    try { await api('PUT', '/api/settings', b); S.settings = await GET('/api/settings'); monacoP = null; toast('设置已保存'); } catch (e) { fail(e); }
  };
  $('#pw-go').onclick = async () => { try { await POST('/api/auth/password', { password: $('#pw-old').value, new_password: $('#pw-new').value }); toast('密码已修改'); $('#pw-old').value = $('#pw-new').value = ''; } catch (e) { fail(e); } };
  loadJava(); loadAvail();
  return {};
}


// ------------------------------------------------------------------ Apple Watch (≤ 260px)
function WatchHome(main) {
  main.innerHTML = `<div class="watch"><div class="watch-head"><span class="logo">${I('rime', 'sm')}</span><span class="ellip" style="flex:1;min-width:0">${BRAND.name}</span><span class="muted" id="wh-run" style="font-size:11px;font-weight:500;white-space:nowrap"></span></div><div id="wh-list"></div></div>`;
  const render = () => {
    const run = S.instances.filter((i) => i.status === 'running').length;
    $('#wh-run').textContent = `${run}/${S.instances.length} 运行`;
    const el = $('#wh-list');
    const html = S.instances.length ? S.instances.map((i) => `<a class="w-item" href="#/instance/${enc(i.id)}"><i class="dot ${i.installing ? 'starting' : i.status}"></i>
      <span class="n"><b>${esc(i.name)}</b><small>${i.installing ? '安装中' : STATUS_TEXT[i.status]}${i.players ? ` · ${i.players[0]}/${i.players[1]} 人` : ''}</small></span>${I('chevR', 'sm')}</a>`).join('')
      : `<div class="muted" style="text-align:center;padding:20px 0;font-size:12px">暂无实例</div>`;
    if (el._h !== html) { el.innerHTML = html; el._h = html; }
  };
  render();
  return { onInstances: render };
}

function WatchInstance(main, id) {
  const lines = [];
  main.innerHTML = `<div class="watch">
    <div class="watch-head"><a href="#/">${I('chevL', 'sm')}</a><span class="ellip" id="wi-name" style="flex:1"></span></div>
    <div class="w-big"><div class="num" id="wi-pl">-</div><div class="lbl" id="wi-st"></div></div>
    <div class="w-stats"><div><small>内存</small><b id="wi-mem">-</b></div><div><small>CPU</small><b id="wi-cpu">-</b></div><div><small>运行</small><b id="wi-up">-</b></div><div><small>端口</small><b id="wi-port">-</b></div></div>
    <div id="wi-btns"></div>
    <div class="w-row"><button class="w-btn" data-w="save">${I('save', 'sm')}保存</button><button class="w-btn" data-w="backup">${I('archive', 'sm')}备份</button></div>
    <div class="w-log" id="wi-log"></div>
  </div>`;
  const render = () => {
    const i = instById(id); if (!i) return;
    $('#wi-name').textContent = i.name;
    $('#wi-pl').textContent = i.players ? `${i.players[0]}/${i.players[1]}` : '-';
    $('#wi-st').innerHTML = `<span class="dot ${i.status}" style="display:inline-block;vertical-align:middle;margin-right:4px"></span>${i.installing ? '安装中' : STATUS_TEXT[i.status]} · 在线玩家`;
    $('#wi-mem').textContent = i.mem_kb ? fmtSize(i.mem_kb * 1024) : '-';
    $('#wi-cpu').textContent = i.status === 'running' ? i.cpu.toFixed(0) + '%' : '-';
    $('#wi-up').textContent = i.uptime != null ? fmtDur(i.uptime) : '-';
    $('#wi-port').textContent = i.port;
    const running = i.status === 'running' || i.status === 'starting';
    const html = running
      ? `<div class="w-row"><button class="w-btn stop" data-w="stop">${I('stop', 'sm')}停止</button><button class="w-btn" data-w="restart">${I('restart', 'sm')}重启</button></div>`
      : `<button class="w-btn go" data-w="start" ${i.installing || i.incomplete || i.status === 'stopping' ? 'disabled' : ''}>${I('play', 'sm')}${i.status === 'stopping' ? '停止中' : '启动'}</button>`;
    const b = $('#wi-btns'); if (b._h !== html) { b.innerHTML = html; b._h = html; }
  };
  main.onclick = async (e) => {
    const b = e.target.closest('[data-w]'); if (!b) return;
    const act = b.dataset.w;
    b.disabled = true;
    try {
      if (act === 'save') { await POST(`/api/instances/${enc(id)}/command`, { command: 'save-all' }); toast('已保存'); }
      else if (act === 'backup') { const r = await POST(`/api/instances/${enc(id)}/backups`); toast('备份中', 'info'); waitTask(r.task); }
      else if (act === 'stop' && !(await confirmDlg('停止服务器？', { danger: true, ok: '停止' }))) { /* cancelled */ }
      else {
        const r = await POST(`/api/instances/${enc(id)}/${act}`);
        if (r && r.java_installing) { toast(`安装 Java ${r.java_installing}`, 'info'); waitTask(r.task); }
      }
    } catch (err) {
      if (err.message === 'EULA_REQUIRED') toast('请先在手机或电脑上同意 EULA', 'err'); else fail(err);
    }
    b.disabled = false;
  };
  render();
  // tail of the console
  let ws, closed = false;
  const log = $('#wi-log');
  const push = (ls) => { for (const l of ls) lines.push(stripAnsi(l).replace(/^(\[[^\]]*\]\s*)+:?\s*/, '')); while (lines.length > 6) lines.shift(); log.textContent = lines.join('\n') || '无输出'; };
  const connect = () => {
    ws = new WebSocket(`${location.protocol === 'https:' ? 'wss' : 'ws'}://${location.host}/api/instances/${enc(id)}/console`);
    ws.onmessage = (e) => { const m = JSON.parse(e.data); push(m.lines.slice(-6)); };
    ws.onclose = () => { if (!closed) setTimeout(connect, 2000); };
  };
  connect();
  return { onInstances: render, destroy: () => { closed = true; ws && ws.close(); main.onclick = null; } };
}

boot();

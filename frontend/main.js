(async function () {
  const { invoke } = window.__TAURI__.core;

  // ── Defaults ────────────────────────────────────────────────────────────────
  const DEFAULTS = {
    activeIdx: 0,
    profiles: [
      { name: 'Monitor A', native: { w: 2560, h: 1440 }, custom: { w: 1920, h: 1080 }, hz: 240 },
      { name: 'Monitor B', native: { w: 3840, h: 2160 }, custom: { w: 1920, h: 1080 }, hz: 144 },
    ],
  };

  // ── State ────────────────────────────────────────────────────────────────────
  let state = structuredClone(DEFAULTS);
  let savedSnapshot = structuredClone(DEFAULTS);

  // ── DOM refs ─────────────────────────────────────────────────────────────────
  const tabs       = document.querySelectorAll('.tab');
  const tabLabels  = document.querySelectorAll('.tab-label');
  const tabSpecs   = document.querySelectorAll('[data-tab-spec]');
  const nw         = document.getElementById('nw');
  const nh         = document.getElementById('nh');
  const cw         = document.getElementById('cw');
  const ch         = document.getElementById('ch');
  const hz         = document.getElementById('hz');
  const nativeTag  = document.querySelector('[data-kind="native"] [data-tag]');
  const customTag  = document.querySelector('[data-kind="custom"] [data-tag]');
  const saveBtn    = document.getElementById('save');
  const statusEl   = document.getElementById('status');
  const statusText = document.getElementById('status-text');
  const toast      = document.getElementById('toast');
  const toastVal   = document.getElementById('toast-val');

  // ── Window controls ──────────────────────────────────────────────────────────
  document.getElementById('btn-minimize').addEventListener('click', () => invoke('minimize_window'));
  document.getElementById('btn-close').addEventListener('click', () => invoke('close_window'));

  // ── Helpers ──────────────────────────────────────────────────────────────────
  function renderTabs() {
    tabs.forEach((t, i) => {
      t.classList.toggle('active', i === state.activeIdx);
      const p = state.profiles[i];
      tabLabels[i].textContent = p.name;
      tabSpecs[i].textContent  = `${p.native.w}×${p.native.h} · ${p.hz}Hz`;
    });
  }

  function renderForm() {
    const p = state.profiles[state.activeIdx];
    nw.value = p.native.w;
    nh.value = p.native.h;
    cw.value = p.custom.w;
    ch.value = p.custom.h;
    hz.value = p.hz;
    updateTags();
    updateStatus();
    validate();
  }

  function updateTags() {
    nativeTag.textContent = `${+nw.value || 0}×${+nh.value || 0}`;
    customTag.textContent = `${+cw.value || 0}×${+ch.value || 0}`;
  }

  function readForm() {
    return {
      name:   state.profiles[state.activeIdx].name,
      native: { w: +nw.value, h: +nh.value },
      custom: { w: +cw.value, h: +ch.value },
      hz:     +hz.value,
    };
  }

  function isDirty() {
    const f = readForm();
    const s = savedSnapshot.profiles[state.activeIdx];
    return (
      f.name       !== s.name       ||
      f.native.w   !== s.native.w   || f.native.h !== s.native.h ||
      f.custom.w   !== s.custom.w   || f.custom.h !== s.custom.h ||
      f.hz         !== s.hz
    );
  }

  function isValid() {
    const f = readForm();
    return (
      f.native.w > 0 && f.native.h > 0 &&
      f.custom.w > 0 && f.custom.h > 0 &&
      f.hz       > 0 &&
      f.name.trim().length > 0
    );
  }

  function validate() {
    saveBtn.disabled = !(isValid() && isDirty());
  }

  function updateStatus() {
    const p = state.profiles[state.activeIdx];
    statusText.textContent = `${p.name} · ${p.native.w}×${p.native.h} @ ${p.hz}Hz`;
  }

  // ── Tab switching ─────────────────────────────────────────────────────────────
  tabs.forEach((t, i) => {
    t.addEventListener('click', (e) => {
      if (e.target.closest('.tab-label')) return;
      if (i === state.activeIdx) return;
      state.activeIdx = i;
      renderForm();
      renderTabs();
    });
  });

  // ── Editable labels ───────────────────────────────────────────────────────────
  tabLabels.forEach((el, i) => {
    el.addEventListener('click', (e) => e.stopPropagation());
    el.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') { e.preventDefault(); el.blur(); }
    });
    el.addEventListener('blur', () => {
      const fallback = i === 0 ? 'Monitor A' : 'Monitor B';
      const txt = el.textContent.trim() || fallback;
      el.textContent = txt;
      state.profiles[i].name = txt;
      if (i === state.activeIdx) validate();
    });
  });

  // ── Inputs ───────────────────────────────────────────────────────────────────
  [nw, nh, cw, ch, hz].forEach(input => {
    input.addEventListener('input', () => { updateTags(); validate(); });
    input.addEventListener('focus', () => input.select());
  });

  // ── Save ──────────────────────────────────────────────────────────────────────
  async function save() {
    if (!(isValid() && isDirty())) return;
    const f = readForm();
    state.profiles[state.activeIdx] = f;
    try {
      await invoke('save_config', { config: state });
      savedSnapshot = structuredClone(state);
      renderTabs();
      updateStatus();
      statusEl.classList.add('ok');
      validate();
      toastVal.textContent = `${f.name} · ${f.native.w}×${f.native.h} @ ${f.hz}Hz`;
      toast.classList.add('show');
      clearTimeout(save._t);
      save._t = setTimeout(() => toast.classList.remove('show'), 1800);
    } catch (err) {
      statusText.textContent = 'Save failed';
      console.error(err);
    }
  }

  saveBtn.addEventListener('click', save);
  document.addEventListener('keydown', (e) => {
    if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') save();
    else if (e.key === 'Enter' && document.activeElement.tagName === 'INPUT') save();
  });

  // ── Init ──────────────────────────────────────────────────────────────────────
  try {
    const loaded = await invoke('load_config');
    if (loaded && Array.isArray(loaded.profiles) && loaded.profiles.length === 2) {
      state         = loaded;
      savedSnapshot = structuredClone(loaded);
    }
  } catch (err) {
    console.warn('Could not load config, using defaults:', err);
  }

  renderTabs();
  renderForm();
  statusEl.classList.add('ok');
})();

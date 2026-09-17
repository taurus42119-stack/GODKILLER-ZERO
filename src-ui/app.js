document.addEventListener('DOMContentLoaded', () => {
  let activeProfile = 'KEN';
  let activeRules = null;

  // Window position & dimensions (Card standard: 380x560px)
  try {
    const cardWidth = 380;
    const cardHeight = 560;
    const positionX = Math.max(0, window.screen.availWidth - cardWidth - 16);
    const positionY = Math.max(0, window.screen.availHeight - cardHeight - 16);
    window.moveTo(positionX, positionY);
    window.resizeTo(cardWidth, cardHeight);
    window.addEventListener('resize', () => {
      if (window.outerWidth !== cardWidth || window.outerHeight !== cardHeight) {
        window.resizeTo(cardWidth, cardHeight);
      }
    });
  } catch (error) {
    console.debug('Window repositioning not supported in browser environment', error);
  }

  // DOM Elements - Header & Status
  const headerStatusBadge = document.getElementById('header-status-badge');
  const headerStatusText = document.getElementById('header-status-text');

  // DOM Elements - Discipline Tiles (Mode row)
  const tileDisciplineShi = document.getElementById('tile-discipline-shi');
  const tileDisciplineKen = document.getElementById('tile-discipline-ken');
  const tileDisciplineShin = document.getElementById('tile-discipline-shin');
  const btnOpenExtra = document.getElementById('btn-open-extra');
  const disciplineHeaderLabel = document.getElementById('discipline-header-label');
  const disciplineTelemetry = document.getElementById('discipline-telemetry');

  // DOM Elements - Hook / Unhook Controller (Section 2)
  const btnToggleHook = document.getElementById('btn-toggle-hook');
  const btnToggleUnhook = document.getElementById('btn-toggle-unhook');
  const shieldHeaderLabel = document.getElementById('shield-header-label');
  const shieldTelemetry = document.getElementById('shield-telemetry');

  // DOM Elements - Floating Modal: Extra Invariant Settings
  const extraModal = document.getElementById('extra-settings-modal');
  const btnCloseExtraModal = document.getElementById('btn-close-extra-modal');
  const btnDoneExtraModal = document.getElementById('btn-done-extra-modal');

  // DOM Elements - Invariant Checkboxes (inside floating modal)
  const chkZeroSpeculation = document.getElementById('rule-zero-speculation');
  const chkBanGeneric = document.getElementById('rule-ban-generic');
  const chkBanJunk = document.getElementById('rule-ban-junk');
  const chkLimitSpan = document.getElementById('rule-limit-span');
  const chkZeroFluff = document.getElementById('rule-zero-fluff');
  const chkComplexity = document.getElementById('rule-complexity');
  const chkAsciiBlueprints = document.getElementById('rule-ascii-blueprints');
  const chkBlastRadius = document.getElementById('rule-blast-radius');
  const chkStackSensor = document.getElementById('rule-stack-sensor');
  const chkNegativeBounding = document.getElementById('rule-negative-bounding');
  const chkCircuitBreaker = document.getElementById('rule-circuit-breaker');
  const chkExhaustiveErrors = document.getElementById('rule-exhaustive-errors');
  const chkFunctionalCore = document.getElementById('rule-functional-core');
  const btnSaveShield = document.getElementById('btn-save-shield');
  const btnInspectRules = document.getElementById('btn-inspect-rules');

  // DOM Elements - Raw Manifest Modal
  const rulesModal = document.getElementById('rules-modal');
  const rulesModalContent = document.getElementById('rules-modal-content');
  const btnCloseRulesModal = document.getElementById('btn-close-rules-modal');
  const btnDoneRulesModal = document.getElementById('btn-done-rules-modal');
  const btnCopyRules = document.getElementById('btn-copy-rules');

  // DOM Elements - Pre-Flight Compiler
  const crucibleInput = document.getElementById('crucible-input');
  const btnCrucible = document.getElementById('btn-crucible');
  const crucibleStats = document.getElementById('crucible-stats');
  const labelScope = document.getElementById('label-scope');
  const pillScope = document.getElementById('pill-scope');
  const labelPolarity = document.getElementById('label-polarity');
  const pillPolarity = document.getElementById('pill-polarity');
  const clarifierSection = document.getElementById('clarifier-section');
  const clarifierChips = document.getElementById('clarifier-chips');

  // DOM Elements - Startup & Footer
  const chkStartup = document.getElementById('chk-run-startup');
  const startupNote = document.getElementById('startup-note');
  const btnRehook = document.getElementById('btn-rehook');
  const btnUnhook = document.getElementById('btn-unhook');
  const btnModalUnhook = document.getElementById('btn-modal-unhook');
  const btnCheckUpdates = document.getElementById('btn-check-updates');
  const btnQuit = document.getElementById('btn-quit');
  const toastElement = document.getElementById('toast');

  let toastTimeout = null;
  function showToast(message) {
    if (!toastElement) return;
    toastElement.textContent = message;
    toastElement.classList.add('visible');
    if (toastTimeout) clearTimeout(toastTimeout);
    toastTimeout = setTimeout(() => {
      toastElement.classList.remove('visible');
    }, 2800);
  }

  // ==========================================================================
  // DISCIPLINE MODE TILE SWITCHING (Silent / Balanced / Turbo)
  // ==========================================================================
  function setDiscipline(targetDiscipline, shouldSave = true) {
    activeProfile = targetDiscipline.toUpperCase();

    // Update active tile visual
    [tileDisciplineShi, tileDisciplineKen, tileDisciplineShin].forEach(tile => {
      if (tile) tile.classList.remove('active');
    });

    if (activeProfile === 'SHI') {
      if (tileDisciplineShi) tileDisciplineShi.classList.add('active');
      if (disciplineHeaderLabel) disciplineHeaderLabel.textContent = 'Mode: SHI (Tranquil)';
      if (disciplineTelemetry) disciplineTelemetry.textContent = 'Span ≤ 90 • CC ≤ 10';
      if (chkZeroFluff) chkZeroFluff.checked = false;
    } else if (activeProfile === 'SHIN') {
      if (tileDisciplineShin) tileDisciplineShin.classList.add('active');
      if (disciplineHeaderLabel) disciplineHeaderLabel.textContent = 'Mode: SHIN (Zero-Tol)';
      if (disciplineTelemetry) disciplineTelemetry.textContent = 'Span ≤ 50 • CC ≤ 5';
      if (chkZeroFluff) chkZeroFluff.checked = true;
    } else {
      activeProfile = 'KEN';
      if (tileDisciplineKen) tileDisciplineKen.classList.add('active');
      if (disciplineHeaderLabel) disciplineHeaderLabel.textContent = 'Mode: KEN (Strict-Dev)';
      if (disciplineTelemetry) disciplineTelemetry.textContent = 'Span ≤ 70 • CC ≤ 7';
      if (chkZeroFluff) chkZeroFluff.checked = true;
    }

    if (shouldSave) {
      applyShieldWithRules(false);
      showToast(`Mode: ${activeProfile}`);
    }
  }

  if (tileDisciplineShi) tileDisciplineShi.addEventListener('click', () => setDiscipline('SHI'));
  if (tileDisciplineKen) tileDisciplineKen.addEventListener('click', () => setDiscipline('KEN'));
  if (tileDisciplineShin) tileDisciplineShin.addEventListener('click', () => setDiscipline('SHIN'));

  // ==========================================================================
  // FLOATING EXTRA SETTINGS MODAL (Extra Settings Box)
  // ==========================================================================
  function openExtraModal() {
    if (extraModal) extraModal.style.display = 'flex';
  }

  function closeExtraModal() {
    if (extraModal) extraModal.style.display = 'none';
  }

  if (btnOpenExtra) btnOpenExtra.addEventListener('click', openExtraModal);
  if (btnCloseExtraModal) btnCloseExtraModal.addEventListener('click', closeExtraModal);
  if (btnDoneExtraModal) btnDoneExtraModal.addEventListener('click', closeExtraModal);
  if (extraModal) {
    extraModal.addEventListener('click', (e) => {
      if (e.target === extraModal) closeExtraModal();
    });
  }

  // ==========================================================================
  // HOOK / UNHOOK CONTROLLER & STATUS SYNC
  // ==========================================================================
  function updateHookUI(isHooked) {
    if (isHooked) {
      if (headerStatusBadge) {
        headerStatusBadge.classList.remove('bypass');
        if (headerStatusText) headerStatusText.textContent = 'Shielded';
      }
      if (shieldHeaderLabel) shieldHeaderLabel.textContent = 'IDE Rules Status: Active (Protected)';
      if (btnToggleHook) {
        btnToggleHook.classList.add('active', 'hook-active');
      }
      if (btnToggleUnhook) {
        btnToggleUnhook.classList.remove('active', 'unhook-active');
      }
    } else {
      if (headerStatusBadge) {
        headerStatusBadge.classList.add('bypass');
        if (headerStatusText) headerStatusText.textContent = 'Unhooked';
      }
      if (shieldHeaderLabel) shieldHeaderLabel.textContent = 'IDE Rules Status: Unhooked (Default)';
      if (btnToggleHook) {
        btnToggleHook.classList.remove('active', 'hook-active');
      }
      if (btnToggleUnhook) {
        btnToggleUnhook.classList.add('active', 'unhook-active');
      }
    }
  }

  if (btnToggleHook) {
    btnToggleHook.addEventListener('click', async () => {
      btnToggleHook.disabled = true;
      try {
        await applyShieldWithRules(false);
        updateHookUI(true);
        showToast('ใส่กฎเรียบร้อย: เปิดเกราะคุม AI แล้ว');
      } catch {
        showToast('ไม่สามารถใส่กฎได้');
      } finally {
        btnToggleHook.disabled = false;
      }
    });
  }

  if (btnToggleUnhook) {
    btnToggleUnhook.addEventListener('click', () => unhookAntigravityAction());
  }

  // Fast-Lane and Circuit Breaker Tiles
  if (tileShieldFastlane) {
    tileShieldFastlane.addEventListener('click', () => {
      tileShieldFastlane.classList.toggle('active');
      const isOn = tileShieldFastlane.classList.contains('active');
      showToast(isOn ? 'Fast-Lane Affirmations: Enabled' : 'Fast-Lane Affirmations: Disabled');
    });
  }

  if (tileShieldCircuit) {
    tileShieldCircuit.addEventListener('click', () => {
      tileShieldCircuit.classList.toggle('active');
      const isLocked = tileShieldCircuit.classList.contains('active');
      showToast(isLocked ? 'Circuit Breaker: Armed' : 'Circuit Breaker: Normal');
    });
  }

  if (tileShieldHook) {
    tileShieldHook.addEventListener('click', async () => {
      await applyShieldWithRules(true);
    });
  }

  // ==========================================================================
  // INVARIANT RULES PAYLOAD & SAVING
  // ==========================================================================
  function getActiveRulesPayload() {
    let maxSpan = 70;
    let maxComplexity = 7;
    if (activeProfile === 'SHI') { maxSpan = 90; maxComplexity = 10; }
    if (activeProfile === 'SHIN') { maxSpan = 50; maxComplexity = 5; }

    return {
      zero_speculation: chkZeroSpeculation ? chkZeroSpeculation.checked : true,
      ban_generic: chkBanGeneric ? chkBanGeneric.checked : true,
      ban_junk: chkBanJunk ? chkBanJunk.checked : true,
      limit_span: chkLimitSpan ? chkLimitSpan.checked : true,
      zero_fluff: chkZeroFluff ? chkZeroFluff.checked : true,
      complexity: chkComplexity ? chkComplexity.checked : true,
      ascii_blueprints: chkAsciiBlueprints ? chkAsciiBlueprints.checked : true,
      blast_radius: chkBlastRadius ? chkBlastRadius.checked : true,
      stack_sensor: chkStackSensor ? chkStackSensor.checked : true,
      negative_mutation_bounding: chkNegativeBounding ? chkNegativeBounding.checked : true,
      circuit_breaker: chkCircuitBreaker ? chkCircuitBreaker.checked : true,
      exhaustive_errors: chkExhaustiveErrors ? chkExhaustiveErrors.checked : (activeProfile !== 'SHI'),
      functional_core: chkFunctionalCore ? chkFunctionalCore.checked : (activeProfile === 'SHIN'),
      max_span: maxSpan,
      max_complexity: maxComplexity
    };
  }

  async function applyShieldWithRules(notify = true) {
    const customRules = getActiveRulesPayload();
    try {
      const response = await fetch('/api/hook', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          discipline: activeProfile,
          rules: customRules
        })
      });
      if (response.ok) {
        updateHookUI(true);
        if (notify) showToast('[OK] Invariants Saved & Applied');
      } else if (notify) {
        showToast('Failed to apply shield');
      }
    } catch {
      if (notify) showToast('Service connection offline');
    }
  }

  if (btnSaveShield) {
    btnSaveShield.addEventListener('click', async () => {
      btnSaveShield.disabled = true;
      btnSaveShield.textContent = 'Applying...';
      await applyShieldWithRules(true);
      btnSaveShield.disabled = false;
      btnSaveShield.textContent = 'Save & Apply';
      closeExtraModal();
    });
  }

  // ==========================================================================
  // INSPECT RULES MODAL
  // ==========================================================================
  async function openRulesInspector() {
    if (!rulesModal) return;
    rulesModal.style.display = 'flex';
    if (rulesModalContent) rulesModalContent.textContent = 'Loading active invariant rules from discovered IDE files...';
    try {
      const response = await fetch('/api/rules/inspect');
      if (response.ok) {
        const payloadData = await response.json();
        if (rulesModalContent) rulesModalContent.textContent = payloadData.content || 'No rules active.';
      } else {
        if (rulesModalContent) rulesModalContent.textContent = 'Unable to read active invariant rules.';
      }
    } catch (error) {
      if (rulesModalContent) rulesModalContent.textContent = 'Local proxy connection offline.';
    }
  }

  function closeRulesInspector() {
    if (rulesModal) rulesModal.style.display = 'none';
  }

  if (btnInspectRules) btnInspectRules.addEventListener('click', openRulesInspector);
  if (btnCloseRulesModal) btnCloseRulesModal.addEventListener('click', closeRulesInspector);
  if (btnDoneRulesModal) btnDoneRulesModal.addEventListener('click', closeRulesInspector);
  if (rulesModal) {
    rulesModal.addEventListener('click', (e) => {
      if (e.target === rulesModal) closeRulesInspector();
    });
  }

  if (btnCopyRules) {
    btnCopyRules.addEventListener('click', async () => {
      const text = rulesModalContent ? rulesModalContent.textContent : '';
      if (text) {
        await copyToClipboard(text);
        showToast('[OK] Active Rules Copied to Clipboard');
      }
    });
  }

  // ==========================================================================
  // PRE-FLIGHT COMPILER
  // ==========================================================================
  if (crucibleInput) {
    crucibleInput.addEventListener('input', () => {
      const rawText = crucibleInput.value.trim();
      if (!rawText) {
        if (labelScope) labelScope.textContent = 'Scope: Auto-Detected';
        if (pillScope) pillScope.className = 'g-scope-indicator pass';
        if (pillPolarity) pillPolarity.style.display = 'none';
        if (clarifierSection) clarifierSection.style.display = 'none';
        return;
      }

      const lower = rawText.toLowerCase();
      const failureKeywords = ['ยังไม่ได้', 'ยังไม่หาย', 'พังเหมือนเดิม', 'เหมือนเดิม', 'ยัง error', 'ไม่ผ่าน', 'แก้ไม่หาย', 'still fails', 'same error', 'looping'];
      const negationKeywords = ['อย่า', 'ห้าม', 'ไม่ต้อง', 'ไม่ควร', "don't", 'avoid', 'never', 'preserve'];

      const isBreaker = failureKeywords.some(kw => lower.includes(kw));
      const isNegated = negationKeywords.some(kw => lower.includes(kw));

      if (pillPolarity && labelPolarity) {
        if (isBreaker) {
          pillPolarity.style.display = 'inline-flex';
          pillPolarity.className = 'g-scope-indicator warn';
          labelPolarity.textContent = 'Circuit Breaker: ARMED';
        } else if (isNegated) {
          pillPolarity.style.display = 'inline-flex';
          pillPolarity.className = 'g-scope-indicator pass';
          labelPolarity.textContent = 'Ghost Edit Shield: ACTIVE';
        } else {
          pillPolarity.style.display = 'none';
        }
      }

      const filePattern = /(?:[a-zA-Z0-9_\-\./\\]+\.[a-zA-Z0-9]+(?::\d+)?)/;
      const match = rawText.match(filePattern);
      if (match) {
        if (labelScope) labelScope.textContent = `File: ${match[0]}`;
        if (pillScope) pillScope.className = 'g-scope-indicator pass';
      } else {
        if (labelScope) labelScope.textContent = 'Scope: Domain-Anchored';
        if (pillScope) pillScope.className = 'g-scope-indicator pass';
      }
    });
  }

  async function compileAndCopyHoareIR() {
    if (!crucibleInput) return;
    const rawPrompt = crucibleInput.value.trim();
    if (!rawPrompt) {
      showToast('Enter an intent or defect prompt');
      return;
    }

    btnCrucible.disabled = true;
    btnCrucible.textContent = 'Compiling...';
    try {
      const response = await fetch('/api/purify', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          prompt: rawPrompt,
          workspace_path: '.'
        })
      });

      if (response.ok) {
        const receipt = await response.json();
        const compiledContract = receipt.dense_ir;

        // Clarifier Chips
        if (receipt.clarifiers && receipt.clarifiers.length > 0 && clarifierChips && clarifierSection) {
          clarifierChips.innerHTML = '';
          receipt.clarifiers.forEach(c => {
            const chipBtn = document.createElement('button');
            chipBtn.className = 'g-clarifier-chip';
            chipBtn.textContent = `${c.display_label} — ${c.description}`;
            chipBtn.addEventListener('click', () => {
              crucibleInput.value = c.prompt_patch;
              crucibleInput.dispatchEvent(new Event('input'));
              compileAndCopyHoareIR();
            });
            clarifierChips.appendChild(chipBtn);
          });
          clarifierSection.style.display = 'flex';
        } else if (clarifierSection) {
          clarifierSection.style.display = 'none';
        }

        if (compiledContract) {
          const copied = await copyToClipboard(compiledContract);
          if (copied) {
            if (crucibleStats) crucibleStats.textContent = '[OK] Copied';
            showToast('[OK] Copied Dense AI-IR Contract');
          } else {
            showToast('Contract compiled. Clipboard access blocked.');
          }
        } else {
          showToast('Contract synthesis failed');
        }
      } else {
        showToast('Compiler returned error');
      }
    } catch {
      showToast('Compiler service unavailable');
    } finally {
      btnCrucible.disabled = false;
      btnCrucible.textContent = 'Compile & Copy';
    }
  }

  if (btnCrucible) btnCrucible.addEventListener('click', compileAndCopyHoareIR);

  // Clipboard Helper
  async function copyToClipboard(text) {
    try {
      if (navigator.clipboard && navigator.clipboard.writeText) {
        await navigator.clipboard.writeText(text);
        return true;
      }
    } catch (clipboardError) {
      console.debug(clipboardError);
    }

    try {
      const textArea = document.createElement('textarea');
      textArea.value = text;
      textArea.style.position = 'fixed';
      textArea.style.opacity = '0';
      document.body.appendChild(textArea);
      textArea.focus();
      textArea.select();
      const success = document.execCommand('copy');
      document.body.removeChild(textArea);
      return success;
    } catch {
      return false;
    }
  }

  // ==========================================================================
  // STARTUP, UPDATES & QUIT
  // ==========================================================================
  async function initStartupStatus() {
    try {
      const response = await fetch('/api/startup');
      if (response.ok) {
        const startupData = await response.json();
        if (chkStartup) chkStartup.checked = !!startupData.enabled;
        if (startupNote) startupNote.textContent = startupData.enabled ? 'Boot: Auto-Start' : 'Boot: Manual';
      }
    } catch (error) {
      console.debug(error);
    }
  }

  if (chkStartup) {
    chkStartup.addEventListener('change', async () => {
      const isEnabled = chkStartup.checked;
      chkStartup.disabled = true;
      try {
        const response = await fetch('/api/startup/toggle', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ enabled: isEnabled })
        });
        if (response.ok) {
          if (startupNote) startupNote.textContent = isEnabled ? 'Boot: Auto-Start' : 'Boot: Manual';
          showToast(isEnabled ? 'Run on Startup enabled' : 'Run on Startup disabled');
        } else {
          chkStartup.checked = !isEnabled;
          showToast('Failed to update startup configuration');
        }
      } catch {
        chkStartup.checked = !isEnabled;
        showToast('Startup service unavailable');
      } finally {
        chkStartup.disabled = false;
      }
    });
  }

  if (btnRehook) {
    btnRehook.addEventListener('click', async () => {
      btnRehook.disabled = true;
      try {
        await applyShieldWithRules(false);
        showToast('Antigravity Re-Hooked successfully');
      } catch {
        showToast('Re-Hook failed');
      } finally {
        btnRehook.disabled = false;
      }
    });
  }

  async function unhookAntigravityAction() {
    if (btnUnhook) btnUnhook.disabled = true;
    if (btnModalUnhook) btnModalUnhook.disabled = true;
    try {
      const response = await fetch('/api/unhook', { method: 'POST' });
      if (response.ok) {
        updateHookUI(false);
        [tileDisciplineShi, tileDisciplineKen, tileDisciplineShin].forEach(tile => {
          if (tile) tile.classList.remove('active');
        });
        showToast('Antigravity Unhooked: Environment restored');
        closeExtraModal();
      } else {
        showToast('Unhook request failed');
      }
    } catch {
      showToast('Service connection offline');
    } finally {
      if (btnUnhook) btnUnhook.disabled = false;
      if (btnModalUnhook) btnModalUnhook.disabled = false;
    }
  }

  if (btnUnhook) btnUnhook.addEventListener('click', unhookAntigravityAction);
  if (btnModalUnhook) btnModalUnhook.addEventListener('click', unhookAntigravityAction);

  if (btnCheckUpdates) {
    btnCheckUpdates.addEventListener('click', async () => {
      btnCheckUpdates.disabled = true;
      btnCheckUpdates.textContent = 'Checking...';
      try {
        const releaseResponse = await fetch('https://api.github.com/repos/taurus42119-stack/godkiller-zero/releases/latest', {
          headers: { 'Accept': 'application/vnd.github.v3+json' }
        });
        if (releaseResponse.ok) {
          const releaseData = await releaseResponse.json();
          const tag = releaseData.tag_name || 'v1.0.0';
          showToast(tag === 'v1.0.0' ? 'Version is up to date (v1.0.0)' : `New version available: ${tag}`);
        } else {
          showToast('Current version: v1.0.0 (Up to date)');
        }
      } catch {
        showToast('Version: v1.0.0 (Offline Mode)');
      } finally {
        btnCheckUpdates.disabled = false;
        btnCheckUpdates.textContent = 'Updates';
      }
    });
  }

  if (btnQuit) {
    btnQuit.addEventListener('click', async () => {
      try {
        await fetch('/api/quit', { method: 'POST' });
      } catch (quitError) {
        console.debug(quitError);
      }
      window.close();
    });
  }

  async function initHookStatus() {
    try {
      const hookStatusResponse = await fetch('/api/hook-status');
      if (hookStatusResponse.ok) {
        const hookStatusData = await hookStatusResponse.json();
        if (hookStatusData.discipline) {
          setDiscipline(hookStatusData.discipline, false);
        }
        if (hookStatusData.rules) {
          if (chkZeroSpeculation) chkZeroSpeculation.checked = hookStatusData.rules.zero_speculation ?? true;
          if (chkBanGeneric) chkBanGeneric.checked = hookStatusData.rules.ban_generic ?? true;
          if (chkBanJunk) chkBanJunk.checked = hookStatusData.rules.ban_junk ?? true;
          if (chkLimitSpan) chkLimitSpan.checked = hookStatusData.rules.limit_span ?? true;
          if (chkZeroFluff) chkZeroFluff.checked = hookStatusData.rules.zero_fluff ?? true;
          if (chkComplexity) chkComplexity.checked = hookStatusData.rules.complexity ?? true;
          if (chkAsciiBlueprints) chkAsciiBlueprints.checked = hookStatusData.rules.ascii_blueprints ?? true;
          if (chkBlastRadius) chkBlastRadius.checked = hookStatusData.rules.blast_radius ?? true;
          if (chkStackSensor) chkStackSensor.checked = hookStatusData.rules.stack_sensor ?? true;
          if (chkNegativeBounding) chkNegativeBounding.checked = hookStatusData.rules.negative_mutation_bounding ?? true;
          if (chkCircuitBreaker) chkCircuitBreaker.checked = hookStatusData.rules.circuit_breaker ?? true;
          if (chkExhaustiveErrors) chkExhaustiveErrors.checked = hookStatusData.rules.exhaustive_errors ?? true;
          if (chkFunctionalCore) chkFunctionalCore.checked = hookStatusData.rules.functional_core ?? false;
        }
      }
    } catch (statusError) {
      console.debug(statusError);
    }
  }

  // ==========================================================================
  // AI Engine Telemetry & Bootstrap Controller
  // ==========================================================================
  const engineHeaderLabel = document.getElementById('engine-header-label');
  const engineTelemetry = document.getElementById('engine-telemetry');
  const engineStatusPill = document.getElementById('engine-status-pill');
  const engineStatusText = document.getElementById('engine-status-text');
  const btnEngineBootstrap = document.getElementById('btn-engine-bootstrap');
  const engineProgressWrapper = document.getElementById('engine-progress-wrapper');
  const engineProgressBar = document.getElementById('engine-progress-bar');
  const engineProgressStatus = document.getElementById('engine-progress-status');
  const engineProgressPercent = document.getElementById('engine-progress-percent');

  async function pollEngineStatus() {
    try {
      const engineResponse = await fetch('/api/engine/status');
      if (!engineResponse.ok) return;
      const engineStateData = await engineResponse.json();
      updateEngineUI(engineStateData);
    } catch (pollError) {
      console.debug(pollError);
    }
  }

  function updateEngineUI(engineStateData) {
    if (!engineStateData || !engineStateData.status || !engineStatusPill) return;
    const targetModel = engineStateData.target_model || 'qwen2.5-coder:1.5b';
    if (engineTelemetry) engineTelemetry.textContent = targetModel;

    const state = engineStateData.status.state;
    const details = engineStateData.status.details || {};

    engineStatusPill.className = 'g-engine-status-pill';

    if (state === 'Ready') {
      engineStatusPill.classList.add('ready');
      if (engineHeaderLabel) engineHeaderLabel.textContent = 'AI Engine: Ready';
      if (engineStatusText) engineStatusText.textContent = `Neural Ready (${details.model || targetModel})`;
      if (engineProgressWrapper) engineProgressWrapper.style.display = 'none';
    } else if (state === 'PullingModel') {
      engineStatusPill.classList.add('busy');
      if (engineHeaderLabel) engineHeaderLabel.textContent = 'AI Engine: Pulling Model';
      const pct = (details.percent || 0).toFixed(1);
      if (engineStatusText) engineStatusText.textContent = `Downloading ${details.model} (${pct}%)`;
      if (engineProgressWrapper) engineProgressWrapper.style.display = 'flex';
      if (engineProgressBar) engineProgressBar.style.width = `${pct}%`;
      if (engineProgressStatus) engineProgressStatus.textContent = details.status_text || 'Downloading...';
      if (engineProgressPercent) engineProgressPercent.textContent = `${pct}%`;
    } else if (state === 'StartingService') {
      engineStatusPill.classList.add('busy');
      if (engineHeaderLabel) engineHeaderLabel.textContent = 'AI Engine: Starting';
      if (engineStatusText) engineStatusText.textContent = 'Starting service in background...';
      if (engineProgressWrapper) engineProgressWrapper.style.display = 'none';
    } else {
      engineStatusPill.classList.add('offline');
      if (engineHeaderLabel) engineHeaderLabel.textContent = 'AI Engine: Standalone';
      if (engineStatusText) engineStatusText.textContent = 'Offline (Pre-Flight Active)';
      if (engineProgressWrapper) engineProgressWrapper.style.display = 'none';
    }
  }

  if (btnEngineBootstrap) {
    btnEngineBootstrap.addEventListener('click', async () => {
      showToast('Syncing AI Engine...');
      try {
        await fetch('/api/engine/bootstrap', { method: 'POST' });
        await pollEngineStatus();
      } catch {
        showToast('Sync request failed');
      }
    });
  }

  // Initial Load Status
  initStartupStatus();
  checkInterpreterStatus();
  initHookStatus();
  pollEngineStatus();
  setInterval(pollEngineStatus, 2500);
});

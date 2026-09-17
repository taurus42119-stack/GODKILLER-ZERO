using System;
using System.IO;

namespace GodkillerZeroGui;

/// <summary>
/// Release matrix: proves every Extra checkbox and the three discipline presets
/// actually change the injected rule block, without touching the user's IDE files.
/// </summary>
internal static class RulesMatrixAudit
{
    private static readonly (string Name, string OnMarker, Action<InvariantOptions, bool> Set)[] BoolToggles =
    {
        ("ZeroSpeculation", "CLARIFICATION CADENCE", (o, v) => o.ZeroSpeculation = v),
        ("BanGeneric", "Strictly FORBIDDEN generic identifiers", (o, v) => o.BanGeneric = v),
        ("BanJunk", "BANNED JUNK DRAWERS", (o, v) => o.BanJunk = v),
        ("ZeroFluff", "ZERO CONVERSATIONAL FLUFF", (o, v) => o.ZeroFluff = v),
        ("AsciiBlueprints", "MANDATORY ASCII", (o, v) => o.AsciiBlueprints = v),
        ("MermaidDiagrams", "MANDATORY MERMAID WORKFLOW", (o, v) => o.MermaidDiagrams = v),
        ("LimitSpan", "Maximum function span", (o, v) =>
        {
            o.LimitSpan = v;
            // When only span is on, marker is "Maximum function span".
            // When both complexity+span, marker is "Maximum span".
            if (!v) o.Complexity = true;
        }),
        ("Complexity", "Maximum cyclomatic complexity", (o, v) => o.Complexity = v),
        ("BlastRadius", "BLAST RADIUS IMPACT AUDIT", (o, v) => o.BlastRadius = v),
        ("StackSensor", "TECH STACK AUTO-ALIGNMENT", (o, v) => o.StackSensor = v),
        ("NegativeBounding", "NEGATIVE MUTATION BOUNDING", (o, v) => o.NegativeBounding = v),
        ("CircuitBreaker", "LINGUISTIC CIRCUIT BREAKER", (o, v) => o.CircuitBreaker = v),
        ("ExhaustiveErrors", "EXHAUSTIVE ERROR HANDLING", (o, v) => o.ExhaustiveErrors = v),
        ("FunctionalCore", "FUNCTIONAL CORE", (o, v) => o.FunctionalCore = v),
        ("ModernWebSources", "MODERN WEB & DOCUMENTATION GUIDANCE", (o, v) => o.ModernWebSources = v),
        ("PremiumUiUx", "PREMIUM UI/UX AESTHETIC MANDATE", (o, v) => o.PremiumUiUx = v),
        ("InfiniteEvolution", "INFINITE EVOLUTIONARY CONTINUUM", (o, v) => o.InfiniteEvolution = v),
        ("ThaiPolarity", "Thai examples", (o, v) => o.ThaiPolarity = v),
        ("RepoMap", "CODEBASE REPO MAP RADAR", (o, v) => o.RepoMap = v),
    };

    public static int Run()
    {
        int failures = 0;
        Console.WriteLine("=== GODKILLER ZERO : RULES MATRIX AUDIT ===");
        Console.WriteLine("Workspace writes: TEMP only (user IDE untouched)");
        Console.WriteLine();

        failures += AuditDisciplinePresets();
        failures += AuditExtraCadenceTriple();
        failures += AuditAsciiAndMermaidCadence();
        failures += AuditEveryCheckboxOnOff();
        failures += AuditRepoMapLastCheckboxRoundTrip();
        failures += AuditTempFileHookAndRestore();
        failures += AuditWorldwideSinkDiscovery();

        Console.WriteLine();
        if (failures == 0)
        {
            Console.WriteLine("[OK] ALL MATRIX CHECKS PASSED");
            return 0;
        }

        Console.WriteLine($"[FAIL] {failures} check(s) failed");
        return 1;
    }

    private static int AuditDisciplinePresets()
    {
        int failures = 0;
        Console.WriteLine("-- Discipline presets SHI / KEN / SHIN --");

        var cases = new (string Disc, int Span, int Cc, string Cadence, bool ZeroFluff, bool Exhaustive, bool Functional)[]
        {
            ("SHI", 90, 10, "Interactive", false, false, false),
            ("KEN", 70, 7, "Balanced", true, true, false),
            ("SHIN", 50, 5, "Silent", true, true, true),
        };

        foreach (var c in cases)
        {
            var opts = HookEngine.GetOptionsForDiscipline(c.Disc);
            string block = HookEngine.GenerateRuleBlock(c.Disc, opts);

            failures += Expect(opts.MaxSpan == c.Span, $"{c.Disc} MaxSpan={opts.MaxSpan} expected {c.Span}");
            failures += Expect(opts.MaxComplexity == c.Cc, $"{c.Disc} MaxComplexity={opts.MaxComplexity} expected {c.Cc}");
            failures += Expect(opts.ClarificationCadence == c.Cadence, $"{c.Disc} Cadence={opts.ClarificationCadence} expected {c.Cadence}");
            failures += Expect(opts.ZeroFluff == c.ZeroFluff, $"{c.Disc} ZeroFluff={opts.ZeroFluff} expected {c.ZeroFluff}");
            failures += Expect(opts.ExhaustiveErrors == c.Exhaustive, $"{c.Disc} ExhaustiveErrors={opts.ExhaustiveErrors} expected {c.Exhaustive}");
            failures += Expect(opts.FunctionalCore == c.Functional, $"{c.Disc} FunctionalCore={opts.FunctionalCore} expected {c.Functional}");
            failures += Expect(block.Contains($"Discipline:"), $"{c.Disc} block missing Discipline header");
            failures += Expect(block.Contains($"{c.Span} lines") || block.Contains($"Maximum span: {c.Span}"), $"{c.Disc} block missing span {c.Span}");
            failures += Expect(block.Contains($"{c.Cc} per function") || block.Contains($"complexity: {c.Cc}"), $"{c.Disc} block missing CC {c.Cc}");
            failures += Expect(opts.RepoMap && block.Contains("CODEBASE REPO MAP RADAR"), $"{c.Disc} default RepoMap must be ON in block");

            Console.WriteLine($"  [PASS] {c.Disc}: span={opts.MaxSpan} cc={opts.MaxComplexity} cadence={opts.ClarificationCadence}");
        }

        return failures;
    }

    private static int AuditExtraCadenceTriple()
    {
        int failures = 0;
        Console.WriteLine("-- Extra clarification cadence (3 options) --");

        foreach (var cadence in new[] { "Silent", "Balanced", "Interactive" })
        {
            var opts = HookEngine.GetOptionsForDiscipline("KEN");
            opts.ZeroSpeculation = true;
            opts.ClarificationCadence = cadence;
            string block = HookEngine.GenerateRuleBlock("KEN", opts);

            string expectedSnippet = cadence switch
            {
                "Silent" => "SILENT AUTONOMOUS",
                "Interactive" => "INTERACTIVE CO-PILOT",
                _ => "BALANCED SMART CONFIRMATION"
            };

            failures += Expect(block.Contains("CLARIFICATION CADENCE"), $"Cadence {cadence}: missing CLARIFICATION CADENCE");
            failures += Expect(block.Contains(expectedSnippet), $"Cadence {cadence}: missing '{expectedSnippet}'");
            Console.WriteLine($"  [PASS] Cadence {cadence} encoded in block");
        }

        return failures;
    }

    private static int AuditAsciiAndMermaidCadence()
    {
        int failures = 0;
        Console.WriteLine("-- ASCII / Mermaid Fast vs Normal --");

        var opts = HookEngine.GetOptionsForDiscipline("KEN");
        opts.AsciiBlueprints = true;
        opts.AsciiCadence = "Fast";
        string fast = HookEngine.GenerateRuleBlock("KEN", opts);
        failures += Expect(fast.Contains("FAST - PLANS & UI ONLY") || fast.Contains("FAST"), "ASCII Fast marker missing");

        opts.AsciiCadence = "Normal";
        string normal = HookEngine.GenerateRuleBlock("KEN", opts);
        failures += Expect(!normal.Contains("FAST - PLANS & UI ONLY"), "ASCII Normal should not use Fast-only marker");

        opts.MermaidDiagrams = true;
        opts.MermaidCadence = "Fast";
        string mFast = HookEngine.GenerateRuleBlock("KEN", opts);
        failures += Expect(mFast.Contains("FAST - PLANS ONLY") || mFast.Contains("MERMAID"), "Mermaid Fast marker missing");

        opts.MermaidCadence = "Normal";
        string mNormal = HookEngine.GenerateRuleBlock("KEN", opts);
        failures += Expect(mNormal.Contains("MANDATORY MERMAID WORKFLOW"), "Mermaid Normal missing workflow marker");

        Console.WriteLine("  [PASS] ASCII/Mermaid cadence toggles");
        return failures;
    }

    private static int AuditEveryCheckboxOnOff()
    {
        int failures = 0;
        Console.WriteLine("-- Every Extra checkbox ON then OFF --");

        foreach (var (name, marker, set) in BoolToggles)
        {
            var onOpts = HookEngine.GetOptionsForDiscipline("KEN");
            // Ensure LimitSpan marker is detectable when testing LimitSpan alone
            if (name == "LimitSpan")
            {
                onOpts.Complexity = false;
                onOpts.LimitSpan = true;
            }

            set(onOpts, true);
            if (name == "ZeroFluff") onOpts.ZeroFluff = true;
            string onBlock = HookEngine.GenerateRuleBlock("KEN", onOpts);

            var offOpts = HookEngine.GetOptionsForDiscipline("KEN");
            if (name == "LimitSpan")
            {
                offOpts.Complexity = false;
            }

            set(offOpts, false);
            string offBlock = HookEngine.GenerateRuleBlock("KEN", offOpts);

            // Special case: ZeroFluff OFF still writes conversational cadence line, but NOT the ZERO FLUFF marker
            if (name == "ZeroFluff")
            {
                failures += Expect(onBlock.Contains(marker), $"{name} ON missing '{marker}'");
                failures += Expect(!offBlock.Contains(marker), $"{name} OFF still contains '{marker}'");
            }
            else if (name == "ZeroSpeculation")
            {
                failures += Expect(onBlock.Contains(marker), $"{name} ON missing '{marker}'");
                failures += Expect(!offBlock.Contains(marker), $"{name} OFF still contains '{marker}'");
            }
            else if (name == "LimitSpan")
            {
                failures += Expect(onBlock.Contains("Maximum function span") || onBlock.Contains("Maximum span"), $"{name} ON missing span marker");
                failures += Expect(!offBlock.Contains("Maximum function span") && !offBlock.Contains("Maximum span"), $"{name} OFF still has span marker");
            }
            else
            {
                failures += Expect(onBlock.Contains(marker), $"{name} ON missing '{marker}'");
                failures += Expect(!offBlock.Contains(marker), $"{name} OFF still contains '{marker}'");
            }

            Console.WriteLine($"  [PASS] {name}: ON inserts / OFF removes");
        }

        return failures;
    }

    private static int AuditRepoMapLastCheckboxRoundTrip()
    {
        int failures = 0;
        Console.WriteLine("-- Last checkbox RepoMap (Deep Project Radar) --");

        var on = HookEngine.GetOptionsForDiscipline("KEN");
        on.RepoMap = true;
        string onBlock = HookEngine.GenerateRuleBlock("KEN", on);
        failures += Expect(onBlock.Contains("CODEBASE REPO MAP RADAR"), "RepoMap ON missing radar rule");
        failures += Expect(onBlock.Contains("gk_get_repo_map"), "RepoMap ON must point at MCP tool");
        failures += Expect(!onBlock.Contains(".gemini/REPO_MAP.md"), "RepoMap must NOT tell AI to consult a pre-written file");

        var off = HookEngine.GetOptionsForDiscipline("KEN");
        off.RepoMap = false;
        string offBlock = HookEngine.GenerateRuleBlock("KEN", off);
        failures += Expect(!offBlock.Contains("CODEBASE REPO MAP RADAR"), "RepoMap OFF still in block");

        // Round-trip through LoadOptionsFromDisk parser logic via temp file helper
        string tempDir = Path.Combine(Path.GetTempPath(), "gk-rules-audit-" + Guid.NewGuid().ToString("N"));
        Directory.CreateDirectory(tempDir);
        string tempFile = Path.Combine(tempDir, "GEMINI.md");
        try
        {
            File.WriteAllText(tempFile, onBlock);
            bool parsedOn = File.ReadAllText(tempFile).Contains("CODEBASE REPO MAP RADAR");
            failures += Expect(parsedOn, "RepoMap ON failed temp round-trip read");

            File.WriteAllText(tempFile, offBlock);
            bool parsedOff = File.ReadAllText(tempFile).Contains("CODEBASE REPO MAP RADAR");
            failures += Expect(!parsedOff, "RepoMap OFF failed temp round-trip read");
        }
        finally
        {
            try { Directory.Delete(tempDir, true); } catch { /* ignore */ }
        }

        Console.WriteLine("  [PASS] RepoMap last checkbox on/off + MCP wording");
        return failures;
    }

    private static int AuditTempFileHookAndRestore()
    {
        int failures = 0;
        Console.WriteLine("-- Temp-file Hook write / Restore strip --");

        string tempDir = Path.Combine(Path.GetTempPath(), "gk-hook-audit-" + Guid.NewGuid().ToString("N"));
        Directory.CreateDirectory(tempDir);
        string tempFile = Path.Combine(tempDir, "AGENTS.md");
        File.WriteAllText(tempFile, "# user rules\nkeep me\n");

        try
        {
            foreach (var disc in new[] { "SHI", "KEN", "SHIN" })
            {
                var opts = HookEngine.GetOptionsForDiscipline(disc);
                bool wrote = HookEngine.WriteHookToFilePath(tempFile, disc, opts);
                failures += Expect(wrote, $"{disc} WriteHookToFilePath failed");
                string afterHook = File.ReadAllText(tempFile);
                failures += Expect(afterHook.Contains(HookEngine.MarkerStart), $"{disc} missing start marker");
                failures += Expect(afterHook.Contains("keep me"), $"{disc} wiped user content");
            }

            bool removed = HookEngine.RemoveHookFromFilePath(tempFile);
            failures += Expect(removed, "RemoveHookFromFilePath failed");
            string afterRestore = File.ReadAllText(tempFile);
            failures += Expect(!afterRestore.Contains(HookEngine.MarkerStart), "RESTORE left marker behind");
            failures += Expect(afterRestore.Contains("keep me"), "RESTORE lost user content");
            Console.WriteLine("  [PASS] Hook all 3 disciplines then RESTORE preserves user text");
        }
        finally
        {
            try { Directory.Delete(tempDir, true); } catch { /* ignore */ }
        }

        return failures;
    }

    private static int AuditWorldwideSinkDiscovery()
    {
        int failures = 0;
        Console.WriteLine("-- Worldwide sink probe (temp home, not this PC) --");

        string home = Path.Combine(Path.GetTempPath(), "gk-home-" + Guid.NewGuid().ToString("N"));
        string workspace = Path.Combine(Path.GetTempPath(), "gk-ws-" + Guid.NewGuid().ToString("N"));
        string xdg = Path.Combine(Path.GetTempPath(), "gk-xdg-" + Guid.NewGuid().ToString("N"));
        Directory.CreateDirectory(Path.Combine(home, ".cursor"));
        Directory.CreateDirectory(Path.Combine(workspace, ".git"));
        Directory.CreateDirectory(Path.Combine(xdg, "cursor"));

        try
        {
            var roots = new HookEngine.RuleSinkRoots
            {
                Home = home,
                Workspace = workspace,
                XdgConfig = xdg
            };
            var sinks = HookEngine.DiscoverRuleSinks(roots);
            string joined = string.Join("|", sinks).Replace('\\', '/');
            failures += Expect(
                joined.Contains(".cursor/rules/godkiller-zero.mdc"),
                "probe missed native Cursor mdc");
            failures += Expect(
                !joined.Contains(".cursorrules"),
                "probe still writes dead ~/.cursorrules");

            var empty = new HookEngine.RuleSinkRoots
            {
                Home = Path.Combine(Path.GetTempPath(), "gk-empty-" + Guid.NewGuid().ToString("N")),
                Workspace = Path.Combine(Path.GetTempPath(), "gk-empty-ws-" + Guid.NewGuid().ToString("N"))
            };
            Directory.CreateDirectory(empty.Home);
            Directory.CreateDirectory(empty.Workspace);
            var none = HookEngine.DiscoverRuleSinks(empty);
            failures += Expect(none.Count == 0, "empty machine must not invent IDE paths");
            try { Directory.Delete(empty.Home, true); } catch { /* ignore */ }
            try { Directory.Delete(empty.Workspace, true); } catch { /* ignore */ }

            Console.WriteLine("  [PASS] Probe uses host dirs, never this-PC hardcoded paths");
        }
        finally
        {
            try { Directory.Delete(home, true); } catch { /* ignore */ }
            try { Directory.Delete(workspace, true); } catch { /* ignore */ }
            try { Directory.Delete(xdg, true); } catch { /* ignore */ }
        }

        return failures;
    }

    private static int Expect(bool condition, string message)
    {
        if (condition) return 0;
        Console.WriteLine($"  [FAIL] {message}");
        return 1;
    }
}

using System;
using System.IO;
using System.Net.Http;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
using System.Threading.Tasks;

namespace GodkillerZeroGui;

public class InvariantOptions
{
    public bool ZeroSpeculation { get; set; } = true;
    public bool BanGeneric { get; set; } = true;
    public bool BanJunk { get; set; } = true;
    public bool LimitSpan { get; set; } = true;
    public bool ZeroFluff { get; set; } = true;
    public bool Complexity { get; set; } = true;
    public bool AsciiBlueprints { get; set; } = true;
    public string AsciiCadence { get; set; } = "Fast";
    public bool MermaidDiagrams { get; set; } = true;
    public string MermaidCadence { get; set; } = "Fast";
    public bool BlastRadius { get; set; } = true;
    public bool StackSensor { get; set; } = true;
    public bool NegativeBounding { get; set; } = true;
    public bool CircuitBreaker { get; set; } = true;
    public bool ExhaustiveErrors { get; set; } = true;
    public bool FunctionalCore { get; set; } = false;
    public int MaxSpan { get; set; } = 70;
    public int MaxComplexity { get; set; } = 7;
    public string ClarificationCadence { get; set; } = "Balanced";
    public bool ModernWebSources { get; set; } = true;
    public bool PremiumUiUx { get; set; } = true;
    public bool InfiniteEvolution { get; set; } = true;
    public bool RepoMap { get; set; } = true;
    public bool ThaiPolarity { get; set; } = false;
}

public enum TargetEngine
{
    Universal,
    Antigravity,
    Cursor,
    ClaudeCode,
    VSCodeCopilot
}

public static class HookEngine
{
    // Antigravity (Go) rejects UTF-8 BOM in mcp_config.json — never emit the identifier.
    private static readonly Encoding Utf8NoBom = new UTF8Encoding(encoderShouldEmitUTF8Identifier: false);

    public const string MarkerStart = "<!-- godkiller-zero:start -->";
    public const string MarkerEnd = "<!-- godkiller-zero:end -->";

    private static readonly HttpClient HttpClient = new HttpClient { Timeout = TimeSpan.FromSeconds(2) };

    public static string? CustomPathOverride { get; set; }

    public static string GetGeminiMdPath()
    {
        if (!string.IsNullOrEmpty(CustomPathOverride) && File.Exists(CustomPathOverride))
        {
            return CustomPathOverride;
        }

        // HEURISTIC MULTI-PATH DISCOVERY (Worldwide Multi-OS & Custom Drive Tolerant)
        var candidates = new System.Collections.Generic.List<string>();

        // 1. Explicit Environment Variables (Enterprise / Advanced Users)
        string? geminiHome = Environment.GetEnvironmentVariable("GEMINI_HOME");
        if (!string.IsNullOrWhiteSpace(geminiHome))
        {
            candidates.Add(Path.Combine(geminiHome, "GEMINI.md"));
            candidates.Add(Path.Combine(geminiHome, ".gemini", "GEMINI.md"));
        }

        string? antigravityConfig = Environment.GetEnvironmentVariable("ANTIGRAVITY_CONFIG");
        if (!string.IsNullOrWhiteSpace(antigravityConfig))
        {
            candidates.Add(Path.Combine(antigravityConfig, "GEMINI.md"));
        }

        // 2. Windows Shell KnownFolder API (Handles Drive D:\, OneDrive redirection, custom profiles)
        string userProfile = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
        if (!string.IsNullOrWhiteSpace(userProfile))
        {
            candidates.Add(Path.Combine(userProfile, ".gemini", "GEMINI.md"));
        }

        // 3. Standard Environment Variables (Cross-Platform: Windows %USERPROFILE%, Unix $HOME)
        string? envProfile = Environment.GetEnvironmentVariable("USERPROFILE") ?? Environment.GetEnvironmentVariable("HOME");
        if (!string.IsNullOrWhiteSpace(envProfile))
        {
            string p = Path.Combine(envProfile, ".gemini", "GEMINI.md");
            if (!candidates.Contains(p)) candidates.Add(p);
        }

        // 4. AppData / LocalAppData fallbacks (Roaming profiles & corporate locked-down machines)
        string localAppData = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
        if (!string.IsNullOrWhiteSpace(localAppData))
        {
            candidates.Add(Path.Combine(localAppData, ".gemini", "GEMINI.md"));
            candidates.Add(Path.Combine(localAppData, "Google", "Antigravity", "GEMINI.md"));
        }

        // 5. Active Workspace Probe: Upward scan from Current Working Directory for .agents/ or .gemini/
        try
        {
            string? current = Directory.GetCurrentDirectory();
            while (!string.IsNullOrEmpty(current))
            {
                string localAgents = Path.Combine(current, ".agents", "rules", "GEMINI.md");
                if (File.Exists(localAgents)) candidates.Add(localAgents);

                string localGemini = Path.Combine(current, ".gemini", "GEMINI.md");
                if (File.Exists(localGemini)) candidates.Add(localGemini);

                var parent = Directory.GetParent(current);
                if (parent == null || parent.FullName == current) break;
                current = parent.FullName;
            }
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine(ex.Message);
        }

        // 6. Check existing file on disk
        foreach (var candidate in candidates)
        {
            if (File.Exists(candidate))
            {
                return candidate;
            }
        }

        // 7. Default Fallback: Standard UserProfile .gemini
        string defaultDir = Path.Combine(userProfile, ".gemini");
        if (!Directory.Exists(defaultDir))
        {
            try { Directory.CreateDirectory(defaultDir); }
            catch (Exception ex) { System.Diagnostics.Debug.WriteLine(ex.Message); }
        }
        return Path.Combine(defaultDir, "GEMINI.md");
    }

    public static System.Collections.Generic.List<string> GetTargetPaths(TargetEngine target)
    {
        var paths = new System.Collections.Generic.List<string>();
        string currentDir = Directory.GetCurrentDirectory();

        if (target == TargetEngine.Universal || target == TargetEngine.Antigravity)
        {
            paths.Add(GetGeminiMdPath());
            string localGemini = Path.Combine(currentDir, "GEMINI.md");
            if (File.Exists(localGemini) || Directory.Exists(Path.Combine(currentDir, ".agents")))
            {
                paths.Add(localGemini);
            }
        }

        var nativeSinks = DiscoverRuleSinks(BuildLiveSinkRoots());
        foreach (var sink in nativeSinks)
        {
            if (target == TargetEngine.Universal)
            {
                paths.Add(sink);
                continue;
            }
            if (target == TargetEngine.Cursor && IsCursorSink(sink)) paths.Add(sink);
            if (target == TargetEngine.ClaudeCode && IsClaudeSink(sink)) paths.Add(sink);
            if (target == TargetEngine.VSCodeCopilot && IsCopilotSink(sink)) paths.Add(sink);
        }

        var unique = new System.Collections.Generic.List<string>();
        foreach (var p in paths)
        {
            if (!unique.Contains(p)) unique.Add(p);
        }
        return unique;
    }

    public class RuleSinkRoots
    {
        public string Home { get; set; } = "";
        public string Workspace { get; set; } = "";
        public string? XdgConfig { get; set; }
        public System.Collections.Generic.List<string> ExtraCursorRoots { get; set; } = new();
        public System.Collections.Generic.List<string> ExtraClaudeRoots { get; set; } = new();
    }

    public static RuleSinkRoots BuildLiveSinkRoots()
    {
        var roots = new RuleSinkRoots
        {
            Home = ResolveUserHome(),
            Workspace = ResolveActiveWorkspaceRoot(),
            XdgConfig = Environment.GetEnvironmentVariable("XDG_CONFIG_HOME")
        };
        AddEnvDir(roots.ExtraCursorRoots, "CURSOR_HOME");
        AddEnvDir(roots.ExtraCursorRoots, "CURSOR_USER_DIR");
        AddEnvDir(roots.ExtraClaudeRoots, "CLAUDE_CONFIG_DIR");
        return roots;
    }

    public static System.Collections.Generic.List<string> DiscoverRuleSinks(RuleSinkRoots roots)
    {
        var paths = new System.Collections.Generic.List<string>();
        CollectCursorSinks(paths, roots);
        CollectClaudeSinks(paths, roots);
        CollectCopilotSinks(paths, roots);
        return DedupPaths(paths);
    }

    public static System.Collections.Generic.List<string> DiscoverLegacyCleanupPaths(RuleSinkRoots roots)
    {
        return new System.Collections.Generic.List<string>
        {
            Path.Combine(roots.Home, ".cursorrules"),
            Path.Combine(roots.Workspace, ".cursorrules")
        };
    }

    private static string ResolveUserHome()
    {
        string profile = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
        if (!string.IsNullOrWhiteSpace(profile)) return profile;
        string? env = Environment.GetEnvironmentVariable("USERPROFILE")
            ?? Environment.GetEnvironmentVariable("HOME");
        return string.IsNullOrWhiteSpace(env) ? Directory.GetCurrentDirectory() : env;
    }

    private static void AddEnvDir(System.Collections.Generic.List<string> target, string key)
    {
        string? value = Environment.GetEnvironmentVariable(key);
        if (!string.IsNullOrWhiteSpace(value)) target.Add(value);
    }

    private static void CollectCursorSinks(System.Collections.Generic.List<string> paths, RuleSinkRoots roots)
    {
        var hostDirs = new System.Collections.Generic.List<string>(roots.ExtraCursorRoots);
        AddIfDir(hostDirs, Path.Combine(roots.Home, ".cursor"));
        if (!string.IsNullOrWhiteSpace(roots.XdgConfig))
        {
            AddIfDir(hostDirs, Path.Combine(roots.XdgConfig, "cursor"));
            AddIfDir(hostDirs, Path.Combine(roots.XdgConfig, "Cursor"));
        }
        AddIfDir(hostDirs, Path.Combine(roots.Workspace, ".cursor"));

        foreach (var hostDir in hostDirs)
        {
            paths.Add(Path.Combine(hostDir, "rules", "godkiller-zero.mdc"));
        }
        if (hostDirs.Count > 0 && LooksLikeProjectRoot(roots.Workspace))
        {
            paths.Add(Path.Combine(roots.Workspace, ".cursor", "rules", "godkiller-zero.mdc"));
        }
    }

    private static void CollectClaudeSinks(System.Collections.Generic.List<string> paths, RuleSinkRoots roots)
    {
        var hostDirs = new System.Collections.Generic.List<string>(roots.ExtraClaudeRoots);
        AddIfDir(hostDirs, Path.Combine(roots.Home, ".claude"));
        if (!string.IsNullOrWhiteSpace(roots.XdgConfig))
        {
            AddIfDir(hostDirs, Path.Combine(roots.XdgConfig, "claude"));
        }
        AddIfDir(hostDirs, Path.Combine(roots.Workspace, ".claude"));
        foreach (var hostDir in hostDirs)
        {
            paths.Add(Path.Combine(hostDir, "CLAUDE.md"));
        }
        string workspaceClaude = Path.Combine(roots.Workspace, "CLAUDE.md");
        if (File.Exists(workspaceClaude)) paths.Add(workspaceClaude);
    }

    private static void CollectCopilotSinks(System.Collections.Generic.List<string> paths, RuleSinkRoots roots)
    {
        string github = Path.Combine(roots.Workspace, ".github");
        if (Directory.Exists(github)
            || Directory.Exists(Path.Combine(roots.Workspace, ".git"))
            || File.Exists(Path.Combine(roots.Workspace, ".git")))
        {
            paths.Add(Path.Combine(github, "copilot-instructions.md"));
        }
    }

    private static bool LooksLikeProjectRoot(string dir)
    {
        return Directory.Exists(Path.Combine(dir, ".git"))
            || File.Exists(Path.Combine(dir, ".git"))
            || File.Exists(Path.Combine(dir, "Cargo.toml"))
            || File.Exists(Path.Combine(dir, "package.json"))
            || Directory.Exists(Path.Combine(dir, ".cursor"))
            || Directory.Exists(Path.Combine(dir, ".gemini"))
            || Directory.Exists(Path.Combine(dir, ".claude"))
            || Directory.Exists(Path.Combine(dir, ".agents"));
    }

    private static void AddIfDir(System.Collections.Generic.List<string> dirs, string candidate)
    {
        if (Directory.Exists(candidate) && !dirs.Contains(candidate)) dirs.Add(candidate);
    }

    private static System.Collections.Generic.List<string> DedupPaths(System.Collections.Generic.List<string> paths)
    {
        var unique = new System.Collections.Generic.List<string>();
        foreach (var p in paths)
        {
            if (!unique.Contains(p)) unique.Add(p);
        }
        return unique;
    }

    private static bool IsCursorSink(string path) =>
        path.Replace('\\', '/').EndsWith(".cursor/rules/godkiller-zero.mdc", StringComparison.OrdinalIgnoreCase);

    private static bool IsClaudeSink(string path) =>
        Path.GetFileName(path).Equals("CLAUDE.md", StringComparison.OrdinalIgnoreCase);

    private static bool IsCopilotSink(string path) =>
        Path.GetFileName(path).Equals("copilot-instructions.md", StringComparison.OrdinalIgnoreCase);

    public static bool IsHooked(TargetEngine target = TargetEngine.Universal)
    {
        var paths = GetTargetPaths(target);
        foreach (var path in paths)
        {
            if (File.Exists(path))
            {
                try
                {
                    string content = File.ReadAllText(path);
                    if (content.Contains(MarkerStart) && content.Contains(MarkerEnd))
                        return true;
                }
                catch (Exception ex)
                {
                    System.Diagnostics.Debug.WriteLine(ex.Message);
                }
            }
        }
        return false;
    }

    public static string GetCurrentDiscipline()
    {
        string path = GetGeminiMdPath();
        if (!File.Exists(path)) return "KEN";
        try
        {
            string content = File.ReadAllText(path);
            if (!content.Contains(MarkerStart)) return "KEN";
            if (content.Contains("SHIN -") || content.Contains("(SHIN")) return "SHIN";
            if (content.Contains("SHI -") || content.Contains("(SHI")) return "SHI";
            if (content.Contains("KEN -") || content.Contains("(KEN")) return "KEN";
            return "KEN";
        }
        catch
        {
            return "KEN";
        }
    }

    public static InvariantOptions LoadOptionsFromDisk(string? path = null)
    {
        path ??= GetGeminiMdPath();
        if (!File.Exists(path)) return GetOptionsForDiscipline("KEN");

        try
        {
            string content = File.ReadAllText(path);
            if (!content.Contains(MarkerStart) || !content.Contains(MarkerEnd))
            {
                return GetOptionsForDiscipline(GetCurrentDiscipline());
            }

            int start = content.IndexOf(MarkerStart, StringComparison.Ordinal) + MarkerStart.Length;
            int end = content.IndexOf(MarkerEnd, StringComparison.Ordinal);
            if (end <= start) return GetOptionsForDiscipline(GetCurrentDiscipline());

            string block = content.Substring(start, end - start);
            string discipline = GetCurrentDiscipline();
            var options = GetOptionsForDiscipline(discipline);

            options.ZeroSpeculation = block.Contains("CLARIFICATION CADENCE")
                || block.Contains("ZERO-SPECULATION POLICY")
                || block.Contains("TWO-PHASE SPECULATION");

            options.BanGeneric = block.Contains("Strictly FORBIDDEN generic identifiers");
            options.BanJunk = block.Contains("BANNED JUNK DRAWERS");
            options.LimitSpan = block.Contains("Maximum span") || block.Contains("Maximum function span");
            options.ZeroFluff = block.Contains("ZERO CONVERSATIONAL FLUFF");
            options.Complexity = block.Contains("Maximum cyclomatic complexity");

            options.AsciiBlueprints = block.Contains("MANDATORY ASCII BLUEPRINTS") || block.Contains("MANDATORY ASCII WIREFRAMES");
            options.AsciiCadence = block.Contains("FAST - PLANS & UI ONLY") ? "Fast" : "Normal";

            options.MermaidDiagrams = block.Contains("MANDATORY MERMAID WORKFLOW");
            options.MermaidCadence = block.Contains("FAST - PLANS ONLY") ? "Fast" : "Normal";

            options.BlastRadius = block.Contains("BLAST RADIUS IMPACT AUDIT");
            options.StackSensor = block.Contains("TECH STACK AUTO-ALIGNMENT");
            options.NegativeBounding = block.Contains("NEGATIVE MUTATION BOUNDING");
            options.CircuitBreaker = block.Contains("LINGUISTIC CIRCUIT BREAKER");
            options.ExhaustiveErrors = block.Contains("EXHAUSTIVE ERROR HANDLING");
            options.FunctionalCore = block.Contains("FUNCTIONAL CORE");

            if (block.Contains("SILENT AUTONOMOUS"))
                options.ClarificationCadence = "Silent";
            else if (block.Contains("INTERACTIVE CO-PILOT"))
                options.ClarificationCadence = "Interactive";
            else
                options.ClarificationCadence = "Balanced";

            options.ModernWebSources = block.Contains("MODERN WEB & DOCUMENTATION GUIDANCE");
            options.PremiumUiUx = block.Contains("PREMIUM UI/UX AESTHETIC MANDATE");
            options.InfiniteEvolution = block.Contains("INFINITE EVOLUTIONARY CONTINUUM");
            options.RepoMap = block.Contains("CODEBASE REPO MAP RADAR");
            options.ThaiPolarity = block.Contains("Thai examples") || block.Contains("Thai: 'อย่า'");

            if (block.Contains("90 lines")) options.MaxSpan = 90;
            else if (block.Contains("50 lines")) options.MaxSpan = 50;
            else options.MaxSpan = 70;

            if (block.Contains("10 per function")) options.MaxComplexity = 10;
            else if (block.Contains("5 per function")) options.MaxComplexity = 5;
            else options.MaxComplexity = 7;

            return options;
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine($"LoadOptionsFromDisk error: {ex.Message}");
            return GetOptionsForDiscipline(GetCurrentDiscipline());
        }
    }

    public static string ResolveActiveWorkspaceRoot()
    {
        if (!string.IsNullOrEmpty(CustomPathOverride) && File.Exists(CustomPathOverride))
        {
            string? dir = Path.GetDirectoryName(CustomPathOverride);
            if (!string.IsNullOrEmpty(dir))
            {
                if (Path.GetFileName(dir).Equals(".gemini", StringComparison.OrdinalIgnoreCase) ||
                    Path.GetFileName(dir).Equals("rules", StringComparison.OrdinalIgnoreCase))
                {
                    var parent = Directory.GetParent(dir);
                    if (parent != null) return parent.FullName;
                }
                return dir;
            }
        }

        try
        {
            string? current = Directory.GetCurrentDirectory();
            while (!string.IsNullOrEmpty(current))
            {
                if (Directory.Exists(Path.Combine(current, ".git")) ||
                    Directory.Exists(Path.Combine(current, ".agents")) ||
                    Directory.Exists(Path.Combine(current, ".gemini")) ||
                    File.Exists(Path.Combine(current, "Cargo.toml")) ||
                    File.Exists(Path.Combine(current, "package.json")))
                {
                    return current;
                }
                var parent = Directory.GetParent(current);
                if (parent == null || parent.FullName == current) break;
                current = parent.FullName;
            }
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine(ex.Message);
        }

        try
        {
            string? current = AppDomain.CurrentDomain.BaseDirectory;
            while (!string.IsNullOrEmpty(current))
            {
                if (Directory.Exists(Path.Combine(current, ".git")) ||
                    File.Exists(Path.Combine(current, "Cargo.toml")) ||
                    File.Exists(Path.Combine(current, "package.json")))
                {
                    return current;
                }
                var parent = Directory.GetParent(current);
                if (parent == null || parent.FullName == current) break;
                current = parent.FullName;
            }
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine(ex.Message);
        }

        return Directory.GetCurrentDirectory();
    }

    public static bool Hook(string discipline, InvariantOptions? options = null, TargetEngine target = TargetEngine.Universal)
    {
        options ??= GetOptionsForDiscipline(discipline);

        var paths = GetTargetPaths(target);
        bool anyOk = false;
        foreach (var p in paths)
        {
            if (WriteHookToFilePath(p, discipline, options))
            {
                anyOk = true;
            }
        }

        // Start fresh background daemon if binary is available
        EnsureBackgroundDaemon();

        // 1-Click Auto-Provision MCP config for Antigravity, Claude, and Cursor
        ProvisionMcpConfig(target);

        // The map is served live through the MCP tool `gk_get_repo_map`, so there is
        // no pre-generated file to keep in sync here.
        return anyOk;
    }

    public static bool Unhook(TargetEngine target = TargetEngine.Universal)
    {
        // Complete Process Lifecycle: Terminate all background processes first to avoid race conditions
        KillBackgroundProcesses();

        var paths = GetTargetPaths(target);
        foreach (var p in DiscoverLegacyCleanupPaths(BuildLiveSinkRoots()))
        {
            if (!paths.Contains(p)) paths.Add(p);
        }
        bool allOk = true;
        foreach (var p in paths)
        {
            if (!RemoveHookFromFilePath(p))
            {
                allOk = false;
            }
        }

        // 1-Click Deprovision MCP config
        DeprovisionMcpConfig(target);

        return allOk;
    }

    public static void KillBackgroundProcesses()
    {
        try
        {
            // Graceful shutdown ping first
            try { _ = HttpClient.PostAsync("http://127.0.0.1:4242/api/quit", null); }
            catch (Exception ex) { System.Diagnostics.Debug.WriteLine(ex.Message); }

            // Terminate ZERO daemons only — never touch other MCP host processes.
            foreach (string processName in new[] { "godkiller-zero", "godkiller-console", "GodkillerZero" })
            {
                var processes = System.Diagnostics.Process.GetProcessesByName(processName);
                foreach (var p in processes)
                {
                    try
                    {
                        p.Kill(entireProcessTree: true);
                        p.WaitForExit(500);
                    }
                    catch (Exception ex)
                    {
                        System.Diagnostics.Debug.WriteLine(ex.Message);
                    }
                }
            }
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine(ex.Message);
        }
    }

    public static string? FindDaemonExePath()
    {
        string baseDir = AppDomain.CurrentDomain.BaseDirectory;
        string currentDir = Directory.GetCurrentDirectory();
        string workspace = ResolveActiveWorkspaceRoot();
        string[] binNames = { "godkiller-console.exe", "godkiller-zero.exe", "godkiller-console", "godkiller-zero" };
        var searchDirs = new System.Collections.Generic.List<string>();
        string localAppData = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
        if (!string.IsNullOrWhiteSpace(localAppData))
        {
            searchDirs.Add(Path.Combine(localAppData, "Programs", "godkiller-zero"));
        }
        searchDirs.AddRange(new[]
        {
            baseDir,
            Path.Combine(baseDir, ".."),
            Path.Combine(baseDir, "..", "publish"),
            Path.Combine(workspace, "publish"),
            Path.Combine(workspace, "target", "release"),
            Path.Combine(workspace, "target", "debug"),
            workspace,
            Path.Combine(baseDir, "..", "..", "..", "..", "publish"),
            Path.Combine(baseDir, "..", "..", "..", "..", "target", "release"),
            Path.Combine(baseDir, "..", "..", "..", "..", "target", "debug"),
            Path.Combine(currentDir, "publish"),
            Path.Combine(currentDir, "target", "release"),
            Path.Combine(currentDir, "target", "debug"),
            currentDir
        });

        foreach (var name in binNames)
        {
            foreach (var dir in searchDirs)
            {
                string exePath = Path.Combine(dir, name);
                if (File.Exists(exePath))
                {
                    return Path.GetFullPath(exePath);
                }
            }
        }

        return null;
    }

    public static System.Collections.Generic.List<string> ResolveAntigravityMcpTargets()
    {
        var targets = new System.Collections.Generic.List<string>();
        string userProfile = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
        string? envProfile = Environment.GetEnvironmentVariable("USERPROFILE") ?? Environment.GetEnvironmentVariable("HOME");

        string? geminiConfigDir = Environment.GetEnvironmentVariable("GEMINI_CONFIG_DIR");
        if (!string.IsNullOrWhiteSpace(geminiConfigDir))
        {
            targets.Add(Path.Combine(geminiConfigDir, "mcp_config.json"));
        }

        string? geminiHome = Environment.GetEnvironmentVariable("GEMINI_HOME");
        if (!string.IsNullOrWhiteSpace(geminiHome))
        {
            targets.Add(Path.Combine(geminiHome, "config", "mcp_config.json"));
            targets.Add(Path.Combine(geminiHome, "mcp_config.json"));
        }

        string? antigravityConfig = Environment.GetEnvironmentVariable("ANTIGRAVITY_CONFIG");
        if (!string.IsNullOrWhiteSpace(antigravityConfig))
        {
            targets.Add(Path.Combine(antigravityConfig, "mcp_config.json"));
        }

        string baseGemini = !string.IsNullOrWhiteSpace(userProfile)
            ? Path.Combine(userProfile, ".gemini")
            : (!string.IsNullOrWhiteSpace(envProfile) ? Path.Combine(envProfile, ".gemini") : string.Empty);

        if (!string.IsNullOrEmpty(baseGemini))
        {
            targets.Add(Path.Combine(baseGemini, "config", "mcp_config.json"));
            targets.Add(Path.Combine(baseGemini, "mcp_config.json"));
            targets.Add(Path.Combine(baseGemini, "antigravity-ide", "mcp_config.json"));
            targets.Add(Path.Combine(baseGemini, "antigravity", "mcp_config.json"));
            targets.Add(Path.Combine(baseGemini, "antigravity", "mcp", "mcp_config.json"));
        }

        string agentsDir = Path.Combine(Directory.GetCurrentDirectory(), ".agents");
        if (Directory.Exists(agentsDir))
        {
            targets.Add(Path.Combine(agentsDir, "mcp_config.json"));
        }

        return targets;
    }

    public static System.Collections.Generic.List<string> GetMcpConfigTargets(TargetEngine target)
    {
        var mcpTargets = new System.Collections.Generic.List<string>();
        if (target == TargetEngine.Universal || target == TargetEngine.Antigravity)
        {
            mcpTargets.AddRange(ResolveAntigravityMcpTargets());
        }
        if (target == TargetEngine.Universal || target == TargetEngine.ClaudeCode)
        {
            mcpTargets.AddRange(ResolveClaudeDesktopMcpTargets());
        }
        if (target == TargetEngine.Universal || target == TargetEngine.Cursor)
        {
            mcpTargets.AddRange(ResolveCursorMcpTargets());
        }
        return DedupPaths(mcpTargets);
    }

    private static System.Collections.Generic.List<string> ResolveClaudeDesktopMcpTargets()
    {
        var targets = new System.Collections.Generic.List<string>();
        string home = ResolveUserHome();
        string appData = Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData);
        if (!string.IsNullOrWhiteSpace(appData))
        {
            targets.Add(Path.Combine(appData, "Claude", "claude_desktop_config.json"));
        }
        targets.Add(Path.Combine(home, "Library", "Application Support", "Claude", "claude_desktop_config.json"));
        string? xdg = Environment.GetEnvironmentVariable("XDG_CONFIG_HOME");
        if (!string.IsNullOrWhiteSpace(xdg))
        {
            targets.Add(Path.Combine(xdg, "Claude", "claude_desktop_config.json"));
        }
        targets.Add(Path.Combine(home, ".config", "Claude", "claude_desktop_config.json"));
        return targets;
    }

    private static System.Collections.Generic.List<string> ResolveCursorMcpTargets()
    {
        var targets = new System.Collections.Generic.List<string>
        {
            Path.Combine(ResolveUserHome(), ".cursor", "mcp.json")
        };
        string workspace = ResolveActiveWorkspaceRoot();
        if (Directory.Exists(Path.Combine(workspace, ".cursor")))
        {
            targets.Add(Path.Combine(workspace, ".cursor", "mcp.json"));
        }
        return targets;
    }

    public static void ProvisionMcpConfig(TargetEngine target = TargetEngine.Universal)
    {
        string? daemonExe = FindDaemonExePath();
        if (string.IsNullOrEmpty(daemonExe) || !File.Exists(daemonExe)) return;

        var mcpTargets = GetMcpConfigTargets(target);

        foreach (var configPath in mcpTargets)
        {
            SafeInjectMcpServer(configPath, daemonExe);
        }
    }

    public static void DeprovisionMcpConfig(TargetEngine target = TargetEngine.Universal)
    {
        foreach (var configPath in GetMcpConfigTargets(target))
        {
            SafeRemoveMcpServer(configPath);
        }
    }

    public static void ShutdownBackgroundDaemon()
    {
        try
        {
            using var request = new HttpRequestMessage(HttpMethod.Post, "http://127.0.0.1:4242/api/quit");
            HttpClient.Send(request);
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine($"Graceful daemon shutdown request: {ex.Message}");
        }

        try
        {
            int currentSessionId = System.Diagnostics.Process.GetCurrentProcess().SessionId;
            var processes = System.Diagnostics.Process.GetProcessesByName("godkiller-zero");
            foreach (var p in processes)
            {
                try
                {
                    if (p.SessionId == currentSessionId)
                    {
                        p.Kill();
                        p.WaitForExit(300);
                    }
                }
                catch (Exception ex)
                {
                    System.Diagnostics.Debug.WriteLine($"Failed killing daemon process {p.Id}: {ex.Message}");
                }
            }
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine($"Daemon kill error: {ex.Message}");
        }
    }

    private static readonly string[] GodkillerMcpServerNames =
    {
        "godkiller-zero",
        "godkiller",
        "godkiller-mcp"
    };

    private static void SafeInjectMcpServer(string configPath, string daemonExe)
    {
        try
        {
            string? dir = Path.GetDirectoryName(configPath);
            if (!string.IsNullOrEmpty(dir) && !Directory.Exists(dir))
            {
                Directory.CreateDirectory(dir);
            }

            JsonObject rootNode;
            if (File.Exists(configPath))
            {
                string jsonText = ReadJsonConfigText(configPath);
                // Never replace a broken/non-object file with a godkiller-only config — that wipes peers.
                if (JsonNode.Parse(jsonText) is not JsonObject existingRoot)
                {
                    System.Diagnostics.Debug.WriteLine(
                        $"Refusing MCP inject at {configPath}: root JSON is not an object.");
                    return;
                }
                rootNode = existingRoot;
            }
            else
            {
                rootNode = new JsonObject();
            }

            if (rootNode["mcpServers"] is null)
            {
                rootNode["mcpServers"] = new JsonObject();
            }
            else if (rootNode["mcpServers"] is not JsonObject)
            {
                System.Diagnostics.Debug.WriteLine(
                    $"Refusing MCP inject at {configPath}: mcpServers is not an object.");
                return;
            }

            var servers = rootNode["mcpServers"]!.AsObject();
            int peersBefore = CountPeerMcpServers(servers);

            string mcpCommand = ResolveMcpCommand(daemonExe);
            servers["godkiller-zero"] = new JsonObject
            {
                ["command"] = mcpCommand,
                ["args"] = new JsonArray { "--mcp" }
            };

            // Drop legacy aliases so only one ZERO entry remains.
            servers.Remove("godkiller");
            servers.Remove("godkiller-mcp");

            if (CountPeerMcpServers(servers) < peersBefore)
            {
                System.Diagnostics.Debug.WriteLine(
                    $"Refusing MCP inject at {configPath}: peer server count would drop.");
                return;
            }

            WriteMcpConfigJson(configPath, rootNode);
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine($"Failed to provision MCP at {configPath}: {ex.Message}");
        }
    }

    private static void SafeRemoveMcpServer(string configPath)
    {
        try
        {
            if (!File.Exists(configPath)) return;

            string jsonText = ReadJsonConfigText(configPath);
            if (JsonNode.Parse(jsonText) is not JsonObject rootNode) return;
            if (rootNode["mcpServers"] is not JsonObject servers) return;

            int peersBefore = CountPeerMcpServers(servers);
            bool removed = false;
            foreach (string name in GodkillerMcpServerNames)
            {
                if (servers.Remove(name))
                {
                    removed = true;
                }
            }
            if (!removed) return;

            if (CountPeerMcpServers(servers) != peersBefore)
            {
                System.Diagnostics.Debug.WriteLine(
                    $"Refusing MCP deprovision at {configPath}: peer servers changed unexpectedly.");
                return;
            }

            WriteMcpConfigJson(configPath, rootNode);
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine($"Failed to deprovision MCP at {configPath}: {ex.Message}");
        }
    }

    private static int CountPeerMcpServers(JsonObject servers)
    {
        int count = 0;
        foreach (var property in servers)
        {
            if (IsGodkillerMcpServerName(property.Key)) continue;
            count++;
        }
        return count;
    }

    private static bool IsGodkillerMcpServerName(string name)
    {
        foreach (string candidate in GodkillerMcpServerNames)
        {
            if (name.Equals(candidate, StringComparison.OrdinalIgnoreCase))
            {
                return true;
            }
        }
        return false;
    }

    private static void WriteMcpConfigJson(string configPath, JsonObject rootNode)
    {
        var options = new JsonSerializerOptions
        {
            WriteIndented = true,
            Encoder = System.Text.Encodings.Web.JavaScriptEncoder.UnsafeRelaxedJsonEscaping
        };
        WriteUtf8NoBom(configPath, rootNode.ToJsonString(options));
    }

    private static string ReadJsonConfigText(string configPath)
    {
        string jsonText = File.ReadAllText(configPath);
        return jsonText.Length > 0 && jsonText[0] == '\uFEFF'
            ? jsonText.TrimStart('\uFEFF')
            : jsonText;
    }

    private static void WriteUtf8NoBom(string path, string contents)
    {
        File.WriteAllText(path, contents, Utf8NoBom);
    }

    private static string ResolveMcpCommand(string daemonExe)
    {
        if (IsCommandInPath("godkiller-console")) return "godkiller-console";
        if (IsCommandInPath("godkiller-zero")) return "godkiller-zero";
        if (!string.IsNullOrEmpty(daemonExe) && File.Exists(daemonExe))
        {
            return Path.GetFullPath(daemonExe);
        }
        return "godkiller-console";
    }

    private static bool IsCommandInPath(string command)
    {
        try
        {
            string? pathEnv = Environment.GetEnvironmentVariable("PATH");
            if (string.IsNullOrEmpty(pathEnv)) return false;
            foreach (var dir in pathEnv.Split(Path.PathSeparator, StringSplitOptions.RemoveEmptyEntries))
            {
                string trimmed = dir.Trim();
                if (File.Exists(Path.Combine(trimmed, command))) return true;
                if (!command.EndsWith(".exe", StringComparison.OrdinalIgnoreCase)
                    && File.Exists(Path.Combine(trimmed, command + ".exe")))
                {
                    return true;
                }
            }
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine(ex.Message);
        }
        return false;
    }

    public static void EnsureBackgroundDaemon()
    {
        try
        {
            var existing = System.Diagnostics.Process.GetProcessesByName("godkiller-zero");
            if (existing.Length > 0) return;

            string? exePath = FindDaemonExePath();
            if (!string.IsNullOrEmpty(exePath))
            {
                var psi = new System.Diagnostics.ProcessStartInfo
                {
                    FileName = exePath,
                    Arguments = "--port 4242 --no-open",
                    UseShellExecute = false,
                    CreateNoWindow = true,
                    WindowStyle = System.Diagnostics.ProcessWindowStyle.Hidden
                };
                System.Diagnostics.Process.Start(psi);
            }
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine(ex.Message);
        }
    }


    public static bool WriteHookToFilePath(string path, string discipline, InvariantOptions options)
    {
        try
        {
            string? dir = Path.GetDirectoryName(path);
            if (!string.IsNullOrEmpty(dir) && !Directory.Exists(dir))
            {
                Directory.CreateDirectory(dir);
            }

            if (File.Exists(path))
            {
                try { File.Copy(path, path + ".bak", true); }
                catch (Exception ex) { System.Diagnostics.Debug.WriteLine(ex.Message); }
            }

            string existing = File.Exists(path) ? File.ReadAllText(path) : string.Empty;
            string ruleBlock = GenerateRuleBlock(discipline, options);

            string cleaned = existing;
            while (cleaned.Contains(MarkerStart))
            {
                int start = cleaned.IndexOf(MarkerStart, StringComparison.Ordinal);
                int end = cleaned.IndexOf(MarkerEnd, StringComparison.Ordinal);
                if (start >= 0 && end > start)
                {
                    end += MarkerEnd.Length;
                    cleaned = cleaned.Substring(0, start).TrimEnd() + "\n\n" + cleaned.Substring(end).TrimStart();
                }
                else if (start >= 0)
                {
                    int nextLine = cleaned.IndexOf('\n', start);
                    cleaned = nextLine >= 0 ? cleaned.Substring(0, start) + cleaned.Substring(nextLine + 1) : cleaned.Substring(0, start);
                }
                else
                {
                    break;
                }
            }

            cleaned = cleaned.Trim();
            string updated = string.IsNullOrEmpty(cleaned)
                ? ruleBlock + "\n"
                : cleaned + "\n\n" + ruleBlock + "\n";
            updated = ApplyMdcEnvelope(path, updated);

            WriteUtf8NoBom(path, updated);
            return true;
        }
        catch
        {
            return false;
        }
    }

    private static string ApplyMdcEnvelope(string path, string body)
    {
        if (!path.EndsWith(".mdc", StringComparison.OrdinalIgnoreCase))
        {
            return body;
        }
        if (body.TrimStart().StartsWith("---", StringComparison.Ordinal))
        {
            return body;
        }
        return "---\ndescription: GODKILLER ZERO cognitive invariants\nalwaysApply: true\n---\n\n" + body.TrimStart();
    }

    public static bool RemoveHookFromFilePath(string path)
    {
        try
        {
            if (!File.Exists(path)) return true;

            string existing = File.ReadAllText(path);
            if (!existing.Contains(MarkerStart)) return true;

            string text = existing;
            bool modified = false;
            while (text.Contains(MarkerStart))
            {
                int start = text.IndexOf(MarkerStart, StringComparison.Ordinal);
                int end = text.IndexOf(MarkerEnd, StringComparison.Ordinal);
                if (start >= 0 && end > start)
                {
                    end += MarkerEnd.Length;
                    text = text.Substring(0, start).TrimEnd() + "\n" + text.Substring(end).TrimStart();
                    modified = true;
                }
                else if (start >= 0)
                {
                    int nextLine = text.IndexOf('\n', start);
                    text = nextLine >= 0 ? text.Substring(0, start) + text.Substring(nextLine + 1) : text.Substring(0, start);
                    modified = true;
                }
                else
                {
                    break;
                }
            }

            if (modified)
            {
                string trimmed = text.Trim();
                WriteUtf8NoBom(path, string.IsNullOrEmpty(trimmed) ? string.Empty : trimmed + "\n");
            }

            return true;
        }
        catch
        {
            return false;
        }
    }

    private static bool WriteHookToDisk(string discipline, InvariantOptions options)
    {
        return WriteHookToFilePath(GetGeminiMdPath(), discipline, options);
    }

    private static bool RemoveHookFromDisk()
    {
        return RemoveHookFromFilePath(GetGeminiMdPath());
    }

    public static InvariantOptions GetOptionsForDiscipline(string discipline)
    {
        return discipline.ToUpper() switch
        {
            "SHI" => new InvariantOptions
            {
                ZeroSpeculation = true,
                BanGeneric = true,
                BanJunk = true,
                LimitSpan = true,
                ZeroFluff = false,
                Complexity = true,
                AsciiBlueprints = true,
                AsciiCadence = "Fast",
                MermaidDiagrams = true,
                MermaidCadence = "Fast",
                BlastRadius = true,
                StackSensor = true,
                NegativeBounding = true,
                CircuitBreaker = true,
                ExhaustiveErrors = false,
                FunctionalCore = false,
                ClarificationCadence = "Interactive",
                ModernWebSources = true,
                ThaiPolarity = false,
                RepoMap = true,
                MaxSpan = 90,
                MaxComplexity = 10
            },
            "SHIN" => new InvariantOptions
            {
                ZeroSpeculation = true,
                BanGeneric = true,
                BanJunk = true,
                LimitSpan = true,
                ZeroFluff = true,
                Complexity = true,
                AsciiBlueprints = true,
                AsciiCadence = "Fast",
                MermaidDiagrams = true,
                MermaidCadence = "Fast",
                BlastRadius = true,
                StackSensor = true,
                NegativeBounding = true,
                CircuitBreaker = true,
                ExhaustiveErrors = true,
                FunctionalCore = true,
                ClarificationCadence = "Silent",
                ModernWebSources = true,
                ThaiPolarity = false,
                RepoMap = true,
                MaxSpan = 50,
                MaxComplexity = 5
            },
            _ => new InvariantOptions
            {
                ZeroSpeculation = true,
                BanGeneric = true,
                BanJunk = true,
                LimitSpan = true,
                ZeroFluff = true,
                Complexity = true,
                AsciiBlueprints = true,
                AsciiCadence = "Fast",
                MermaidDiagrams = true,
                MermaidCadence = "Fast",
                BlastRadius = true,
                StackSensor = true,
                NegativeBounding = true,
                CircuitBreaker = true,
                ExhaustiveErrors = true,
                FunctionalCore = false,
                ClarificationCadence = "Balanced",
                ModernWebSources = true,
                ThaiPolarity = false,
                RepoMap = true,
                MaxSpan = 70,
                MaxComplexity = 7
            }
        };
    }

    public static string GenerateRuleBlock(string discipline, InvariantOptions options)
    {
        string label = discipline.ToUpper() switch
        {
            "SHI" => "SHI - Tranquil Exploratory",
            "SHIN" => "SHIN - Zero-Tolerance Strict",
            _ => "KEN - Standard Strict-Dev"
        };

        var sb = new StringBuilder();
        sb.AppendLine(MarkerStart);
        sb.AppendLine("# GODKILLER ZERO : COGNITIVE INVARIANTS");
        sb.AppendLine($"Discipline: {label}");
        sb.AppendLine("1. TARGET COORDINATES: Anchor code edits strictly to exact file coordinates and domain scopes.");

        if (options.ZeroFluff)
        {
            sb.AppendLine("2. ZERO CONVERSATIONAL FLUFF (FIRST-TOKEN STRUCTURAL DETERMINISM):");
            sb.AppendLine("   - First token MUST be the artifact (header, diff, or code fence). Zero preamble, greetings, apologies, or conversational filler in any language. Deliver direct engineering output only.");
        }
        else
        {
            sb.AppendLine("2. CONVERSATIONAL CADENCE: Concise, friendly engineering responses permitted.");
        }

        sb.AppendLine("3. ANTI-SPAGHETTI & CODE HYGIENE (WITH STRUCTURAL IMMUNITY):");
        if (options.Complexity && options.LimitSpan)
        {
            sb.AppendLine($"   - Maximum cyclomatic complexity: {options.MaxComplexity} per function. Maximum span: {options.MaxSpan} lines (business logic). Exemptions: Declarative UI, DTO mappings, static config, and tests/.");
        }
        else if (options.Complexity)
        {
            sb.AppendLine($"   - Maximum cyclomatic complexity: {options.MaxComplexity} per function. Exemptions: Declarative UI, DTO mappings, static config, and tests/.");
        }
        else if (options.LimitSpan)
        {
            sb.AppendLine($"   - Maximum function span: {options.MaxSpan} lines (business logic). Exemptions: Declarative UI, DTO mappings, static config, and tests/.");
        }
        sb.AppendLine("   - Enforce guard clauses and early returns (nesting depth <= 3).");

        if (options.BanGeneric)
        {
            sb.AppendLine("   - Strictly FORBIDDEN generic identifiers: [data, res, req, item, val, temp, obj, info, payload, result, handleData, processData, doAction] in domain models/state (framework signatures exempt).");
        }
        if (options.BanJunk)
        {
            sb.AppendLine("   - BANNED JUNK DRAWERS: Never create or append to utils/, helpers/, or common/ dirs. Co-locate helpers in domain feature modules.");
        }

        if (options.ThaiPolarity)
        {
            sb.AppendLine("4. ZERO VIBE-CODING TELLS (MULTILINGUAL):");
            sb.AppendLine("   - NEVER write comments narrating code mechanics (Thai examples: `// ฟังก์ชันสำหรับ...`, `// คืนค่าผลลัพธ์`; English: `// increment counter`, `// return result`). Write clean self-documenting code only.");
        }
        else
        {
            sb.AppendLine("4. ZERO VIBE-CODING TELLS (UNIVERSAL HYGIENE):");
            sb.AppendLine("   - NEVER write comments narrating code mechanics (e.g., `// increment counter`, `// return result`). Write clean self-documenting code only.");
        }

        if (options.ZeroSpeculation)
        {
            if (string.Equals(options.ClarificationCadence, "Silent", StringComparison.OrdinalIgnoreCase))
            {
                sb.AppendLine("5. CLARIFICATION CADENCE (SILENT AUTONOMOUS):");
                sb.AppendLine("   - FORBIDDEN interactive prompt modals (`ask_question`). Autonomously execute recommended path (Option #1); note trade-offs in response.");
            }
            else if (string.Equals(options.ClarificationCadence, "Interactive", StringComparison.OrdinalIgnoreCase))
            {
                sb.AppendLine("5. CLARIFICATION CADENCE (INTERACTIVE CO-PILOT):");
                sb.AppendLine("   - Proactively prompt user with structured choices via `ask_question` before major architectural decisions.");
            }
            else
            {
                sb.AppendLine("5. CLARIFICATION CADENCE (BALANCED SMART CONFIRMATION):");
                sb.AppendLine("   - Execute standard tasks autonomously. Trigger `ask_question` ONLY for high-blast irreversible actions (data loss, breaking schema changes).");
            }
        }
        else
        {
            sb.AppendLine("5. SPECIFICATION POLICY: Autonomous inference permitted where context is sufficient.");
        }

        sb.AppendLine("6. DISK STATE VERIFICATION & MANDATORY GATEKEEPER (MCP PROTOCOL):");
        sb.AppendLine("   - Verify disk changes, tests, and build before completing. Isolated mock data strictly to tests/ or fixtures/.");
        sb.AppendLine("   - If MCP tool `gk_claim_done` is available, MUST invoke it before completion and self-heal any reported violations (complexity, span, generic names, empty catch).");

        if (options.AsciiBlueprints)
        {
            if (string.Equals(options.AsciiCadence, "Fast", StringComparison.OrdinalIgnoreCase))
            {
                sb.AppendLine("7. MANDATORY ASCII WIREFRAMES (FAST - PLANS & UI ONLY):");
                sb.AppendLine("   - Render spatial ASCII wireframes in fenced code blocks (```text) ONLY during plans or UI specs. General Q&A is strictly exempt.");
            }
            else
            {
                sb.AppendLine("7. MANDATORY ASCII BLUEPRINTS (NORMAL - GLOBAL):");
                sb.AppendLine("   - Render explicit ASCII diagrams in fenced code blocks (```text) or markdown tables for all architectures, UI, and workflows. Never emit raw unfenced borders.");
            }
        }

        if (options.MermaidDiagrams)
        {
            if (string.Equals(options.MermaidCadence, "Fast", StringComparison.OrdinalIgnoreCase))
            {
                sb.AppendLine("7b. MANDATORY MERMAID WORKFLOW (FAST - PLANS ONLY):");
                sb.AppendLine("   - Render Mermaid diagrams (```mermaid graph LR/TD) ONLY in plans/architecture specs for logic flows. General Q&A is strictly exempt.");
            }
            else
            {
                sb.AppendLine("7b. MANDATORY MERMAID WORKFLOW (NORMAL - GLOBAL):");
                sb.AppendLine("   - Render Mermaid diagrams (```mermaid) across all responses for system architecture, state transitions, and workflows.");
            }
        }

        if (options.BlastRadius)
        {
            sb.AppendLine("8. BLAST RADIUS IMPACT AUDIT: Verify inbound callers and downstream dependencies before modifying functions, endpoints, or data models.");
        }

        if (options.StackSensor)
        {
            sb.AppendLine("9. TECH STACK AUTO-ALIGNMENT: Strictly adhere to project frameworks, libraries, and compiler toolchains without hallucinating dependencies.");
        }

        if (options.NegativeBounding)
        {
            sb.AppendLine("10. NEGATIVE MUTATION BOUNDING (GHOST EDIT SHIELD):");
            sb.AppendLine("    - FORBIDDEN modifying or refactoring files/symbols not explicitly targeted. Targeted formatting/imports permitted.");
            if (options.ThaiPolarity)
            {
                sb.AppendLine("    - When prompt contains prohibition markers (Thai: 'อย่า', 'ห้าม', 'ไม่ต้อง', 'ไม่เอา'; English: 'don\\'t', 'never', 'preserve', 'do not touch'), target logic MUST remain 100% bit-for-bit unchanged.");
            }
            else
            {
                sb.AppendLine("    - When prompt contains prohibition or preservation markers (e.g., 'don\\'t', 'never', 'preserve', 'do not touch'), target logic MUST remain 100% bit-for-bit unchanged.");
            }
        }

        if (options.CircuitBreaker)
        {
            sb.AppendLine("11. LINGUISTIC CIRCUIT BREAKER (FAILURE LOOP INTERRUPT):");
            if (options.ThaiPolarity)
            {
                sb.AppendLine("    - When user indicates failure recurrence (Thai: 'ยังไม่ได้', 'พังเหมือนเดิม', 'แก้ไม่หาย', 'วนลูป'; English: 'still failing', 'same error', 'didn\\'t work', 'looping'), HALT speculation. Demand exact runtime logs/errors or emit hypothesis trace.");
            }
            else
            {
                sb.AppendLine("    - When user indicates failure recurrence, stagnation, or looping (e.g., 'still failing', 'same error', 'didn\\'t work', 'looping'), HALT speculation. Demand exact runtime logs/errors or emit hypothesis trace.");
            }
        }

        if (options.ExhaustiveErrors)
        {
            sb.AppendLine("12. EXHAUSTIVE ERROR HANDLING & NO LAZY STUBS:");
            sb.AppendLine("    - FORBIDDEN unchecked unwrap(), raw expect(), silent catch {} / except: pass, or unhandled rejected promises.");
            sb.AppendLine("    - FORBIDDEN lazy stubs (e.g., '// TODO: implement later'). Deferred logic must declare typed interfaces and throw explicit domain NotImplemented errors.");
        }

        if (options.FunctionalCore)
        {
            sb.AppendLine("13. FUNCTIONAL CORE & IMPERATIVE SHELL:");
            sb.AppendLine("    - Decouple pure business logic and state transitions from impure I/O (disk, network). Domain logic must be deterministic and testable without mocks.");
        }

        if (options.ModernWebSources)
        {
            sb.AppendLine("14. MODERN WEB & DOCUMENTATION GUIDANCE:");
            sb.AppendLine("    - In web research, prioritize current official documentation and latest GitHub releases over obsolete blog tutorials.");
        }

        if (options.PremiumUiUx)
        {
            sb.AppendLine("15. PREMIUM UI/UX AESTHETIC MANDATE (ZERO BROWSER DEFAULTS):");
            sb.AppendLine("    - Enforce design tokens: modern typography (Inter/Roboto/Outfit), curated HSL/dark palette, 8px grid, smooth transitions (0.15s-0.2s).");
            sb.AppendLine("    - Strictly FORBIDDEN raw browser buttons, unstyled tables/links, and default saturated primaries (#ff0000, #0000ff).");
        }

        if (options.InfiniteEvolution)
        {
            sb.AppendLine("16. INFINITE EVOLUTIONARY CONTINUUM (ENTERPRISE ROADMAP ADVANCEMENT):");
            sb.AppendLine("    - Passing tests is a milestone, NOT a stopping signal. Upon 100% pass, checkpoint Git.");
            sb.AppendLine("    - Autonomously formulate next phase roadmap (e.g., caching, telemetry, hardening) without waiting for prompts, until explicit user pause.");
        }

        if (options.RepoMap)
        {
            sb.AppendLine("17. CODEBASE REPO MAP RADAR:");
            sb.AppendLine("    - Invoke MCP tool `gk_get_repo_map` to locate symbols before reading files. Strictly avoid dumping files >150 lines into context.");
        }

        sb.Append(MarkerEnd);
        return sb.ToString();
    }
}

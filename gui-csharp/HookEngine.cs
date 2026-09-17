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
        string userProfile = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
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

        if (target == TargetEngine.Universal || target == TargetEngine.Cursor)
        {
            paths.Add(Path.Combine(userProfile, ".cursorrules"));
            paths.Add(Path.Combine(currentDir, ".cursorrules"));
        }

        if (target == TargetEngine.Universal || target == TargetEngine.ClaudeCode)
        {
            string claudeHome = Path.Combine(userProfile, ".claude");
            if (!Directory.Exists(claudeHome))
            {
                try { Directory.CreateDirectory(claudeHome); }
                catch (Exception ex) { System.Diagnostics.Debug.WriteLine(ex.Message); }
            }
            paths.Add(Path.Combine(claudeHome, "CLAUDE.md"));
            paths.Add(Path.Combine(currentDir, "CLAUDE.md"));
        }

        if (target == TargetEngine.Universal || target == TargetEngine.VSCodeCopilot)
        {
            string githubDir = Path.Combine(currentDir, ".github");
            if (!Directory.Exists(githubDir))
            {
                try { Directory.CreateDirectory(githubDir); }
                catch (Exception ex) { System.Diagnostics.Debug.WriteLine(ex.Message); }
            }
            paths.Add(Path.Combine(githubDir, "copilot-instructions.md"));
        }

        var unique = new System.Collections.Generic.List<string>();
        foreach (var p in paths)
        {
            if (!unique.Contains(p)) unique.Add(p);
        }
        return unique;
    }

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

        // If RepoMap is enabled, trigger background scan to generate/update .gemini/REPO_MAP.md
        if (options.RepoMap)
        {
            TriggerRepoMapGeneration();
        }

        return anyOk;
    }

    public static void TriggerRepoMapGeneration()
    {
        try
        {
            string? daemonExe = FindDaemonExePath();
            if (!string.IsNullOrEmpty(daemonExe) && File.Exists(daemonExe))
            {
                string workspace = ResolveActiveWorkspaceRoot();
                var psi = new System.Diagnostics.ProcessStartInfo
                {
                    FileName = daemonExe,
                    Arguments = $"--repo-map \"{workspace}\"",
                    WorkingDirectory = workspace,
                    UseShellExecute = false,
                    CreateNoWindow = true,
                    WindowStyle = System.Diagnostics.ProcessWindowStyle.Hidden
                };
                System.Diagnostics.Process.Start(psi);
            }
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine($"RepoMap generation error: {ex.Message}");
        }
    }

    public static bool Unhook(TargetEngine target = TargetEngine.Universal)
    {
        // Complete Process Lifecycle: Terminate all background processes first to avoid race conditions
        KillBackgroundProcesses();

        var paths = GetTargetPaths(target);
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

            // Terminate any godkiller-zero processes in memory
            var processes = System.Diagnostics.Process.GetProcessesByName("godkiller-zero");
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
        string[] searchPaths = {
            Path.Combine(workspace, "publish", "godkiller-zero.exe"),
            Path.Combine(workspace, "target", "release", "godkiller-zero.exe"),
            Path.Combine(workspace, "target", "debug", "godkiller-zero.exe"),
            Path.Combine(workspace, "godkiller-zero.exe"),
            Path.Combine(baseDir, "godkiller-zero.exe"),
            Path.Combine(baseDir, "..", "godkiller-zero.exe"),
            Path.Combine(baseDir, "..", "publish", "godkiller-zero.exe"),
            Path.Combine(baseDir, "..", "..", "..", "..", "publish", "godkiller-zero.exe"),
            Path.Combine(baseDir, "..", "..", "..", "..", "target", "release", "godkiller-zero.exe"),
            Path.Combine(baseDir, "..", "..", "..", "..", "target", "debug", "godkiller-zero.exe"),
            Path.Combine(currentDir, "publish", "godkiller-zero.exe"),
            Path.Combine(currentDir, "target", "release", "godkiller-zero.exe"),
            Path.Combine(currentDir, "target", "debug", "godkiller-zero.exe"),
            Path.Combine(currentDir, "godkiller-zero.exe")
        };

        foreach (var exePath in searchPaths)
        {
            if (File.Exists(exePath))
            {
                return Path.GetFullPath(exePath);
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
        }

        string agentsDir = Path.Combine(Directory.GetCurrentDirectory(), ".agents");
        if (Directory.Exists(agentsDir))
        {
            targets.Add(Path.Combine(agentsDir, "mcp_config.json"));
        }

        return targets;
    }

    public static void ProvisionMcpConfig(TargetEngine target = TargetEngine.Universal)
    {
        string? daemonExe = FindDaemonExePath();
        if (string.IsNullOrEmpty(daemonExe) || !File.Exists(daemonExe)) return;

        var mcpTargets = new System.Collections.Generic.List<string>();
        string userProfile = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
        string appData = Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData);

        if (target == TargetEngine.Universal || target == TargetEngine.Antigravity)
        {
            mcpTargets.AddRange(ResolveAntigravityMcpTargets());
        }
        if (target == TargetEngine.Universal || target == TargetEngine.ClaudeCode)
        {
            mcpTargets.Add(Path.Combine(appData, "Claude", "claude_desktop_config.json"));
        }
        if (target == TargetEngine.Universal || target == TargetEngine.Cursor)
        {
            mcpTargets.Add(Path.Combine(userProfile, ".cursor", "mcp.json"));
        }

        foreach (var configPath in mcpTargets)
        {
            SafeInjectMcpServer(configPath, daemonExe);
        }
    }

    public static void DeprovisionMcpConfig(TargetEngine target = TargetEngine.Universal)
    {
        var mcpTargets = new System.Collections.Generic.List<string>();
        string userProfile = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
        string appData = Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData);

        if (target == TargetEngine.Universal || target == TargetEngine.Antigravity)
        {
            mcpTargets.AddRange(ResolveAntigravityMcpTargets());
        }
        if (target == TargetEngine.Universal || target == TargetEngine.ClaudeCode)
        {
            mcpTargets.Add(Path.Combine(appData, "Claude", "claude_desktop_config.json"));
        }
        if (target == TargetEngine.Universal || target == TargetEngine.Cursor)
        {
            mcpTargets.Add(Path.Combine(userProfile, ".cursor", "mcp.json"));
        }

        foreach (var configPath in mcpTargets)
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
            var processes = System.Diagnostics.Process.GetProcessesByName("godkiller-zero");
            foreach (var p in processes)
            {
                try
                {
                    p.Kill();
                    p.WaitForExit(300);
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
                string jsonText = File.ReadAllText(configPath);
                var parsed = JsonNode.Parse(jsonText);
                rootNode = parsed as JsonObject ?? new JsonObject();
            }
            else
            {
                rootNode = new JsonObject();
            }

            if (!rootNode.ContainsKey("mcpServers") || rootNode["mcpServers"] is not JsonObject)
            {
                rootNode["mcpServers"] = new JsonObject();
            }

            var servers = rootNode["mcpServers"]!.AsObject();
            var godkillerServer = new JsonObject
            {
                ["command"] = daemonExe,
                ["args"] = new JsonArray { "--mcp" }
            };

            servers["godkiller-zero"] = godkillerServer;

            var options = new JsonSerializerOptions { WriteIndented = true };
            string outputJson = rootNode.ToJsonString(options);
            File.WriteAllText(configPath, outputJson, Encoding.UTF8);
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

            string jsonText = File.ReadAllText(configPath);
            var parsed = JsonNode.Parse(jsonText);
            if (parsed is not JsonObject rootNode) return;

            if (rootNode.ContainsKey("mcpServers") && rootNode["mcpServers"] is JsonObject servers)
            {
                if (servers.Remove("godkiller-zero"))
                {
                    var options = new JsonSerializerOptions { WriteIndented = true };
                    string outputJson = rootNode.ToJsonString(options);
                    File.WriteAllText(configPath, outputJson, Encoding.UTF8);
                }
            }
        }
        catch (Exception ex)
        {
            System.Diagnostics.Debug.WriteLine($"Failed to deprovision MCP at {configPath}: {ex.Message}");
        }
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

            string existing = File.Exists(path) ? File.ReadAllText(path) : string.Empty;
            string ruleBlock = GenerateRuleBlock(discipline, options);

            string updated;
            if (existing.Contains(MarkerStart) && existing.Contains(MarkerEnd))
            {
                int start = existing.IndexOf(MarkerStart, StringComparison.Ordinal);
                int end = existing.IndexOf(MarkerEnd, StringComparison.Ordinal) + MarkerEnd.Length;
                updated = existing.Substring(0, start) + ruleBlock + existing.Substring(end);
            }
            else
            {
                updated = existing.TrimEnd() + "\n\n" + ruleBlock + "\n";
            }

            File.WriteAllText(path, updated, Encoding.UTF8);
            return true;
        }
        catch
        {
            return false;
        }
    }

    public static bool RemoveHookFromFilePath(string path)
    {
        try
        {
            if (!File.Exists(path)) return true;

            string existing = File.ReadAllText(path);
            if (!existing.Contains(MarkerStart)) return true;

            int start = existing.IndexOf(MarkerStart, StringComparison.Ordinal);
            int end = existing.IndexOf(MarkerEnd, StringComparison.Ordinal);
            if (start >= 0 && end > start)
            {
                end += MarkerEnd.Length;
                string clean = existing.Substring(0, start).TrimEnd() + "\n" + existing.Substring(end).TrimStart();
                string trimmed = clean.Trim();
                if (string.IsNullOrEmpty(trimmed))
                {
                    string fileName = Path.GetFileName(path).ToLowerInvariant();
                    if (fileName == ".cursorrules" || fileName == "claude.md" || fileName == "copilot-instructions.md")
                    {
                        File.Delete(path);
                        return true;
                    }
                    File.WriteAllText(path, string.Empty, Encoding.UTF8);
                }
                else
                {
                    File.WriteAllText(path, trimmed + "\n", Encoding.UTF8);
                }
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
        sb.AppendLine("# GODKILLER ZERO : COGNITIVE PRE-FLIGHT INVARIANTS & ZERO-VIBE SHIELD");
        sb.AppendLine($"Discipline: {label}");
        sb.AppendLine("Target Runtime: Google Antigravity IDE & Antigravity CLI (agy)");
        sb.AppendLine();
        sb.AppendLine("You are strictly governed by the GODKILLER ZERO Invariant Protocol:");
        sb.AppendLine("1. TARGET COORDINATES: When editing code, anchor to exact file coordinates or domain scopes.");

        if (options.ZeroFluff)
        {
            sb.AppendLine("2. ZERO CONVERSATIONAL FLUFF (FIRST-TOKEN STRUCTURAL DETERMINISM):");
            sb.AppendLine("   - FIRST-TOKEN PROTOCOL: The very first token of your response MUST be the technical artifact itself (a Markdown code fence, diff block, or technical specification).");
            sb.AppendLine("   - ZERO PREAMBLE & ZERO EPILOGUE: Strictly FORBIDDEN from generating any opening greetings, acknowledgments, affirmative phrases, apologies, transitions, or concluding polite offers in ANY language.");
            sb.AppendLine("   - Deliver pure, clean engineering results directly.");
        }
        else
        {
            sb.AppendLine("2. CONVERSATIONAL CADENCE: Concise, friendly engineering responses permitted.");
        }

        sb.AppendLine("3. ANTI-SPAGHETTI & CODE HYGIENE (WITH STRUCTURAL IMMUNITY):");
        if (options.Complexity && options.LimitSpan)
        {
            sb.AppendLine($"   - Maximum cyclomatic complexity: {options.MaxComplexity} per function (procedural logic; pattern matching and flat dispatch count as 1 branch). Maximum span: {options.MaxSpan} lines (business logic only).");
        }
        else if (options.Complexity)
        {
            sb.AppendLine($"   - Maximum cyclomatic complexity: {options.MaxComplexity} per function (procedural logic; pattern matching and flat dispatch count as 1 branch).");
        }
        else if (options.LimitSpan)
        {
            sb.AppendLine($"   - Maximum function span: {options.MaxSpan} lines (business logic only).");
        }
        sb.AppendLine("   - STRUCTURAL EXEMPTIONS: Declarative UI (React JSX, Flutter widgets, WinForms layout trees), DTO/entity mappings, static configuration tables, and `tests/` directories are 100% EXEMPT from span and complexity limits.");
        sb.AppendLine("   - GUARD CLAUSES & EARLY RETURNS: Strictly enforce Guard Clauses and early returns to maintain nesting depth <= 3.");

        if (options.BanGeneric)
        {
            sb.AppendLine("   - Strictly FORBIDDEN generic identifiers: [data, res, req, item, val, temp, obj, info, payload, result, handleData, processData, doAction] in domain entities, state variables, and return values. Standard framework parameter signatures (e.g., Express req/res) are permitted.");
        }
        if (options.BanJunk)
        {
            sb.AppendLine("   - BANNED JUNK DRAWERS: Never create or append to `utils/`, `helpers/`, or `common/` directories. Co-locate helper functions within their specific domain feature module.");
        }

        if (options.ThaiPolarity)
        {
            sb.AppendLine("4. ZERO VIBE-CODING TELLS (MULTILINGUAL):");
            sb.AppendLine("   - NEVER write redundant restatement comments that merely repeat or translate what code does in any language:");
            sb.AppendLine("     * Thai examples: `// ฟังก์ชันสำหรับ...`, `// เพิ่มตัวแปร`, `// คืนค่าผลลัพธ์`, `// ตรวจสอบเงื่อนไข`, `// นำเข้าโมดูล`, `// วนลูปข้อมูล`");
            sb.AppendLine("     * English examples: `// increment counter`, `// return result`, `// check condition`, `// import modules`, `// loop through items`");
            sb.AppendLine("   - Write self-documenting, clean production-grade code only.");
        }
        else
        {
            sb.AppendLine("4. ZERO VIBE-CODING TELLS (UNIVERSAL HYGIENE):");
            sb.AppendLine("   - NEVER write redundant restatement comments that merely narrate or translate what code mechanics do in ANY language (e.g., `// increment counter`, `// return result`, `// loop through items`).");
            sb.AppendLine("   - Write self-documenting, clean production-grade code only.");
        }

        if (options.ZeroSpeculation)
        {
            if (string.Equals(options.ClarificationCadence, "Silent", StringComparison.OrdinalIgnoreCase))
            {
                sb.AppendLine("5. CLARIFICATION CADENCE (SILENT AUTONOMOUS):");
                sb.AppendLine("   - Strictly FORBIDDEN from using interactive prompt modals (`ask_question`).");
                sb.AppendLine("   - Autonomously execute the recommended engineering path (Option #1). Present any trade-offs concisely in the final response.");
            }
            else if (string.Equals(options.ClarificationCadence, "Interactive", StringComparison.OrdinalIgnoreCase))
            {
                sb.AppendLine("5. CLARIFICATION CADENCE (INTERACTIVE CO-PILOT):");
                sb.AppendLine("   - MANDATORY PAIR-PROGRAMMING: Proactively prompt user with structured choices via interactive modals (`ask_question`) before executing major architectural decisions.");
            }
            else
            {
                sb.AppendLine("5. CLARIFICATION CADENCE (BALANCED SMART CONFIRMATION):");
                sb.AppendLine("   - Autonomously execute standard engineering tasks without modal popups.");
                sb.AppendLine("   - Trigger interactive prompt modals (`ask_question`) ONLY for high-blast critical forks: irreversible data deletion, breaking schema alterations, or conflicting file targets.");
            }
        }
        else
        {
            sb.AppendLine("5. SPECIFICATION POLICY: Autonomous inference permitted where context is sufficient.");
        }

        sb.AppendLine("6. DISK STATE VERIFICATION & MANDATORY GATEKEEPER (MCP PROTOCOL):");
        sb.AppendLine("   - Verify target files on disk and confirm unit tests or compiler build passes before declaring completion.");
        sb.AppendLine("   - MANDATORY MCP TOOL: If the MCP tool `gk_claim_done` is available, you MUST invoke it with your workspace path before declaring completion. If any violations (complexity > 7, span > 70, generic names, empty catch) are reported, you MUST self-heal and resolve them before concluding.");
        sb.AppendLine("   - Mock data and fixtures are strictly isolated to `tests/` or `fixtures/`. Production code must bind to typed interfaces or real data sources.");

        if (options.AsciiBlueprints)
        {
            if (string.Equals(options.AsciiCadence, "Fast", StringComparison.OrdinalIgnoreCase))
            {
                sb.AppendLine("7. MANDATORY ASCII WIREFRAMES (FAST - PLANS & UI ONLY):");
                sb.AppendLine("   - Render ASCII component layouts and spatial wireframes ONLY when formulating implementation plans or UI designs within an isolated code fence (```text ... ```).");
                sb.AppendLine("   - General conversational responses, explanations, and trivial Q&A are STRICTLY EXEMPT from ASCII diagrams to conserve token budget.");
            }
            else
            {
                sb.AppendLine("7. MANDATORY ASCII BLUEPRINTS (NORMAL - GLOBAL):");
                sb.AppendLine("   - When formulating implementation plans, UI designs, or architectural specs, MUST render explicit ASCII diagrams and component wireframes.");
                sb.AppendLine("   - All ASCII diagrams, boxes, or tables MUST ALWAYS be enclosed within an isolated code fence (```text ... ```) or standard GitHub Markdown table (| col | col |); NEVER emit unfenced raw ASCII borders that collapse into single-line paragraphs.");
            }
        }

        if (options.MermaidDiagrams)
        {
            if (string.Equals(options.MermaidCadence, "Fast", StringComparison.OrdinalIgnoreCase))
            {
                sb.AppendLine("7b. MANDATORY MERMAID WORKFLOW (FAST - PLANS ONLY):");
                sb.AppendLine("   - When formulating implementation plans or complex architectural workflows, MUST render Mermaid diagrams (```mermaid graph LR/TD ... ```) to visualize system transitions and logic flows.");
                sb.AppendLine("   - General conversational Q&A and minor one-off queries are STRICTLY EXEMPT from Mermaid diagrams to conserve tokens.");
            }
            else
            {
                sb.AppendLine("7b. MANDATORY MERMAID WORKFLOW (NORMAL - GLOBAL):");
                sb.AppendLine("   - Every system architecture explanation, lifecycle flow, sequence, or state transition across all responses and Q&A MUST render Mermaid diagrams (```mermaid ... ```).");
            }
        }

        if (options.BlastRadius)
        {
            sb.AppendLine("8. BLAST RADIUS IMPACT AUDIT: When modifying functions, endpoints, or data models, verify all inbound callers and outbound downstream dependencies before applying changes.");
        }

        if (options.StackSensor)
        {
            sb.AppendLine("9. TECH STACK AUTO-ALIGNMENT: Strictly adhere to project-detected frameworks, libraries, and compiler toolchains without hallucinating mismatched dependencies.");
        }

        if (options.NegativeBounding)
        {
            sb.AppendLine("10. NEGATIVE MUTATION BOUNDING (GHOST EDIT SHIELD):");
            sb.AppendLine("    - Strictly FORBIDDEN from modifying, refactoring, or renaming any symbol, function, or file not explicitly targeted by the user prompt. Import resolution and formatting within targeted functions are permitted.");
            if (options.ThaiPolarity)
            {
                sb.AppendLine("    - When prompt contains prohibition markers (Thai: 'อย่า', 'ห้าม', 'ไม่ต้อง', 'ไม่เอา'; English: 'don\'t', 'never', 'preserve', 'do not touch'), target logic MUST remain 100% bit-for-bit unchanged.");
            }
            else
            {
                sb.AppendLine("    - When prompt contains prohibition or preservation markers (e.g., 'don\'t', 'never', 'preserve', 'do not touch', or linguistic equivalents), target logic MUST remain 100% bit-for-bit unchanged.");
            }
        }

        if (options.CircuitBreaker)
        {
            sb.AppendLine("11. LINGUISTIC CIRCUIT BREAKER (FAILURE LOOP INTERRUPT):");
            if (options.ThaiPolarity)
            {
                sb.AppendLine("    - When user indicates failure recurrence (Thai: 'ยังไม่ได้', 'พังเหมือนเดิม', 'แก้ไม่หาย', 'วนลูป'; English: 'still failing', 'same error', 'didn\'t work', 'looping'), AI is strictly forbidden from guessing another fix.");
            }
            else
            {
                sb.AppendLine("    - When user indicates failure recurrence, stagnation, or looping across any language (e.g., 'still failing', 'same error', 'didn\'t work', 'looping'), AI is strictly forbidden from guessing another fix.");
            }
            sb.AppendLine("    - AI MUST halt speculation and demand exact runtime logs, compiler errors, or emit a minimal hypothesis trace diagram.");
        }

        if (options.ExhaustiveErrors)
        {
            sb.AppendLine("12. EXHAUSTIVE ERROR HANDLING & NO LAZY STUBS:");
            sb.AppendLine("    - FORBIDDEN unchecked unwrap(), raw expect() without context, silent catch {} / except: pass, or unhandled rejected promises.");
            sb.AppendLine("    - FORBIDDEN lazy stub comments (e.g., '// TODO: implement later'). Any deferred logic must declare a full typed interface and explicitly throw a domain NotImplemented error with tracking context.");
        }

        if (options.FunctionalCore)
        {
            sb.AppendLine("13. FUNCTIONAL CORE & IMPERATIVE SHELL:");
            sb.AppendLine("    - Decouple pure business logic and state transitions from impure I/O (network, disk, subprocesses). Pure domain calculations must be deterministic and testable without mocks.");
        }

        if (options.ModernWebSources)
        {
            sb.AppendLine("14. MODERN WEB & DOCUMENTATION GUIDANCE:");
            sb.AppendLine("    - When conducting web research, prioritize modern official documentation, current API references, and latest GitHub releases.");
            sb.AppendLine("    - Avoid relying on obsolete blog tutorials or deprecated legacy patterns.");
        }

        if (options.PremiumUiUx)
        {
            sb.AppendLine("15. PREMIUM UI/UX AESTHETIC MANDATE (ZERO BROWSER DEFAULTS):");
            sb.AppendLine("    - DESIGN TOKENS FIRST: Enforce cohesive typography (Google Fonts: Inter, Roboto, or Outfit) and curated color schemes (Sleek Dark Mode, tailored HSL).");
            sb.AppendLine("    - BANNED BROWSER DEFAULTS: Strictly FORBIDDEN raw browser buttons, unstyled links, basic HTML tables, and plain primary colors (#ff0000, #0000ff).");
            sb.AppendLine("    - INTERACTIVE MICRO-ANIMATIONS: Every interactive element (buttons, cards, tabs, inputs) MUST feature hover, active, and focus states with smooth transitions (0.15s - 0.2s). The interface must feel responsive and alive.");
            sb.AppendLine("    - RESPONSIVE FLUID LAYOUT: Enforce Flexbox/Grid on an 8px spacing system, structured card layering, elevation, and mobile responsiveness.");
        }

        if (options.InfiniteEvolution)
        {
            sb.AppendLine("16. INFINITE EVOLUTIONARY CONTINUUM (ENTERPRISE ROADMAP ADVANCEMENT):");
            sb.AppendLine("    - CONTINUOUS ITERATION: Passing compilation and unit tests is a baseline milestone, NOT a stopping signal. Software evolution is an infinite continuum.");
            sb.AppendLine("    - CHECKPOINT & ELEVATE: Upon achieving 100% test pass and zero errors, record a Git checkpoint, then proactively benchmark against world-class enterprise standards (e.g. SAP, Oracle WMS, Stripe).");
            sb.AppendLine("    - AUTONOMOUS ROADMAP EXPANSION: Proactively initiate the next advancement phase (e.g. Predictive Analytics, Multi-Warehouse Routing, In-Memory Caching, WebSockets Telemetry, Audit Logs, RBAC) and formulate the execution roadmap without waiting for user prompting.");
            sb.AppendLine("    - HALT CONDITION: Continue iterative advancement until explicit user pause or token budget termination.");
        }

        if (options.RepoMap)
        {
            sb.AppendLine("17. CODEBASE REPO MAP RADAR:");
            sb.AppendLine("    - Consult `.gemini/REPO_MAP.md` or invoke MCP tool `gk_get_repo_map` to pinpoint target symbols before reading source files. Strictly avoid dumping massive files (>150 lines) into context.");
        }

        sb.Append(MarkerEnd);
        return sb.ToString();
    }
}

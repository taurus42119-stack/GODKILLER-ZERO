using System;
using System.Runtime.InteropServices;
using System.Threading;
using System.Windows.Forms;

namespace GodkillerZeroGui;

static class Program
{

    private static Mutex? _mutex;

    [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
    private static extern uint RegisterWindowMessage(string lpString);

    [DllImport("user32.dll")]
    private static extern bool PostMessage(IntPtr hWnd, uint Msg, IntPtr wParam, IntPtr lParam);

    [DllImport("user32.dll")]
    private static extern bool SetForegroundWindow(IntPtr hWnd);

    [DllImport("user32.dll")]
    private static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);

    [DllImport("user32.dll", SetLastError = true)]
    private static extern IntPtr FindWindow(string? lpClassName, string lpWindowName);

    public const string ActivateMessageName = "GODKILLER_ZERO_ACTIVATE_WINDOW";
    public const string WakeUpEventName = "GodkillerZeroGui_ShowEvent";

    [STAThread]
    static void Main(string[] args)
    {
        if (args.Length > 0)
        {
            string cmd = args[0].ToLowerInvariant();
            if (cmd == "--hook")
            {
                string disc = args.Length > 1 ? args[1].ToUpperInvariant() : "KEN";
                bool ok = HookEngine.Hook(disc);
                Console.WriteLine(ok ? $"SUCCESS: Hooked mode {disc}" : "FAILED: Could not hook");
                return;
            }
            if (cmd == "--unhook")
            {
                bool ok = HookEngine.Unhook();
                Console.WriteLine(ok ? "SUCCESS: Unhooked rules" : "FAILED: Could not unhook");
                return;
            }
            if (cmd == "--status")
            {
                bool hooked = HookEngine.IsHooked();
                string disc = HookEngine.GetCurrentDiscipline();
                Console.WriteLine($"STATUS: Hooked={hooked}, Discipline={disc}");
                return;
            }
            if (cmd == "--test")
            {
                RunSelfTest();
                return;
            }
        }

        const string mutexName = "Local\\GodkillerZeroGui_SingleInstance_Mutex";
        bool isOnlyInstance;
        try
        {
            _mutex = new Mutex(true, mutexName, out isOnlyInstance);
        }
        catch (AbandonedMutexException)
        {
            isOnlyInstance = true;
        }

        if (!isOnlyInstance)
        {
            ActivateExistingInstance();
            return;
        }

        KillOrphanedInstances();

        ApplicationConfiguration.Initialize();
        var form = new MainForm();
        Application.Run(form);

        GC.KeepAlive(_mutex);
    }

    public const string PipeName = "GodkillerZero_WakePipe";

    private static bool ActivateExistingInstance()
    {
        // 1. Primary: Named Pipe IPC (Cross-process, 100% reliable)
        try
        {
            using var client = new System.IO.Pipes.NamedPipeClientStream(".", PipeName, System.IO.Pipes.PipeDirection.Out);
            client.Connect(300);
            client.WriteByte(1);
            client.Flush();
            return true;
        }
        catch (Exception exNamedPipe)
        {
            System.Diagnostics.Debug.WriteLine($"NamedPipe IPC wake error: {exNamedPipe.Message}");
        }

        // 2. Secondary: EventWaitHandle
        try
        {
            using var wakeEvent = EventWaitHandle.OpenExisting(WakeUpEventName);
            if (wakeEvent.Set())
            {
                return true;
            }
        }
        catch (Exception exEventHandle)
        {
            System.Diagnostics.Debug.WriteLine($"WakeEvent error: {exEventHandle.Message}");
        }

        // 3. Tertiary: Target window message (specific handle, not broadcast)
        try
        {
            IntPtr hwnd = FindWindow(null, "GODKILLER ZORO 1.0");
            if (hwnd != IntPtr.Zero)
            {
                uint activateMsg = RegisterWindowMessage(ActivateMessageName);
                if (activateMsg != 0)
                {
                    PostMessage(hwnd, activateMsg, IntPtr.Zero, IntPtr.Zero);
                    SetForegroundWindow(hwnd);
                    ShowWindow(hwnd, 9);
                    return true;
                }
            }
        }
        catch (Exception exWin)
        {
            System.Diagnostics.Debug.WriteLine($"FindWindow error: {exWin.Message}");
        }

        return false;
    }

    private static void KillOrphanedInstances()
    {
        try
        {
            int currentPid = Environment.ProcessId;
            var processes = System.Diagnostics.Process.GetProcessesByName("GodkillerZeroGui");
            foreach (var p in processes)
            {
                if (p.Id != currentPid)
                {
                    try
                    {
                        p.Kill();
                        p.WaitForExit(500);
                    }
                    catch (Exception exKillProcess)
                    {
                        System.Diagnostics.Debug.WriteLine($"Could not kill orphaned GUI process {p.Id}: {exKillProcess.Message}");
                    }
                }
            }
        }
        catch (Exception exKillOrphans)
        {
            System.Diagnostics.Debug.WriteLine($"KillOrphanedInstances error: {exKillOrphans.Message}");
        }
    }

    private static void RunSelfTest()
    {
        Console.WriteLine("[TEST] Starting GodkillerZeroGui Self-Test...");
        
        // 1. Test Unhook
        bool unhookOk = HookEngine.Unhook();
        System.Threading.Thread.Sleep(50);
        Console.WriteLine($"[TEST] Unhook: {unhookOk} (IsHooked={HookEngine.IsHooked()})");
        if (HookEngine.IsHooked()) throw new Exception("Expected IsHooked == false after Unhook");

        // 2. Test Workspace Resolution
        string workspace = HookEngine.ResolveActiveWorkspaceRoot();
        Console.WriteLine($"[TEST] ResolveActiveWorkspaceRoot: {workspace}");
        if (string.IsNullOrEmpty(workspace) || !Directory.Exists(workspace))
            throw new Exception("Expected valid workspace directory");

        // 3. Test Hook KEN with RepoMap = true
        var optionsWithMap = HookEngine.GetOptionsForDiscipline("KEN");
        optionsWithMap.RepoMap = true;
        bool hookOk = HookEngine.Hook("KEN", optionsWithMap);
        System.Threading.Thread.Sleep(50);
        Console.WriteLine($"[TEST] Hook KEN (RepoMap=true): {hookOk} (IsHooked={HookEngine.IsHooked()}, Discipline={HookEngine.GetCurrentDiscipline()})");
        if (!HookEngine.IsHooked()) throw new Exception("Expected IsHooked == true after Hook");
        if (HookEngine.GetCurrentDiscipline() != "KEN") throw new Exception("Expected Discipline == KEN");

        // Verify LoadOptionsFromDisk recognizes RepoMap = true
        var loadedOptions = HookEngine.LoadOptionsFromDisk();
        Console.WriteLine($"[TEST] LoadOptionsFromDisk RepoMap={loadedOptions.RepoMap}");
        if (!loadedOptions.RepoMap) throw new Exception("Expected loadedOptions.RepoMap == true");

        // 4. Test Hook KEN with RepoMap = false
        var optionsWithoutMap = HookEngine.GetOptionsForDiscipline("KEN");
        optionsWithoutMap.RepoMap = false;
        bool hookNoMapOk = HookEngine.Hook("KEN", optionsWithoutMap);
        System.Threading.Thread.Sleep(50);
        var loadedNoMap = HookEngine.LoadOptionsFromDisk();
        Console.WriteLine($"[TEST] Hook KEN (RepoMap=false): {hookNoMapOk}, Loaded RepoMap={loadedNoMap.RepoMap}");
        if (loadedNoMap.RepoMap) throw new Exception("Expected loadedOptions.RepoMap == false");

        // 5. Test Hook SHIN with RepoMap = true
        bool hookShinOk = HookEngine.Hook("SHIN");
        System.Threading.Thread.Sleep(50);
        Console.WriteLine($"[TEST] Hook SHIN: {hookShinOk} (IsHooked={HookEngine.IsHooked()}, Discipline={HookEngine.GetCurrentDiscipline()})");
        if (HookEngine.GetCurrentDiscipline() != "SHIN") throw new Exception("Expected Discipline == SHIN");
        var loadedShin = HookEngine.LoadOptionsFromDisk();
        if (!loadedShin.RepoMap) throw new Exception("Expected SHIN preset to have RepoMap == true");

        // 6. Test TriggerRepoMapGeneration
        HookEngine.TriggerRepoMapGeneration();
        Console.WriteLine("[TEST] TriggerRepoMapGeneration executed successfully without throwing.");

        // 7. Restore KEN state
        HookEngine.Hook("KEN");
        System.Threading.Thread.Sleep(50);

        Console.WriteLine("[TEST] ALL SELF-TESTS PASSED SUCCESSFULLY!");
    }
}

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

    public static void Log(string msg)
    {
        try
        {
            string p = System.IO.Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "startup.log");
            System.IO.File.AppendAllText(p, $"[{DateTime.Now:HH:mm:ss.fff}] [PID {Environment.ProcessId}] {msg}\n");
        }
        catch { }
    }

    [STAThread]
    static void Main(string[] args)
    {
        Log($"Started with args: {string.Join(" ", args)}");
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
            Log($"Mutex acquired, isOnlyInstance={isOnlyInstance}");
        }
        catch (AbandonedMutexException)
        {
            isOnlyInstance = true;
            Log("Mutex abandoned, taking ownership");
        }

        if (!isOnlyInstance)
        {
            Log("Instance exists, attempting wake...");
            if (ActivateExistingInstance())
            {
                Log("Existing instance activated, exiting.");
                return;
            }

            Log("Activation failed, purging orphaned instances...");
            KillOrphanedInstances();
            try
            {
                _mutex?.Dispose();
                _mutex = new Mutex(true, mutexName, out isOnlyInstance);
                Log($"Re-acquired mutex, isOnlyInstance={isOnlyInstance}");
            }
            catch (AbandonedMutexException)
            {
                isOnlyInstance = true;
            }
        }

        Log("Purging any remaining orphans...");
        KillOrphanedInstances();

        Log("ApplicationConfiguration.Initialize()...");
        ApplicationConfiguration.Initialize();
        Log("Instantiating MainForm...");
        var form = new MainForm();
        Log("Calling Application.Run(form)...");
        Application.Run(form);
        Log("Application.Run exited.");

        GC.KeepAlive(_mutex);
    }

    public const string PipeName = "GodkillerZero_WakePipe";
    public const string MainWindowTitle = "GODKILLER ZORO 1.0";

    private static bool ActivateExistingInstance()
    {
        // 1. Primary: Named Pipe IPC (Cross-desktop, 100% reliable)
        try
        {
            using var client = new System.IO.Pipes.NamedPipeClientStream(".", PipeName, System.IO.Pipes.PipeDirection.Out);
            client.Connect(350);
            client.WriteByte(1);
            client.Flush();
            Log("Activated existing instance via NamedPipe.");
            return true;
        }
        catch (Exception exPipe)
        {
            Log($"NamedPipe wake error: {exPipe.Message}");
        }

        // 2. Secondary: EventWaitHandle (Cross-desktop kernel object)
        try
        {
            using var wakeEvent = EventWaitHandle.OpenExisting(WakeUpEventName);
            if (wakeEvent.Set())
            {
                Log("Activated existing instance via EventWaitHandle.");
                return true;
            }
        }
        catch (Exception exEvent)
        {
            Log($"EventWaitHandle wake error: {exEvent.Message}");
        }

        // 3. Fallback: FindWindow + WM message (same desktop only)
        try
        {
            IntPtr hwnd = FindWindow(null, MainWindowTitle);
            if (hwnd != IntPtr.Zero)
            {
                uint activateMsg = RegisterWindowMessage(ActivateMessageName);
                if (activateMsg != 0)
                {
                    PostMessage(hwnd, activateMsg, IntPtr.Zero, IntPtr.Zero);
                }
                ShowWindow(hwnd, 9);
                SetForegroundWindow(hwnd);
                Log("Activated existing instance via FindWindow/Win32.");
                return true;
            }
        }
        catch (Exception exWin)
        {
            Log($"FindWindow wake error: {exWin.Message}");
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

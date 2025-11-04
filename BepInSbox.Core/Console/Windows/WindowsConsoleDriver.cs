using System;
using System.IO;
using System.Linq;
using System.Reflection;
using BepInSbox.ConsoleUtil;
using HarmonyLib;
using Microsoft.Win32.SafeHandles;
using MonoMod.Utils;
using UnityInjector.ConsoleUtil;

namespace BepInSbox;

internal class WindowsConsoleDriver : IConsoleDriver
{
    private bool useManagedEncoder;

    private int ConsoleWidth
    {
        get
        {
            try
            {
                return Console.WindowWidth;
            }
            catch (IOException)
            {
                return 0;
            }
        }
    }

    private int ConsoleHeight
    {
        get
        {
            try
            {
                return Console.WindowHeight;
            }
            catch (IOException)
            {
                return 0;
            }
        }
    }

    public TextWriter StandardOut { get; private set; }
    public TextWriter ConsoleOut { get; private set; }

    public bool ConsoleActive { get; private set; }
    public bool ConsoleIsExternal => true;

    public void Initialize(bool alreadyActive, bool useManagedEncoder)
    {
        ConsoleActive = alreadyActive;
        this.useManagedEncoder = useManagedEncoder;

        if (ConsoleActive)
        {
            // We're in a .NET framework / XNA environment; console *is* stdout
            //bepinsbox: I think this is still the case for .NET Core?
            ConsoleOut = Console.Out;
            StandardOut = new StreamWriter(Console.OpenStandardOutput());
        }
        else
        {
            StandardOut = Console.Out;
        }
    }

    public void CreateConsole(uint codepage)
    {
        ConsoleWindow.Attach();

        // Make sure of ConsoleEncoding helper class because on some Monos
        // Encoding.GetEncoding throws NotImplementedException on most codepages
        // NOTE: We don't set Console.OutputEncoding because it resets any existing Console.Out writers
        if (!useManagedEncoder)
            ConsoleEncoding.ConsoleCodePage = codepage;

        // If stdout exists, write to it, otherwise make it the same as console out
        // Not sure if this is needed? Does the original Console.Out still work?
        var stdout = GetOutHandle();
        if (stdout == IntPtr.Zero)
        {
            StandardOut = TextWriter.Null;
            ConsoleOut = TextWriter.Null;
            return;
        }

        var originalOutStream = OpenFileStream(stdout);
        StandardOut = new StreamWriter(originalOutStream, Utility.UTF8NoBom)
        {
            AutoFlush = true
        };

        var consoleOutStream = OpenFileStream(ConsoleWindow.ConsoleOutHandle);
        // Can't use Console.OutputEncoding because it can be null (i.e. not preference by user)
        ConsoleOut = new StreamWriter(consoleOutStream,
                                      useManagedEncoder ? Utility.UTF8NoBom : ConsoleEncoding.OutputEncoding)
        {
            AutoFlush = true
        };
        ConsoleActive = true;
    }

    public void PreventClose() => ConsoleWindow.PreventClose();

    public void DetachConsole()
    {
        ConsoleWindow.Detach();

        ConsoleOut.Close();
        ConsoleOut = null;

        ConsoleActive = false;
    }

    public void SetConsoleColor(ConsoleColor color)
    {
        SafeConsole.ForegroundColor = color;
        Kon.ForegroundColor = color;
    }

    public void SetConsoleTitle(string title) => ConsoleWindow.Title = title;

    private static Stream OpenFileStream(IntPtr handle)
    {
        var windowsConsoleStreamType = Type.GetType("System.ConsolePal+WindowsConsoleStream, System.Console", true);
        var constructor = AccessTools.Constructor(windowsConsoleStreamType,
                                                  new[] { typeof(IntPtr), typeof(FileAccess), typeof(bool) });
        return (Stream)constructor.Invoke(new object[] { handle, FileAccess.Write, true });
    }

    private IntPtr GetOutHandle()
    {
        switch (ConsoleManager.ConfigConsoleOutRedirectType.Value)
        {
            case ConsoleManager.ConsoleOutRedirectType.ConsoleOut:
                return ConsoleWindow.ConsoleOutHandle;
            case ConsoleManager.ConsoleOutRedirectType.StandardOut:
                return ConsoleWindow.OriginalStdoutHandle;
            case ConsoleManager.ConsoleOutRedirectType.Auto:
            default:
                return ConsoleWindow.OriginalStdoutHandle != IntPtr.Zero
                           ? ConsoleWindow.OriginalStdoutHandle
                           : ConsoleWindow.ConsoleOutHandle;
        }
    }
}

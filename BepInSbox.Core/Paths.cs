using System.IO;
using System.Linq;
using System.Reflection;
using MonoMod.Utils;
using SemanticVersioning;

namespace BepInSbox;

/// <summary>
///     Paths used by BepInS&amp;x
/// </summary>
public static class Paths
{
    /// <summary>
    ///     The path to the Managed folder that contains the main managed assemblies.
    /// </summary>
    public static string ManagedPath { get; private set; }

    /// <summary>
    ///     The path to the assets folder of the currently running sbox game.
    /// </summary>
    public static string AssetsPath { get; private set; }

    /// <summary>
    ///     The directory that the core BepInS&amp;x DLLs reside in.
    /// </summary>
    public static string BepInSboxAssemblyDirectory { get; private set; }

    /// <summary>
    ///     The path to the core BepInS&amp;x DLL.
    /// </summary>
    public static string BepInSboxAssemblyPath { get; private set; }

    /// <summary>
    ///     The path to the main BepInS&amp;x folder.
    /// </summary>
    public static string BepInSboxRootPath { get; private set; }

    /// <summary>
    ///     The path of the currently executing program BepInS&amp;x is encapsulated in.
    /// </summary>
    public static string ExecutablePath { get; private set; }

    /// <summary>
    ///     The directory that the currently executing process resides in.
    ///     <para>On OSX however, this is the parent directory of the game.app folder.</para>
    /// </summary>
    public static string GameRootPath { get; private set; }

    /// <summary>
    ///     The path to the config directory.
    /// </summary>
    public static string ConfigPath { get; private set; }

    /// <summary>
    ///     The path to the global BepInS&amp;x configuration file.
    /// </summary>
    public static string BepInSboxConfigPath { get; private set; }

    /// <summary>
    ///     The path to temporary cache files.
    /// </summary>
    public static string CachePath { get; private set; }

    /// <summary>
    ///     The path to the patcher plugin folder which resides in the BepInS&amp;x folder.
    /// </summary>
    public static string PatcherPluginPath { get; private set; }

    /// <summary>
    ///     The path to the plugin folder which resides in the BepInS&amp;x folder.
    ///     <para>
    ///         This is ONLY guaranteed to be set correctly when Chainloader has been initialized.
    ///     </para>
    /// </summary>
    public static string PluginPath { get; private set; }

    /// <summary>
    ///     The name of the currently executing process.
    /// </summary>
    public static string ProcessName { get; private set; }

    /// <summary>
    ///     List of directories from where Mono will search assemblies before assembly resolving is invoked.
    /// </summary>
    //bepinsbox: Don't think this is relevant here :)
    public static string[] DllSearchPaths { get; private set; }

    public static void SetExecutablePath(string executablePath,
                                         string bepinRootPath = null,
                                         string[] dllSearchPath = null)
    {
        ExecutablePath = executablePath;
        ProcessName = Path.GetFileNameWithoutExtension(executablePath);

        GameRootPath = PlatformDetection.OS.Is(OSKind.OSX)
                           ? Utility.ParentDirectory(executablePath, 4)
                           : Path.GetDirectoryName(executablePath);

        AssetsPath = Path.Combine(GameRootPath, $"assets");

        ManagedPath = Path.Combine(GameRootPath, "bin", "managed");

        //bepinsbox: Initially I wanted those to be called the proper name of this fork, i.e. BepInS&x,
        //bepinsbox: from the two minutes of research I did, it seems that most OSes can handle ampersands in file/directory names,
        //bepinsbox: but in case some user is running this from a weird ass Linux distro with a weirder ass File System that doesn't support UTF-8 for file names, better to be safe than sorry.
        //bepinsbox: this seemingly is also what s&box has opted to do, as all paths/files are either called "Sandbox" or "sbox"
        BepInSboxRootPath = bepinRootPath ?? Path.Combine(GameRootPath, "BepInSbox");
        ConfigPath = Path.Combine(BepInSboxRootPath, "config");
        BepInSboxConfigPath = Path.Combine(ConfigPath, "BepInSbox.cfg");
        PluginPath = Path.Combine(BepInSboxRootPath, "plugins");
        PatcherPluginPath = Path.Combine(BepInSboxRootPath, "patchers");
        BepInSboxAssemblyDirectory = Path.Combine(BepInSboxRootPath, "core");
        BepInSboxAssemblyPath = Path.Combine(BepInSboxAssemblyDirectory,
                                           $"{Assembly.GetExecutingAssembly().GetName().Name}.dll");
        CachePath = Path.Combine(BepInSboxRootPath, "cache");
        DllSearchPaths = (dllSearchPath ?? new string[0]).Concat(new[] { ManagedPath }).Distinct().ToArray();
    }

    internal static void SetPluginPath(string pluginPath) =>
        PluginPath = Utility.CombinePaths(BepInSboxRootPath, pluginPath);
}

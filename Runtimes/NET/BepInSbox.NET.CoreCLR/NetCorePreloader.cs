using System;
using System.Diagnostics;
using System.IO;
using System.Reflection;
using System.Runtime.CompilerServices;
using BepInSbox.Bootstrap;
using BepInSbox.Core.Sbox;
using BepInSbox.Logging;
using BepInSbox.NET.Common;
using BepInSbox.Preloader.Core;
using BepInSbox.Preloader.Core.Logging;
using BepInSbox.Preloader.Core.Patching;
using HarmonyLib;
using Mono.Cecil;

namespace BepInSbox.NET.CoreCLR
{
    internal class NetCorePreloader
    {
        private static readonly ManualLogSource Log = PreloaderLogger.Log;

        public static void Start()
        {
            var preloaderListener = new PreloaderConsoleListener();
            Logger.Listeners.Add(preloaderListener);

            TypeLoader.SearchDirectories.Add(Paths.GameRootPath);
            
            Logger.Sources.Add(TraceLogSource.CreateSource());

            ChainloaderLogHelper.PrintLogInfo(Log);

            Log.LogInfo($"CLR runtime version: {Environment.Version}");

            Log.LogInfo($"Current executable: {Paths.ExecutablePath}");
            Log.LogInfo($"Launch arguments: {string.Join(' ', Environment.GetCommandLineArgs())}");

            Log.LogMessage("Preloader started");

            using (var assemblyPatcher = new AssemblyPatcher((data, _) => Assembly.Load(data)))
            {
                assemblyPatcher.AddPatchersFromDirectory(Paths.PatcherPluginPath);

                Log.LogInfo($"{assemblyPatcher.PatcherContext.PatchDefinitions.Count} patcher definition(s) loaded");

                assemblyPatcher.LoadAssemblyDirectories(new[] { Paths.GameRootPath }, new[] { "dll", "exe" });

                Log.LogInfo($"{assemblyPatcher.PatcherContext.AvailableAssemblies.Count} assemblies discovered");

                assemblyPatcher.PatchAndLoad();
            }

            Log.LogMessage("Preloader finished");

            EngineHooks.Patch();

            Logger.Listeners.Remove(preloaderListener);

            var chainloader = new NetChainloader();
            chainloader.Initialize();
        }
    }
}

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Runtime.Loader;

// This code is shared between the .NET Framework launcher and the .NET Core entrypoint projects.
// However, it cannot be placed in the NetLauncher.Common project because this has to do with assembly resolution,
//   and placing this logic inside of an external assembly will cause a crash.
// A shared project is the only way to deduplicate code here

namespace BepInSbox.NET.Shared
{
    internal static class SharedEntrypoint
    {
        public static Func<AssemblyLoadContext, AssemblyName, Assembly> RemoteResolve(List<string> resolveDirectories)
        {
            return (context, name) => RemoteResolveInternal(context, name, resolveDirectories);
        }

        private static Assembly RemoteResolveInternal(AssemblyLoadContext context,
                                                      AssemblyName assemblyName,
                                                      List<string> resolveDirectories)
        {
            //bepinsbox: AppDomain seemingly lists all loaded assemblies, including BepInSbox, which doesn't reside in AssemblyLoadContext.Default, but instead in IsolatedComponentLoadContext, maybe because we're loaded from HostFXR?
            //bepinsbox: Regardless, this is how we resolve the plugins' reference to us; Prevents BepInSbox.Core, Harmony, and SemanticVersioning from being loaded twice and the duplicated assemblies' members not being assignable to the original ones
            foreach (var assembly in AppDomain.CurrentDomain.GetAssemblies())
            {
                if (assembly.GetName().Name == assemblyName.Name && assembly.GetName().Version >= assemblyName.Version)
                {
                    if (assembly.GetName().Version > assemblyName.Version)
                    {
                        Logging.Logger.Log(Logging.LogLevel.Debug, $"Found matching assembly for dependency with a higher version than requested: {assembly.GetName().Version}, requested: {assemblyName.Version}");
                    }

                    return assembly;
                }
            }

            foreach (var directory in resolveDirectories)
            {
                if (!Directory.Exists(directory))
                    continue;

                var potentialDirectories = new List<string> { directory };

                potentialDirectories.AddRange(Directory.GetDirectories(directory, "*", SearchOption.AllDirectories));

                var potentialFiles = potentialDirectories.Select(x => Path.Combine(x, $"{assemblyName.Name}.dll"))
                                                         .Concat(potentialDirectories.Select(x =>
                                                                     Path
                                                                         .Combine(x,
                                                                             $"{assemblyName.Name}.exe")));

                foreach (var path in potentialFiles)
                {
                    if (!File.Exists(path))
                        continue;

                    Assembly assembly;

                    try
                    {
                        assembly = Assembly.LoadFrom(path);
                    }
                    catch (Exception ex)
                    {
                        continue;
                    }

                    if (assembly.GetName().Name == assemblyName.Name)
                    {
                        return assembly;
                    }
                }
            }

            return null;
        }

        public static Assembly LocalResolve(object sender, ResolveEventArgs args)
        {
            foreach (var assembly in AppDomain.CurrentDomain.GetAssemblies())
            {
                Logging.Logger.Log(Logging.LogLevel.Info, assembly.FullName);
            }

            var assemblyName = new AssemblyName(args.Name);

            var foundAssembly = AppDomain.CurrentDomain.GetAssemblies()
                                         .FirstOrDefault(x => x.GetName().Name == assemblyName.Name);

            if (foundAssembly != null)
                return foundAssembly;

            if (LocalUtility.TryResolveDllAssembly(assemblyName, Paths.BepInSboxAssemblyDirectory, out foundAssembly)
             || LocalUtility.TryResolveDllAssembly(assemblyName, Paths.PatcherPluginPath, out foundAssembly)
             || LocalUtility.TryResolveDllAssembly(assemblyName, Paths.PluginPath, out foundAssembly))
                return foundAssembly;

            return null;
        }
    }


    /// <summary>
    ///     Generic helper properties and methods.
    /// </summary>
    internal static class LocalUtility
    {
        /// <summary>
        ///     Try to resolve and load the given assembly DLL.
        /// </summary>
        /// <param name="assemblyName">Name of the assembly, of the type <see cref="AssemblyName" />.</param>
        /// <param name="directory">Directory to search the assembly from.</param>
        /// <param name="assembly">The loaded assembly.</param>
        /// <returns>True, if the assembly was found and loaded. Otherwise, false.</returns>
        private static bool TryResolveDllAssembly<T>(AssemblyName assemblyName,
                                                     string directory,
                                                     Func<string, T> loader,
                                                     out T assembly) where T : class
        {
            assembly = null;

            if (!Directory.Exists(directory))
                return false;

            var potentialDirectories = new List<string> { directory };

            potentialDirectories.AddRange(Directory.GetDirectories(directory, "*", SearchOption.AllDirectories));

            foreach (var subDirectory in potentialDirectories)
            {
                var path = Path.Combine(subDirectory, $"{assemblyName.Name}.dll");

                if (!File.Exists(path))
                    continue;

                try
                {
                    assembly = loader(path);
                }
                catch (Exception)
                {
                    continue;
                }

                return true;
            }

            return false;
        }

        /// <summary>
        ///     Try to resolve and load the given assembly DLL.
        /// </summary>
        /// <param name="assemblyName">Name of the assembly, of the type <see cref="AssemblyName" />.</param>
        /// <param name="directory">Directory to search the assembly from.</param>
        /// <param name="assembly">The loaded assembly.</param>
        /// <returns>True, if the assembly was found and loaded. Otherwise, false.</returns>
        public static bool TryResolveDllAssembly(AssemblyName assemblyName, string directory, out Assembly assembly) =>
            TryResolveDllAssembly(assemblyName, directory, Assembly.LoadFrom, out assembly);
    }
}

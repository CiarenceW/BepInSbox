using System;
using System.Reflection;
using BepInSbox.Bootstrap;
using BepInSbox.Core.Sbox;
using BepInSbox.Logging;
using BepInSbox.Preloader.Core.Logging;
using HarmonyLib;
using Sandbox;
using Sandbox.Internal;

namespace BepInSbox.NET.Common
{
    public class NetChainloader : BaseChainloader<BaseSandboxPlugin>
    {
        // TODO: Remove once proper instance handling exists
        public static NetChainloader Instance { get; protected set; }

        public static GameObject ManagerObject { get; private set; }

        private delegate Component CreateComponentDelegate(ComponentList instance, Type type, bool startEnabled = true);

        //in s&box you'd usually create a new component on a GameObject by calling GameObject.AddComponent<>, which we can't use because it's a generic method
        //That method is just a shorthand for GameObject.ComponentList.Create<>, which actually exists in two other non-generic versions, one uses the internal whitelisted reflection library s&box uses called TypeLibrary
        //The other uses Type, but is internal, so we can only call it with reflections, I figured it'd be simpler and quicker to just get a delegate to this version, to avoid any problems that might arise from using TypeLibrary
        private static readonly CreateComponentDelegate ComponentList_CreateComponent = AccessTools.MethodDelegate<CreateComponentDelegate>(AccessTools.Method(typeof(ComponentList), nameof(ComponentList.Create), [ typeof(Type), typeof(bool) ]));

        private delegate void AddAssemblyDelegate(TypeLibrary instance, Assembly incoming, bool isDynamic);

        //as mentioned above, s&box has its own reflection library called TypeLibrary, it's mainly used for serialisation purposes, but it's safer for us to just add the modded types to that library
        //to do that, we can call TypeLibrary.AddAssembly(Assembly, bool), if the bool is true, it'll add all types from the assembly
        private static readonly AddAssemblyDelegate TypeLibrary_AddAssembly = AccessTools.MethodDelegate<AddAssemblyDelegate>(AccessTools.Method(typeof(TypeLibrary), "AddAssembly"));

        public override void Initialize()
        {
            Instance = this;

            //We can only create game objects on the main thread, sbox has a class to help us with that
            //This means the plugins will get loaded at the end of the very first frame of the game, as that's when MainThread executes all the queued methods
            MainThread.Queue(CreateManagerObjectAndInitialise);
        }

        private void CreateManagerObjectAndInitialise()
        {
            base.Initialize();

            //if we want s&box (and mainly Source 2) to recognise assets, and make them loadable, we need to mount them to the file system
            //calling the EngineFileSystem.AddAssetPath() method is the best way for us to do that, it creates a path in Source 2 and s&box at the same time, so shaders and materials can be loaded
            //PROBLEM: for a plugin with resources located in a path like this: plugins/awesomeplugin/assets/awesomeshader.shader_c
            //anytime a plugin will want to load the awesomeshader.shader_c file, it'll have to type out the whole path: Shader.Load("awesomeplugin/assets/awesomeshader")
            //kinda annoying, but considering we might be loaded by R2ModMan/Gale, which loves to flatten folder structure, this'll be a problem
            //you can prevent flattening by explicitely packaging your mod like specified here https://wiki.thunderstore.io/mods/packaging-your-mods, but it still kinda sucks? idk
            //(also as I'm speaking now I have absolutely not contacted thunderstore about this at all, but it also seems that each game needs to have which paths to preserve configured manually)
            //if we instead "mount" each plugin at their root like the method just below, there'll still be the problem of assets being in a subfolder, which, again, would get flattened by R2ModMan/Gale
            AccessTools.Method(AccessTools.TypeByName("Sandbox.EngineFileSystem"), "AddAssetPath").Invoke(null, ["bepins&x-plugins", Paths.PluginPath]);
            AccessTools.Method(AccessTools.TypeByName("Sandbox.EngineFileSystem"), "AddAssetPath").Invoke(null, ["bepins&x-patchers", Paths.PatcherPluginPath]);

            ManagerObject = new GameObject("BepInS&x_Manager");
            ManagerObject.Flags |= GameObjectFlags.DontDestroyOnLoad;

            EngineHooks.ManagerObject = ManagerObject;

            base.Execute();
        }

        public override BaseSandboxPlugin LoadPlugin(PluginInfo pluginInfo, Assembly pluginAssembly)
        {
            var type = pluginAssembly.GetType(pluginInfo.TypeName);

            TypeLibrary_AddAssembly(Game.TypeLibrary, pluginAssembly, true);

            //We're looking for if the type has overriden the OnUpdate, OnFixedUpdate, or OnPreRender methods with those flags
            var implementedFlags = BindingFlags.DeclaredOnly | BindingFlags.Instance | BindingFlags.NonPublic;

            //go through all types in the assembly and check if they explicitely override those methods
            foreach (var assemblyType in type.Assembly.GetTypes())
            {
                if (assemblyType.GetMethod("OnUpdate", implementedFlags) != null)
                {
                    Logger.Log(LogLevel.Debug, $"Type {assemblyType.Name} implements onUpdate");
                    EngineHooks.ImplementsOnUpdateComponentList.Add(assemblyType);
                }

                if (assemblyType.GetMethod("OnFixedUpdate", implementedFlags) != null)
                {
                    Logger.Log(LogLevel.Debug, $"Type {assemblyType.Name} implements OnFixedUpdate");
                    EngineHooks.ImplementsOnFixedUpdateComponentList.Add(assemblyType);
                }

                if (assemblyType.GetMethod("OnPreRender", implementedFlags) != null)
                {
                    Logger.Log(LogLevel.Debug, $"Type {assemblyType.Name} implements OnPreRender");
                    EngineHooks.ImplementsOnPreRenderComponentList.Add(assemblyType);
                }
            }

            var comp = (BaseSandboxPlugin)ComponentList_CreateComponent(ManagerObject.Components, type, true);

            //calls the plugin's load method, if it has one
            comp.InternalLoad();

            return comp;
        }

        protected override void InitializeLoggers()
        {
            base.InitializeLoggers();

            Logger.Sources.Add(new BepInSbox.Core.Sbox.Logging.SandboxLogSource());

            Logger.Listeners.Add(new BepInSbox.Core.Sbox.Logging.SandboxLogListener());

            ChainloaderLogHelper.RewritePreloaderLogs();
        }
    }
}

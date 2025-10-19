using System;
using System.Collections.Generic;
using System.Reflection;
using System.Threading;
using System.Threading.Tasks;
using BepInEx.Bootstrap;
using BepInEx.Core.Sbox;
using BepInEx.Logging;
using BepInEx.Preloader.Core;
using BepInEx.Preloader.Core.Logging;
using HarmonyLib;
using Sandbox;
using Sandbox.Internal;

namespace BepInEx.NET.Common
{
    public class NetChainloader : BaseChainloader<BaseSandboxPlugin>
    {
        // TODO: Remove once proper instance handling exists
        public static NetChainloader Instance { get; protected set; }

        public static GameObject ManagerObject { get; private set; }

        private delegate Component CreateComponentDelegate(ComponentList instance, Type type, bool startEnabled = true);

        //in Sbox you'd usually create a new component on a GameObject by calling GameObject.AddComponent<>, which we can't use because it's a generic method
        //That method is just a shorthand for GameObject.ComponentList.Create<>, which actually exists in two other non-generic versions, one uses the internal whitelisted reflection library sbox uses called TypeLibrary
        //The other uses Type, but is internal, so we can only call it with reflections, I figured it'd be simpler and quicker to just get a delegate to this version, to avoid any problems that might arise from using TypeLibrary
        private static CreateComponentDelegate createComponent = AccessTools.MethodDelegate<CreateComponentDelegate>(AccessTools.Method(typeof(ComponentList), nameof(ComponentList.Create), [ typeof(Type), typeof(bool) ]));

        public override void Initialize(string gameExePath = null)
        {
            Instance = this;

            //We can only create game objects on the main thread, sbox has a class to help us with that
            //This means the plugins will get loaded at the end of the very first frame of the game, as that's when MainThread executes all the queued methods
            MainThread.Queue(CreateManagerObjectAndInitialise);
        }

        private void CreateManagerObjectAndInitialise()
        {
            ManagerObject = new GameObject("BepInS&x_Manager");
            ManagerObject.Flags |= GameObjectFlags.DontDestroyOnLoad;

            EngineHooks.ManagerObject = ManagerObject;

            base.Initialize();

            base.Execute();
        }

        public override BaseSandboxPlugin LoadPlugin(PluginInfo pluginInfo, Assembly pluginAssembly)
        {
            var type = pluginAssembly.GetType(pluginInfo.TypeName);

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

            var comp = (BaseSandboxPlugin)createComponent(ManagerObject.Components, type, true);

            //calls the plugin's load method, if it has one
            comp.InternalLoad();

            return comp;
        }

        protected override void InitializeLoggers()
        {
            base.InitializeLoggers();

            ChainloaderLogHelper.RewritePreloaderLogs();
        }
    }
}

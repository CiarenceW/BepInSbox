using System;
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
            //Create temporary scene, this is needed for our components to awake
            AccessTools.PropertySetter(typeof(Game), nameof(Game.ActiveScene)).Invoke(null, [new Scene()]);

            ManagerObject = new GameObject("BepInEx_Manager");
            ManagerObject.Flags |= GameObjectFlags.DontDestroyOnLoad;

            Logger.Log(LogLevel.Message, $"Active scene: {Game.ActiveScene}");

            base.Initialize();

            base.Execute();
        }

        public override BaseSandboxPlugin LoadPlugin(PluginInfo pluginInfo, Assembly pluginAssembly)
        {
            var type = pluginAssembly.GetType(pluginInfo.TypeName);

            return (BaseSandboxPlugin)createComponent(ManagerObject.Components, type, true);
        }

        protected override void InitializeLoggers()
        {
            base.InitializeLoggers();

            ChainloaderLogHelper.RewritePreloaderLogs();
        }
    }
}

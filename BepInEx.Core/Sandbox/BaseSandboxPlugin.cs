using BepInEx.Configuration;
using BepInEx.Logging;
using HarmonyLib;
using Sandbox;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.IO;

namespace BepInEx.Core.Sbox
{
    /// <summary>
    ///     Base S&amp;Box plugin component, this is what gets loaded by BepInEx :)
    /// </summary>
    public abstract class BaseSandboxPlugin : Sandbox.Component
    {
        /// <summary>
        ///     Create a new instance of a plugin and all of its tied in objects.
        /// </summary>
        /// <exception cref="InvalidOperationException">BepInPlugin attribute is missing.</exception>
        protected BaseSandboxPlugin()
        {
            var metadata = MetadataHelper.GetMetadata(this);

            if (metadata == null)
            {
                throw new InvalidOperationException($"Can't create and instance of {GetType().FullName} because it inherits from BaseSandboxPlugin and the BepInPlugin attribute is missing.");
            }

            Info = new PluginInfo
            {
                Metadata = metadata,
                Instance = this,
                Dependencies = MetadataHelper.GetDependencies(this.GetType()),
                Processes = MetadataHelper.GetAttributes<BepInProcess>(this.GetType()),
                Location = this.GetType().Assembly.Location
            };

            HarmonyInstance = new Harmony("BepInEx.Plugin." + metadata.GUID);

            Logger = global::BepInEx.Logging.Logger.CreateLogSource(metadata.Name);

            Config = new ConfigFile(Utility.CombinePaths(Paths.ConfigPath, metadata.GUID + ".cfg"), false, metadata);
        }

        /// <summary>
        ///     If you create a new GameObject during this, mark it as <see cref="GameObjectFlags.DontDestroyOnLoad"/>, otherwise, it'll get destroyed. (see comment on NetChainloader.CreateManagerObjectAndInitialise() for an explanation)
        /// </summary>
        protected override void OnAwake() => base.OnAwake();

        /// <summary>
        ///     Logger instance tied to this plugin.
        /// </summary>
        public ManualLogSource Logger { get; }

        /// <summary>
        ///     Default config file tied to this plugin. The config file will not be created until
        ///     any settings are added and changed, or <see cref="ConfigFile.Save" /> is called.
        /// </summary>
        public ConfigFile Config { get; }

        /// <summary>
        ///     Your very own Harmony Instance, use this to patch your own stuff, instead of <see cref="Harmony.CreateAndPatchAll(Type, string)"/>
        /// </summary>
        //Shouldn't this be private? whatever
        public Harmony HarmonyInstance { get; protected set; }

        /// <summary>
        ///     Information about this plugin as it was loaded.
        /// </summary>
        public PluginInfo Info { get; }
    }
}

using HarmonyLib;
using System;
using System.Collections.Generic;
using Sandbox;
using System.Reflection;
using BepInSbox.Logging;
using System.Reflection.Emit;
using static System.Reflection.Emit.OpCodes;

//if we call it "Sandbox" it'll be annoying anytime we want to call anything in the actual Sandbox namespace
#pragma warning disable IDE0130 // Namespace does not match folder structure
namespace BepInSbox.Core.Sbox
#pragma warning restore IDE0130 // Namespace does not match folder structure
{
    /// <summary>
    ///     S&amp;box specific engine hooks
    /// </summary>
    public static class EngineHooks
    {
        static Harmony HarmonyInstance { get; } = new Harmony("BepInS&x-EngineHooks");

        static ManualLogSource Logger { get; } = BepInSbox.Logging.Logger.CreateLogSource("EngineHooks");

        private delegate void ComponentUpdateEnabledStatusDelegate(Component instance);

        private static readonly ComponentUpdateEnabledStatusDelegate UpdateEnabledStatus = AccessTools.MethodDelegate<ComponentUpdateEnabledStatusDelegate>(AccessTools.Method(typeof(Component), "UpdateEnabledStatus"));

        private delegate void SceneAddObjectToDirectoryDelegate(Scene instance, object obj);

        private static readonly SceneAddObjectToDirectoryDelegate AddObjectToDirectory = AccessTools.MethodDelegate<SceneAddObjectToDirectoryDelegate>(AccessTools.Method(typeof(Scene), "AddObjectToDirectory"));

        //prevents "tried to unregister unregistered id" warning
        private delegate void GameObjectDirectoryAddComponentDelegate(GameObjectDirectory directory, Component component);

        private static readonly GameObjectDirectoryAddComponentDelegate AddComponentToIdList = AccessTools.MethodDelegate<GameObjectDirectoryAddComponentDelegate>(AccessTools.Method(typeof(GameObjectDirectory), "Add", [typeof(Component)]));

        //ditto
        private delegate void GameObjectDirectoryAddGameObjectDelegate(GameObjectDirectory directory, GameObject gameObject);

        private static readonly GameObjectDirectoryAddGameObjectDelegate AddGameObjectToIdList = AccessTools.MethodDelegate<GameObjectDirectoryAddGameObjectDelegate>(AccessTools.Method(typeof(GameObjectDirectory), "Add", [typeof(GameObject)]));

        /// <summary>
        ///     We have a handle to the manager object here, since we don't have access to NetChainloader from this assembly, and we can't access the object any other way.
        /// </summary>
        internal static GameObject ManagerObject { get; set; }

        internal static HashSet<Type> ImplementsOnUpdateComponentList { get; } = new HashSet<Type>();

        internal static HashSet<Type> ImplementsOnFixedUpdateComponentList { get; } = new HashSet<Type>();

        internal static HashSet<Type> ImplementsOnPreRenderComponentList { get; } = new HashSet<Type>();

        /// <summary>
        /// We load very early in the engine's intialisation, it doesn't even have time to load the Sandbox.GameInstance.dll yet, which we need for a patch later.
        /// Instead we start listening to <seealso cref="AppDomain.AssemblyLoad"/> for whenever the dll we're looking for gets loaded, which then kicks off the chainloader
        /// </summary>
        /// <param name="onEngineLoaded">The chainloader's <c>Initialize</c> method, ideally</param>
        internal static void PreHook(Action onEngineLoaded)
        {
            AppDomain.CurrentDomain.AssemblyLoad += OnAssemblyLoad;

            PatchEngine(onEngineLoaded);
        }

        //types in Sandbox.Engine and Sandbox.System (and another one, I forgot which lol) are already loaded (I think it's technically because we use types from these assemblies, so they're loaded with us?)
        private static void PatchEngine(Action onEngineLoaded)
        {
            Logger.LogDebug("Engine patch");

            HarmonyInstance.PatchAll(typeof(ComponentUpdateFix));

            //don't have to bother if we're standalone, it's already returning false
            if (!Application.IsStandalone)
            {
                HarmonyInstance.Patch(AccessTools.Method(AccessTools.TypeByName("Sandbox.Engine.ErrorReporter"), "get_IsUsingSentry"), prefix: new HarmonyMethod(NeuterErrorReporter.PreventErrorReporterFromRunning));
            }

            onEngineLoaded();
        }

        //this one is loaded slightly later
        private static void LatePatch()
        {
            Logger.LogDebug("Late patch");

            //GameInstance is an internal type in an assembly with no public types
            HarmonyInstance.Patch(AccessTools.Method(AccessTools.TypeByName("Sandbox.GameInstance"), "OpenStartupScene"), postfix: new HarmonyMethod(PatchOpenStartupScene));
        }

        private static void OnAssemblyLoad(object sender, AssemblyLoadEventArgs args)
        {
            if (args.LoadedAssembly.FullName.Contains("Sandbox.GameInstance"))
            {
                LatePatch();

                AppDomain.CurrentDomain.AssemblyLoad -= OnAssemblyLoad;
            }
        }

        [HarmonyPostfix]
        internal static void PatchOpenStartupScene()
        {
            Logger.LogDebug("setting scene to gameobject");

            var scene = Game.ActiveScene;

            //Adds the scene reference to the GameObject, in turns allows all Components to have access to a Scene
            AccessTools.PropertySetter(typeof(GameObject), nameof(GameObject.Scene)).Invoke(ManagerObject, [ scene ]);
            ManagerObject.Parent = scene;

            AddGameObjectToIdList(scene.Directory, ManagerObject);

            foreach (var component in ManagerObject.Components.GetAll()) 
            {
                AddComponentToIdList(scene.Directory, component);

                //adds the component to the scene's directory, so that their Update methods get called
                AddObjectToDirectory(scene, component);

                //Kickstart the component's Awake()
                UpdateEnabledStatus(component);
            }

            Logger.LogDebug($"new scene: {ManagerObject.Scene}");
        }

        //if someone tries using this with the editor or whatever, at least Facepunch won't be flooded with bullshit errors or whatever :(
        private static class NeuterErrorReporter
        {
            [HarmonyPrefix]
            internal static bool PreventErrorReporterFromRunning(ref bool __result)
            {
                __result = false;

                return false;
            }
        }

        //in sbox, it's the scene's role to call the various update methods for components. Usually, component that overrides each update type (OnUpdate, OnFixedUpdate, OnPreRender) automatically have an interface, added to them when s&box compiles the code, that signals which update type it overrides
        //that means that, unless sbox compiles your code, you'll have to manually add IUpdateSubscriber/IFixedUpdateSubscriber/IPreRenderSubscriber, which would be an annoying quirk
        //when a new component gets added to the scene, it checks if the component implements those type, and adds them to one (or more) of 3 three lists, depending on what it implements
        //we check in the NetChainloader's plugin loading method if the types in the plugin's assembly override those methods, and if so, we add them to our own list of Type
        //then, here, when the component gets added to a scene, we add 3 checks to see if the component's type is in one of our lists, and if so, adds it to the scene's list
        //this allows components that override the aforementioned update methods to properly update without needing to add the interfaces
        private static class ComponentUpdateFix
        {
            [HarmonyPatch(typeof(Scene), "AddObjectToDirectory")]
            [HarmonyTranspiler]
            internal static IEnumerable<CodeInstruction> TranspileComponentImplementsCheck(IEnumerable<CodeInstruction> instructions, ILGenerator generator, MethodBase __originalMethod)
            {
                CodeMatcher codeMatcher = new CodeMatcher(instructions, generator);



                //IUpdateSubscriber
                {
                    //copy the destination of the brtrue label to use later
                    var implementsOnUpdateTrueTarget = (Label)codeMatcher
                        .Start()
                        .MatchForward(true,
                            [
                                new (Ldloc_0),
                                new (Isinst, typeof(Sandbox.Internal.IUpdateSubscriber)),
                                new (Brtrue)
                            ]
                        )
                        .ThrowIfInvalid("Failed to match to component is IUpdateSubscriber")
                        .Operand
                        ;

                    codeMatcher
                        .Advance(1)
                        .Insert(
                            [
                                //ImplementsOnUpdateComponentList.Contains(type)
                                new (Call, AccessTools.PropertyGetter(typeof(EngineHooks), nameof(ImplementsOnUpdateComponentList))),

                                new (Ldarg_1),
                                new (Callvirt, AccessTools.Method(typeof(object), nameof(GetType))),

                                new (Callvirt, AccessTools.Method(typeof(HashSet<Type>), nameof(HashSet<Type>.Contains), [ typeof(Type) ])),
                                new (Brtrue_S, implementsOnUpdateTrueTarget)
                            ]
                        )
                        ;
                }

                //IFixedUpdateSubscriber
                {
                    var implementsOnFixedUpdateTrueTarget = (Label)codeMatcher
                        .MatchForward(true,
                            [
                                new (Ldloc_0),
                                new (Isinst, typeof(Sandbox.Internal.IFixedUpdateSubscriber)),
                                new (Brtrue)
                            ]
                        )
                        .ThrowIfInvalid("Failed to match to component is IFixedUpdateSubscriber")
                        .Operand
                        ;

                    codeMatcher
                        .Advance(1)
                        .Insert(
                            [
                                //ImplementsOnFixedUpdateComponentList.Contains(type)
                                new (Call, AccessTools.PropertyGetter(typeof(EngineHooks), nameof(ImplementsOnFixedUpdateComponentList))),

                                new (Ldarg_1),
                                new (Callvirt, AccessTools.Method(typeof(object), nameof(GetType))),

                                new (Callvirt, AccessTools.Method(typeof(HashSet<Type>), nameof(HashSet<Type>.Contains), [ typeof(Type) ])),
                                new (Brtrue_S, implementsOnFixedUpdateTrueTarget)
                            ]
                        )
                        ;
                }

                //IOnPreRenderSubscriber
                {
                    //Here we need to remove the destination labels, we'll be adding them back to the beginning code we're going to add
                    var ifStartLabel = codeMatcher
                        .MatchForward(false,
                            [
                                new (Ldloc_0),
                                new (Isinst, typeof(Sandbox.Internal.IPreRenderSubscriber)),
                                new (Brfalse)
                            ]
                        )
                        .ThrowIfInvalid("Failed to match to component is IPreRenderSubscriber")
                        .Instruction.ExtractLabels()
                        ;

                    codeMatcher
                        //base conditional didn't have any branches, we're adding one that targets ret
                        .CreateLabelWithOffsets(3, out Label isInListLabel)
                        .Insert(
                            [
                                //ImplementsOnPreRenderComponentList.Contains(type)
                                new (Call, AccessTools.PropertyGetter(typeof(EngineHooks), nameof(ImplementsOnPreRenderComponentList))),

                                new (Ldarg_1),
                                new (Callvirt, AccessTools.Method(typeof(object), nameof(GetType))),

                                new (Callvirt, AccessTools.Method(typeof(HashSet<Type>), nameof(HashSet<Type>.Contains), [ typeof(Type) ])),
                                new (Brtrue_S, isInListLabel)
                            ]
                        )
                        .AddLabels(ifStartLabel)
                        ;
                }

                return codeMatcher.InstructionEnumeration();
            }
        }
    }
}

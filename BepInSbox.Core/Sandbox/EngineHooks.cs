using HarmonyLib;
using System;
using System.Collections.Generic;
using System.Linq;
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

        delegate void ComponentUpdateEnabledStatus(Component instance);

        static ComponentUpdateEnabledStatus updateEnabledStatus = AccessTools.MethodDelegate<ComponentUpdateEnabledStatus>(AccessTools.Method(typeof(Component), "UpdateEnabledStatus"));

        delegate void SceneAddObjectToDirectory(Scene instance, object obj);

        static SceneAddObjectToDirectory addObjectToDirectory = AccessTools.MethodDelegate<SceneAddObjectToDirectory>(AccessTools.Method(typeof(Scene), "AddObjectToDirectory"));

        /// <summary>
        ///     We have a handle to the manager object here, since we don't have access to NetChainloader from this assembly, and we can't access the object any other way.
        /// </summary>
        internal static GameObject ManagerObject { get; set; }

        internal static HashSet<Type> ImplementsOnUpdateComponentList { get; } = new HashSet<Type>();

        internal static HashSet<Type> ImplementsOnFixedUpdateComponentList { get; } = new HashSet<Type>();

        internal static HashSet<Type> ImplementsOnPreRenderComponentList { get; } = new HashSet<Type>();

        internal static void Patch()
        {
            //GameInstance is an internal type in an assembly with no public types
            MethodBase openStartupSceneMethod = AccessTools.Method(AccessTools.AllTypes().Where(type => type.Name == "GameInstance").First(), "OpenStartupScene");

            HarmonyInstance.Patch(openStartupSceneMethod, postfix: new HarmonyMethod(PatchOpenStartupScene));

            HarmonyInstance.PatchAll(typeof(ComponentUpdateFix));
        }

        [HarmonyPostfix]
        internal static void PatchOpenStartupScene()
        {
            Logger.LogDebug("setting scene to gameobject");

            var scene = Game.ActiveScene;

            //Adds the scene reference to the GameObject, in turns allows all Components to have access to a Scene
            AccessTools.PropertySetter(typeof(GameObject), nameof(GameObject.Scene)).Invoke(ManagerObject, [ scene ]);
            ManagerObject.Parent = scene;

            foreach (var component in ManagerObject.Components.GetAll()) 
            {
                //adds the component to the scene's directory, so that their Update methods get called
                addObjectToDirectory(scene, component);

                //Kickstart the component's Awake()
                updateEnabledStatus(component);
            }

            Logger.LogDebug($"new scene: {ManagerObject.Scene}");
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
                                new (Ldloc_1),
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
                                new (Ldloc_1),
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
                                new (Ldloc_1),
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

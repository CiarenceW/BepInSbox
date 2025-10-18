using HarmonyLib;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Sandbox;
using System.Reflection;
using System.Threading.Channels;
using System.Collections.Concurrent;
using BepInEx.Logging;
using System.Reflection.Emit;
using static System.Reflection.Emit.OpCodes;

//if we call it "Sandbox" it'll be annoying anytime we want to call anything in the actual Sandbox namespace
#pragma warning disable IDE0130 // Namespace does not match folder structure
namespace BepInEx.Core.Sbox
#pragma warning restore IDE0130 // Namespace does not match folder structure
{
    /// <summary>
    /// S&amp;box specific engine hooks
    /// </summary>
    public static class EngineHooks
    {
        static Harmony HarmonyInstance { get; } = new Harmony("BepInEx-EngineHooks");

        static ManualLogSource Logger { get; } = new ManualLogSource("EngineHooks");

        public static void Patch()
        {
            HarmonyInstance.PatchAll(typeof(EngineHooks));
        }

    }
}

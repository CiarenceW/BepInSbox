using BepInEx.Configuration;
using BepInEx.Logging;
using HarmonyLib;
using System;
using System.Collections.Generic;
using System.Linq;

using BepInExLogLevel = BepInEx.Logging.LogLevel;
using SboxLogLevel = Sandbox.LogLevel;

namespace BepInEx.Core.Sbox.Logging
{
    /// <summary>
    ///     Listens for BepInEx log events and prints them to sbox's outputs.
    /// </summary>
    public class SandboxLogListener : ILogListener
    {
        private static readonly ConfigEntry<BepInExLogLevel> ConfigSboxLogLevel = ConfigFile.CoreConfig.Bind(
            "Logging.Sbox", "LogLevels",
            BepInExLogLevel.Fatal | BepInExLogLevel.Error | BepInExLogLevel.Warning | BepInExLogLevel.Message | BepInExLogLevel.Info,
            "What log levels to s&box's output log.");

        private static readonly ConfigEntry<bool> LogConsoleToSbox = ConfigFile.CoreConfig.Bind("Logging",
            "LogConsoleToSboxLog", false,
            "Writes standard output messages to s&box's log.");

        /// <inheritdoc/>
        public BepInExLogLevel LogLevelFilter => ConfigSboxLogLevel.Value;

        /// <inheritdoc/>
        public void Dispose() { }

        delegate void WriteLog(in Sandbox.LogEvent logEvent);

        WriteLog writeLog = AccessTools.MethodDelegate<WriteLog>(AccessTools.Method(typeof(Sandbox.Diagnostics.Logger).Assembly.GetTypes().Where((type) => type.Name == "Logging").First(), "Write"));

        delegate object WrapObjectToHTMLDelegate(object obj, List<object> list);

        WrapObjectToHTMLDelegate wrapObjectToHtml = AccessTools.MethodDelegate<WrapObjectToHTMLDelegate>(AccessTools.Method(typeof(Sandbox.Diagnostics.Logger), "WrapObject"));

        /// <inheritdoc/>
        public void LogEvent(object sender, LogEventArgs eventArgs)
        {
            if (sender is SandboxLogSource || eventArgs.Source is SandboxLogSource || LogConsoleToSbox.Value == false)
                return;

            List<object> argumentsList = new List<object>();

            string htmlMessage = (string)wrapObjectToHtml(eventArgs.Data.ToString(), argumentsList);

            writeLog(new Sandbox.LogEvent()
            {
                Level = BepInExLogLevelToSboxLogLevel(eventArgs.Level),
                Logger = eventArgs.Source.SourceName,
                Exception = eventArgs.Data as Exception,
                Message = eventArgs.Data.ToString(),
                HtmlMessage = htmlMessage,
                Stack = (eventArgs.Data as Exception)?.StackTrace,
                Time = DateTime.Now,
                Arguments = argumentsList.ToArray()
            });
        }

        private static SboxLogLevel BepInExLogLevelToSboxLogLevel(BepInExLogLevel logLevel)
        {
            switch (logLevel)
            {
                case BepInExLogLevel.Debug:
                    return SboxLogLevel.Info;
                case BepInExLogLevel.Warning:
                    return SboxLogLevel.Warn;
                case BepInExLogLevel.Fatal:
                case BepInExLogLevel.Error:
                    return SboxLogLevel.Error;
                case BepInExLogLevel.Info:
                default:
                    return SboxLogLevel.Trace;
            }
        }
    }
}

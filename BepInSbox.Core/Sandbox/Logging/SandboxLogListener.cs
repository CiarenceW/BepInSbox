using BepInSbox.Configuration;
using BepInSbox.Logging;
using HarmonyLib;
using System;
using System.Collections.Generic;
using System.Linq;

using BepInLogLevel = BepInSbox.Logging.LogLevel;
using SboxLogLevel = Sandbox.LogLevel;

namespace BepInSbox.Core.Sbox.Logging
{
    /// <summary>
    ///     Listens for BepInSbox log events and prints them to sbox's outputs.
    /// </summary>
    public class SandboxLogListener : ILogListener
    {
        private static readonly ConfigEntry<BepInLogLevel> ConfigSboxLogLevel = ConfigFile.CoreConfig.Bind(
            "Logging.Sbox", "LogLevels",
            BepInLogLevel.Fatal | BepInLogLevel.Error | BepInLogLevel.Warning | BepInLogLevel.Message | BepInLogLevel.Info,
            "What log levels to s&box's output log.");

        private static readonly ConfigEntry<bool> LogConsoleToSbox = ConfigFile.CoreConfig.Bind("Logging",
            "LogConsoleToSboxLog", false,
            "Writes standard output messages to s&box's log.");

        /// <inheritdoc/>
        public BepInLogLevel LogLevelFilter => ConfigSboxLogLevel.Value;

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
                Level = BepInLogLevelToSboxLogLevel(eventArgs.Level),
                Logger = eventArgs.Source.SourceName,
                Exception = eventArgs.Data as Exception,
                Message = eventArgs.Data.ToString(),
                HtmlMessage = htmlMessage,
                Stack = (eventArgs.Data as Exception)?.StackTrace,
                Time = DateTime.Now,
                Arguments = argumentsList.ToArray()
            });
        }

        private static SboxLogLevel BepInLogLevelToSboxLogLevel(BepInLogLevel logLevel)
        {
            switch (logLevel)
            {
                case BepInLogLevel.Debug:
                    return SboxLogLevel.Info;
                case BepInLogLevel.Warning:
                    return SboxLogLevel.Warn;
                case BepInLogLevel.Fatal:
                case BepInLogLevel.Error:
                    return SboxLogLevel.Error;
                case BepInLogLevel.Info:
                default:
                    return SboxLogLevel.Trace;
            }
        }
    }
}

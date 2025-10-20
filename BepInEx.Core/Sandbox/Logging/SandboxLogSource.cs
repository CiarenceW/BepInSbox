using BepInEx.Logging;
using Sandbox;
using System;
using System.Linq;

using BepInExLogLevel = BepInEx.Logging.LogLevel;
using SboxLogLevel = Sandbox.LogLevel;

namespace BepInEx.Core.Sbox.Logging
{
    /// <summary>
    ///     Logs s&amp;box entries to standard outputs.
    /// </summary>
    public class SandboxLogSource : ILogSource
    {
        public SandboxLogSource() 
        {
#pragma warning disable CS8974 // Converting method group to non-delegate type

            //Sandbox.Diagnostics.Logging.OnMessage += HandleSandboxMessage;
            var loggingType = typeof(Sandbox.Diagnostics.Logger).Assembly.GetTypes().Where((type) => type.Name == "Logging").First();

            var onMessageMethod = loggingType.GetMethod("add_OnMessage", (System.Reflection.BindingFlags)int.MaxValue);

            onMessageMethod.Invoke(null, [HandleSandboxMessage]);

#pragma warning restore CS8974 // Converting method group to non-delegate type
        }

        /// <inheritdoc/>
        public string SourceName => "s&box Log";

        /// <inheritdoc/>
        public event EventHandler<LogEventArgs> LogEvent;

        private void HandleSandboxMessage(LogEvent logEvent)
        {
            LogEvent?.Invoke(this, new LogEventArgs((logEvent.Exception != null) ? $"Stack trace: {logEvent.Stack}" : "" + logEvent.Message, SandboxLogLevelToBepInExLogLevel(logEvent.Level), this));
        }

        private static BepInExLogLevel SandboxLogLevelToBepInExLogLevel(SboxLogLevel sboxLogLevel)
        {
            switch (sboxLogLevel)
            {
                case SboxLogLevel.Info:
                    return BepInExLogLevel.Message;
                case SboxLogLevel.Warn:
                    return BepInExLogLevel.Warning;
                case SboxLogLevel.Error:
                    return BepInExLogLevel.Error;
                default:
                    return BepInExLogLevel.Info;
            }
        }

        /// <inheritdoc/>
        public void Dispose()
        {
#pragma warning disable CS8974 // Converting method group to non-delegate type

            //Sandbox.Diagnostics.Logging.OnMessage -= HandleSandboxMessage;
            var loggingType = typeof(Logger).Assembly.GetTypes().Where((type) => type.Name == "Logging").First();

            var onMessageMethod = loggingType.GetMethod("remove_OnMessage", (System.Reflection.BindingFlags)int.MaxValue);

            onMessageMethod.Invoke(null, [HandleSandboxMessage]);

#pragma warning restore CS8974 // Converting method group to non-delegate type
        }
    }
}

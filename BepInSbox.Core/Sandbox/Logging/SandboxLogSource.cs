using BepInSbox.Logging;
using Sandbox;
using System;
using System.Linq;

using BepInLogLevel = BepInSbox.Logging.LogLevel;
using SboxLogLevel = Sandbox.LogLevel;

namespace BepInSbox.Core.Sbox.Logging
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
            LogEvent?.Invoke(this, new LogEventArgs((logEvent.Exception != null) ? $"Stack trace: {logEvent.Stack}" : "" + logEvent.Message, SandboxLogLevelToBepInLogLevel(logEvent.Level), this));
        }

        private static BepInLogLevel SandboxLogLevelToBepInLogLevel(SboxLogLevel sboxLogLevel)
        {
            switch (sboxLogLevel)
            {
                case SboxLogLevel.Info:
                    return BepInLogLevel.Message;
                case SboxLogLevel.Warn:
                    return BepInLogLevel.Warning;
                case SboxLogLevel.Error:
                    return BepInLogLevel.Error;
                default:
                    return BepInLogLevel.Info;
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

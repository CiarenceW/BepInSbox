using BepInSbox.Logging;

namespace BepInSbox.Preloader.Core;

public static class PreloaderLogger
{
    public static ManualLogSource Log { get; } = Logger.CreateLogSource("Preloader");
}

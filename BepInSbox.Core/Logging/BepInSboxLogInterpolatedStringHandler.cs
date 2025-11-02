using System;
using System.Runtime.CompilerServices;
using System.Text;
using BepInSbox.Logging;

namespace BepInSbox.Core.Logging.Interpolation;

/// <summary>
///     Interpolated string handler for BepInSbox <see cref="Logger" />. This allows to conditionally skip logging certain
///     messages and speed up logging in certain places.
/// </summary>
/// <remarks>
///     The class isn't meant to be constructed manually.
///     Instead, use <see cref="ManualLogSource.Log(BepInSbox.Logging.LogLevel,BepInSboxLogInterpolatedStringHandler)" /> with
///     string interpolation.
/// </remarks>
[InterpolatedStringHandler]
public class BepInSboxLogInterpolatedStringHandler
{
    // See https://source.dot.net/#System.Private.CoreLib/DefaultInterpolatedStringHandler.cs,29
    private const int GUESSED_LENGTH_PER_HOLE = 11;

    // We can't use an array pool to support net35 builds, so default to StringBuilder
    private readonly StringBuilder sb;

    /// <summary>
    ///     Constructs a log handler.
    /// </summary>
    /// <param name="literalLength">Length of the literal string.</param>
    /// <param name="formattedCount">Number for formatted items.</param>
    /// <param name="logLevel">Log level the message belongs to.</param>
    /// <param name="isEnabled">Whether this string should be logged.</param>
    public BepInSboxLogInterpolatedStringHandler(int literalLength,
                                               int formattedCount,
                                               LogLevel logLevel,
                                               out bool isEnabled)
    {
        Enabled = (logLevel & Logger.ListenedLogLevels) != LogLevel.None;
        isEnabled = Enabled;
        sb = Enabled ? new StringBuilder(literalLength + formattedCount * GUESSED_LENGTH_PER_HOLE) : null;
    }

    /// <summary>
    ///     Whether the interpolation is enabled and string will be logged.
    /// </summary>
    public bool Enabled { get; }

    /// <summary>
    ///     Appends a literal string to the interpolation.
    /// </summary>
    /// <param name="s">String to append.</param>
    public void AppendLiteral(string s)
    {
        if (!Enabled)
            return;
        sb.Append(s);
    }

    /// <summary>
    ///     Appends a value to the interpolation.
    /// </summary>
    /// <param name="t">Value to append.</param>
    /// <typeparam name="T">Type of the value to append.</typeparam>
    public void AppendFormatted<T>(T t)
    {
        if (!Enabled)
            return;

        sb.Append(t);
    }

    /// <summary>
    ///     Append a formattable item.
    /// </summary>
    /// <param name="t">Item to append.</param>
    /// <param name="format">Format to append with.</param>
    /// <typeparam name="T">Item type.</typeparam>
    public void AppendFormatted<T>(T t, string format) where T : IFormattable
    {
        if (!Enabled)
            return;

        sb.Append(t?.ToString(format, null));
    }

    /// <summary>
    ///     Append an IntPtr.
    /// </summary>
    /// <param name="t">Item to append.</param>
    /// <param name="format">Format to append with.</param>
    public void AppendFormatted(IntPtr t, string format)
    {
        if (!Enabled)
            return;

        sb.Append(t.ToString(format));
    }

    /// <inheritdoc />
    public override string ToString() => sb?.ToString() ?? string.Empty;
}

/// <inheritdoc />
[InterpolatedStringHandler]
public class BepInSboxFatalLogInterpolatedStringHandler : BepInSboxLogInterpolatedStringHandler
{
    /// <inheritdoc />
    public BepInSboxFatalLogInterpolatedStringHandler(int literalLength,
                                                    int formattedCount,
                                                    out bool isEnabled) : base(literalLength, formattedCount,
                                                                               LogLevel.Fatal, out isEnabled) { }
}

/// <inheritdoc />
[InterpolatedStringHandler]
public class BepInSboxErrorLogInterpolatedStringHandler : BepInSboxLogInterpolatedStringHandler
{
    /// <inheritdoc />
    public BepInSboxErrorLogInterpolatedStringHandler(int literalLength,
                                                    int formattedCount,
                                                    out bool isEnabled) : base(literalLength, formattedCount,
                                                                               LogLevel.Error, out isEnabled) { }
}

/// <inheritdoc />
[InterpolatedStringHandler]
public class BepInSboxWarningLogInterpolatedStringHandler : BepInSboxLogInterpolatedStringHandler
{
    /// <inheritdoc />
    public BepInSboxWarningLogInterpolatedStringHandler(int literalLength,
                                                      int formattedCount,
                                                      out bool isEnabled) : base(literalLength, formattedCount,
        LogLevel.Warning, out isEnabled) { }
}

/// <inheritdoc />
[InterpolatedStringHandler]
public class BepInSboxMessageLogInterpolatedStringHandler : BepInSboxLogInterpolatedStringHandler
{
    /// <inheritdoc />
    public BepInSboxMessageLogInterpolatedStringHandler(int literalLength,
                                                      int formattedCount,
                                                      out bool isEnabled) : base(literalLength, formattedCount,
        LogLevel.Message, out isEnabled) { }
}

/// <inheritdoc />
[InterpolatedStringHandler]
public class BepInSboxInfoLogInterpolatedStringHandler : BepInSboxLogInterpolatedStringHandler
{
    /// <inheritdoc />
    public BepInSboxInfoLogInterpolatedStringHandler(int literalLength,
                                                   int formattedCount,
                                                   out bool isEnabled) : base(literalLength, formattedCount,
                                                                              LogLevel.Info, out isEnabled) { }
}

/// <inheritdoc />
[InterpolatedStringHandler]
public class BepInSboxDebugLogInterpolatedStringHandler : BepInSboxLogInterpolatedStringHandler
{
    /// <inheritdoc />
    public BepInSboxDebugLogInterpolatedStringHandler(int literalLength,
                                                    int formattedCount,
                                                    out bool isEnabled) : base(literalLength, formattedCount,
                                                                               LogLevel.Debug, out isEnabled) { }
}

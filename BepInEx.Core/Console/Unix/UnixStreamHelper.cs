using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;
using MonoMod.Utils;

namespace BepInEx.Unix;

internal static class UnixStreamHelper
{
    public delegate int dupDelegate(int fd);

    public delegate int fcloseDelegate(IntPtr stream);

    public delegate IntPtr fdopenDelegate(int fd, string mode);

    public delegate int fflushDelegate(IntPtr stream);

    public delegate IntPtr freadDelegate(IntPtr ptr, IntPtr size, IntPtr nmemb, IntPtr stream);

    public delegate int fwriteDelegate(IntPtr ptr, IntPtr size, IntPtr nmemb, IntPtr stream);

    public delegate int isattyDelegate(int fd);

    public static dupDelegate dup;

    public static fdopenDelegate fdopen;

    public static freadDelegate fread;

    public static fwriteDelegate fwrite;

    public static fcloseDelegate fclose;

    public static fflushDelegate fflush;

    public static isattyDelegate isatty;

    static UnixStreamHelper()
    {
        #warning someone needs to test this eventually ^^

        if (DynDll.TryOpenLibrary("libc", out nint libcPtr))
        {
            if (libcPtr.TryGetExport(nameof(dup), out nint dupFptr))
            {
                dup = Marshal.GetDelegateForFunctionPointer<dupDelegate>(dupFptr);
            }

            if (libcPtr.TryGetExport(nameof(fdopen), out nint fdopenFptr))
            {
                fdopen = Marshal.GetDelegateForFunctionPointer<fdopenDelegate>(fdopenFptr);
            }

            if (libcPtr.TryGetExport(nameof(fread), out nint freadFptr))
            {
                fread = Marshal.GetDelegateForFunctionPointer<freadDelegate>(freadFptr);
            }

            if (libcPtr.TryGetExport(nameof(fwrite), out nint fwriteFptr))
            {
                fwrite = Marshal.GetDelegateForFunctionPointer<fwriteDelegate>(fwriteFptr);
            }

            if (libcPtr.TryGetExport(nameof(fclose), out nint fcloseFptr))
            {
                fclose = Marshal.GetDelegateForFunctionPointer<fcloseDelegate>(fcloseFptr);
            }

            if (libcPtr.TryGetExport(nameof(fflush), out nint fflushFptr))
            {
                fflush = Marshal.GetDelegateForFunctionPointer<fflushDelegate>(fflushFptr);
            }

            if (libcPtr.TryGetExport(nameof(isatty), out nint isattyFptr))
            {
                isatty = Marshal.GetDelegateForFunctionPointer<isattyDelegate>(isattyFptr);
            }
        }
    }

    public static Stream CreateDuplicateStream(int fileDescriptor)
    {
        var newFd = dup(fileDescriptor);

        return new UnixStream(newFd, FileAccess.Write);
    }
}

set "CONFIG=-c Debug"

IF "%~1" NEQ "" set "CONFIG=-c %1"

dotnet build "./BepInEx.Core/BepInEx.Core.csproj" %CONFIG%
dotnet build "./BepInEx.Preloader.Core/BepInEx.Preloader.Core.csproj" %CONFIG%
dotnet build "./Runtimes/NET/BepInEx.NET.Common/BepInEx.NET.Common.csproj" %CONFIG%
dotnet build "./Runtimes/NET/BepInEx.NET.CoreCLR/BepInEx.NET.CoreCLR.csproj" %CONFIG%


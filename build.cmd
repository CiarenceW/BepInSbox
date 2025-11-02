set "CONFIG=-c Debug"

IF "%~1" NEQ "" set "CONFIG=-c %1"

dotnet build "./BepInSbox.Core/BepInSbox.Core.csproj" %CONFIG%
dotnet build "./BepInSbox.Preloader.Core/BepInSbox.Preloader.Core.csproj" %CONFIG%
dotnet build "./Runtimes/NET/BepInSbox.NET.Common/BepInSbox.NET.Common.csproj" %CONFIG%
dotnet build "./Runtimes/NET/BepInSbox.NET.CoreCLR/BepInSbox.NET.CoreCLR.csproj" %CONFIG%


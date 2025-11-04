set "CONFIG=-c Debug"

IF "%~1" NEQ "" set "CONFIG=-c %1"

dotnet build "./BepInSbox.sln" %CONFIG%

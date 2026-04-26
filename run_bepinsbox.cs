#!/usr/bin/env dotnet

//I think it's slighty faster to build JIT? this doesn't really need to be AOT anyways
#:property PublishAot = false

using System.Diagnostics;
using System.Runtime.InteropServices;

//later: get executable name from steam's launch args
var executable_name = "";

//TODO: add stuff to infer the executable name and root path from the steam launch args

var root_path = Environment.CurrentDirectory;
var doorstop_name = "libDoorstop";

var doorstop_extension = OperatingSystem.IsLinux() ? ".so" : ".dylib";

//load the doorstop!!
Environment.SetEnvironmentVariable("LD_PRELOAD", $"{Path.Combine(root_path, doorstop_name + doorstop_extension)}:{Environment.GetEnvironmentVariable("LD_PRELOAD")}");

Process.Start
	( new ProcessStartInfo(Path.Combine(root_path, executable_name), Environment.GetCommandLineArgs()) 
	{ 
		UseShellExecute = true 
	} 
	);
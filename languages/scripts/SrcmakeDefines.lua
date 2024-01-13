-- SrcmakeDefines.lua

-- NOTE: Do NOT require, include or reference this file from your language script. This file is NOT
-- used and is only provided for the sake of intellisense when writing language scripts.

-- The following global variables will be defined by Srcmake before loading the language script.
SMFileName  = ""  -- A string containing the name of the file without the directory and file extention.
SMSafeName  = ""  -- A string containing the name of the file with all unsafe type name characters replaced. Use this for class/struct type names ect.
SMArguments = { } -- An array of strings containing the extra arguments given when running Srcmake (everything after the name flag).

-- Example.lua
-- Example Srcmake language script.

-- The following global variables will be defined by Srcmake before loading the language script:
--   SMFileName  -- A string containing the name of the file without the directory and file extention.
--   SMSafeName  -- A string containing the name of the file with all unsafe type name characters replaced. Use this for class/struct type names ect.
--   SMArguments -- An array of strings containing the extra arguments given when running Srcmake (everything after the name flag).

-- Example local variable used to hold data from `ProcessArguments`.
local y_arg = false

-- `ProcessArguments` is an optional function that is called only once, after the script is loaded.
-- This is where you can initialise your script and variables from `SMArguments`.
function ProcessArguments()

	-- Iterate through the arguments.
	for i = 1, #SMArguments do
		-- Convert argument to lowercase to allow case-insensitive arguments.
		local low = string.lower( SMArguments[ i ] )

		-- Check possible aliases for an argument.
		if( low == "--y" or low == "--why" ) then
			y_arg = true
		end
	end

end

-- `ReplaceMacro` takes in a macro string (surrounded by `$`s) and returns the string it should be
-- replaced with. `macro` is always uppercase. `macro` should be returned if the script is not
-- intended to process the macro. Replacing a macro with another macro or a combination is possible
-- and will be handled as expected. Universal macros are replaced after all language specific macros.
function ReplaceMacro( macro )

	-- Replace our custom macro based on data we processed in ProcessArguments.
	if macro == "$WHY$" then
		if y_arg == true then
			return "Yes"
		else
			return "No"
		end
	end

	-- Return the same macro back since we did not handle it.
	return macro
end

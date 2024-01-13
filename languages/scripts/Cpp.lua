-- Cpp.lua
-- Srcmake template for the C++ programming language

local header_guards    = string.upper( SMSafeName ) .. "_HPP"
local include_string   = ""
local namespace_string = ""
local is_virtual       = false

function ProcessArguments()
	local includes  = {}
	local ininclude = false

	for i = 1, #SMArguments do
		local low = string.lower( SMArguments[ i ] )

		if #SMArguments[ i ] > 0 and SMArguments[ i ][ 1 ] == "-" then
			ininclude = false
		end

		if ininclude == false then
			if low == "--i" or low == "--include" then
				ininclude = true
			end
		else
			table.insert( includes, "#include <" .. SMArguments[ i ] .. ">" )
		end

		if( low == "--ns" or low == "--namespace" ) and i < #SMArguments then
			namespace_string = "namespace " .. SMArguments[ i ] .. "\n{"
			i = i + 1
		elseif low == "--v" or low == "--virtual" then
			is_virtual = true
		end

	end

	if #includes > 0 then
		include_string = table.concat( includes, "\n" )
	end
end
function ReplaceMacro( macro )
	if macro == "$HEADER_EXT$" then
		return "hpp"
	elseif macro == "$SOURCE_EXT$" then
		return "cpp"
	elseif macro == "$INLINE_EXT$" then
		return "inl"
	elseif macro == "$HEADER_GUARD$" then
		return header_guards
	elseif macro == "$INCLUDES$" then
		return include_string
	elseif macro == "$NAMESPACE_BEGIN$" then
		return namespace_string
	elseif macro == "$NAMESPACE_END$" then
		if #namespace_string > 0 then
			return "}"
		else
			return ""
		end
	elseif macro == "$VIRTUAL$" then
		if is_virtual then
			return "virtual"
		else
			return ""
		end
	end

	return macro
end

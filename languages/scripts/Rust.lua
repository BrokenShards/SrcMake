-- Rust.lua
-- Srcmake template for the Rust programming language

local using_string = ""

function ProcessArguments()
	local usings       = {}
	local inusings     = false

	for i = 1, #SMArguments do
		local low = string.lower( SMArguments[ i ] )

		if #SMArguments[ i ] > 0 and SMArguments[ i ][ 1 ] == "-" then
			inusings = false
		end

		if inusings == false then
			if low == "--u" or low == "--use" then
				inusings = true
			end
		else
			table.insert( usings, "use " .. SMArguments[ i ] .. ";" )
		end
	end

	if #usings > 0 then
		using_string = table.concat( usings, "\n" )
	end
end
function ReplaceMacro( macro )
	if macro == "$USES$" then
		return using_string
	end

	return macro
end

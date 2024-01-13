-- CSharp.lua
-- Srcmake template for the CSharp programming language

local using_string     = ""
local namespace_string = ""
local access_string    = ""
local modifier_string  = ""
local is_virtual       = false

function ProcessArguments()
	local usings       = {}
	local inusings     = false
	local is_public    = false
	local is_private   = false
	local is_protected = false
	local is_abstract  = false
	local is_partial   = false
	local is_static    = false
	local is_sealed    = false

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
			table.insert( usings, "using " .. SMArguments[ i ] .. ";" )
		end

		if( low == "--ns" or low == "--namespace" ) and i < #SMArguments then
			namespace_string = "namespace " .. SMArguments[ i ] .. "\n{"
			i = i + 1
		elseif low == "--v" or low == "--virtual" then
			is_virtual = true
		elseif low == "--pub" or low == "--public" then
			is_public = true
		elseif low == "--prot" or low == "--protected" then
			is_protected = true
		elseif low == "--priv" or low == "--private" then
			is_private = true
		elseif low == "--ab" or low == "--abstract" then
			is_abstract = true
		elseif low == "--pt" or low == "--partial" then
			is_partial = true
		elseif low == "--st" or low == "--static" then
			is_static = true
		elseif low == "--sl" or low == "--sealed" then
			is_sealed = true
		end
	end

	if( is_public and is_protected ) or ( is_public and is_private ) or ( is_protected and is_private )
	then
		print( "Only one C# class accessability modifier can be defined at one time. Defaulting to private.\n" )
		is_public     = false
		is_protected  = false
		is_private    = true
	elseif( not is_public ) and ( not is_protected ) and ( not is_private ) then
		is_private = true
	end

	if( is_abstract and is_partial ) or ( is_abstract and is_static ) or
	  ( is_abstract and is_sealed )  or ( is_partial  and is_static ) or
	  ( is_partial  and is_sealed )  or ( is_static   and is_sealed )
	then
		print( "Only one C# class modifier can be defined at one time. Class modifiers will be ignored.\n" )
		is_abstract = false;
		is_partial  = false;
		is_static   = false;
		is_sealed   = false;
	end

	if #usings > 0 then
		using_string = table.concat( usings, "\n" )
	end

	if is_public then
		access_string = "public"
	elseif is_protected then
		access_string = "protected"
	else
		access_string = "private"
	end

	if is_abstract then
		modifier_string = "abstract"
	elseif is_partial then
		modifier_string = "partial"
	elseif is_static then
		modifier_string = "static"
	elseif is_sealed then
		modifier_string = "sealed"
	end
end
function ReplaceMacro( macro )
	if macro == "$USINGS$" then
		return using_string
	elseif macro == "$CLASS_MODIFIER$" then
		return modifier_string
	elseif macro == "$ACCESS$" then
		return access_string
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

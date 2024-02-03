//
// $FILE_NAME$.$FILE_EXT$
// $AUTHOR$ $DATETIME$
//

#include "$NAME$.$HEADER_EXT$"

$NAMESPACE_BEGIN$

$NAME$::$NAME$() noexcept
{ }
$NAME$::~$NAME$() noexcept
{ }

$NAME$ &$NAME$::getInstance() noexcept
{
	static $NAME$ instance;
	return instance;
}

$NAMESPACE_END$

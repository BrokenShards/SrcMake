//
// $FILE_NAME$
// $AUTHOR$ $DATETIME$
//

#include "$NAME$.$HEADER_EXT$"

$NAMESPACE_BEGIN$

std::unique_ptr<$NAME$> $NAME$::_instance;
std::once_flag $NAME$::_onceFlag;

$NAME$::$NAME$() noexcept
{ }
$NAME$::~$NAME$() noexcept
{ }

$NAME$ &$NAME$::getInstance() noexcept
{
	std::call_once( _onceFlag, []
	{
		_instance.reset( new $NAME$() );
	} );

	return *_instance.get();
}

$NAMESPACE_END$

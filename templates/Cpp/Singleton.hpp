//
// $FILE_NAME$.$FILE_EXT$
// $AUTHOR$ $DATETIME$
//

#ifndef $HEADER_GUARD$
#define $HEADER_GUARD$

$INCLUDES$

$NAMESPACE_BEGIN$

class $NAME$
{
public:
	$VIRTUAL$ ~$NAME$() noexcept;

	static $NAME$ &getInstance() noexcept;

private:
	$NAME$() noexcept;
	$NAME$( const $NAME$ & ) = delete;
	$NAME$ &operator=( const $NAME$ & ) = delete;
};

$NAMESPACE_END$

#endif

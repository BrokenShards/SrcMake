//
// $FILE_NAME$
// $AUTHOR$ $DATETIME$
//

#ifndef $HEADER_GUARD$
#define $HEADER_GUARD$

$INCLUDES$
#include <memory>
#include <mutex>

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

	static std::unique_ptr<$NAME$> _instance;
	static std::once_flag _onceFlag;
};

$NAMESPACE_END$

#endif

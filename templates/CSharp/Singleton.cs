// $FILE_NAME$.$FILE_EXT$ //

$USINGS$

$NAMESPACE_BEGIN$
$ACCESS$ $CLASS_MODIFIER$ class $NAME$
{
	private $NAME$()
	{ }

	public static $NAME$ Instance
	{
		get
		{
			if( _instance == null )
			{
				lock( _syncRoot )
				{
					if( _instance == null )
						_instance = new $NAME$();
				}
			}

			return _instance;
		}
	}

	private static volatile $NAME$ _instance;
	private static readonly object _syncRoot = new object();
}
$NAMESPACE_END$

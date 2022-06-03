# adopted from: https://gist.github.com/Morreski/c1d08a3afa4040815eafd3891e16b945?permalink_comment_id=3521580#gistcomment-3521580
from functools import lru_cache, wraps
from time import monotonic_ns


def timed_lru_cache(_func=None, *, seconds: int = 600, maxsize: int = 128, typed: bool = False):
    """Extension of functools lru_cache with a timeout

    Parameters:
    seconds (int): Timeout in seconds to clear the WHOLE cache, default = 10 minutes
    maxsize (int): Maximum Size of the Cache
    typed (bool): Same value of different type will be a different entry

    """

    def wrapper_cache(f):
        f = lru_cache(maxsize=maxsize, typed=typed)(f)
        f.delta = seconds * 10**9  # type: ignore
        f.expiration = monotonic_ns() + f.delta  # type: ignore

        @wraps(f)
        def wrapped_f(*args, **kwargs):
            if monotonic_ns() >= f.expiration:  # type: ignore
                f.cache_clear()
                f.expiration = monotonic_ns() + f.delta  # type: ignore
            return f(*args, **kwargs)

        wrapped_f.cache_info = f.cache_info
        wrapped_f.cache_clear = f.cache_clear
        return wrapped_f

    # To allow decorator to be used without arguments
    if _func is None:
        return wrapper_cache
    else:
        return wrapper_cache(_func)

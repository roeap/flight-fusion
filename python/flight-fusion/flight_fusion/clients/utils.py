from __future__ import annotations

import asyncio
from threading import Thread
from typing import Any, Callable, Coroutine, TypeVar

T = TypeVar("T")


class _RunThread(Thread):
    def __init__(self, func, args, kwargs):
        self.func = func
        self.args = args
        self.kwargs = kwargs
        super().__init__()

    def run(self):
        self.result = asyncio.run(self.func(*self.args, **self.kwargs))


def run_async(func: Callable[..., Coroutine[Any, Any, T]], *args, **kwargs) -> T:
    """Helper function to execute async code.

    Will use current loop if executed inside a running loop, will use asyncio.run otherwise.
    """
    try:
        loop = asyncio.get_running_loop()
    except RuntimeError:
        loop = None
    if loop and loop.is_running():
        thread = _RunThread(func, args, kwargs)
        thread.start()
        thread.join()
        return thread.result
    else:
        return asyncio.run(func(*args, **kwargs))

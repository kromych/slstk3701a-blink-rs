MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = ...
  RAM : ORIGIN = ..., LENGTH = ...
}

_heap_size = ...;
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

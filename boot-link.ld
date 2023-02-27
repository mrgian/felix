ENTRY(_start)

SECTIONS {
    . = 0x500;
    _stack_start = .;
    . = 0x7c00;
    _stack_end = .;

    _message = .;

    .message :
    {
        BYTE(0X48)
        BYTE(0X0d)
        BYTE(0X0a)
    }

    . = 0x7c00 + 510;

    .magic_number :
    {
        SHORT(0xaa55)
    }

    _second_stage_start = .;
}
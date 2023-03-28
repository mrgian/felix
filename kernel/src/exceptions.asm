.section .exceptions, "awx"
.code32

# interrupt service routines for various exceptions

div_error:
   push 0x00
   call exception_handler
   jmp spin

invalid_opcode:
   push 0x06
   call exception_handler
   jmp spin

double_fault:
   push 0x08
   call exception_handler
   jmp spin

general_protection_fault:
   push 0x0d
   call exception_handler
   jmp spin

page_fault:
   push 0x0e
   call exception_handler
   jmp spin

generic_handler:
   push 0xff
   call exception_handler
   jmp spin

spin:
    hlt
    jmp spin

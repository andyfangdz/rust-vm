MOVI R0 50
JEQI R0 1 1
JA 3
MOVI R0 1
INT 0
HALT
JEQI R0 2 1
JA 3
MOVI R0 12
INT 0
HALT
MOVI R1 1
MOVI R2 1
MOVI R4 2
JEQ R0 R4 5
MOV R3 R1
ADD R1 R2
MOV R2 R3
ADDI R4 1
JA -6
MOV R0 R1
INT 0
HALT
.code16
.global _start

.set LOG_DEVICE_ADDR, 0x500
.set UART_DATA_BITS,  8
.set UART_PARITY,     0
.set UART_STOP_BITS,  1
.set UART_TX_RX_ADDR, 0x3F8
.set UART_LCR_ADDR,   0x3FB
.set UART_LSR_ADDR,   0x3FD
.set UART_LSR_TX_BIT, 0x20

_start:
    movw $UART_LCR_ADDR, %dx
    movb $UART_DATA_BITS, %al
    orb  $UART_PARITY, %al
    orb  $UART_STOP_BITS, %al
    outb %al, (%dx)             # Configure UART settings

    movw $LOG_DEVICE_ADDR, %si  # Prepare character pointer

loop:
    movw $UART_LSR_ADDR, %dx
    inb (%dx), %al              # Read LSR
    testb $UART_LSR_TX_BIT, %al # Test the THR empty bit
    jz loop                     # Loop if the bit is not set

    movb (%si), %al             # Load character from memory
    cmpb $0, %al
    je done

    movw $UART_TX_RX_ADDR, %dx
    outb %al, (%dx)             # Transmit character

    inc %si                     # Increment pointer to next character
    jmp loop

done:
    hlt

.fill 510 - (. - _start), 1, 0
.word 0xaa55

.global _start

.set LOG_DEVICE_ADDR, 0xA3F00000
.set UART_DATA_BITS,  8
.set UART_PARITY,     0
.set UART_STOP_BITS,  1
.set UART_TX_RX_ADDR, 0x40100000
.set UART_LCR_ADDR,   0x4010000C
.set UART_LSR_ADDR,   0x40100014
.set UART_LSR_TX_BIT, 0x20

// trap table vector (instruction executed on trap)
_start:
_trap_table:
    // Each trap instruction loads the address of its trap handler.
    ldr pc, $handlers + 0	 // Reset
    ldr pc, $handlers + 4	 // Supervisor Call
    ldr pc, $handlers + 8	 // Supervisor Call
    ldr pc, $handlers + 12	 // Prefetch Abort
    ldr pc, $handlers + 16	 // Data Abort
    nop				         // Reserved// no-op.
    ldr pc, $handlers + 20	 // Regular Interrupt
    ldr pc, $handlers + 24	 // Fast Interrupt

// pointers to trap handler routines
handlers:
    .int reset, reset, reset, reset, reset, reset, reset

// reset routine
reset:
    mrs r0, cpsr	            // Get status register
    bic r0, r0, #0xff	        // Clear status register bits 0-7
    orr r0, r0, #0xd3	        // Mask with 0x11010011
    msr cpsr_fc, r0	            // Load value into status register
    b init		                // Jump to looping routine

init:
    ldr r1, =UART_LCR_ADDR
    mov r0, #UART_DATA_BITS
    orr r0, r0, #UART_PARITY
    orr r0, r0, #UART_STOP_BITS
    str r0, [r1]                // Configure UART settings

    ldr r0, =LOG_DEVICE_ADDR    // Prepare character pointer
    ldr r1, =UART_TX_RX_ADDR    // Prepare transmission pointer
    ldr r2, =UART_LSR_ADDR      // Prepare status register pointer

loop:
    ldr r3, [r2]                // Read LSR
    tst r3, #UART_LSR_TX_BIT    // Test the THR empty bit
    beq loop                    // Loop if the bit is not set

    ldrb r3, [r0]               // Load character from memory
    cmp r3, #0
    beq done

    strb r3, [r1]               // Transmit character

    add r0, r0, #1              // Increment pointer to next character
    b loop

done:
    b done

.fill 510 - (. - _start), 1, 0
.word 0xaa55

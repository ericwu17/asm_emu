mov r0 0x500
mov r1 0
mov [PUSH_BTNS_ADDR] r1
mov [LED_ADDR] r1
mov [COLUMN_0_ADDR] r1
mov [COLUMN_1_ADDR] r1
mov [COLUMN_2_ADDR] r1
mov [COLUMN_3_ADDR] r1
mov [COLUMN_4_ADDR] r1
mov [COLUMN_5_ADDR] r1
mov [COLUMN_6_ADDR] r1
mov r15 3  ; current cursor
mov r14 0  ; next player to move (player 0 goes first, player 1 goes next)


.main_loop_begin
  
  call .clear_screen
  call .draw_screen

  call .wait_for_any_btns_down

  call .handle_btn_press

  

  call .wait_for_all_btns_up
  mov [LED_ADDR] r15

jmp .main_loop_begin



.wait_for_all_btns_up
  mov r1 [PUSH_BTNS_ADDR]
  jnz .wait_for_all_btns_up r1
  ret

.wait_for_any_btns_down
  mov r1 [PUSH_BTNS_ADDR]
  jz .wait_for_any_btns_down r1
  ret

.clear_screen
  mov r1 0
  mov r2 VGA_BEGIN_ADDR
  .begin_clr_screen_loop
    mov [r2] r1
    mov r3 r2
    sub r3 VGA_END_ADDR
    add r2 1
  jnz .begin_clr_screen_loop r3
  ret

.handle_btn_press
  mov r1 [PUSH_BTNS_ADDR]
  and r1 0x04
  jz .handle_btn_press_priv_1 r1
    call .handle_left_btn
    ret
  .handle_btn_press_priv_1
  mov r1 [PUSH_BTNS_ADDR]
  and r1 0x02
  jz .handle_btn_press_priv_2 r1
    call .handle_right_btn
    ret
  .handle_btn_press_priv_2
  mov r1 [PUSH_BTNS_ADDR]
  and r1 0x01
  jz .handle_btn_press_priv_3 r1
    call .handle_middle_btn
    ret
  .handle_btn_press_priv_3
  ret

.handle_right_btn
  mov r1 r15
  sub r1 6
  jz .handle_right_btn_priv_1 r1
  add r15 1
  ret
.handle_right_btn_priv_1
  mov r15 0
  ret

.handle_middle_btn
  mov r1 COLUMN_0_ADDR
  add r1 r15
  mov r2 [r1]   ; now r2 contains the encoding of the column which we want

  mov r3 r2
  and r3 0x0C00
  jz .handle_middle_btn_priv_1 r3
    ; if we reach here, then the column is full, thus we return
    ret
  .handle_middle_btn_priv_1


  mov r4 0  ; r4 is a counter of how many discs are in the current column
  .handle_middle_btn_begin_loop
    ; r5 is a mask of the current square to check
    mov r5 0x03
    mov r6 r4
    add r6 r6
    shl r5 r6

    and r5 r2
    jz .handle_middle_btn_end_loop r5
    add r4 1
    jmp .handle_middle_btn_begin_loop
  .handle_middle_btn_end_loop
  
  mov r5 1
  shl r5 r4
  shl r5 r4
  shl r5 r14

  or r2 r5
  mov [r1] r2

  ; toggle next player
  jz .handle_middle_btn_priv_2 r14
    mov r14 0
    ret
  .handle_middle_btn_priv_2 r14
    mov r14 1
    ret


.handle_left_btn
  jz .handle_left_btn_priv_1 r15
  sub r15 1
  ret
.handle_left_btn_priv_1
  mov r15 6
  ret


.draw_screen
  ; DRAW CURSOR (cursor position stored in r15)

  ; put the start address in r1
  mov r1 VGA_BEGIN_ADDR
  add r1 r15
  mov r2 0xFFFF
  mov [r1] r2
  add r1 0x0A
  mov r2 0x7FFE
  mov [r1] r2
  add r1 0x0A
  mov r2 0x3FFC
  mov [r1] r2
  add r1 0x0A
  mov r2 0x1FF8
  mov [r1] r2
  add r1 0x0A
  mov r2 0x0FF0
  mov [r1] r2
  add r1 0x0A
  mov r2 0x07E0
  mov [r1] r2
  add r1 0x0A
  mov r2 0x03C0
  mov [r1] r2
  add r1 0x0A
  mov r2 0x0180
  mov [r1] r2

  ; END DRAW CURSOR
  ; DRAW BOARD
  
  mov r7 0 ; column counter
  .draw_screen_start_draw_board_loop
    mov r2 COLUMN_0_ADDR
    add r2 r7
    mov r3 [r2]
    

    mov r1 r7   ; parameter 1: column number
    mov r2 r3 ; parameter 2: number representing column
    push r7
    call .draw_column
    pop r7

    mov r2 r7
    sub r2 6
    add r7 1
  jnz .draw_screen_start_draw_board_loop r2




  ; END DRAW BOARD
ret



.draw_column
  ; paramters: column number and a number representing the state of the column

  mov r3 VGA_END_ADDR
  sub r3 9 
  add r3 r1    ;r3 now points to where we want to draw

  ; we continue drawing until there are no more pieces
  .draw_column_begin_loop
    mov r5 r2 
    and r5 0x0003   ; r5 contains the bitwise representation of the lowest disc
    mov r6 r5
    sub r6 0x01
    jnz .draw_column_priv_1 r6
      mov r1 0xFFFF
      
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      
      jmp .draw_column_cases_end
    .draw_column_priv_1
    mov r6 r5
    sub r6 0x02
    jnz .draw_column_priv_2 r6
      
      mov r1 0x5555
      
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      mov [r3] r1
      sub r3 0x0A
      

      jmp .draw_column_cases_end
    .draw_column_priv_2
    ret ; return because we must have reached the top of the column to draw (no more players pieces)
    .draw_column_cases_end

    shr r2 2 ; shift the column down
    jmp .draw_column_begin_loop
  ret
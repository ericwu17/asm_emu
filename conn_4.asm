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

call .clear_screen
call .draw_grid_dots
mov r1 3
call .draw_cursor_at_column


.main_loop_begin
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

.handle_middle_btn
  mov r1 COLUMN_0_ADDR
  add r1 r15
  mov r2 [r1]   ; now r2 contains the encoding of the column which we want

  ;; check if column is full:
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
    mov r1 r15
    mov r2 r4
    call .draw_player_1_disc
    mov r14 0
    ret
  .handle_middle_btn_priv_2 r14
    mov r1 r15
    mov r2 r4
    call .draw_player_0_disc
    mov r14 1
    ret


.handle_right_btn
  mov r1 r15
  call .clear_cursor_at_column
  mov r1 r15
  sub r1 6
  jz .handle_right_btn_priv_1 r1
  add r15 1
  mov r1 r15
  call .draw_cursor_at_column
  ret
.handle_right_btn_priv_1
  mov r15 0
  mov r1 r15
  call .draw_cursor_at_column
  ret

.handle_left_btn
  mov r1 r15
  call .clear_cursor_at_column
  jz .handle_left_btn_priv_1 r15
  sub r15 1
  mov r1 r15
  call .draw_cursor_at_column
  ret
.handle_left_btn_priv_1
  mov r15 6
  mov r1 r15
  call .draw_cursor_at_column
  ret


.draw_cursor_at_column
  ; parameter: cursor column to draw
  add r1 VGA_BEGIN_ADDR
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
ret

.clear_cursor_at_column
  ; parameter: cursor column to clear
  add r1 VGA_BEGIN_ADDR
  mov r2 0x00
  mov [r1] r2
  add r1 0x0A
  mov [r1] r2
  add r1 0x0A
  mov [r1] r2
  add r1 0x0A
  mov [r1] r2
  add r1 0x0A
  mov [r1] r2
  add r1 0x0A
  mov [r1] r2
  add r1 0x0A
  mov [r1] r2
  add r1 0x0A
  mov [r1] r2
ret


.draw_player_0_disc
  ; parameters: column (r1) and row (r2)
  mov r3 VGA_END_ADDR
  sub r3 9 
  add r3 r1

  ; multiply r2 by 160
  mov r4 r2
  shl r2 7
  shl r4 5
  add r2 r4
  
  sub r3 r2 ;r3 now points to where we want to draw

  mov r1 0x0000
  ;mov [r3] r1
  sub r3 0x0A

  mov r1 0x07E0
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x0FF0
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x1FF8
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x3FFC
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x7FFE
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x7FFE
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x7FFE
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x7FFE
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x7FFE
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x7FFE
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x3FFC
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x1FF8
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x0FF0
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x07E0
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x0000
  ;mov [r3] r1
  sub r3 0x0A
ret

.draw_player_1_disc
  ; parameters: column (r1) and row (r2)
  mov r3 VGA_END_ADDR
  sub r3 9 
  add r3 r1

  ; multiply r2 by 160
  mov r4 r2
  shl r2 7
  shl r4 5
  add r2 r4
  
  sub r3 r2 ;r3 now points to where we want to draw

  mov r1 0x0000
  ;mov [r3] r1
  sub r3 0x0A

  mov r1 0x0540
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x0AA0
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x1550
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x2AA8
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x5554
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x2AAA
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x5554
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x2AAA
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x5554
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x2AAA
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x1554
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x0AA8
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x0550
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x02A0
  mov [r3] r1
  sub r3 0x0A

  mov r1 0x0000
  ;mov [r3] r1
  sub r3 0x0A
ret


.draw_grid_dots
  mov r1 VGA_END_ADDR
  add r1 1
  
  sub r1 10
  push r1
  call .draw_row_dots
  pop r1
  sub r1 150
  push r1
  call .draw_row_dots
  pop r1

  sub r1 10
  push r1
  call .draw_row_dots
  pop r1
  sub r1 150
  push r1
  call .draw_row_dots
  pop r1

  sub r1 10
  push r1
  call .draw_row_dots
  pop r1
  sub r1 150
  push r1
  call .draw_row_dots
  pop r1

  sub r1 10
  push r1
  call .draw_row_dots
  pop r1
  sub r1 150
  push r1
  call .draw_row_dots
  pop r1

  sub r1 10
  push r1
  call .draw_row_dots
  pop r1
  sub r1 150
  push r1
  call .draw_row_dots
  pop r1

  sub r1 10
  push r1
  call .draw_row_dots
  pop r1
  sub r1 150
  push r1
  call .draw_row_dots
  pop r1

ret

.draw_row_dots
  ; parameter: r1 contains the start vga memory address
  
  mov r3 [r1]
  or r3 0x8001
  mov [r1] r3
  add r1 1

  mov r3 [r1]
  or r3 0x8001
  mov [r1] r3
  add r1 1

  mov r3 [r1]
  or r3 0x8001
  mov [r1] r3
  add r1 1

  mov r3 [r1]
  or r3 0x8001
  mov [r1] r3
  add r1 1

  mov r3 [r1]
  or r3 0x8001
  mov [r1] r3
  add r1 1

  mov r3 [r1]
  or r3 0x8001
  mov [r1] r3
  add r1 1

  mov r3 [r1]
  or r3 0x8001
  mov [r1] r3
  add r1 1
ret

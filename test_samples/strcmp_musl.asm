0000000000403d90 <strcmp>:
  403d90:       f3 0f 1e fa             endbr64
  403d94:       0f b6 17                movzbl (%rdi),%edx
  403d97:       0f b6 0e                movzbl (%rsi),%ecx
  403d9a:       b8 01 00 00 00          mov    $0x1,%eax
  403d9f:       38 ca                   cmp    %cl,%dl
  403da1:       74 16                   je     403db9 <strcmp+0x29>
  403da3:       eb 23                   jmp    403dc8 <strcmp+0x38>
  403da5:       0f 1f 00                nopl   (%rax)
  403da8:       0f b6 14 07             movzbl (%rdi,%rax,1),%edx
  403dac:       48 83 c0 01             add    $0x1,%rax
  403db0:       0f b6 4c 06 ff          movzbl -0x1(%rsi,%rax,1),%ecx
  403db5:       38 ca                   cmp    %cl,%dl
  403db7:       75 0f                   jne    403dc8 <strcmp+0x38>
  403db9:       84 d2                   test   %dl,%dl
  403dbb:       75 eb                   jne    403da8 <strcmp+0x18>
  403dbd:       31 c0                   xor    %eax,%eax
  403dbf:       29 c8                   sub    %ecx,%eax
  403dc1:       c3                      ret
  403dc2:       66 0f 1f 44 00 00       nopw   0x0(%rax,%rax,1)
  403dc8:       0f b6 c2                movzbl %dl,%eax
  403dcb:       29 c8                   sub    %ecx,%eax
  403dcd:       c3                      ret
  403dce:       66 90                   xchg   %ax,%ax
